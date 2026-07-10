#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;
use crate::ir::*;
use crate::object::{RuntimeObject, GorgeObject};
use crate::types::TypeCount;
use crate::class::{RuntimeClass, GorgeClass};
use crate::injector::{RuntimeInjector, Injector};
use crate::value_pool::FixedFieldValuePool;

/// 值类型的栈
///
/// 每种值类型维护独立的栈容器，同时追踪调用帧边界。
/// `frames` 记录每帧的起始位置，`PushFrame` 分配局部变量空间，
/// `PopFrame` 释放当前帧所有变量。
#[derive(Debug, Clone)]
pub struct VmStack<T: Clone + Default> {
    data: Vec<T>,
    frames: Vec<usize>,
}

impl<T: Clone + Default> VmStack<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            frames: vec![0], // 根帧起始于 0
        }
    }

    /// 推入一个调用帧，分配 `local_count` 个局部变量空间
    pub fn push_frame(&mut self, local_count: usize) {
        let start = self.data.len();
        self.frames.push(start);
        self.data.resize(start + local_count, T::default());
    }

    /// 弹出当前调用帧，释放局部变量空间
    pub fn pop_frame(&mut self) {
        if self.frames.len() <= 1 {
            return; // 保留根帧
        }
        let start = self.frames.pop().unwrap();
        self.data.truncate(start);
    }

    /// 读取地址处的值
    pub fn read(&self, index: usize) -> &T {
        &self.data[index]
    }

    /// 写入地址处的值
    pub fn write(&mut self, index: usize, value: T) {
        if index >= self.data.len() {
            self.data.resize(index + 1, T::default());
        }
        self.data[index] = value;
    }

    /// 栈深度（用于调试）
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 当前帧数量
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}

impl<T: Clone + Default> Default for VmStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 虚拟机执行结果
pub type VmResult<T> = Result<T, String>;

/// 类型分离栈的 Gorge 解释型虚拟机
///
/// 采用类型分离栈设计——int、float、bool、string、object 各使用独立栈。
/// 这种设计避免了类型标记开销，与 C# 参考实现保持一致。
#[derive(Debug)]
pub struct VirtualMachine {
    pub int_stack: VmStack<i64>,
    pub float_stack: VmStack<f64>,
    pub bool_stack: VmStack<bool>,
    pub string_stack: VmStack<String>,
    /// object_stack 存储对象 ID（在运行时对象表中的索引）
    pub object_stack: VmStack<usize>,

    /// 对象 ID 分配器
    next_object_id: usize,

    /// 调用参数池（用于方法调用参数传递和返回值）
    pub param_pool: crate::param_pool::InvokeParameterPool,
    /// 正式对象表：对象ID → RuntimeObject 实例
    pub objects: HashMap<usize, RuntimeObject>,
    /// 注入器对象表：注入器ID → RuntimeInjector 实例
    pub injectors: HashMap<usize, RuntimeInjector>,
    /// 类字段数量注册表：类全名 → 字段类型计数（供 DoConstruct 使用）
    class_field_counts: HashMap<String, TypeCount>,
    /// 类静态字段注册表：类全名 → 静态字段值池
    pub class_static_fields: HashMap<String, FixedFieldValuePool>,
    /// 类委托实现表：类全名 → 委托列表
    class_delegate_impls: HashMap<String, Vec<(CompiledMethod, Vec<ValueType>)>>,
    /// 类注册表：类全名 → RuntimeClass（供方法分派使用）
    pub class_table: HashMap<String, Arc<RuntimeClass>>,

    /// 按类名索引的静态方法表
    class_static_methods: HashMap<String, Vec<(CompiledMethod, Vec<ValueType>)>>,
    /// 当前执行上下文所属的类名
    current_class: String,
    /// 当前调用上下文中的注入器对象 ID（对应 C# InvokeParameterPool.Injector）
    pub current_injector: Option<usize>,

