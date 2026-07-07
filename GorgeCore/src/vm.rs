#![allow(dead_code)]

use std::collections::HashMap;
use crate::ir::*;

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

/// Gorge 解释型虚拟机
///
/// 采用类型分离栈设计——int、float、bool、string、object 各使用独立栈。
/// 这种设计避免了类型标记开销，与 C# 参考实现保持一致。
#[derive(Debug, Clone)]
pub struct VirtualMachine {
    pub int_stack: VmStack<i64>,
    pub float_stack: VmStack<f64>,
    pub bool_stack: VmStack<bool>,
    pub string_stack: VmStack<String>,
    /// object_stack 存储对象 ID（在运行时对象表中的索引）
    pub object_stack: VmStack<usize>,

    /// 调用参数池（用于方法调用参数传递和返回值）
    pub param_pool: crate::param_pool::InvokeParameterPool,
    /// 临时对象字段存储（模拟，后续替换为正式对象表）
    /// key: (对象ID, 字段索引)
    field_int_storage: HashMap<(usize, usize), i64>,
    field_float_storage: HashMap<(usize, usize), f64>,
    field_bool_storage: HashMap<(usize, usize), bool>,
    field_string_storage: HashMap<(usize, usize), String>,
    field_object_storage: HashMap<(usize, usize), usize>,

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
            param_pool: crate::param_pool::InvokeParameterPool::new(),
            field_int_storage: HashMap::new(),
            field_float_storage: HashMap::new(),
            field_bool_storage: HashMap::new(),
            field_string_storage: HashMap::new(),
            field_object_storage: HashMap::new(),
            delegate_impls: Vec::new(),
            pc: 0,
            return_int: None,
            return_float: None,
            return_bool: None,
            return_string: None,
            return_object: None,
        }
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
                // TODO: 加载注入器实例到栈，当前简化为 Nop
            }
            IntermediateOperator::SetInjector => {
                // TODO: 将栈顶注入器写入目标对象，当前简化为 Nop
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

            // === 对象字段读取（模拟实现，TODO: 替换为正式对象表） ===
            IntermediateOperator::LoadIntField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.field_int_storage
                    .get(&(obj_id, *field_idx))
                    .copied()
                    .unwrap_or(0);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::LoadFloatField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.field_float_storage
                    .get(&(obj_id, *field_idx))
                    .copied()
                    .unwrap_or(0.0);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::LoadBoolField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.field_bool_storage
                    .get(&(obj_id, *field_idx))
                    .copied()
                    .unwrap_or(false);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::LoadStringField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.field_string_storage
                    .get(&(obj_id, *field_idx))
                    .cloned()
                    .unwrap_or_default();
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::LoadObjectField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.field_object_storage
                    .get(&(obj_id, *field_idx))
                    .copied()
                    .unwrap_or(0);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }

            // === 对象字段写入（模拟实现，TODO: 替换为正式对象表） ===
            IntermediateOperator::SetIntField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_int_operand(&code.left);
                self.field_int_storage.insert((obj_id, *field_idx), val);
            }
            IntermediateOperator::SetFloatField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_float_operand(&code.left);
                self.field_float_storage.insert((obj_id, *field_idx), val);
            }
            IntermediateOperator::SetBoolField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_bool_operand(&code.left);
                self.field_bool_storage.insert((obj_id, *field_idx), val);
            }
            IntermediateOperator::SetStringField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_string_operand(&code.left);
                self.field_string_storage.insert((obj_id, *field_idx), val);
            }
            IntermediateOperator::SetObjectField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_object_operand(&code.left);
                self.field_object_storage.insert((obj_id, *field_idx), val);
            }

            // === 注入器字段读写（TODO: 实现注入器字段访问） ===
            IntermediateOperator::LoadIntInjectorField(_) => {}
            IntermediateOperator::LoadFloatInjectorField(_) => {}
            IntermediateOperator::LoadBoolInjectorField(_) => {}
            IntermediateOperator::LoadStringInjectorField(_) => {}
            IntermediateOperator::LoadObjectInjectorField(_) => {}
            IntermediateOperator::SetIntInjectorField(_) => {}
            IntermediateOperator::SetFloatInjectorField(_) => {}
            IntermediateOperator::SetBoolInjectorField(_) => {}
            IntermediateOperator::SetStringInjectorField(_) => {}
            IntermediateOperator::SetObjectInjectorField(_) => {}

            // === 方法调用（TODO: 实现方法分派，当前简化为 Nop） ===
            IntermediateOperator::InvokeInstance(_) => {}
            IntermediateOperator::InvokeStatic(_) => {}
            IntermediateOperator::InvokeInterface(_) => {}
            IntermediateOperator::InvokeDelegate(idx) => {
                if *idx >= self.delegate_impls.len() {
                    return Err(format!("委托索引 {} 越界", idx));
                }
                let (delegate_method, param_types) = self.delegate_impls[*idx].clone();
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

            // === 构造（TODO: 实现对象构造，当前简化为 Nop） ===
            IntermediateOperator::InvokeConstructor(_) => {}
            IntermediateOperator::DoConstruct(_) => {}

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
}