    /// 程序计数器
    pc: usize,
    /// 委托实现表（Lambda 编译产物），含参数类型用于参数传递
    pub delegate_impls: Vec<(CompiledMethod, Vec<ValueType>)>,
    /// 返回值存储（按类型分）
    return_int: Option<i64>,
    return_float: Option<f64>,
    return_bool: Option<bool>,
    return_string: Option<String>,
    return_object: Option<usize>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            int_stack: VmStack::new(),
            float_stack: VmStack::new(),
            bool_stack: VmStack::new(),
            string_stack: VmStack::new(),
            object_stack: VmStack::new(),
            next_object_id: 1, // 0 保留给 null/默认
            param_pool: crate::param_pool::InvokeParameterPool::new(),
            objects: HashMap::new(),
            injectors: HashMap::new(),
            class_field_counts: HashMap::new(),
            class_static_fields: HashMap::new(),
            class_delegate_impls: HashMap::new(),
            class_table: HashMap::new(),
            class_static_methods: HashMap::new(),
            current_class: String::new(),
            current_injector: None,
            delegate_impls: Vec::new(),
            pc: 0,
            return_int: None,
            return_float: None,
            return_bool: None,
            return_string: None,
            return_object: None,
        }
    }

    /// 为类注册静态方法表
    pub fn register_class_methods(
        &mut self,
        class_name: &str,
        methods: Vec<(CompiledMethod, Vec<ValueType>)>,
    ) {
        self.class_static_methods.insert(class_name.to_string(), methods);
    }

    /// 注册类的字段类型计数（供 DoConstruct 使用）
    pub fn register_class_field_counts(&mut self, class_name: &str, counts: TypeCount) {
        self.class_field_counts.insert(class_name.to_string(), counts);
    }

    /// 注册运行时类（供方法分派使用）
    pub fn register_runtime_class(&mut self, class_name: &str, cls: Arc<RuntimeClass>) {
        self.class_table.insert(class_name.to_string(), cls);
    }

    /// 注册类的委托实现（供 InvokeDelegate 按类查找）
    pub fn register_class_delegates(&mut self, class_name: &str, delegates: Vec<(CompiledMethod, Vec<ValueType>)>) {
        self.class_delegate_impls.insert(class_name.to_string(), delegates);
    }

    /// 设置当前执行上下文所属的类名
    pub fn set_current_class(&mut self, class_name: &str) {
        self.current_class = class_name.to_string();
    }

    /// 执行已编译方法的 IR 指令序列
    ///
    /// 调用前需先通过 `push_frame` 分配局部变量空间，调用后手动 `pop_frame` 释放。
    pub fn execute(&mut self, method: &CompiledMethod) -> VmResult<()> {
        self.pc = 0;

        let code_count = method.codes.len();
        while self.pc < code_count {
            let code_span = &method.codes[self.pc];
            let advance = self.execute_one(&code_span.code)?;
            if !advance {
                continue;
            }
            self.pc += 1;
        }

        Ok(())
    }

    /// 推入调用帧
    pub fn push_frame(&mut self, local_count: usize) {
        self.int_stack.push_frame(local_count);
        self.float_stack.push_frame(local_count);
        self.bool_stack.push_frame(local_count);
        self.string_stack.push_frame(local_count);
        self.object_stack.push_frame(local_count);
    }

    /// 弹出调用帧
    pub fn pop_frame(&mut self) {
        self.int_stack.pop_frame();
        self.float_stack.pop_frame();
        self.bool_stack.pop_frame();
        self.string_stack.pop_frame();
        self.object_stack.pop_frame();
    }

    /// 执行单条 IR 指令
    ///
    /// 返回 false 表示 pc 已被控制流指令修改（如 Jump），调用者不应自增 pc。
    fn execute_one(&mut self, code: &IntermediateCode) -> VmResult<bool> {
        match &code.operator {
            // === 本地变量赋值 ===
            IntermediateOperator::IntAssign => {
                let val = self.read_int_operand(&code.left);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::FloatAssign => {
                let val = self.read_float_operand(&code.left);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::BoolAssign => {
                let val = self.read_bool_operand(&code.left);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::StringAssign => {
                let val = self.read_string_operand(&code.left);
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::ObjectAssign => {
                let val = self.read_object_operand(&code.left);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }

            // === this 加载 ===
            IntermediateOperator::LoadThis => {
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, *self.object_stack.read(0));
            }

            // === 参数设置 ===
            IntermediateOperator::SetIntParameter => {
                // 完整实现：从 left 操作数读取值，以 result 地址为参数索引写入参数池
                let val = self.read_int_operand(&code.left);
                let index = self.get_int_addr(code.result);
                self.param_pool.set_int_param(index, val);
            }
            IntermediateOperator::SetFloatParameter => {
                let val = self.read_float_operand(&code.left);
                let index = self.get_int_addr(code.result);
                self.param_pool.set_float_param(index, val);
            }
            IntermediateOperator::SetBoolParameter => {
                let val = self.read_bool_operand(&code.left);
                let index = self.get_int_addr(code.result);
                self.param_pool.set_bool_param(index, val);
            }
            IntermediateOperator::SetStringParameter => {
                let val = self.read_string_operand(&code.left);
                let index = self.get_int_addr(code.result);
                self.param_pool.set_string_param(index, val);
            }
            IntermediateOperator::SetObjectParameter => {
                let val = self.read_object_operand(&code.left);
                let index = self.get_object_addr(code.result);
                self.param_pool.set_object_param(index, val);
            }

            // === 参数加载 ===
            IntermediateOperator::LoadIntParameter => {
                // 完整实现：以 left 操作数为参数索引从参数池读取，写入 result 地址
                let index = self.read_int_operand(&code.left) as usize;
                let val = self.param_pool.get_int_param(index);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::LoadFloatParameter => {
                let index = self.read_int_operand(&code.left) as usize;
                let val = self.param_pool.get_float_param(index);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::LoadBoolParameter => {
                let index = self.read_int_operand(&code.left) as usize;
                let val = self.param_pool.get_bool_param(index);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::LoadStringParameter => {
                let index = self.read_int_operand(&code.left) as usize;
                let val = self.param_pool.get_string_param(index);
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::LoadObjectParameter => {
                let index = self.read_int_operand(&code.left) as usize;
                let val = self.param_pool.get_object_param(index);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }

            // === 注入器 ===
            IntermediateOperator::LoadInjector => {
                // 将当前注入器对象 ID 加载到栈（对应 C# InvokeParameterPool.Injector）
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, self.current_injector.unwrap_or(0));
            }
            IntermediateOperator::SetInjector => {
                // 从 left 操作数读取对象 ID 并设为当前注入器
                self.current_injector = Some(self.read_object_operand(&code.left));
            }

            // === 算术运算 ===
            IntermediateOperator::IntAdd => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, lhs + rhs);
            }
            IntermediateOperator::IntSub => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, lhs - rhs);
            }
            IntermediateOperator::IntMul => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, lhs * rhs);
            }
            IntermediateOperator::IntDiv => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                if rhs == 0 {
                    return Err("除零错误".into());
                }
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, lhs / rhs);
            }
            IntermediateOperator::IntMod => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                if rhs == 0 {
                    return Err("取模除零错误".into());
                }
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, lhs % rhs);
            }

            IntermediateOperator::FloatAdd => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, lhs + rhs);
            }
            IntermediateOperator::FloatSub => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, lhs - rhs);
            }
            IntermediateOperator::FloatMul => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, lhs * rhs);
            }
            IntermediateOperator::FloatDiv => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, lhs / rhs);
            }

            // === 字符串加法 ===
            IntermediateOperator::StringAddition => {
                let lhs = self.read_string_operand(&code.left);
                let rhs = self.read_string_operand(code.right.as_ref().unwrap());
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, lhs + &rhs);
            }

            // === 比较运算 ===
            IntermediateOperator::IntLess => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs < rhs);
            }
            IntermediateOperator::IntLessEqual => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs <= rhs);
            }
            IntermediateOperator::IntGreater => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs > rhs);
            }
            IntermediateOperator::IntGreaterEqual => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs >= rhs);
            }
            IntermediateOperator::IntEqual => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs == rhs);
            }
            IntermediateOperator::IntNotEqual => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs != rhs);
            }

            IntermediateOperator::FloatLess => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs < rhs);
            }
            IntermediateOperator::FloatLessEqual => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs <= rhs);
            }
            IntermediateOperator::FloatGreater => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs > rhs);
            }
            IntermediateOperator::FloatGreaterEqual => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs >= rhs);
            }
            IntermediateOperator::FloatEqual => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs == rhs);
            }
            IntermediateOperator::FloatNotEqual => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs != rhs);
            }

            IntermediateOperator::BoolEqual => {
                let lhs = self.read_bool_operand(&code.left);
                let rhs = self.read_bool_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs == rhs);
            }
            IntermediateOperator::BoolNotEqual => {
                let lhs = self.read_bool_operand(&code.left);
                let rhs = self.read_bool_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs != rhs);
            }

            // === 逻辑运算 ===
            IntermediateOperator::LogicalAnd => {
                let lhs = self.read_bool_operand(&code.left);
                let rhs = self.read_bool_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs && rhs);
            }
            IntermediateOperator::LogicalOr => {
                let lhs = self.read_bool_operand(&code.left);
                let rhs = self.read_bool_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs || rhs);
            }
            IntermediateOperator::LogicalNot => {
                let val = self.read_bool_operand(&code.left);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, !val);
            }

            // === 类型转换 ===
            IntermediateOperator::IntToFloat => {
                let val = self.read_int_operand(&code.left);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val as f64);
            }
            IntermediateOperator::FloatToInt => {
                let val = self.read_float_operand(&code.left);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val as i64);
            }
            IntermediateOperator::IntToBool => {
                let val = self.read_int_operand(&code.left);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val != 0);
            }
            IntermediateOperator::BoolToInt => {
                let val = self.read_bool_operand(&code.left);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, if val { 1 } else { 0 });
            }

            // === 转字符串 ===
            IntermediateOperator::IntCastToString => {
                let val = self.read_int_operand(&code.left);
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val.to_string());
            }
            IntermediateOperator::FloatCastToString => {
                let val = self.read_float_operand(&code.left);
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val.to_string());
            }
            IntermediateOperator::BoolCastToString => {
                let val = self.read_bool_operand(&code.left);
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val.to_string());
            }

            // === 控制流 ===
            IntermediateOperator::Jump(target) => {
                self.pc = *target;
                return Ok(false); // pc 已修改
            }
            IntermediateOperator::JumpIfFalse(target) => {
                let cond = self.read_bool_operand(&code.left);
                if !cond {
                    self.pc = *target;
                    return Ok(false);
                }
            }
            IntermediateOperator::JumpIfTrue(target) => {
                let cond = self.read_bool_operand(&code.left);
                if cond {
                    self.pc = *target;
                    return Ok(false);
                }
            }

            // === 返回值获取 ===
            IntermediateOperator::GetReturnInt => {
                // 完整实现：从参数池获取整数返回值，写入 result 地址
                let val = self.param_pool.get_int_return();
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::GetReturnFloat => {
                let val = self.param_pool.get_float_return();
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::GetReturnBool => {
                let val = self.param_pool.get_bool_return();
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::GetReturnString => {
                let val = self.param_pool.get_string_return();
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::GetReturnObject => {
                let val = self.param_pool.get_object_return();
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }

            // === 构造委托 ===
            IntermediateOperator::ConstructDelegate(idx) => {
                if let Some(ref result) = code.result {
                    self.object_stack.write(result.index, *idx);
                }
            }

            // === 返回 ===
            IntermediateOperator::ReturnInt => {
                let val = self.read_int_operand(&code.left);
                self.return_int = Some(val);
                self.pc = usize::MAX; // 退出循环
                return Ok(false);
            }
            IntermediateOperator::ReturnFloat => {
                let val = self.read_float_operand(&code.left);
                self.return_float = Some(val);
                self.pc = usize::MAX;
                return Ok(false);
            }
            IntermediateOperator::ReturnBool => {
                let val = self.read_bool_operand(&code.left);
                self.return_bool = Some(val);
                self.pc = usize::MAX;
                return Ok(false);
            }
            IntermediateOperator::ReturnString => {
                let val = self.read_string_operand(&code.left);
                self.return_string = Some(val);
                self.pc = usize::MAX;
                return Ok(false);
            }
            IntermediateOperator::ReturnObject => {
                let val = self.read_object_operand(&code.left);
                self.return_object = Some(val);
                self.pc = usize::MAX;
                return Ok(false);
            }
            IntermediateOperator::ReturnVoid => {
                self.pc = usize::MAX;
                return Ok(false);
            }

            // === 对象字段读取 ===
            IntermediateOperator::LoadIntField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.objects
                    .get(&obj_id)
                    .map(|obj| obj.get_int_field(*field_idx))
                    .unwrap_or(0);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::LoadFloatField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.objects
                    .get(&obj_id)
                    .map(|obj| obj.get_float_field(*field_idx))
                    .unwrap_or(0.0);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::LoadBoolField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.objects
                    .get(&obj_id)
                    .map(|obj| obj.get_bool_field(*field_idx))
                    .unwrap_or(false);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::LoadStringField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.objects
                    .get(&obj_id)
                    .map(|obj| obj.get_string_field(*field_idx))
                    .unwrap_or_default();
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::LoadObjectField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.objects
                    .get(&obj_id)
                    .map(|obj| obj.get_object_field(*field_idx))
                    .unwrap_or(0);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }

            // === 对象字段写入 ===
            IntermediateOperator::SetIntField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_int_operand(&code.left);
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_int_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetFloatField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_float_operand(&code.left);
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_float_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetBoolField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_bool_operand(&code.left);
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_bool_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetStringField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_string_operand(&code.left);
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_string_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetObjectField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_object_operand(&code.left);
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_object_field(*field_idx, val);
                }
            }

            // === 静态字段读写 ===
            IntermediateOperator::LoadStaticIntField(field_idx) => {
                let val = self.class_static_fields
                    .get(&self.current_class)
                    .map(|pool| pool.get_int(*field_idx))
                    .unwrap_or(0);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::LoadStaticFloatField(field_idx) => {
                let val = self.class_static_fields
                    .get(&self.current_class)
                    .map(|pool| pool.get_float(*field_idx))
                    .unwrap_or(0.0);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::LoadStaticBoolField(field_idx) => {
                let val = self.class_static_fields
                    .get(&self.current_class)
                    .map(|pool| pool.get_bool(*field_idx))
                    .unwrap_or(false);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::LoadStaticStringField(field_idx) => {
                let val = self.class_static_fields
                    .get(&self.current_class)
                    .map(|pool| pool.get_string(*field_idx).to_string())
                    .unwrap_or_default();
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::LoadStaticObjectField(field_idx) => {
                let val = self.class_static_fields
                    .get(&self.current_class)
                    .map(|pool| pool.get_object(*field_idx))
                    .unwrap_or(0);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }
            IntermediateOperator::SetStaticIntField(field_idx) => {
                let val = self.read_int_operand(&code.left);
                if let Some(pool) = self.class_static_fields.get_mut(&self.current_class) {
                    pool.set_int(*field_idx, val);
                }
            }
            IntermediateOperator::SetStaticFloatField(field_idx) => {
                let val = self.read_float_operand(&code.left);
                if let Some(pool) = self.class_static_fields.get_mut(&self.current_class) {
                    pool.set_float(*field_idx, val);
                }
            }
            IntermediateOperator::SetStaticBoolField(field_idx) => {
                let val = self.read_bool_operand(&code.left);
                if let Some(pool) = self.class_static_fields.get_mut(&self.current_class) {
                    pool.set_bool(*field_idx, val);
                }
            }
            IntermediateOperator::SetStaticStringField(field_idx) => {
                let val = self.read_string_operand(&code.left);
                if let Some(pool) = self.class_static_fields.get_mut(&self.current_class) {
                    pool.set_string(*field_idx, val);
                }
            }
            IntermediateOperator::SetStaticObjectField(field_idx) => {
                let val = self.read_object_operand(&code.left);
                if let Some(pool) = self.class_static_fields.get_mut(&self.current_class) {
                    pool.set_object(*field_idx, val);
                }
            }

            // === 注入器字段读写 ===
            IntermediateOperator::LoadIntInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = if let Some(inj) = self.injectors.get(&inj_id) {
                    if inj.get_injector_int_default_value(*field_idx) {
                        // 默认值：从类级默认值池查找
                        self.lookup_injector_default_int(inj_id, *field_idx).unwrap_or(0)
                    } else {
                        inj.get_injector_int(*field_idx)
                    }
                } else { 0 };
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::LoadFloatInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = if let Some(inj) = self.injectors.get(&inj_id) {
                    if inj.get_injector_float_default_value(*field_idx) { 0.0 }
                    else { inj.get_injector_float(*field_idx) }
                } else { 0.0 };
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::LoadBoolInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = if let Some(inj) = self.injectors.get(&inj_id) {
                    if inj.get_injector_bool_default_value(*field_idx) { false }
                    else { inj.get_injector_bool(*field_idx) }
                } else { false };
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::LoadStringInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = if let Some(inj) = self.injectors.get(&inj_id) {
                    if inj.get_injector_string_default_value(*field_idx) { String::new() }
                    else { inj.get_injector_string(*field_idx) }
                } else { String::new() };
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::LoadObjectInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = if let Some(inj) = self.injectors.get(&inj_id) {
                    if inj.get_injector_object_default_value(*field_idx) { 0 }
                    else { inj.get_injector_object(*field_idx) }
                } else { 0 };
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }
            IntermediateOperator::SetIntInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = self.read_int_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_int(*field_idx, val);
                }
            }
            IntermediateOperator::SetFloatInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = self.read_float_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_float(*field_idx, val);
                }
            }
            IntermediateOperator::SetBoolInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = self.read_bool_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_bool(*field_idx, val);
                }
            }
            IntermediateOperator::SetStringInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = self.read_string_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_string(*field_idx, val);
                }
            }
            IntermediateOperator::SetObjectInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = self.read_object_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_object(*field_idx, val);
                }
            }

            // === 方法调用 ===
            IntermediateOperator::InvokeInstance(method_id) => {
                // 从 left 操作数获取目标对象 ID
                let target_obj_id = self.read_object_operand(&code.left);
                if target_obj_id == 0 {
                    return Err("调用目标对象为空".into());
                }
                // 从对象表获取 RuntimeObject
                let class_name = self.objects
                    .get(&target_obj_id)
                    .map(|obj| obj.class_name.clone())
                    .unwrap_or_default();
                if class_name.is_empty() {
                    return Err("无法确定目标对象的类".into());
                }
                // 查找类的方法实现（找不到则静默跳过）
                let method = match self.class_table
                    .get(&class_name)
                    .and_then(|cls| cls.find_method(*method_id))
                {
                    Some(m) => m,
                    None => return Ok(true),
                };
                // 参数计数从 left 操作数获取（如果 left 是地址则默认 0）
                // 实际参数数量由 codegen 在生成 SetXxxParameter 时确定
                let param_count = match &code.left {
                    Operand::Immediate(im) => match im {
                        crate::ir::ImmediateValue::Int(v) => *v as usize,
                        _ => 0,
                    },
                    _ => 0,
                };
                // 保存栈状态并执行
                let saved_pc = self.pc;
                let save_len = self.int_stack.data.len();
                let saved_ints: Vec<i64> = (0..save_len).map(|i| *self.int_stack.read(i)).collect();
                let saved_floats: Vec<f64> = (0..save_len).map(|i| *self.float_stack.read(i)).collect();
                let saved_bools: Vec<bool> = (0..save_len).map(|i| *self.bool_stack.read(i)).collect();
                let saved_objects: Vec<usize> = (0..save_len).map(|i| *self.object_stack.read(i)).collect();
                let max_locals = method.local_count;
                self.int_stack.write(max_locals.saturating_sub(1), 0);
                self.float_stack.write(max_locals.saturating_sub(1), 0.0);
                self.bool_stack.write(max_locals.saturating_sub(1), false);
                self.string_stack.write(max_locals.saturating_sub(1), String::new());
                self.object_stack.write(max_locals.saturating_sub(1), 0);
                // 将参数从 param_pool 复制到栈指定位置
                for i in 0..param_count {
                    let val = self.param_pool.get_int_param(i);
                    self.int_stack.write(i, val);
                }
                // 将目标对象放在 object_stack[0] 作为 callee 的 this
                self.object_stack.write(0, target_obj_id);
                // 执行子方法
                self.pc = 0;
                let count = method.codes.len();
                while self.pc < count {
                    let cs = &method.codes[self.pc];
                    let advance = self.execute_one(&cs.code)?;
                    if !advance {
                        if self.pc >= count { break; }
                        continue;
                    }
                    self.pc += 1;
                }
                // 将返回值写入 result
                let result_index = code.result.as_ref().map(|r| r.index);
                if let Some(ri) = result_index {
                    if let Some(v) = self.return_int {
                        self.int_stack.write(ri, v);
                    }
                }
                // 恢复调用者栈
                for (i, v) in saved_ints.iter().enumerate() {
                    if Some(i) != result_index {
                        self.int_stack.write(i, *v);
                    }
                }
                for (i, v) in saved_floats.iter().enumerate() {
                    self.float_stack.write(i, *v);
                }
                for (i, v) in saved_bools.iter().enumerate() {
                    self.bool_stack.write(i, *v);
                }
                for (i, v) in saved_objects.iter().enumerate() {
                    if Some(i) != result_index {
                        self.object_stack.write(i, *v);
                    }
                }
                self.pc = saved_pc;
                return Ok(true);
            }
            IntermediateOperator::InvokeStatic(idx) => {
                let methods = self.class_static_methods
                    .get(&self.current_class)
                    .ok_or_else(|| format!("类 `{}` 未注册方法表", self.current_class))?;
                if *idx >= methods.len() {
                    return Err(format!("静态方法索引 {} 越界", idx));
                }
                let (static_method, _param_types) = methods[*idx].clone();
                // left 操作数存参数计数
                let param_count = match &code.left {
                    Operand::Immediate(im) => match im {
                        crate::ir::ImmediateValue::Int(v) => *v as usize,
                        _ => 0,
                    },
                    _ => 0,
                };
                let saved_pc = self.pc;
                let save_len = self.int_stack.data.len();
                let saved_ints: Vec<i64> = (0..save_len).map(|i| *self.int_stack.read(i)).collect();
                let saved_floats: Vec<f64> = (0..save_len).map(|i| *self.float_stack.read(i)).collect();
                let saved_bools: Vec<bool> = (0..save_len).map(|i| *self.bool_stack.read(i)).collect();
                let max_locals = static_method.local_count;
                self.int_stack.write(max_locals.saturating_sub(1), 0);
                self.float_stack.write(max_locals.saturating_sub(1), 0.0);
                self.bool_stack.write(max_locals.saturating_sub(1), false);
                self.string_stack.write(max_locals.saturating_sub(1), String::new());
                self.object_stack.write(max_locals.saturating_sub(1), 0);
                // 将参数从 param_pool 复制到栈（前 param_count 个位置）
                for i in 0..param_count {
                    let val = self.param_pool.get_int_param(i);
                    self.int_stack.write(i, val);
                }
                // 执行子方法
                self.pc = 0;
                let count = static_method.codes.len();
                while self.pc < count {
                    let cs = &static_method.codes[self.pc];
                    let advance = self.execute_one(&cs.code)?;
                    if !advance {
                        if self.pc >= count { break; }
                        continue;
                    }
                    self.pc += 1;
                }
                let result_index = code.result.as_ref().map(|r| r.index);
                if let Some(ri) = result_index {
                    if let Some(v) = self.return_int {
                        self.int_stack.write(ri, v);
                    }
                }
                for (i, v) in saved_ints.iter().enumerate() {
                    if Some(i) != result_index {
                        self.int_stack.write(i, *v);
                    }
                }
                for (i, v) in saved_floats.iter().enumerate() {
                    self.float_stack.write(i, *v);
                }
                for (i, v) in saved_bools.iter().enumerate() {
                    self.bool_stack.write(i, *v);
                }
                self.pc = saved_pc;
                return Ok(true);
            }
            IntermediateOperator::InvokeInterface(_) => {}
            IntermediateOperator::InvokeDelegate(idx) => {
                let delegates = self.class_delegate_impls
                    .get(&self.current_class)
                    .ok_or_else(|| format!("类 `{}` 未注册委托表", self.current_class))?;
                if *idx >= delegates.len() {
                    return Err(format!("类 `{}` 委托索引 {} 越界（共 {} 个）", self.current_class, idx, delegates.len()));
                }
                let (delegate_method, param_types) = delegates[*idx].clone();
                let saved_pc = self.pc;
                let saved_return_int = self.return_int;
                // 保存父帧索引范围的旧值
                let save_len = self.int_stack.data.len();
                let saved_ints: Vec<i64> = (0..save_len).map(|i| *self.int_stack.read(i)).collect();
                let saved_floats: Vec<f64> = (0..save_len).map(|i| *self.float_stack.read(i)).collect();
                let saved_bools: Vec<bool> = (0..save_len).map(|i| *self.bool_stack.read(i)).collect();
                // 确保栈包含委托需要的所有局部变量空间
                let max_locals = delegate_method.local_count;
                self.int_stack.write(max_locals.saturating_sub(1), 0);
                self.float_stack.write(max_locals.saturating_sub(1), 0.0);
                self.bool_stack.write(max_locals.saturating_sub(1), false);
                self.string_stack.write(max_locals.saturating_sub(1), String::new());
                self.object_stack.write(max_locals.saturating_sub(1), 0);
                // 将参数从 param_pool 复制到栈前 max_locals 位置
                for (i, pt) in param_types.iter().enumerate() {
                    match pt {
                        ValueType::Int => {
                            let val = self.param_pool.get_int_param(i);
                            self.int_stack.write(i, val);
                        }
                        ValueType::Float => {
                            let val = self.param_pool.get_float_param(i);
                            self.float_stack.write(i, val);
                        }
                        ValueType::Bool => {
                            let val = self.param_pool.get_bool_param(i);
                            self.bool_stack.write(i, val);
                        }
                        ValueType::String => {
                            let val = self.param_pool.get_string_param(i);
                            self.string_stack.write(i, val);
                        }
                        ValueType::Object => {
                            let val = self.param_pool.get_object_param(i);
                            self.object_stack.write(i, val);
                        }
                    }
                }
                self.pc = 0;
                let count = delegate_method.codes.len();
                while self.pc < count {
                    let cs = &delegate_method.codes[self.pc];
                    let advance = self.execute_one(&cs.code)?;
                    if !advance {
                        if self.pc >= count { break; }
                        continue;
                    }
                    self.pc += 1;
                }
                if let Some(ref r) = code.result {
                    if let Some(v) = self.return_int {
                        self.int_stack.write(r.index, v);
                    }
                }
                // 恢复父帧值（除返回结果位置外）
                let result_index = code.result.as_ref().map(|r| r.index);
                for (i, v) in saved_ints.iter().enumerate() {
                    if Some(i) != result_index {
                        self.int_stack.write(i, *v);
                    }
                }
                for (i, v) in saved_floats.iter().enumerate() {
                    self.float_stack.write(i, *v);
                }
                for (i, v) in saved_bools.iter().enumerate() {
                    self.bool_stack.write(i, *v);
                }
                self.return_int = saved_return_int;
                self.pc = saved_pc;
                return Ok(true);
            }

            // === 构造 ===
            IntermediateOperator::InvokeConstructor(ctor_id) => {
                // 1. 创建对象
                let obj_id = self.next_object_id;
                self.next_object_id += 1;
                let field_counts = self.class_field_counts
                    .get(&self.current_class)
                    .cloned()
                    .unwrap_or_default();
                let obj = RuntimeObject::new_simple(self.current_class.clone(), &field_counts);
                self.objects.insert(obj_id, obj);
                self.object_stack.write(0, obj_id);

                // 2. 写入结果地址
                let result_addr = self.get_object_addr(code.result);
                self.object_stack.write(result_addr, obj_id);

                // 3. 查找并执行构造方法（若无显式构造方法则仅创建对象）
                if let Some(ctor_method) = self.class_table
                    .get(&self.current_class)
                    .and_then(|cls| cls.find_constructor(*ctor_id))
                {
                    let param_count = match &code.left {
                        Operand::Immediate(im) => match im {
                            crate::ir::ImmediateValue::Int(v) => *v as usize,
                            _ => 0,
                        },
                        _ => 0,
                    };
                    let saved_pc = self.pc;
                    let save_len = self.int_stack.data.len();
                    let saved_ints: Vec<i64> = (0..save_len).map(|i| *self.int_stack.read(i)).collect();
                    let saved_floats: Vec<f64> = (0..save_len).map(|i| *self.float_stack.read(i)).collect();
                    let saved_bools: Vec<bool> = (0..save_len).map(|i| *self.bool_stack.read(i)).collect();
                    let saved_objects: Vec<usize> = (0..save_len).map(|i| *self.object_stack.read(i)).collect();
                    let max_locals = ctor_method.local_count;
                    self.int_stack.write(max_locals.saturating_sub(1), 0);
                    self.float_stack.write(max_locals.saturating_sub(1), 0.0);
                    self.bool_stack.write(max_locals.saturating_sub(1), false);
                    self.string_stack.write(max_locals.saturating_sub(1), String::new());
                    self.object_stack.write(max_locals.saturating_sub(1), 0);
                    for i in 0..param_count {
                        let val = self.param_pool.get_int_param(i);
                        self.int_stack.write(i, val);
                    }
                    self.pc = 0;
                    let count = ctor_method.codes.len();
                    while self.pc < count {
                        let cs = &ctor_method.codes[self.pc];
                        let advance = self.execute_one(&cs.code)?;
                        if !advance {
                            if self.pc >= count { break; }
                            continue;
                        }
                        self.pc += 1;
                    }
                    let result_index = code.result.as_ref().map(|r| r.index);
                    for (i, v) in saved_ints.iter().enumerate() {
                        if Some(i) != result_index {
                            self.int_stack.write(i, *v);
                        }
                    }
                    for (i, v) in saved_floats.iter().enumerate() {
                        self.float_stack.write(i, *v);
                    }
                    for (i, v) in saved_bools.iter().enumerate() {
                        self.bool_stack.write(i, *v);
                    }
                    for (i, v) in saved_objects.iter().enumerate() {
                        if Some(i) != result_index {
                            self.object_stack.write(i, *v);
                        }
                    }
                    // 恢复后重新写入结果（避免被 max_locals 准备覆盖）
                    let addr = self.get_object_addr(code.result);
                    self.object_stack.write(addr, obj_id);
                    self.pc = saved_pc;
                }
                return Ok(true);
            }
            IntermediateOperator::DoConstruct(_) => {
                // 创建正式运行时对象
                let obj_id = self.next_object_id;
                self.next_object_id += 1;
                // 从类字段计数表中获取字段数量
                let field_counts = self.class_field_counts
                    .get(&self.current_class)
                    .cloned()
                    .unwrap_or_default();
                let obj = RuntimeObject::new_simple(self.current_class.clone(), &field_counts);
                // 存入对象表，并将 this(=0) 更新为当前对象 ID
                self.objects.insert(obj_id, obj);
                self.object_stack.write(0, obj_id);
                // 返回新对象 ID
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, obj_id);
            }

            // === Nop ===
            IntermediateOperator::Nop => {}

            // 未实现的操作码
            _ => {
                return Err(format!("未实现的操作码: {:?}", code.operator));
            }
        }

        Ok(true) // pc 正常自增
    }

    /// 获取 int 类型的返回值
    pub fn get_return_int(&self) -> Option<i64> {
        self.return_int
    }

    /// 获取 float 类型的返回值
    pub fn get_return_float(&self) -> Option<f64> {
        self.return_float
    }

    /// 获取 bool 类型的返回值
    pub fn get_return_bool(&self) -> Option<bool> {
        self.return_bool
    }

    /// 获取执行后的所有栈数据（调试用）
    pub fn dump_stacks(&self) -> String {
        format!(
            "int: {:?}\nfloat: {:?}\nbool: {:?}",
            self.int_stack.data,
            self.float_stack.data,
            self.bool_stack.data,
        )
    }

    /// 从类级注入器默认值池查找整数默认值
    fn lookup_injector_default_int(&self, inj_id: usize, field_idx: usize) -> Option<i64> {
        let obj = self.objects.get(&inj_id)?;
        let cls = self.class_table.get(&obj.class_name)?;
        Some(cls.get_injector_int_default(field_idx))
    }

    // === 操作数读取辅助方法 ===

    fn read_int_operand(&self, op: &Operand) -> i64 {
        match op {
            Operand::Immediate(ImmediateValue::Int(v)) => *v,
            Operand::Address(addr) => *self.int_stack.read(addr.index),
            _ => 0,
        }
    }

    fn read_float_operand(&self, op: &Operand) -> f64 {
        match op {
            Operand::Immediate(ImmediateValue::Float(v)) => *v,
            Operand::Address(addr) => *self.float_stack.read(addr.index),
            _ => 0.0,
        }
    }

    fn read_bool_operand(&self, op: &Operand) -> bool {
        match op {
            Operand::Immediate(ImmediateValue::Bool(v)) => *v,
            Operand::Address(addr) => *self.bool_stack.read(addr.index),
            _ => false,
        }
    }

    fn read_string_operand(&self, op: &Operand) -> String {
        match op {
            Operand::Immediate(ImmediateValue::String(s)) => s.clone(),
            Operand::Address(addr) => self.string_stack.read(addr.index).clone(),
            _ => String::new(),
        }
    }

    fn read_object_operand(&self, op: &Operand) -> usize {
        match op {
            Operand::Address(addr) => *self.object_stack.read(addr.index),
            _ => 0,
        }
    }

    fn get_int_addr(&self, addr: Option<Address>) -> usize {
        addr.map(|a| a.index).unwrap_or(0)
    }

    fn get_float_addr(&self, addr: Option<Address>) -> usize {
        addr.map(|a| a.index).unwrap_or(0)
    }

    fn get_bool_addr(&self, addr: Option<Address>) -> usize {
        addr.map(|a| a.index).unwrap_or(0)
    }

    fn get_string_addr(&self, addr: Option<Address>) -> usize {
        addr.map(|a| a.index).unwrap_or(0)
    }

    fn get_object_addr(&self, addr: Option<Address>) -> usize {
        addr.map(|a| a.index).unwrap_or(0)
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for VirtualMachine {
    fn clone(&self) -> Self {
        Self {
            int_stack: self.int_stack.clone(),
            float_stack: self.float_stack.clone(),
            bool_stack: self.bool_stack.clone(),
            string_stack: self.string_stack.clone(),
            object_stack: self.object_stack.clone(),
            next_object_id: self.next_object_id,
            param_pool: self.param_pool.clone(),
            objects: HashMap::new(),
            injectors: HashMap::new(),
            class_field_counts: self.class_field_counts.clone(),
            class_static_fields: self.class_static_fields.clone(),
            class_delegate_impls: self.class_delegate_impls.clone(),
            class_table: self.class_table.clone(),
            class_static_methods: self.class_static_methods.clone(),
            current_class: self.current_class.clone(),
            current_injector: self.current_injector,
            delegate_impls: self.delegate_impls.clone(),
            pc: self.pc,
            return_int: self.return_int,
            return_float: self.return_float,
            return_bool: self.return_bool,
            return_string: self.return_string.clone(),
            return_object: self.return_object,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_int_addr(index: usize) -> Address {
        Address::new(ValueType::Int, index)
    }

    fn make_bool_addr(index: usize) -> Address {
        Address::new(ValueType::Bool, index)
    }

    fn make_float_addr(index: usize) -> Address {
        Address::new(ValueType::Float, index)
    }

    #[test]
    fn test_vm_stack_push_pop() {
        let mut stack: VmStack<i64> = VmStack::new();
        stack.push_frame(3);
        stack.write(0, 10);
        stack.write(1, 20);
        stack.write(2, 30);
        assert_eq!(*stack.read(0), 10);
        assert_eq!(*stack.read(1), 20);
        assert_eq!(stack.len(), 3);

        stack.pop_frame();
        assert_eq!(stack.len(), 0); // 退回根帧
    }

    #[test]
    fn test_execute_int_add() {
        let a = make_int_addr(0);
        let b = make_int_addr(1);
        let r = make_int_addr(2);

        let codes = vec![
            CodeWithSpan::new(
                IntermediateCode::assign(a, Operand::int(1)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::assign(b, Operand::int(2)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::binary(
                    IntermediateOperator::IntAdd,
                    Operand::Address(a),
                    Operand::Address(b),
                    r,
                ),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let method = CompiledMethod {
            name: "test".into(),
            codes,
            local_count: 3,
        };

        let mut vm = VirtualMachine::new();
        vm.push_frame(method.local_count);
        vm.execute(&method).unwrap();
        assert_eq!(*vm.int_stack.read(2), 3);
    }

    #[test]
    fn test_execute_return_int() {
        let r = make_int_addr(0);

        let codes = vec![
            CodeWithSpan::new(
                IntermediateCode::assign(r, Operand::int(42)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::return_value(ValueType::Int),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let method = CompiledMethod {
            name: "test".into(),
            codes,
            local_count: 1,
        };

        let mut vm = VirtualMachine::new();
        vm.push_frame(method.local_count);
        vm.execute(&method).unwrap();
        assert_eq!(vm.get_return_int(), Some(42));
    }

    #[test]
    fn test_execute_comparison() {
        let a = make_int_addr(0);
        let b = make_int_addr(1);
        let r = make_bool_addr(0);

        // 先初始化 a=5, b=10，然后比较
        let codes = vec![
            CodeWithSpan::new(
                IntermediateCode::assign(a, Operand::int(5)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::assign(b, Operand::int(10)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::binary(
                    IntermediateOperator::IntLess,
                    Operand::Address(a),
                    Operand::Address(b),
                    r,
                ),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let method = CompiledMethod {
            name: "test".into(),
            codes,
            local_count: 3,
        };

        let mut vm = VirtualMachine::new();
        vm.push_frame(method.local_count);
        vm.execute(&method).unwrap();
        assert_eq!(*vm.bool_stack.read(0), true);
    }

    #[test]
    fn test_execute_jump_if_false() {
        let cond = make_bool_addr(0);

        // cond = false, 如果 !cond 则跳转到 end
        let codes = vec![
            CodeWithSpan::new(
                IntermediateCode::assign(cond, Operand::boolean(false)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::jump_if_false(Operand::Address(cond), 3),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::assign(make_int_addr(1), Operand::int(1)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::nop(),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let method = CompiledMethod {
            name: "test".into(),
            codes,
            local_count: 2,
        };

        let mut vm = VirtualMachine::new();
        vm.push_frame(method.local_count);
        vm.execute(&method).unwrap();
        assert_eq!(vm.int_stack.len(), 2);
    }

    #[test]
    fn test_vm_arithmetic_sequence() {
        let x = make_int_addr(0);
        let y = make_int_addr(1);
        let tmp = make_int_addr(2);
        let result = make_int_addr(3);

        // result = x * 2 + y
        let codes = vec![
            CodeWithSpan::new(
                IntermediateCode::assign(x, Operand::int(3)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::assign(y, Operand::int(4)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::binary(
                    IntermediateOperator::IntMul,
                    Operand::Address(x),
                    Operand::int(2),
                    tmp,
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::binary(
                    IntermediateOperator::IntAdd,
                    Operand::Address(tmp),
                    Operand::Address(y),
                    result,
                ),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let method = CompiledMethod {
            name: "compute".into(),
            codes,
            local_count: 4,
        };

        let mut vm = VirtualMachine::new();
        vm.push_frame(method.local_count);
        vm.execute(&method).unwrap();
        assert_eq!(*vm.int_stack.read(3), 10); // 3*2 + 4 = 10
    }

    #[test]
    fn test_invoke_instance_method() {
        use std::sync::Arc;
        use crate::class::RuntimeClass;
        use crate::declaration::ClassDeclaration;
        use crate::types::GorgeType;

        // 构造一个简单的 RuntimeMethod，返回 int 42
        let callee_method = CompiledMethod {
            name: "getValue".into(),
            codes: vec![
                CodeWithSpan::new(
                    IntermediateCode::assign(
                        Address::new(ValueType::Int, 0),
                        Operand::int(42),
                    ),
                    crate::diagnostics::Span::dummy(),
                ),
                CodeWithSpan::new(
                    IntermediateCode::return_value(ValueType::Int),
                    crate::diagnostics::Span::dummy(),
                ),
            ],
            local_count: 1,
        };

        let decl = ClassDeclaration {
            class_type: GorgeType::class("Widget", None),
            is_native: false,
            annotations: vec![],
            fields: vec![],
            methods: vec![],
            static_methods: vec![],
            constructors: vec![],
            injector_fields: vec![],
            super_class: None,
            super_interfaces: vec![],
            field_type_count: TypeCount::zero(),
            method_count: 1,
            static_method_count: 0,
            constructor_count: 0,
            injector_field_type_count: TypeCount::zero(),
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0,
            constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
        };
        let mut cls = RuntimeClass::new(decl, None);
        cls.register_method(0, callee_method);

        let cls_arc = Arc::new(cls);

        // 创建 VM 并注册类
        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("Widget", cls_arc.clone());
        vm.register_class_field_counts("Widget", TypeCount::zero());

        // 创建对象并存入对象表
        let obj_id = vm.next_object_id;
        vm.next_object_id += 1;
        let obj = RuntimeObject::new_simple("Widget".into(), &TypeCount::zero());
        vm.objects.insert(obj_id, obj);
        vm.object_stack.push_frame(3);
        vm.int_stack.push_frame(3);
        vm.float_stack.push_frame(3);
        vm.bool_stack.push_frame(3);
        vm.string_stack.push_frame(3);

        // 调用 InvokeInstance(method_id=0)
        let result_addr = Address::new(ValueType::Int, 1);
        let invoke_code = IntermediateCode::new(
            IntermediateOperator::InvokeInstance(0),
            Operand::Address(Address::new(ValueType::Object, 0)), // target object 在 slot 0
            None,
            Some(result_addr),
        );

        vm.object_stack.write(0, obj_id); // this = 目标对象
        let _advance = vm.execute_one(&invoke_code).unwrap();

        // 返回值写入 result_addr
        assert_eq!(*vm.int_stack.read(1), 42);
    }

    #[test]
    fn test_invoke_constructor() {
        use std::sync::Arc;
        use crate::class::RuntimeClass;
        use crate::declaration::ClassDeclaration;
        use crate::types::GorgeType;

        // 构造方法：将参数 a 存入 this.counter (field 0)
        let ctor_codes = vec![
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::LoadIntParameter,
                    Operand::int(0),
                    None,
                    Some(Address::new(ValueType::Int, 0)),
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::SetIntField(0),
                    Operand::Address(Address::new(ValueType::Int, 0)),
                    None,
                    None,
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::return_void(),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let ctor = CompiledMethod { name: "Widget".into(), codes: ctor_codes, local_count: 2 };
        let field_counts = TypeCount { int_count: 1, ..TypeCount::zero() };

        let decl = ClassDeclaration {
            class_type: GorgeType::class("Widget", None),
            is_native: false, annotations: vec![], fields: vec![],
            methods: vec![], static_methods: vec![],
            constructors: vec![], injector_fields: vec![],
            super_class: None, super_interfaces: vec![],
            field_type_count: field_counts.clone(),
            method_count: 0, static_method_count: 0,
            constructor_count: 1,
            injector_field_type_count: TypeCount::zero(),
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0, constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
        };
        let mut cls = RuntimeClass::new(decl, None);
        cls.register_constructor(0, ctor);

        let cls_arc = Arc::new(cls);

        let mut vm = VirtualMachine::new();
        vm.int_stack.push_frame(4);
        vm.float_stack.push_frame(4);
        vm.bool_stack.push_frame(4);
        vm.string_stack.push_frame(4);
        vm.object_stack.push_frame(4);

        vm.register_runtime_class("Widget", cls_arc.clone());
        vm.register_class_field_counts("Widget", field_counts);
        vm.set_current_class("Widget");

        // 设置构造参数
        vm.param_pool.set_int_param(0, 99);

        // InvokeConstructor(0), 1 个参数, result → object addr 1
        let result_obj = Address::new(ValueType::Object, 1);
        let invoke_ctor = IntermediateCode::new(
            IntermediateOperator::InvokeConstructor(0),
            Operand::int(1),
            None,
            Some(result_obj),
        );

        let _advance = vm.execute_one(&invoke_ctor).unwrap();

        // 结果放在 object_stack[1]
        let obj_id = *vm.object_stack.read(1);
        assert_ne!(obj_id, 0, "应创建非空对象: obj_id={}", obj_id);
        let obj = vm.objects.get(&obj_id).expect("对象表中应有新对象");
        assert_eq!(obj.get_int_field(0), 99, "counter 应为 99");
    }

    #[test]
    fn test_set_and_load_injector() {
        let mut vm = VirtualMachine::new();
        vm.object_stack.push_frame(4);

        // 创建一个 RuntimeObject 模拟注入器对象
        let inj_id = 42;
        let obj = RuntimeObject::new_simple("Injector".into(), &TypeCount::zero());
        vm.objects.insert(inj_id, obj);

        // SetInjector: 将对象 42 设为当前注入器
        let set_code = IntermediateCode::new(
            IntermediateOperator::SetInjector,
            Operand::Address(Address::new(ValueType::Object, 0)), // 读取 object_stack[0]
            None,
            None,
        );
        vm.object_stack.write(0, inj_id);
        let _ = vm.execute_one(&set_code).unwrap();
        assert_eq!(vm.current_injector, Some(42), "SetInjector 应设置注入器 ID");

        // LoadInjector: 加载注入器到结果地址
        let result_addr = Address::new(ValueType::Object, 2);
        let load_code = IntermediateCode::new(
            IntermediateOperator::LoadInjector,
            Operand::int(0),
            None,
            Some(result_addr),
        );
        let _ = vm.execute_one(&load_code).unwrap();
        assert_eq!(*vm.object_stack.read(2), 42, "LoadInjector 应加载注入器 ID");
    }

    #[test]
    fn test_injector_field_set_and_load() {
        use std::sync::Arc;
        use crate::class::RuntimeClass;
        use crate::declaration::ClassDeclaration;
        use crate::types::GorgeType;

        // 创建带注入器字段的类
        let field_counts = TypeCount { int_count: 1, ..TypeCount::zero() };
        let decl = ClassDeclaration {
            class_type: GorgeType::class("Scene", None),
            is_native: false, annotations: vec![], fields: vec![],
            methods: vec![], static_methods: vec![],
            constructors: vec![], injector_fields: vec![],
            super_class: None, super_interfaces: vec![],
            field_type_count: field_counts.clone(),
            method_count: 0, static_method_count: 0, constructor_count: 0,
            injector_field_type_count: TypeCount { int_count: 2, ..TypeCount::zero() },
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0, constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(), method_override_id: HashMap::new(),
        };
        let cls = Arc::new(RuntimeClass::new(decl, None));

        // 创建注入器，含 2 个 int 字段
        let inj = RuntimeInjector::new(Arc::new(cls.declaration().clone()));
        let inj_id = 100;
        let mut vm = VirtualMachine::new();
        vm.int_stack.push_frame(4);
        vm.object_stack.push_frame(4);

        // 将注入器存入 VM
        vm.injectors.insert(inj_id, inj);
        // 在 object_stack[0] 设为当前注入器
        vm.object_stack.write(0, inj_id);

        // SetIntInjectorField(0, 77)
        let set_code = IntermediateCode::new(
            IntermediateOperator::SetIntInjectorField(0),
            Operand::int(77),
            None, None,
        );
        let _ = vm.execute_one(&set_code).unwrap();

        // LoadIntInjectorField(0) → result int[1]
        let result = Address::new(ValueType::Int, 1);
        let load_code = IntermediateCode::new(
            IntermediateOperator::LoadIntInjectorField(0),
            Operand::int(0),
            None,
            Some(result),
        );
        let _ = vm.execute_one(&load_code).unwrap();
        assert_eq!(*vm.int_stack.read(1), 77, "注入器字段值应为 77");

        // 验证默认值标记：未设置的字段 1 应返回默认值
        let load_default = IntermediateCode::new(
            IntermediateOperator::LoadIntInjectorField(1),
            Operand::int(0),
            None,
            Some(Address::new(ValueType::Int, 2)),
        );
        let _ = vm.execute_one(&load_default).unwrap();
        assert_eq!(*vm.int_stack.read(2), 0, "未设置的注入器字段默认值应为 0");
    }

    #[test]
    fn test_static_field_set_and_load() {
        let mut vm = VirtualMachine::new();
        vm.int_stack.push_frame(4);
        vm.string_stack.push_frame(4);
        vm.set_current_class("Config");

        // 注册静态字段池
        let counts = TypeCount { int_count: 1, string_count: 1, ..TypeCount::zero() };
        vm.class_static_fields.insert("Config".into(), FixedFieldValuePool::new(&counts));

        // SetStaticIntField(0, 100)
        let set_int = IntermediateCode::new(
            IntermediateOperator::SetStaticIntField(0),
            Operand::int(100),
            None, None,
        );
        let _ = vm.execute_one(&set_int).unwrap();

        // LoadStaticIntField(0) → result
        let ri = Address::new(ValueType::Int, 1);
        let load_int = IntermediateCode::new(
            IntermediateOperator::LoadStaticIntField(0),
            Operand::int(0), None, Some(ri),
        );
        let _ = vm.execute_one(&load_int).unwrap();
        assert_eq!(*vm.int_stack.read(1), 100, "静态 int 字段应为 100");

        // SetStaticStringField(0, "hello")
        let set_str = IntermediateCode::new(
            IntermediateOperator::SetStaticStringField(0),
            Operand::string("hello"),
            None, None,
        );
        let _ = vm.execute_one(&set_str).unwrap();

        // LoadStaticStringField(0)
        let rs = Address::new(ValueType::String, 0);
        let load_str = IntermediateCode::new(
            IntermediateOperator::LoadStaticStringField(0),
            Operand::int(0), None, Some(rs),
        );
        let _ = vm.execute_one(&load_str).unwrap();
        assert_eq!(vm.string_stack.read(0), "hello");
    }
}
