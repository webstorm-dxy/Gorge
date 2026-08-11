#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use crate::virtual_machine::ir::*;
use crate::objective::object::{RuntimeObject, GorgeObject};
use crate::objective::types::TypeCount;
use crate::objective::class::{RuntimeClass, GorgeClass};
use crate::system::native::injector::{RuntimeInjector, Injector};
use crate::objective::value_pool::FixedFieldValuePool;
use crate::objective::bytecode::CompiledFieldInitializer;
use crate::system::native::list::*;

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

/// 统一方法调用时的参数布置模式（1a）
///
/// 覆盖 8 处内联调用点的参数传递差异。
#[derive(Debug, Clone)]
pub enum ParamMode {
    /// 无参数（字段初始化器）
    None,
    /// 批量复制：使用 copy_params_to_locals 按类型分组连续复制已设置参数
    Batch,
    /// 按值类型逐一复制：用于委托调用，param_types[i] 指示参数池中第 i 个槽的类型
    ByType(Vec<ValueType>),
    /// 按显式计数复制：每种类型各复制 count 项（super 构造参数传递）
    ByCount(usize),
}

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
    pub next_object_id: usize,

    /// 调用参数池（用于方法调用参数传递和返回值）
    pub param_pool: crate::virtual_machine::param_pool::InvokeParameterPool,
    /// 正式对象表：对象ID → RuntimeObject 实例
    pub objects: HashMap<usize, RuntimeObject>,
    /// 注入器对象表：注入器ID → RuntimeInjector 实例
    pub injectors: HashMap<usize, RuntimeInjector>,
    /// 类字段数量注册表：类全名 → 字段类型计数（供 DoConstruct 使用）
    class_field_counts: HashMap<String, TypeCount>,
    /// 类字段初始化器注册表（Phase P）：类全名 → 字段初始化器列表
    /// 对齐 C# CompiledGorgeClass.FieldInitializerImplementations
    pub class_field_initializers: HashMap<String, Vec<CompiledFieldInitializer>>,
    /// 类静态字段注册表：类全名 → 静态字段值池
    pub class_static_fields: HashMap<String, FixedFieldValuePool>,
    /// 类委托实现表：类全名 → 委托列表（方法体, 自身参数类型, 返回类型, 捕获变量类型）
    class_delegate_impls: HashMap<String, Vec<(CompiledMethod, Vec<ValueType>, ValueType, Vec<ValueType>)>>,
    /// 运行时委托表（1b）：对象 ID → RuntimeDelegate
    /// ConstructDelegate 操作码创建运行时委托后存入此表
    pub runtime_delegates: HashMap<usize, crate::objective::delegate::RuntimeDelegate>,
    /// 类注册表：类全名 → RuntimeClass（供方法分派使用）
    pub class_table: HashMap<String, Arc<RuntimeClass>>,

    /// 类 ID → 类全名映射（供 DoConstruct 按 class_id 查找类名）
    pub class_name_by_id: Vec<String>,

    /// Native 类注册表：类全名 → NativeClass（供 native 方法/构造分派）
    pub native_class_table: HashMap<String, Arc<dyn crate::objective::native::NativeClass>>,
    /// Native 对象载荷表：对象 ID → 类型化的 Rust 数据
    pub native_payloads: HashMap<usize, Box<dyn std::any::Any>>,
    /// 类 → 父类名映射（供跨 native/compiled 边界的祖先查找，F2）
    pub class_super_name: HashMap<String, String>,
    /// 注入器常量池（G2）：由 runner 反序列化后注册，运行时通过 LoadInjectorConstant 访问
    pub injector_constants: Vec<crate::objective::bytecode::InjectorConstantDef>,

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
    pub return_int: Option<i64>,
    pub return_float: Option<f64>,
    pub return_bool: Option<bool>,
    pub return_string: Option<String>,
    pub return_object: Option<usize>,
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
            param_pool: crate::virtual_machine::param_pool::InvokeParameterPool::new(),
            objects: HashMap::new(),
            injectors: HashMap::new(),
            class_field_counts: HashMap::new(),
            class_field_initializers: HashMap::new(),
            class_static_fields: HashMap::new(),
            class_delegate_impls: HashMap::new(),
            runtime_delegates: HashMap::new(),
            class_table: HashMap::new(),
            class_name_by_id: Vec::new(),
            native_class_table: HashMap::new(),
            native_payloads: HashMap::new(),
            class_super_name: HashMap::new(),
            injector_constants: Vec::new(),
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

    /// 注册类字段初始化器（Phase P）
    pub fn register_class_field_initializers(&mut self, class_name: &str, initals: Vec<CompiledFieldInitializer>) {
        self.class_field_initializers.insert(class_name.to_string(), initals);
    }

    /// 注册运行时类（供方法分派使用）
    pub fn register_runtime_class(&mut self, class_name: &str, cls: Arc<RuntimeClass>) {
        self.class_table.insert(class_name.to_string(), cls);
    }

    /// 按 class_id 注册类名（供 DoConstruct 按 ID 查找类名）
    pub fn register_class_name_by_id(&mut self, class_name: &str) {
        self.class_name_by_id.push(class_name.to_string());
    }

    /// 注册 native 类（供 native 方法/构造分派使用）
    pub fn register_native_class(
        &mut self,
        class_name: &str,
        cls: Arc<dyn crate::objective::native::NativeClass>,
    ) {
        // 编译期部分路径仍使用短类名，而 native 调用与运行时对象保存完整类名。
        // 两种名称必须指向同一实现，否则全限定的 native 调用会被静默跳过。
        let full_name = cls.full_name().to_string();
        let simple_name = full_name.rsplit('.').next().unwrap_or(&full_name).to_string();
        self.native_class_table.insert(class_name.to_string(), cls.clone());
        self.native_class_table.insert(full_name, cls.clone());
        self.native_class_table.insert(simple_name, cls);
    }

    /// 注册类的委托实现（供 InvokeDelegate 按类查找，含返回类型与捕获变量类型）
    pub fn register_class_delegates(&mut self, class_name: &str, delegates: Vec<(CompiledMethod, Vec<ValueType>, ValueType, Vec<ValueType>)>) {
        self.class_delegate_impls.insert(class_name.to_string(), delegates);
    }

    /// 设置当前执行上下文所属的类名
    pub fn set_current_class(&mut self, class_name: &str) {
        self.current_class = class_name.to_string();
    }

    // ==================== Native 分派 ====================

    /// 把调用参数池中的参数按值类型分组复制到 callee 的局部槽位。
    ///
    /// 参数按值类型分组存储（B-2），方法参数占据每种类型最低的连续局部索引。
    /// 对每种类型从 0 起连续复制「已设置」的参数，遇首个未设置即停，`bound` 为上界。
    fn copy_params_to_locals(&mut self, bound: usize) {
        // 逐类型复制（借用 param_pool 的各数组判断 is_set）
        {
            let mut j = 0;
            while j < bound && self.param_pool.int_params.borrow()[j].is_set {
                let v = self.param_pool.get_int_param(j);
                self.int_stack.write(j, v);
                j += 1;
            }
        }
        {
            let mut j = 0;
            while j < bound && self.param_pool.float_params.borrow()[j].is_set {
                let v = self.param_pool.get_float_param(j);
                self.float_stack.write(j, v);
                j += 1;
            }
        }
        {
            let mut j = 0;
            while j < bound && self.param_pool.bool_params.borrow()[j].is_set {
                let v = self.param_pool.get_bool_param(j);
                self.bool_stack.write(j, v);
                j += 1;
            }
        }
        {
            let mut j = 0;
            while j < bound && self.param_pool.string_params.borrow()[j].is_set {
                let v = self.param_pool.get_string_param(j);
                self.string_stack.write(j, v);
                j += 1;
            }
        }
        {
            let mut j = 0;
            while j < bound && self.param_pool.object_params.borrow()[j].is_set {
                let v = self.param_pool.get_object_param(j);
                // 局部 object 槽 0 固定保留给 this（codegen 的
                // next_local 对 Object 从 1 起分配），参数从 1 开始布置
                self.object_stack.write(j + 1, v);
                j += 1;
            }
        }
        // 注意：不重置参数池——编译方法可能通过 LoadParameter 再次读取参数池。
    }


    /// 沿编译类的 super_class 链查找最近的 native 祖先类名（F2）
    ///
    /// 用于编译子类调用继承自 native 父类的方法时定位 native 实现。
    /// 通过 `class_super_name` 映射逐级上溯（含 native 父类，class_table 不含 native）。
    fn find_native_ancestor(&self, class_name: &str) -> Option<String> {
        let mut current = self.class_super_name.get(class_name).cloned();
        let mut guard = 0;
        while let Some(name) = current {
            if self.native_class_table.contains_key(&name) {
                return Some(name);
            }
            current = self.class_super_name.get(&name).cloned();
            guard += 1;
            if guard > 1000 { break; }
        }
        None
    }

    /// 注册类的父类名（供 F2 跨边界祖先查找）
    pub fn register_class_super(&mut self, class_name: &str, super_name: &str) {
        self.class_super_name.insert(class_name.to_string(), super_name.to_string());
    }

    /// 分派 native 静态方法：参数已由 SetXxxParameter 写入参数池，返回值写回参数池。
    fn dispatch_native_static(&mut self, class_name: &str, method_id: usize) {
        if let Some(cls) = self.native_class_table.get(class_name).cloned() {
            let mut ctx = crate::objective::native::NativeContext::new(self);
            cls.invoke_native_static(&mut ctx, method_id);
        }
    }

    /// 构造 native 上下文并分派实例方法
    fn dispatch_native_method(&mut self, class_name: &str, obj_id: usize, method_id: usize) {
        if let Some(cls) = self.native_class_table.get(class_name).cloned() {
            // 解析真实 native 对象 ID：若 obj_id 是编译层的包装对象，
            // 则取其 native_object_id；否则直接使用 obj_id
            let real_native_id = self.objects
                .get(&obj_id)
                .and_then(|o| o.native_object_id)
                .unwrap_or(obj_id);
            let mut ctx = crate::objective::native::NativeContext::new(self);
            cls.invoke_native_method(&mut ctx, real_native_id, method_id);
        }
    }

    /// 构造 native 上下文并分派构造方法，返回新对象 ID
    ///
    /// 注入器专用位在调用前应已由 SetInjector 逻辑写入参数池。
    fn dispatch_native_construct(
        &mut self,
        class_name: &str,
        target: Option<usize>,
        ctor_id: usize,
    ) -> usize {
        if let Some(cls) = self.native_class_table.get(class_name).cloned() {
            // 编译产物使用继承空间中的全局构造 ID，native 分派表按本类从 0 编号。
            let native_ctor_id = self.class_table.get(class_name)
                .and_then(|runtime_class| {
                    ctor_id.checked_sub(runtime_class.declaration.constructor_start_id)
                })
                .unwrap_or(ctor_id);
            let is_new = target.is_none();
            let current_inj = self.current_injector.unwrap_or(0);
            let class_name_owned = class_name.to_string();
            let obj_id = {
                let mut ctx = crate::objective::native::NativeContext::with_injector(self, current_inj);
                cls.do_construct_native(&mut ctx, target, native_ctor_id)
            };
            // ctx 已销毁，self 可再次借用
            if is_new {
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.class_name = class_name_owned;
                }
            }
            obj_id
        } else {
            0
        }
    }

    /// 把 native 方法写入参数池的返回值按结果地址类型落到对应栈
    ///
    /// native 桥接层将返回值写入参数池的返回位，本方法据结果地址的值类型
    /// 取出并写入相应类型栈。
    fn write_native_return_to_result(&mut self, result: Option<&Address>) {
        if let Some(addr) = result {
            match addr.value_type {
                ValueType::Int => {
                    let v = self.param_pool.get_int_return();
                    self.int_stack.write(addr.index, v);
                }
                ValueType::Float => {
                    let v = self.param_pool.get_float_return();
                    self.float_stack.write(addr.index, v);
                }
                ValueType::Bool => {
                    let v = self.param_pool.get_bool_return();
                    self.bool_stack.write(addr.index, v);
                }
                ValueType::String => {
                    let v = self.param_pool.get_string_return();
                    self.string_stack.write(addr.index, v);
                }
                ValueType::Object => {
                    let v = self.param_pool.get_object_return();
                    self.object_stack.write(addr.index, v);
                }
            }
        }
    }

    /// 建立编译对象与其 native 子对象之间的双向引用
    ///
    /// 对应 C# 中 `CompiledGorgeObject.NativeObject` 与 native 对象
    /// `OuterCompiledObject` 的互指。用于编译类继承 native 类的场景：
    /// - `compiled_id`：外层编译对象 ID
    /// - `native_id`：内层 native 子对象 ID
    ///
    /// 建立后：
    /// - 编译对象的 `native_object_id` = native_id
    /// - native 对象的 `outer_compiled_id` = compiled_id
    ///
    /// 注意：本机制的真正触发依赖“编译类继承 native 类”，需要编译器实现继承
    /// 编号冻结（Backlog B-3）与 native 类导入（Backlog B-4）后才会在端到端流程中
    /// 被使用；当前通过单元测试直接构造验证。
    pub fn link_native_and_compiled(&mut self, compiled_id: usize, native_id: usize) {
        if let Some(compiled) = self.objects.get_mut(&compiled_id) {
            compiled.native_object_id = Some(native_id);
        }
        if let Some(native) = self.objects.get_mut(&native_id) {
            native.outer_compiled_id = Some(compiled_id);
        }
    }

    /// 解析对象的“真实对象” ID（对应 C# `GorgeObject.RealObject`）
    ///
    /// 若对象是被编译类包裹的 native 子对象（存在 `outer_compiled_id`），
    /// 返回外层编译对象 ID；否则返回自身 ID。
    pub fn resolve_real_object_id(&self, obj_id: usize) -> usize {
        self.objects
            .get(&obj_id)
            .and_then(|o| o.outer_compiled_id)
            .unwrap_or(obj_id)
    }

    /// 从指令的 right 操作数解析目标类名（跨类静态调用/构造用）
    ///
    /// codegen 把 `InvokeStatic`/`InvokeConstructor` 的目标类名写入 right 操作数
    /// （String 立即数）。若无 right 或非字符串，返回 None（回退到 current_class）。
    fn read_target_class(right: Option<&Operand>) -> Option<String> {
        match right {
            Some(Operand::Immediate(crate::virtual_machine::ir::ImmediateValue::String(s))) => Some(s.clone()),
            _ => None,
        }
    }

    /// 将 Gorge 元素类型映射为运行时 native 数组类。
    fn native_array_class_name(element_type: &str) -> &'static str {
        match element_type.rsplit('.').next().unwrap_or(element_type).to_ascii_lowercase().as_str() {
            "int" | "enum" => "IntArray",
            "float" => "FloatArray",
            "bool" => "BoolArray",
            "string" => "StringArray",
            _ => "ObjectArray",
        }
    }

    /// 按对象 ID 调用委托（1c）
    ///
    /// 查询 runtime_delegates 表，取得委托的方法实现，通过 call_compiled_method
    /// 执行并将返回值手动复制到 param_pool 的 return 位，供调用方从 param_pool 读取。
    pub fn invoke_delegate_object(&mut self, delegate_obj_id: usize) -> VmResult<()> {
        let delegate = self.runtime_delegates
            .get(&delegate_obj_id)
            .ok_or_else(|| format!("委托对象 {} 未注册", delegate_obj_id))?
            .clone();
        // 将捕获值按类型分组写入参数池（委托方法体的 LoadXxxParameter 从此读取）
        let cap = &delegate.captured_values;
        for (i, v) in cap.ints.iter().enumerate() { self.param_pool.set_int_param(i, *v); }
        for (i, v) in cap.floats.iter().enumerate() { self.param_pool.set_float_param(i, *v); }
        for (i, v) in cap.bools.iter().enumerate() { self.param_pool.set_bool_param(i, *v); }
        for (i, v) in cap.strings.iter().enumerate() { self.param_pool.set_string_param(i, (*v).clone()); }
        for (i, v) in cap.objects.iter().enumerate() { self.param_pool.set_object_param(i, *v); }
        // 执行委托方法体（无参数，捕获值已在 param_pool 中）
        // save_return_regs=false：保留 callee 的 return_* 寄存器供手动复制
        let method = delegate.method_impl.clone();
        let param_mode = ParamMode::None;
        self.call_compiled_method(&method, &param_mode, None, ValueType::Int, None, None, false)?;
        // 将 return_* 寄存器复制到 param_pool 的返回位（供调用方 get_*_return 读取）
        if let Some(v) = self.return_int { self.param_pool.set_int_return(v); }
        if let Some(v) = self.return_float { self.param_pool.set_float_return(v); }
        if let Some(v) = self.return_bool { self.param_pool.set_bool_return(v); }
        if let Some(ref v) = self.return_string { self.param_pool.set_string_return(v.clone()); }
        if let Some(v) = self.return_object { self.param_pool.set_object_return(v); }
        Ok(())
    }

    /// 按方法全局 ID 调用编译方法（S3c）
    ///
    /// 查 class_table 中目标类的 find_method 获取 IR，通过 call_compiled_method 执行。
    /// 无参数，target_obj_id=Some 时设置 this 指针。
    /// 返回值留在 return_* 寄存器中，调用方通过 get_return_* 读取。
    pub fn invoke_method_by_id(
        &mut self,
        class_name: &str,
        target_obj_id: Option<usize>,
        method_global_id: usize,
    ) -> VmResult<()> {
        let method = self.class_table
            .get(class_name)
            .ok_or_else(|| format!("类 `{}` 未注册", class_name))?
            .find_method(method_global_id)
            .ok_or_else(|| format!("类 `{}` 中未找到方法全局 ID {}", class_name, method_global_id))?;
        self.call_compiled_method(
            &method,
            &ParamMode::None,
            None,
            ValueType::Int,
            Some(class_name),
            target_obj_id,
            false,
        )
    }

    /// 按方法全局 ID 调用编译方法，并把参数池按类型分组批量布置到局部栈。
    ///
    /// 与 `invoke_method_by_id`（`ParamMode::None`）不同：普通编译方法
    /// 的参数在 codegen 中注册为局部变量（`register_parameters`），调用方
    /// 须用 `ParamMode::Batch` 把 `param_pool` 复制到局部槽。框架侧调用
    /// 编译类接口实现（如 `ITransformer.Transform(float now)`）必须走此入口。
    pub fn invoke_instance_method_by_global_id_batch(
        &mut self,
        class_name: &str,
        target_obj_id: usize,
        method_global_id: usize,
    ) -> VmResult<()> {
        let method = self.class_table
            .get(class_name)
            .ok_or_else(|| format!("类 `{}` 未注册", class_name))?
            .find_method(method_global_id)
            .ok_or_else(|| format!("类 `{}` 中未找到方法全局 ID {}", class_name, method_global_id))?;
        self.call_compiled_method(
            &method,
            &ParamMode::Batch,
            None,
            ValueType::Int,
            Some(class_name),
            Some(target_obj_id),
            false,
        )
    }

    /// 按全局方法 ID 调用类的静态方法（对齐 C# `ClassInfo.InvokeStaticMethod`）。
    ///
    /// 与 `invoke_method_by_id`（实例方法路径）不同，本方法沿 `class_static_methods`
    /// 静态方法表分派，`set_this` 传 None（静态方法无 `this`）。`@PeriodModifier`
    /// 等静态注解方法必须走此路径，否则会被误判为实例方法而报"调用目标对象为空"。
    ///
    /// 参数经 `set_*_param` 预置在参数池，方法体以 `LoadObjectParameter` 直读
    /// （与 `invoke_method_by_id` 的 `ParamMode::None` 约定一致）。
    ///
    /// 静态方法索引 = `method_global_id - method_start_id`（编译器注解表按
    /// `method_start_id + 局部索引` 记录全局 ID，`class_static_methods` 按声明顺序
    /// 存储整类方法，二者索引一致）。
    pub fn invoke_static_method_by_global_id(
        &mut self,
        class_name: &str,
        method_global_id: usize,
    ) -> VmResult<()> {
        let start = self.class_table
            .get(class_name)
            .ok_or_else(|| format!("类 `{}` 未注册", class_name))?
            .declaration
            .method_start_id;
        let static_idx = method_global_id.saturating_sub(start);
        let method = self.class_static_methods
            .get(class_name)
            .ok_or_else(|| format!("类 `{}` 未注册方法表", class_name))?
            .get(static_idx)
            .map(|(m, _types)| m.clone())
            .ok_or_else(|| format!("类 `{}` 静态方法索引 {} 越界", class_name, static_idx))?;
        self.call_compiled_method(
            &method,
            &ParamMode::None,
            None,
            ValueType::Int,
            Some(class_name),
            None,
            false,
        )
    }

    /// 通过注入器实例化对象（S3d）
    ///
    /// 创建空对象 → 保存并设置 current_injector=injector_obj_id →
    /// 执行字段初始化器 + 构造方法 → 恢复 current_injector → 返回新对象 ID。
    pub fn instantiate_with_injector(
        &mut self,
        class_name: &str,
        ctor_global_id: usize,
        injector_obj_id: usize,
    ) -> VmResult<usize> {
        let cls = self.class_table
            .get(class_name)
            .ok_or_else(|| format!("类 `{}` 未注册", class_name))?
            .clone();
        let total_with_inheritance = self.class_field_counts.get(class_name)
            .cloned()
            .unwrap_or_else(|| {
                let mut total = TypeCount::zero();
                let mut current = Some(cls.clone());
                while let Some(runtime_class) = current {
                    let own = &runtime_class.declaration.field_type_count;
                    total.int_count += own.int_count;
                    total.float_count += own.float_count;
                    total.bool_count += own.bool_count;
                    total.string_count += own.string_count;
                    total.object_count += own.object_count;
                    current = runtime_class.super_class.clone();
                }
                total
            });
        let obj = RuntimeObject::new_simple(class_name.to_string(), &total_with_inheritance);
        let obj_id = self.next_object_id;
        self.next_object_id += 1;
        self.objects.insert(obj_id, obj);

        let saved_injector = self.current_injector;
        self.current_injector = Some(injector_obj_id);

        // 执行字段初始化器
        if let Err(error) = self.run_field_initializers(class_name, obj_id) {
            self.current_injector = saved_injector;
            return Err(error);
        }

        let ctor = match cls.find_constructor(ctor_global_id) {
            Some(ctor) => ctor,
            None => {
                self.current_injector = saved_injector;
                return Err(format!("类 `{}` 中未找到构造方法全局 ID {}", class_name, ctor_global_id));
            }
        };
        let construct_result = self.call_compiled_method(
            &ctor,
            &ParamMode::Batch,
            None,
            ValueType::Int,
            Some(class_name),
            Some(obj_id),
            false,
        );

        self.current_injector = saved_injector;
        construct_result?;

        Ok(obj_id)
    }

    /// 收集类的继承链（本类在前，根类在后，含 native 父类名）。
    fn inheritance_chain(&self, class_name: &str) -> Vec<String> {
        let mut chain = vec![class_name.to_string()];
        let mut current = self.class_super_name.get(class_name).cloned();
        while let Some(name) = current {
            chain.push(name.clone());
            current = self.class_super_name.get(&name).cloned();
        }
        chain
    }

    /// 沿继承链（根类在前）执行各类的字段初始化器。
    ///
    /// 字段初始化器按类分别注册（如 DremuLane 的 `name = ^name` 注册在
    /// DremuLane 名下），实例化 DremuMainLane 时也必须执行父类初始化器，
    /// 否则父类注入器字段（name/generateTime 等）不会落到对象上。
    fn run_field_initializers(&mut self, class_name: &str, obj_id: usize) -> VmResult<()> {
        let chain = self.inheritance_chain(class_name);
        for owner in chain.iter().rev() {
            if let Some(initials) = self.class_field_initializers.get(owner).cloned() {
                for init in &initials {
                    let field_initializer_injector = self.current_injector;
                    let method = CompiledMethod {
                        name: "field_init".into(),
                        codes: init.codes.clone(),
                        local_count: init.local_count,
                    };
                    let initialize_result = self.call_compiled_method(
                        &method,
                        &ParamMode::None,
                        None,
                        ValueType::Int,
                        Some(owner),
                        Some(obj_id),
                        false,
                    );
                    self.current_injector = field_initializer_injector;
                    initialize_result?;
                }
            }
        }
        Ok(())
    }

    /// 带构造参数注入实例化（A-3）
    ///
    /// 对应 C# CompiledInjector.Instantiate(int constructorIndex, params object[] args)。
    /// args 为按值类型分组的参数，写入 param_pool 后走完整构造流程。
    pub fn instantiate_with_injector_args(
        &mut self,
        class_name: &str,
        ctor_global_id: usize,
        injector_obj_id: usize,
        args: &InstantiateArgs,
    ) -> VmResult<usize> {
        // 将参数按类型写入 param_pool
        for (i, &v) in args.ints.iter().enumerate() {
            self.param_pool.set_int_param(i, v);
        }
        for (i, &v) in args.floats.iter().enumerate() {
            self.param_pool.set_float_param(i, v);
        }
        for (i, &v) in args.bools.iter().enumerate() {
            self.param_pool.set_bool_param(i, v);
        }
        for (i, v) in args.strings.iter().enumerate() {
            self.param_pool.set_string_param(i, v.clone());
        }
        for (i, &v) in args.objects.iter().enumerate() {
            self.param_pool.set_object_param(i, v);
        }
        self.instantiate_with_injector(class_name, ctor_global_id, injector_obj_id)
    }

    /// 深拷贝对象（A-2）
    ///
    /// 按对象类型分派克隆逻辑：
    /// - 注入器对象：新建 RuntimeInjector 并递归深拷贝（值字段直拷、object 字段递归 clone_object）
    /// - Native List/Array：复制元素（ObjectList/ObjectArray 元素递归 clone_object，值类型直接复制）
    /// - 编译对象：复制 FixedFieldValuePool 全部字段（object 字段保持引用复制）
    ///
    /// 对齐 C# CompiledInjector.Clone（递归深拷贝 object 字段）、
    /// ObjectList.Clone（递归克隆元素）、IntList.Clone（值类型元素直接复制）。
    ///
    /// # 防循环引用
    /// depth 参数控制递归深度上限（默认 64），超出返回错误。
    pub fn clone_object(&mut self, obj_id: usize) -> VmResult<usize> {
        self.clone_object_impl(obj_id, 0)
    }

    /// clone_object 内部实现（带深度计数）
    fn clone_object_impl(&mut self, obj_id: usize, depth: usize) -> VmResult<usize> {
        const MAX_CLONE_DEPTH: usize = 64;

        if depth > MAX_CLONE_DEPTH {
            return Err(format!("clone_object 超过最大递归深度 {}", MAX_CLONE_DEPTH));
        }
        if obj_id == 0 {
            return Ok(0); // null
        }

        // 检查是否为注入器对象
        if self.injectors.contains_key(&obj_id) {
            return self.clone_injector(obj_id, depth);
        }

        // 检查是否为 native 集合载荷
        if self.native_payloads.contains_key(&obj_id) {
            return self.clone_native_payload(obj_id, depth);
        }

        // 检查是否为编译对象
        if let Some(obj) = self.objects.get(&obj_id) {
            let new_obj = RuntimeObject {
                class_name: obj.class_name.clone(),
                class: obj.class.clone(),
                native_object: None, // native 子对象不复用（简化实现，对齐 C# CompiledGorgeObject 无 Clone 重写）
                compiled_fields: obj.compiled_fields.clone(),
                native_field_bounds: obj.native_field_bounds.clone(),
                native_object_id: None,
                outer_compiled_id: None,
            };
            let new_id = self.next_object_id;
            self.next_object_id += 1;
            self.objects.insert(new_id, new_obj);
            return Ok(new_id);
        }

        Err(format!("clone_object: 对象 {} 不存在", obj_id))
    }

    /// 深拷贝注入器对象
    fn clone_injector(&mut self, obj_id: usize, depth: usize) -> VmResult<usize> {
        let inj = self.injectors.get(&obj_id)
            .ok_or_else(|| format!("注入器 {} 不存在", obj_id))?;

        let class_decl = inj.class_decl.clone();
        let mut new_inj = RuntimeInjector::new(class_decl);

        // 值类型字段复制（含默认值标记）
        let n = new_inj.int_field_count().min(inj.int_field_count());
        for i in 0..n {
            let val = inj.get_injector_int(i);
            let is_def = inj.get_injector_int_default_value(i);
            if is_def {
                new_inj.set_injector_int(i, val);
                new_inj.set_injector_int_default_value(i);
            } else {
                new_inj.set_injector_int(i, val);
            }
        }
        let n = new_inj.float_field_count().min(inj.float_field_count());
        for i in 0..n {
            let val = inj.get_injector_float(i);
            let is_def = inj.get_injector_float_default_value(i);
            if is_def {
                new_inj.set_injector_float(i, val);
                new_inj.set_injector_float_default_value(i);
            } else {
                new_inj.set_injector_float(i, val);
            }
        }
        let n = new_inj.bool_field_count().min(inj.bool_field_count());
        for i in 0..n {
            let val = inj.get_injector_bool(i);
            let is_def = inj.get_injector_bool_default_value(i);
            if is_def {
                new_inj.set_injector_bool(i, val);
                new_inj.set_injector_bool_default_value(i);
            } else {
                new_inj.set_injector_bool(i, val);
            }
        }
        let n = new_inj.string_field_count().min(inj.string_field_count());
        for i in 0..n {
            let val = inj.get_injector_string(i);
            let is_def = inj.get_injector_string_default_value(i);
            if is_def {
                new_inj.set_injector_string(i, val);
                new_inj.set_injector_string_default_value(i);
            } else {
                new_inj.set_injector_string(i, val);
            }
        }
        // object 字段递归深拷贝（对齐 C# CompiledInjector.Clone 的 `_object[i].Item1?.Clone()`）
        let n = new_inj.object_field_count().min(inj.object_field_count());
        let mut object_fields: Vec<(usize, bool)> = Vec::with_capacity(n);
        for i in 0..n {
            object_fields.push(inj.object_field(i));
        }
        for (i, (val, is_def)) in object_fields.into_iter().enumerate() {
            if is_def {
                new_inj.set_injector_object(i, val);
                new_inj.set_injector_object_default_value(i);
            } else {
                let cloned = self.clone_object_impl(val, depth + 1)?;
                new_inj.set_injector_object(i, cloned);
            }
        }

        let new_id = self.next_object_id;
        self.next_object_id += 1;
        self.injectors.insert(new_id, new_inj);
        Ok(new_id)
    }

    /// 深拷贝 native 载荷（List/Array 集合）
    fn clone_native_payload(&mut self, obj_id: usize, depth: usize) -> VmResult<usize> {
        use crate::system::native::list::*;
        use crate::system::native::array::*;

        let new_id = self.next_object_id;
        self.next_object_id += 1;

        // 检查各 List 类型
        if let Some(list) = self.native_payloads.get(&obj_id)
            .and_then(|p| p.downcast_ref::<IntList>())
        {
            let items = list.items.clone();
            self.native_payloads.insert(new_id, Box::new(IntList { items }));
        } else if let Some(list) = self.native_payloads.get(&obj_id)
            .and_then(|p| p.downcast_ref::<FloatList>())
        {
            let items = list.items.clone();
            self.native_payloads.insert(new_id, Box::new(FloatList { items }));
        } else if let Some(list) = self.native_payloads.get(&obj_id)
            .and_then(|p| p.downcast_ref::<BoolList>())
        {
            let items = list.items.clone();
            self.native_payloads.insert(new_id, Box::new(BoolList { items }));
        } else if let Some(list) = self.native_payloads.get(&obj_id)
            .and_then(|p| p.downcast_ref::<StringList>())
        {
            let items = list.items.clone();
            self.native_payloads.insert(new_id, Box::new(StringList { items }));
        } else if let Some(list) = self.native_payloads.get(&obj_id)
            .and_then(|p| p.downcast_ref::<ObjectList>())
        {
            // ObjectList 需要递归克隆每个元素对象
            let item_ids: Vec<usize> = list.items.clone();
            let mut items = Vec::with_capacity(item_ids.len());
            for item_id in item_ids {
                let cloned = self.clone_object_impl(item_id, depth + 1)?;
                items.push(cloned);
            }
            self.native_payloads.insert(new_id, Box::new(ObjectList { items }));
        } else if let Some(arr) = self.native_payloads.get(&obj_id)
            .and_then(|p| p.downcast_ref::<IntArray>())
        {
            let items = arr.items.clone();
            self.native_payloads.insert(new_id, Box::new(IntArray { items }));
        } else if let Some(arr) = self.native_payloads.get(&obj_id)
            .and_then(|p| p.downcast_ref::<FloatArray>())
        {
            let items = arr.items.clone();
            self.native_payloads.insert(new_id, Box::new(FloatArray { items }));
        } else if let Some(arr) = self.native_payloads.get(&obj_id)
            .and_then(|p| p.downcast_ref::<BoolArray>())
        {
            let items = arr.items.clone();
            self.native_payloads.insert(new_id, Box::new(BoolArray { items }));
        } else if let Some(arr) = self.native_payloads.get(&obj_id)
            .and_then(|p| p.downcast_ref::<StringArray>())
        {
            let items = arr.items.clone();
            self.native_payloads.insert(new_id, Box::new(StringArray { items }));
        } else if let Some(arr) = self.native_payloads.get(&obj_id)
            .and_then(|p| p.downcast_ref::<ObjectArray>())
        {
            // ObjectArray 需要递归克隆每个元素对象
            let item_ids: Vec<usize> = arr.items.clone();
            let mut items = Vec::with_capacity(item_ids.len());
            for item_id in item_ids {
                let cloned = self.clone_object_impl(item_id, depth + 1)?;
                items.push(cloned);
            }
            self.native_payloads.insert(new_id, Box::new(ObjectArray { items }));
        } else {
            return Err(format!("clone_object: 对象 {} 的载荷类型不支持克隆", obj_id));
        }

        Ok(new_id)
    }

    /// 统一方法调用辅助（1a）
    ///
    /// 保存当前栈状态 → 布置参数 → 执行方法体 IR → 写回返回值 → 恢复栈。
    /// 通过 `ParamMode` 覆盖 8 处调用点的参数布置差异。
    fn call_compiled_method(
        &mut self,
        method: &CompiledMethod,
        param_mode: &ParamMode,
        result_addr: Option<&Address>,
        _return_type: ValueType,
        switch_class: Option<&str>,
        set_this: Option<usize>,
        save_return_regs: bool,
    ) -> VmResult<()> {
        // 1. 保存栈状态
        let saved_pc = self.pc;
        let save_len = self.int_stack.data.len();
        let saved_ints: Vec<i64> = (0..save_len).map(|i| *self.int_stack.read(i)).collect();
        let saved_floats: Vec<f64> = (0..save_len).map(|i| *self.float_stack.read(i)).collect();
        let saved_bools: Vec<bool> = (0..save_len).map(|i| *self.bool_stack.read(i)).collect();
        let saved_objects: Vec<usize> = (0..save_len).map(|i| *self.object_stack.read(i)).collect();
        // 字符串栈保存（字段初始化器需要）
        let saved_strings: Vec<String> = (0..save_len).map(|i| self.string_stack.read(i).clone()).collect();

        // 1b. 保存 return_* 寄存器（委托调用需要）
        let saved_return_int = if save_return_regs { self.return_int } else { None };
        let saved_return_float = if save_return_regs { self.return_float } else { None };
        let saved_return_bool = if save_return_regs { self.return_bool } else { None };
        let saved_return_string = if save_return_regs { self.return_string.clone() } else { None };
        let saved_return_object = if save_return_regs { self.return_object } else { None };
        // 清空 return_* 寄存器供 callee 使用
        self.return_int = None;
        self.return_float = None;
        self.return_bool = None;
        self.return_string = None;
        self.return_object = None;

        // 2. 确保 callee 栈空间
        let max_locals = method.local_count;
        self.int_stack.write(max_locals.saturating_sub(1), 0);
        self.float_stack.write(max_locals.saturating_sub(1), 0.0);
        self.bool_stack.write(max_locals.saturating_sub(1), false);
        self.string_stack.write(max_locals.saturating_sub(1), String::new());
        self.object_stack.write(max_locals.saturating_sub(1), 0);

        // 3. 布置参数（按 ParamMode）
        match param_mode {
            ParamMode::None => {}
            ParamMode::Batch => {
                self.copy_params_to_locals(max_locals);
            }
            ParamMode::ByType(param_types) => {
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
                            // 局部 object 槽 0 固定保留给 this，参数从 1 开始
                            self.object_stack.write(i + 1, val);
                        }
                    }
                }
            }
            ParamMode::ByCount(count) => {
                for i in 0..*count {
                    self.int_stack.write(i, self.param_pool.get_int_param(i));
                    self.float_stack.write(i, self.param_pool.get_float_param(i));
                    self.bool_stack.write(i, self.param_pool.get_bool_param(i));
                    self.string_stack.write(i, self.param_pool.get_string_param(i));
                    // 局部 object 槽 0 固定保留给 this，参数从 1 开始
                    self.object_stack.write(i + 1, self.param_pool.get_object_param(i));
                }
            }
        }

        // 4. 设置 this 指针（放在 object_stack[0]）
        if let Some(this_id) = set_this {
            self.object_stack.write(0, this_id);
        }

        // 5. 切换执行上下文类名
        let saved_class = switch_class.map(|c| {
            let sc = self.current_class.clone();
            self.current_class = c.to_string();
            sc
        });

        // 6. 执行方法体
        self.pc = 0;
        let count = method.codes.len();
        while self.pc < count {
            let cs = &method.codes[self.pc];
            let advance = self.execute_one(&cs.code).map_err(|error| {
                format!("{}（方法 `{}`）", error, method.name)
            })?;
            if !advance {
                if self.pc >= count { break; }
                continue;
            }
            self.pc += 1;
        }

        // 7. 写回返回值到 result 地址
        let result_index = result_addr.as_ref().map(|r| r.index);
        if let Some(addr) = result_addr {
            match addr.value_type {
                ValueType::Int => { if let Some(v) = self.return_int { self.int_stack.write(addr.index, v); } }
                ValueType::Float => { if let Some(v) = self.return_float { self.float_stack.write(addr.index, v); } }
                ValueType::Bool => { if let Some(v) = self.return_bool { self.bool_stack.write(addr.index, v); } }
                ValueType::String => { if let Some(ref v) = self.return_string { self.string_stack.write(addr.index, v.clone()); } }
                ValueType::Object => { if let Some(v) = self.return_object { self.object_stack.write(addr.index, v); } }
            }
        }

        // 8. 恢复调用者栈
        for (i, v) in saved_ints.iter().enumerate() {
            if result_index.is_none() || Some(i) != result_index {
                self.int_stack.write(i, *v);
            }
        }
        for (i, v) in saved_floats.iter().enumerate() {
            if result_index.is_none() || Some(i) != result_index {
                self.float_stack.write(i, *v);
            }
        }
        for (i, v) in saved_bools.iter().enumerate() {
            if result_index.is_none() || Some(i) != result_index {
                self.bool_stack.write(i, *v);
            }
        }
        for (i, v) in saved_strings.iter().enumerate() {
            self.string_stack.write(i, v.clone());
        }
        for (i, v) in saved_objects.iter().enumerate() {
            if result_index.is_none() || Some(i) != result_index {
                self.object_stack.write(i, *v);
            }
        }

        // 9. 恢复 return_* 寄存器
        if save_return_regs {
            self.return_int = saved_return_int;
            self.return_float = saved_return_float;
            self.return_bool = saved_return_bool;
            self.return_string = saved_return_string;
            self.return_object = saved_return_object;
        }

        // 10. 恢复类上下文
        if let Some(sc) = saved_class {
            self.current_class = sc;
        }

        self.pc = saved_pc;
        Ok(())
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

    /// 校验对象字段索引是否在（native 边界 + 编译字段）范围内。
    ///
    /// 越界时返回带对象类名与当前执行类的明确错误，避免 value_pool
    /// 裸索引 panic（对象字段布局与 IR 字段索引不一致时便于定位）。
    fn check_field_bounds(
        &self,
        obj: &crate::objective::object::RuntimeObject,
        obj_id: usize,
        field_idx: usize,
        native_count: usize,
        compiled_len: usize,
        type_name: &str,
    ) -> VmResult<()> {
        let total = native_count + compiled_len;
        if field_idx >= total {
            return Err(format!(
                "对象 `{}`（ID {}）的 {} 字段索引 {} 越界（共 {} 个），当前执行类 `{}`",
                obj.class_name,
                obj_id,
                type_name,
                field_idx,
                total,
                self.current_class,
            ));
        }
        Ok(())
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
                let value = self.read_object_operand(&code.left);
                self.current_injector = Some(value);
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
            IntermediateOperator::IntOpposite => {
                let val = self.read_int_operand(&code.left);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, -val);
            }
            IntermediateOperator::FloatOpposite => {
                let val = self.read_float_operand(&code.left);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, -val);
            }
            IntermediateOperator::FloatMod => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, lhs % rhs);
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

            IntermediateOperator::StringEqual => {
                let lhs = self.read_string_operand(&code.left);
                let rhs = self.read_string_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs == rhs);
            }
            IntermediateOperator::ObjectEqual => {
                let lhs = self.read_object_operand(&code.left);
                let rhs = self.read_object_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs == rhs);
            }
            IntermediateOperator::StringNotEqual => {
                let lhs = self.read_string_operand(&code.left);
                let rhs = self.read_string_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs != rhs);
            }
            IntermediateOperator::ObjectNotEqual => {
                let lhs = self.read_object_operand(&code.left);
                let rhs = self.read_object_operand(code.right.as_ref().unwrap());
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
            IntermediateOperator::ObjectCastToObject => {
                // 对象以 ID 传递，转型仅复制对象 ID（运行期不做类型检查，对齐 C#）
                let val = self.read_object_operand(&code.left);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
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
                // 从类委托表获取委托信息（含捕获变量类型 V5）
                let (method_impl, param_types, _return_type, captured_var_types) = self.class_delegate_impls
                    .get(&self.current_class)
                    .and_then(|d| d.get(*idx).cloned())
                    .unwrap_or_else(|| (CompiledMethod { name: "lambda".into(), codes: vec![], local_count: 0 }, vec![], ValueType::Int, vec![]));
                // 从参数池按类型分组读取捕获值（codegen 端 emit_set_param 已将捕获值按类型分组写入参数池）
                let mut captured = crate::objective::value_pool::FixedFieldValuePool::default();
                let mut int_idx: usize = 0;
                let mut float_idx: usize = 0;
                let mut bool_idx: usize = 0;
                let mut string_idx: usize = 0;
                let mut object_idx: usize = 0;
                for cv in &captured_var_types {
                    match cv {
                        ValueType::Int => { captured.ints.push(self.param_pool.get_int_param(int_idx)); int_idx += 1; }
                        ValueType::Float => { captured.floats.push(self.param_pool.get_float_param(float_idx)); float_idx += 1; }
                        ValueType::Bool => { captured.bools.push(self.param_pool.get_bool_param(bool_idx)); bool_idx += 1; }
                        ValueType::String => { captured.strings.push(self.param_pool.get_string_param(string_idx)); string_idx += 1; }
                        ValueType::Object => { captured.objects.push(self.param_pool.get_object_param(object_idx)); object_idx += 1; }
                    }
                }
                // 若当前在实例方法中，记录 creator_this 供委托执行时恢复 this 指针
                let this_id = *self.object_stack.read(0);
                let creator_this = if this_id != 0 { Some(this_id) } else { None };
                // 创建 RuntimeDelegate 对象
                let delegate_obj = crate::objective::delegate::RuntimeDelegate {
                    delegate_type: crate::objective::types::GorgeType::new(crate::objective::types::BasicType::Delegate),
                    method_impl,
                    captured_values: captured,
                    param_types,
                    captured_var_types,
                    creator_this,
                };
                let obj_id = self.next_object_id; self.next_object_id += 1;
                // 将委托对象存入 runtime_delegates 表
                self.runtime_delegates.insert(obj_id, delegate_obj);
                // 将委托对象以普通 RuntimeObject 形式存入 objects 表（保持编译兼容）
                let rt_obj = crate::objective::object::RuntimeObject::new_simple("Delegate".into(), &TypeCount::zero());
                self.objects.insert(obj_id, rt_obj);
                // 写入结果地址
                if let Some(ref result) = code.result {
                    self.object_stack.write(result.index, obj_id);
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
            // 对齐 C# LoadField: left = 对象引用, field_idx 在 variant 内, result = 输出地址
            IntermediateOperator::LoadIntField(field_idx) => {
                let obj_id = self.read_object_operand(&code.left);
                let val = match self.objects.get(&obj_id) {
                    Some(obj) => {
                        self.check_field_bounds(
                            obj,
                            obj_id,
                            *field_idx,
                            obj.native_field_bounds.int_count,
                            obj.compiled_fields.ints.len(),
                            "int",
                        )?;
                        obj.get_int_field(*field_idx)
                    }
                    None => 0,
                };
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::LoadFloatField(field_idx) => {
                let obj_id = self.read_object_operand(&code.left);
                let val = match self.objects.get(&obj_id) {
                    Some(obj) => {
                        self.check_field_bounds(
                            obj,
                            obj_id,
                            *field_idx,
                            obj.native_field_bounds.float_count,
                            obj.compiled_fields.floats.len(),
                            "float",
                        )?;
                        obj.get_float_field(*field_idx)
                    }
                    None => 0.0,
                };
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::LoadBoolField(field_idx) => {
                let obj_id = self.read_object_operand(&code.left);
                let val = match self.objects.get(&obj_id) {
                    Some(obj) => {
                        self.check_field_bounds(
                            obj,
                            obj_id,
                            *field_idx,
                            obj.native_field_bounds.bool_count,
                            obj.compiled_fields.bools.len(),
                            "bool",
                        )?;
                        obj.get_bool_field(*field_idx)
                    }
                    None => false,
                };
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::LoadStringField(field_idx) => {
                let obj_id = self.read_object_operand(&code.left);
                let val = match self.objects.get(&obj_id) {
                    Some(obj) => {
                        self.check_field_bounds(
                            obj,
                            obj_id,
                            *field_idx,
                            obj.native_field_bounds.string_count,
                            obj.compiled_fields.strings.len(),
                            "string",
                        )?;
                        obj.get_string_field(*field_idx)
                    }
                    None => String::new(),
                };
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::LoadObjectField(field_idx) => {
                let obj_id = self.read_object_operand(&code.left);
                let val = match self.objects.get(&obj_id) {
                    Some(obj) => {
                        self.check_field_bounds(
                            obj,
                            obj_id,
                            *field_idx,
                            obj.native_field_bounds.object_count,
                            obj.compiled_fields.objects.len(),
                            "object",
                        )?;
                        obj.get_object_field(*field_idx)
                    }
                    None => 0,
                };
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }

            // === 对象字段写入 ===
            // 对齐 C# SetField: left = 对象引用, right = 值, field_idx 在 variant 内
            IntermediateOperator::SetIntField(field_idx) => {
                let obj_id = self.read_object_operand(&code.left);
                let val = self.read_int_operand(code.right.as_ref().unwrap());
                if let Some(obj) = self.objects.get(&obj_id) {
                    let (native_count, compiled_len) =
                        (obj.native_field_bounds.int_count, obj.compiled_fields.ints.len());
                    self.check_field_bounds(
                        obj,
                        obj_id,
                        *field_idx,
                        native_count,
                        compiled_len,
                        "int",
                    )?;
                }
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_int_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetFloatField(field_idx) => {
                let obj_id = self.read_object_operand(&code.left);
                let val = self.read_float_operand(code.right.as_ref().unwrap());
                if let Some(obj) = self.objects.get(&obj_id) {
                    let (native_count, compiled_len) =
                        (obj.native_field_bounds.float_count, obj.compiled_fields.floats.len());
                    self.check_field_bounds(
                        obj,
                        obj_id,
                        *field_idx,
                        native_count,
                        compiled_len,
                        "float",
                    )?;
                }
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_float_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetBoolField(field_idx) => {
                let obj_id = self.read_object_operand(&code.left);
                let val = self.read_bool_operand(code.right.as_ref().unwrap());
                if let Some(obj) = self.objects.get(&obj_id) {
                    let (native_count, compiled_len) =
                        (obj.native_field_bounds.bool_count, obj.compiled_fields.bools.len());
                    self.check_field_bounds(
                        obj,
                        obj_id,
                        *field_idx,
                        native_count,
                        compiled_len,
                        "bool",
                    )?;
                }
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_bool_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetStringField(field_idx) => {
                let obj_id = self.read_object_operand(&code.left);
                let val = self.read_string_operand(code.right.as_ref().unwrap());
                if let Some(obj) = self.objects.get(&obj_id) {
                    let (native_count, compiled_len) =
                        (obj.native_field_bounds.string_count, obj.compiled_fields.strings.len());
                    self.check_field_bounds(
                        obj,
                        obj_id,
                        *field_idx,
                        native_count,
                        compiled_len,
                        "string",
                    )?;
                }
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_string_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetObjectField(field_idx) => {
                let obj_id = self.read_object_operand(&code.left);
                let val = self.read_object_operand(code.right.as_ref().unwrap());
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    let total_object_fields = obj.native_field_bounds.object_count
                        + obj.compiled_fields.objects.len();
                    if *field_idx >= total_object_fields {
                        return Err(format!(
                            "对象 `{}`（ID {}）的 object 字段索引 {} 越界（共 {} 个），当前执行类 `{}`",
                            obj.class_name,
                            obj_id,
                            field_idx,
                            total_object_fields,
                            self.current_class,
                        ));
                    }
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
            // LoadXxxInjectorField: left 操作数传递注入器对象 ID，result 存放读取结果
            IntermediateOperator::LoadIntInjectorField(field_idx) => {
                let inj_id = self.read_object_operand(&code.left);
                let val = if let Some(inj) = self.injectors.get(&inj_id) {
                    if inj.get_injector_int_default_value(*field_idx) {
                        // 默认值：优先类级默认值池（旧路径），
                        // 其次注入器槽内已物化的默认值
                        self.lookup_injector_default_int(inj_id, *field_idx)
                            .unwrap_or_else(|| inj.get_injector_int(*field_idx))
                    } else {
                        inj.get_injector_int(*field_idx)
                    }
                } else { 0 };
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::LoadFloatInjectorField(field_idx) => {
                let inj_id = self.read_object_operand(&code.left);
                // 默认值已在物化阶段写入槽位（apply_injector_defaults），
                // 直接读存储值；标记仅用于编辑期相等语义。
                let val = self.injectors.get(&inj_id)
                    .map(|inj| inj.get_injector_float(*field_idx))
                    .unwrap_or(0.0);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::LoadBoolInjectorField(field_idx) => {
                let inj_id = self.read_object_operand(&code.left);
                let val = self.injectors.get(&inj_id)
                    .map(|inj| inj.get_injector_bool(*field_idx))
                    .unwrap_or(false);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::LoadStringInjectorField(field_idx) => {
                let inj_id = self.read_object_operand(&code.left);
                let val = self.injectors.get(&inj_id)
                    .map(|inj| inj.get_injector_string(*field_idx))
                    .unwrap_or_default();
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::LoadObjectInjectorField(field_idx) => {
                let inj_id = self.read_object_operand(&code.left);
                let val = self.injectors.get(&inj_id)
                    .map(|inj| inj.get_injector_object(*field_idx))
                    .unwrap_or(0);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }
            // SetXxxInjectorField: left 操作数传递要写入的值，right 操作数传递注入器对象 ID
            IntermediateOperator::SetIntInjectorField(field_idx) => {
                let inj_id = self.read_object_operand(code.right.as_ref().unwrap());
                let val = self.read_int_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_int(*field_idx, val);
                }
            }
            IntermediateOperator::SetFloatInjectorField(field_idx) => {
                let inj_id = self.read_object_operand(code.right.as_ref().unwrap());
                let val = self.read_float_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_float(*field_idx, val);
                }
            }
            IntermediateOperator::SetBoolInjectorField(field_idx) => {
                let inj_id = self.read_object_operand(code.right.as_ref().unwrap());
                let val = self.read_bool_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_bool(*field_idx, val);
                }
            }
            IntermediateOperator::SetStringInjectorField(field_idx) => {
                let inj_id = self.read_object_operand(code.right.as_ref().unwrap());
                let val = self.read_string_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_string(*field_idx, val);
                }
            }
            IntermediateOperator::SetObjectInjectorField(field_idx) => {
                let inj_id = self.read_object_operand(code.right.as_ref().unwrap());
                let val = self.read_object_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_object(*field_idx, val);
                }
            }

            // === 方法调用 ===
            IntermediateOperator::InvokeInstance(method_id) => {
                let target_obj_id = self.read_object_operand(&code.left);
                if target_obj_id == 0 {
                    return Err(format!(
                        "调用目标对象为空（当前执行类 `{}`，指令 pc={}，left={:?}）",
                        self.current_class,
                        self.pc,
                        code.left,
                    ));
                }
                let class_name = self.objects
                    .get(&target_obj_id)
                    .map(|obj| obj.class_name.clone())
                    .unwrap_or_default();
                if class_name.is_empty() {
                    return Err("无法确定目标对象的类".into());
                }
                // native 类分派
                if self.native_class_table.contains_key(&class_name) {
                    self.dispatch_native_method(&class_name, target_obj_id, *method_id);
                    self.write_native_return_to_result(code.result.as_ref());
                    return Ok(true);
                }
                // 查找编译类方法实现
                let method = match self.class_table
                    .get(&class_name)
                    .and_then(|cls| cls.find_method(*method_id))
                {
                    Some(m) => m,
                None => {
                    // 编译子类继承 native 类：方法可能属于 native 祖先（F2）
                    if let Some(native_anc) = self.find_native_ancestor(&class_name) {
                        self.dispatch_native_method(&native_anc, target_obj_id, *method_id);
                        self.write_native_return_to_result(code.result.as_ref());
                        return Ok(true);
                    }
                    return Err(format!(
                        "未找到方法（全局 ID {}）在类 `{}`（含 native 祖先）中",
                        method_id, class_name
                    ));
                }
                };
                let return_type = code.result.as_ref().map(|r| r.value_type).unwrap_or(ValueType::Int);
                // switch_class 必须切换到目标类：否则方法体内的字段解析、
                // 嵌套实例调用（如 EvaluateLaneLine 调 animation.Evaluate）
                // 会沿用调用方（如 transformer）的执行类上下文。
                self.call_compiled_method(
                    &method,
                    &ParamMode::Batch,
                    code.result.as_ref(),
                    return_type,
                    Some(&class_name),
                    Some(target_obj_id),
                    false,
                )?;
                return Ok(true);
            }
            IntermediateOperator::InvokeStatic(idx) => {
                let target_class = Self::read_target_class(code.right.as_ref())
                    .unwrap_or_else(|| self.current_class.clone());
                if self.native_class_table.contains_key(&target_class) {
                    self.dispatch_native_static(&target_class, *idx);
                    self.write_native_return_to_result(code.result.as_ref());
                    return Ok(true);
                }
                let methods = self.class_static_methods
                    .get(&target_class)
                    .ok_or_else(|| format!("类 `{}` 未注册方法表", target_class))?;
                if *idx >= methods.len() {
                    return Err(format!("静态方法索引 {} 越界", idx));
                }
                let (static_method, _param_types) = methods[*idx].clone();
                let return_type = code.result.as_ref().map(|r| r.value_type).unwrap_or(ValueType::Int);
                self.call_compiled_method(&static_method, &ParamMode::Batch, code.result.as_ref(), return_type, Some(&target_class), None, false)?;
                return Ok(true);
            }
            IntermediateOperator::InvokeInterface(iface_method_id) => {
                let target_obj_id = self.read_object_operand(&code.left);
                if target_obj_id == 0 {
                    return Err("接口方法调用目标对象为空".into());
                }
            let iface_name = match Self::read_target_class(code.right.as_ref()) {
                Some(n) => n,
                None => return Err("接口方法调用缺少接口名（right 操作数非 String）".into()),
            };
                let class_name = self.objects
                    .get(&target_obj_id)
                    .map(|o| o.class_name.clone())
                    .unwrap_or_default();
                if class_name.is_empty() {
                    return Err("无法确定接口调用目标对象的类".into());
                }
                let global_method_id = self.class_table
                    .get(&class_name)
                    .and_then(|cls| cls.declaration.interface_method_impl_id.get(&iface_name))
                    .and_then(|ids| ids.get(*iface_method_id))
                    .copied();
            let global_method_id = match global_method_id {
                Some(id) if id != usize::MAX => id,
                _ => return Err(format!(
                    "类 `{}` 未实现接口 `{}` 的方法（接口方法 ID {}）",
                    class_name, iface_name, iface_method_id
                )),
            };
            let method = match self.class_table
                .get(&class_name)
                .and_then(|cls| cls.find_method(global_method_id))
            {
                Some(m) => m,
                None => return Err(format!(
                    "类 `{}` 中未找到接口方法实现（全局 ID {}，接口 `{}` 方法 {}）",
                    class_name, global_method_id, iface_name, iface_method_id
                )),
            };
                let return_type = code.result.as_ref().map(|r| r.value_type).unwrap_or(ValueType::Int);
                self.call_compiled_method(&method, &ParamMode::Batch, code.result.as_ref(), return_type, Some(&class_name), Some(target_obj_id), false)?;
                return Ok(true);
            }
            IntermediateOperator::InvokeDelegate(idx) => {
                // 读取 left 操作数：如果是运行时委托对象 ID，优先按对象分派
                let left_obj_id = self.read_object_operand(&code.left);
                if left_obj_id != 0 {
                    if let Some(del) = self.runtime_delegates.get(&left_obj_id) {
                        let method = del.method_impl.clone();
                        let cap = &del.captured_values;
                        let own_param_types = &del.param_types;
                        let cv_types = &del.captured_var_types;
                        let creator_this = del.creator_this;
                        // 计算自身参数在各类型中的偏移量（实参已占据 0..param_count_of_type）
                        let int_off = own_param_types.iter().filter(|t| matches!(t, ValueType::Int)).count();
                        let float_off = own_param_types.iter().filter(|t| matches!(t, ValueType::Float)).count();
                        let bool_off = own_param_types.iter().filter(|t| matches!(t, ValueType::Bool)).count();
                        let string_off = own_param_types.iter().filter(|t| matches!(t, ValueType::String)).count();
                        let object_off = own_param_types.iter().filter(|t| matches!(t, ValueType::Object)).count();
                        // 将捕获值追加到参数池实参之后（类型分组偏移）
                        for (i, v) in cap.ints.iter().enumerate() { self.param_pool.set_int_param(int_off + i, *v); }
                        for (i, v) in cap.floats.iter().enumerate() { self.param_pool.set_float_param(float_off + i, *v); }
                        for (i, v) in cap.bools.iter().enumerate() { self.param_pool.set_bool_param(bool_off + i, *v); }
                        for (i, v) in cap.strings.iter().enumerate() { self.param_pool.set_string_param(string_off + i, (*v).clone()); }
                        for (i, v) in cap.objects.iter().enumerate() { self.param_pool.set_object_param(object_off + i, *v); }
                        // 构造联合参数类型列表：自身参数在前，捕获变量在后
                        let mut combined_types: Vec<ValueType> = own_param_types.clone();
                        combined_types.extend_from_slice(cv_types);
                        let return_type = code.result.as_ref().map(|r| r.value_type).unwrap_or(ValueType::Int);
                        self.call_compiled_method(&method, &ParamMode::ByType(combined_types), code.result.as_ref(), return_type, None, creator_this, true)?;
                        return Ok(true);
                    }
                }
                // 回退：按类名 + 编译时 Lambda 分派（兼容既有编译产物）
                let delegates = self.class_delegate_impls
                    .get(&self.current_class)
                    .ok_or_else(|| format!("类 `{}` 未注册委托表", self.current_class))?;
                if *idx >= delegates.len() {
                    return Err(format!("类 `{}` 委托索引 {} 越界（共 {} 个）", self.current_class, idx, delegates.len()));
                }
                let (delegate_method, param_types, return_type, _cv_types) = delegates[*idx].clone();
                let param_mode = ParamMode::ByType(param_types);
                self.call_compiled_method(&delegate_method, &param_mode, code.result.as_ref(), return_type, None, None, true)?;
                return Ok(true);
            }

            // === 构造 ===
            IntermediateOperator::InvokeConstructor(ctor_id) => {
                let target_class = Self::read_target_class(code.right.as_ref())
                    .unwrap_or_else(|| self.current_class.clone());
                if self.native_class_table.contains_key(&target_class) {
                    let new_id = self.dispatch_native_construct(&target_class, None, *ctor_id);
                    let obj_id = if new_id != 0 { new_id } else { self.param_pool.get_object_return() };
                    let result_addr = self.get_object_addr(code.result);
                    self.object_stack.write(result_addr, obj_id);
                    return Ok(true);
                }
                // 1. 创建对象
                let obj_id = self.next_object_id;
                self.next_object_id += 1;
                let field_counts = self.class_field_counts.get(&target_class).cloned().unwrap_or_default();
                let obj = RuntimeObject::new_simple(target_class.clone(), &field_counts);
                self.objects.insert(obj_id, obj);
                // 保存调用者 this 槽：嵌套构造会临时把 object[0] 换成新对象，
                // 返回后必须恢复，否则外层 LoadThis 读到被构造出的对象。
                let saved_caller_this = *self.object_stack.read(0);
                self.object_stack.write(0, obj_id);
                let result_addr_index = self.get_object_addr(code.result);
                self.object_stack.write(result_addr_index, obj_id);

                // 2. 执行字段初始化器（Phase P，沿继承链：父类先于子类）
                if let Err(error) = self.run_field_initializers(&target_class, obj_id) {
                    self.object_stack.write(0, saved_caller_this);
                    return Err(error);
                }

                // 3. 执行构造方法
                if let Some(ctor_method) = self.class_table
                    .get(&target_class)
                    .and_then(|cls| cls.find_constructor(*ctor_id))
                {
                    let result_ref = code.result.as_ref();
                    if let Err(error) = self.call_compiled_method(
                        &ctor_method,
                        &ParamMode::Batch,
                        result_ref,
                        ValueType::Object,
                        Some(&target_class),
                        Some(obj_id),
                        false,
                    ) {
                        self.object_stack.write(0, saved_caller_this);
                        return Err(error);
                    }
                    self.object_stack.write(result_addr_index, obj_id);
                }
                // 恢复调用者 this 槽（构造方法可能嵌套创建其他对象）
                self.object_stack.write(0, saved_caller_this);
                return Ok(true);
            }
            IntermediateOperator::DoConstruct(class_id) => {
                let class_name = if *class_id != 0 && *class_id <= self.class_name_by_id.len() {
                    &self.class_name_by_id[*class_id - 1]
                } else {
                    &self.current_class
                };
                let obj_id = self.next_object_id;
                self.next_object_id += 1;
                let field_counts = self.class_field_counts
                    .get(class_name)
                    .cloned()
                    .unwrap_or_default();
                let obj = RuntimeObject::new_simple(class_name.clone(), &field_counts);
                self.objects.insert(obj_id, obj);
                self.object_stack.write(0, obj_id);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, obj_id);
            }

            // === 父类构造调用（super）===
            IntermediateOperator::InvokeSuperConstructor(ctor_id) => {
            let super_class = match Self::read_target_class(code.right.as_ref()) {
                Some(c) => c,
                None => return Err("super 构造调用缺少父类名（right 操作数非 String）".into()),
            };
                let this_id = *self.object_stack.read(0);
                if self.native_class_table.contains_key(&super_class) {
                    let _ = self.dispatch_native_construct(&super_class, Some(this_id), *ctor_id);
                    return Ok(true);
                }
            let ctor_method = match self.class_table
                .get(&super_class)
                .and_then(|cls| cls.find_constructor(*ctor_id))
            {
                Some(m) => m,
                None => return Err(format!(
                    "父类 `{}` 中未找到构造方法（全局 ID {}）",
                    super_class, ctor_id
                )),
            };
                let param_count = match &code.left {
                    Operand::Immediate(crate::virtual_machine::ir::ImmediateValue::Int(v)) => *v as usize,
                    _ => 0,
                };
                self.call_compiled_method(&ctor_method, &ParamMode::ByCount(param_count), None, ValueType::Object, Some(&super_class), Some(this_id), false)?;
            }

            // === 注入器构造方法（G3）===
            IntermediateOperator::InvokeInjectorConstructor(inj_ctor_idx) => {
                // left 是参数数量；真实目标类取自 SetInjector 激活的动态注入器。
                let injector_id = self.current_injector
                    .filter(|id| *id != 0)
                    .ok_or_else(|| "注入器构造调用缺少当前注入器".to_string())?;
                let (declared_class, ctor_id) = {
                    let injector = self.injectors.get(&injector_id)
                        .ok_or_else(|| format!("当前注入器对象 {} 不存在", injector_id))?;
                    let declaration = injector.injection_class_declaration();
                    // native 类未声明显式注入器构造时，注入器构造 ID 即
                    // native 分派表的本地构造 ID（多为 0）；编译类必须显式映射。
                    let implementation_id = declaration.injector_constructor_impl_id
                        .get(*inj_ctor_idx)
                        .copied()
                        .unwrap_or_else(|| {
                            if declaration.is_native { *inj_ctor_idx } else { usize::MAX }
                        });
                    if implementation_id == usize::MAX {
                        return Err(format!(
                            "类 `{}` 中未找到注入器构造方法映射（注入器构造 ID {}）",
                            declaration.class_type.full_name(), inj_ctor_idx
                        ));
                    }
                    (declaration.class_type.full_name(), implementation_id)
                };
                let simple_class = declared_class.rsplit('.').next().unwrap_or(&declared_class).to_string();
                let target_class = if self.class_table.contains_key(&declared_class) {
                    declared_class
                } else if self.class_table.contains_key(&simple_class) {
                    simple_class
                } else {
                    declared_class
                };
                if self.native_class_table.contains_key(&target_class) {
                    let new_id = self.dispatch_native_construct(&target_class, None, ctor_id);
                    let obj_id = if new_id != 0 { new_id } else { self.param_pool.get_object_return() };
                    let result_addr = self.get_object_addr(code.result);
                    self.object_stack.write(result_addr, obj_id);
                    return Ok(true);
                }
                let obj_id = self.instantiate_with_injector(&target_class, ctor_id, injector_id)?;
                let result_addr = self.get_object_addr(code.result);
                self.object_stack.write(result_addr, obj_id);
            }

            // === 数组构造（Phase H）===
            // 通过 native Array 类创建真实数组对象，不再创建裸 RuntimeObject
            // left = 数组长度(int), right = 元素类型(String), result = 数组对象地址
            IntermediateOperator::InvokeArrayConstructor => {
                let size = self.read_int_operand(&code.left) as usize;
                let elem_type = Self::read_target_class(code.right.as_ref()).unwrap_or_default();
                let array_class = Self::native_array_class_name(&elem_type).to_string();

                // 尝试通过 native 类构造
                if let Some(native_cls) = self.native_class_table.get(&array_class).cloned() {
                    self.param_pool.set_int_param(0, size as i64);
                    let native_id = native_cls.do_construct_native(
                        &mut crate::objective::native::NativeContext::new(self),
                        None,
                        0,
                    );
                    // 注入器数组构造 `new (^laneLines)[N]`：当前注入器是
                    // ObjectArray 包装时复制其元素（注入器引用），否则
                    // `laneLines[i]` 会读到空数组、动态 new 目标为 0。
                    if let Some(injector_id) = self.current_injector {
                        if let Some(array_payload_id) = self.objects.get(&injector_id)
                            .and_then(|object| object.native_object_id)
                        {
                            if let Some(source) = self.native_payloads
                                .get(&array_payload_id)
                                .and_then(|payload| payload.downcast_ref::<crate::system::native::array::ObjectArray>())
                            {
                                let items = source.items.clone();
                                if let Some(target) = self.native_payloads.get_mut(&native_id)
                                    .and_then(|payload| payload.downcast_mut::<crate::system::native::array::ObjectArray>())
                                {
                                    target.items = items;
                                }
                            }
                        }
                    }
                    // 创建编译层包装对象并建立双向引用
                    let obj_id = self.next_object_id; self.next_object_id += 1;
                    let mut wrapper = RuntimeObject::new_simple(array_class.clone(), &TypeCount { int_count: 1, ..TypeCount::zero() });
                    // int 字段 0 存放 length（对齐 C# native 数组类的 `int length` 字段），
                    // 使编译侧 `arr.length` 的 LoadIntField(0) 能读到数组长度
                    wrapper.set_int_field(0, size as i64);
                    wrapper.native_object_id = Some(native_id);
                    self.objects.insert(obj_id, wrapper);
                    if let Some(native_obj) = self.objects.get_mut(&native_id) {
                        native_obj.outer_compiled_id = Some(obj_id);
                    }
                    let addr = self.get_object_addr(code.result);
                    self.object_stack.write(addr, obj_id);
                } else {
                    // 回退：创建裸 RuntimeObject（无 native 类时）
                    let type_count = match elem_type.as_str() {
                        "int" => TypeCount { int_count: size, ..TypeCount::zero() },
                        "float" => TypeCount { float_count: size, ..TypeCount::zero() },
                        "bool" => TypeCount { bool_count: size, ..TypeCount::zero() },
                        "string" => TypeCount { string_count: size, ..TypeCount::zero() },
                        _ => TypeCount { object_count: size, ..TypeCount::zero() },
                    };
                    let obj = RuntimeObject::new_simple(format!("{}Array", elem_type), &type_count);
                    let obj_id = self.next_object_id; self.next_object_id += 1;
                    self.objects.insert(obj_id, obj);
                    let addr = self.get_object_addr(code.result);
                    self.object_stack.write(addr, obj_id);
                }
            }

            // === Nop ===
            IntermediateOperator::Nop => {}

            // 注入器常量加载（G2）
            //
            // 常量池条目由 runner/loader 注册（`CompiledClass.injector_constants`
            // 合并进 `self.injector_constants`）。本指令负责把常量定义物化为
            // 可执行对象：
            // - 标量字段（Int/Float/Bool/String）直接填入注入器；
            // - `InjectObject` 嵌套注入器递归物化为 RuntimeInjector，写入外层
            //   注入器的 object 槽位；
            // - `Array` 物化为 native 数组载荷（元素递归物化），并用编译层
            //   RuntimeObject 包装（`length` 字段语义，对齐 InvokeArrayConstructor）；
            // - `Object` 字段保留编译期记录的对象 ID（与常量定义的语义一致）。
            IntermediateOperator::LoadInjectorConstant(idx) => {
            let constant = match self.injector_constants.get(*idx) {
                Some(c) => c.clone(),
                None => {
                    return Err(format!(
                        "注入器常量索引 {} 越界（共 {} 个常量）",
                        idx, self.injector_constants.len()
                    ));
                }
            };
                let inj_id = self.materialize_injector_constant(&constant)?;
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, inj_id);
            }
        }

        Ok(true) // pc 正常自增
    }

    /// 将注入器常量定义物化为运行时对象，返回对象 ID。
    ///
    /// 顶层返回注入器对象（`RuntimeInjector`）。实现分两步：
    /// 1. `RuntimeInjector::from_constant` 按常量定义完成标量字段填充
    ///    （object 槽位先按 0 占位）；
    /// 2. 沿常量定义二次遍历，把嵌套 `InjectObject` 递归物化为注入器、
    ///    `Array` 物化为 native 数组对象，写入对应 object 槽位。
    fn materialize_injector_constant(
        &mut self,
        constant: &crate::objective::bytecode::InjectorConstantDef,
    ) -> VmResult<usize> {
        // 接口/抽象类型的注入器字面量 `Interface^ : {Concrete : {...}}`：
        // 外层类名是接口（如 FunctionCurve）无法物化，唯一子对象字段名与
        // 内层具体类名一致时（如 ConstantFunctionCurve），递归物化内层。
        if constant.fields.len() == 1 {
            if let crate::objective::bytecode::InjectorConstField::InjectObject(
                nested_class,
                nested_field_name,
                nested_fields,
            ) = &constant.fields[0]
            {
                let nested_simple = nested_class.rsplit('.').next().unwrap_or(nested_class);
                if *nested_field_name == nested_simple {
                    let outer_simple = constant.class_name.rsplit('.').next().unwrap_or(&constant.class_name);
                    let outer_resolvable = self.class_table.contains_key(&constant.class_name)
                        || self.native_class_table.contains_key(&constant.class_name)
                        || self.class_table.contains_key(outer_simple)
                        || self.native_class_table.contains_key(outer_simple);
                    if !outer_resolvable {
                        return self.materialize_injector_constant(
                            &crate::objective::bytecode::InjectorConstantDef {
                                class_name: nested_class.clone(),
                                fields: nested_fields.clone(),
                            },
                        );
                    }
                }
            }
        }
        let class_decl = self.injector_declaration_for_constant(&constant.class_name);
        let mut injector = RuntimeInjector::from_constant(constant, class_decl);

        // 第二遍：递归物化嵌套对象/数组，写入 object 槽位。
        // InjectObject/Array 在常量中无名（InjectObject 首槽位是类名），
        // 按常量内 object 相对顺序写入（与 from_constant 的占槽规则一致，
        // 常量字段按声明顺序输出时该顺序即声明分组索引）
        let mut object_seq = 0usize;
        for field in &constant.fields {
            match field {
                crate::objective::bytecode::InjectorConstField::Object(..) => {
                    object_seq += 1; // 编译期记录的对象 ID 已由 from_constant 写入
                }
                crate::objective::bytecode::InjectorConstField::InjectObject(nested_class, _, fields) => {
                    let nested = crate::objective::bytecode::InjectorConstantDef {
                        class_name: nested_class.clone(),
                        fields: fields.clone(),
                    };
                    let nested_id = self.materialize_injector_constant(&nested)?;
                    if object_seq < injector.object_field_count() {
                        injector.set_injector_object(object_seq, nested_id);
                    }
                    object_seq += 1;
                }
                crate::objective::bytecode::InjectorConstField::Array(_, elements) => {
                    let array_id = self.materialize_injector_array(elements)?;
                    if object_seq < injector.object_field_count() {
                        injector.set_injector_object(object_seq, array_id);
                    }
                    object_seq += 1;
                }
                _ => {}
            }
        }
        // 应用类声明默认值：常量未显式提供的字段回退到 metadata 默认值。
        // 对象型默认值先收集，再脱离注入器借用递归物化后写回 object 槽位。
        let mut pending_defaults: Vec<(usize, bool, crate::objective::bytecode::InjectorConstField)> = Vec::new();
        Self::apply_injector_defaults_to(&mut injector, &mut pending_defaults)?;
        for (object_index, is_array, default_value) in pending_defaults {
            let object_id = self.materialize_injector_default_const(&default_value)?;
            // 默认值本身是 Array 常量时已是数组（元素为具体类注入器），
            // 只有单对象默认值（InjectObject）才需要按数组字段包装。
            let final_id = if is_array
                && matches!(default_value, crate::objective::bytecode::InjectorConstField::InjectObject(..))
            {
                self.materialize_object_array(&[object_id])?
            } else {
                object_id
            };
            injector.set_injector_object(object_index, final_id);
        }
        let inj_id = self.next_object_id;
        self.next_object_id += 1;
        self.injectors.insert(inj_id, injector);
        Ok(inj_id)
    }

    /// 对已创建的注入器应用类声明默认值（chart JSON / 编译常量共用）。
    ///
    /// 谱面 JSON 或注入器常量未显式提供的字段（如 `animation` 的
    /// `auto defaultValue = LinearFunctionCurve : {...}`）按类声明的
    /// `default_value` 常量补齐：标量直接写入；`InjectObject`/`Array`
    /// 递归物化为注入器/数组对象后写入 object 槽位。
    pub fn apply_injector_defaults(&mut self, injector_id: usize) -> VmResult<()> {
        let mut pending: Vec<(usize, bool, crate::objective::bytecode::InjectorConstField)> = Vec::new();
        {
            let Some(injector) = self.injectors.get_mut(&injector_id) else {
                return Ok(());
            };
            Self::apply_injector_defaults_to(injector, &mut pending)?;
        }
        for (object_index, is_array, default_value) in pending {
            let object_id = self.materialize_injector_default_const(&default_value)?;
            // 默认值本身是 Array 常量时已是数组（元素为具体类注入器），
            // 只有单对象默认值（InjectObject）才需要按数组字段包装。
            let final_id = if is_array
                && matches!(default_value, crate::objective::bytecode::InjectorConstField::InjectObject(..))
            {
                // 数组字段默认值若是单对象（如 laneLines 的
                // `FunctionCurve^ : {ConstantFunctionCurve : {...}}`），
                // 按注入器语义包装为单元素数组，否则字段初始化器
                // `new (^field)[^field.length]` 会得到空数组。
                self.materialize_object_array(&[object_id])?
            } else {
                object_id
            };
            if let Some(injector) = self.injectors.get_mut(&injector_id) {
                injector.set_injector_object(object_index, final_id);
            }
        }
        Ok(())
    }

    /// 将一组对象 ID 物化为 ObjectArray 编译层包装（chart JSON 数组字段）。
    ///
    /// 数组元素为已物化的注入器/对象 ID；返回包装对象 ID（int 字段 0 为
    /// length），供编译侧 `arr.length` / `arr[i]` 访问。
    pub fn materialize_object_array(&mut self, item_ids: &[usize]) -> VmResult<usize> {
        use crate::objective::bytecode::InjectorConstField;
        let fields: Vec<InjectorConstField> = item_ids.iter()
            .map(|id| InjectorConstField::Object(String::new(), *id))
            .collect();
        self.materialize_injector_array(&fields)
    }

    /// 单注入器默认值应用（标量就地写入；对象默认值先收集到 `pending`，
    /// 由调用方在脱离注入器借用后递归物化）。
    fn apply_injector_defaults_to(
        injector: &mut crate::system::native::injector::RuntimeInjector,
        pending: &mut Vec<(usize, bool, crate::objective::bytecode::InjectorConstField)>,
    ) -> VmResult<()> {
        use crate::objective::bytecode::InjectorConstField;
        use crate::objective::types::BasicType;
        use crate::system::native::injector::Injector;

        let declaration = injector.injection_class_declaration().clone();
        let mut int_index = 0usize;
        let mut float_index = 0usize;
        let mut bool_index = 0usize;
        let mut string_index = 0usize;
        let mut object_index = 0usize;
        for field in &declaration.injector_fields {
            match field.field_type.basic_type {
                BasicType::Int | BasicType::Enum => {
                    if injector.get_injector_int_default_value(int_index) {
                        if let Some(InjectorConstField::Int(_, value)) = &field.default_value {
                            injector.set_injector_int(int_index, *value);
                        }
                    }
                    int_index += 1;
                }
                BasicType::Float => {
                    if injector.get_injector_float_default_value(float_index) {
                        if let Some(InjectorConstField::Float(_, value)) = &field.default_value {
                            injector.set_injector_float(float_index, *value);
                        }
                    }
                    float_index += 1;
                }
                BasicType::Bool => {
                    if injector.get_injector_bool_default_value(bool_index) {
                        if let Some(InjectorConstField::Bool(_, value)) = &field.default_value {
                            injector.set_injector_bool(bool_index, *value);
                        }
                    }
                    bool_index += 1;
                }
                BasicType::String => {
                    if injector.get_injector_string_default_value(string_index) {
                        if let Some(InjectorConstField::String(_, value)) = &field.default_value {
                            injector.set_injector_string(string_index, value.clone());
                        }
                    }
                    string_index += 1;
                }
                BasicType::Object | BasicType::Interface | BasicType::Delegate => {
                    if injector.get_injector_object_default_value(object_index) {
                        if let Some(default_value) = &field.default_value {
                            pending.push((object_index, field.is_array, default_value.clone()));
                        }
                    }
                    object_index += 1;
                }
                BasicType::Void | BasicType::Null => {}
            }
        }
        Ok(())
    }

    /// 将注入器字段默认值常量物化为对象 ID（仅对象型默认值）。
    fn materialize_injector_default_const(
        &mut self,
        default_value: &crate::objective::bytecode::InjectorConstField,
    ) -> VmResult<usize> {
        use crate::objective::bytecode::InjectorConstField;
        match default_value {
            InjectorConstField::InjectObject(class_name, _, fields) => {
                self.materialize_injector_constant(&crate::objective::bytecode::InjectorConstantDef {
                    class_name: class_name.clone(),
                    fields: fields.clone(),
                })
            }
            InjectorConstField::Array(_, elements) => self.materialize_injector_array(elements),
            InjectorConstField::Object(_, object_id) => Ok(*object_id),
            _ => Ok(0),
        }
    }

    /// 为注入器常量解析类声明。
    ///
    /// 常量中保存源码中的类名（可能为全限定名），而 VM 类表按 loader 约定
    /// 以简单名注册，因此按「全名 → 末段短名」双键解析；未注册的类回退为
    /// 哑声明（字段槽位按常量字段数分配，保证标量数据不丢失）。
    fn injector_declaration_for_constant(
        &self,
        class_name: &str,
    ) -> std::sync::Arc<crate::objective::declaration::ClassDeclaration> {
        // 编译类走 class_table，native 类走 native_class_table（如
        // ConstantFunctionCurve 等曲线族，仅注册在 native 表）。
        let simple = class_name.rsplit('.').next().unwrap_or(class_name);
        let candidates = [class_name, simple];
        for name in candidates {
            if let Some(cls) = self.class_table.get(name) {
                let mut decl = cls.declaration.clone();
                decl.class_type = crate::objective::types::GorgeType::class(name.to_string(), None);
                return std::sync::Arc::new(decl);
            }
            if let Some(native) = self.native_class_table.get(name) {
                let mut decl = native.declaration();
                decl.class_type = crate::objective::types::GorgeType::class(name.to_string(), None);
                return std::sync::Arc::new(decl);
            }
        }
        // 未注册的类：使用哑声明（槽位按类型计数 0 分配，标量字段越界时
        // 静默跳过，保证常量物化不失败）
        std::sync::Arc::new(crate::objective::declaration::ClassDeclaration::dummy(class_name.to_string()))
    }

    /// 将注入器数组常量物化为 native 数组对象，返回对象 ID。
    ///
    /// 数组项为 `InjectObject` 时递归物化为注入器对象，其余标量项直接写入
    /// 对应类型的 native 数组载荷；元素类型由常量中首项推导，空数组退化为
    /// ObjectArray。同时创建编译层包装 RuntimeObject（int 字段 0 存放 length，
    /// 对齐 `InvokeArrayConstructor` 的数组构造语义）。
    fn materialize_injector_array(
        &mut self,
        elements: &[crate::objective::bytecode::InjectorConstField],
    ) -> VmResult<usize> {
        use crate::system::native::array::*;

        // 对象与接口数组统一使用 ObjectArray；标量数组按基本类型映射。
        let elem_type_name = match elements.first() {
            Some(crate::objective::bytecode::InjectorConstField::InjectObject(..)) => "object".to_string(),
            Some(crate::objective::bytecode::InjectorConstField::Int(..)) => "int".to_string(),
            Some(crate::objective::bytecode::InjectorConstField::Float(..)) => "float".to_string(),
            Some(crate::objective::bytecode::InjectorConstField::Bool(..)) => "bool".to_string(),
            Some(crate::objective::bytecode::InjectorConstField::String(..)) => "string".to_string(),
            _ => "object".to_string(),
        };
        let array_class = Self::native_array_class_name(&elem_type_name).to_string();

        // 预分配元素 ID（避免与插入 native 载荷的借用冲突）
        let element_ids: Vec<Option<usize>> = elements.iter()
            .map(|f| match f {
                crate::objective::bytecode::InjectorConstField::InjectObject(nested_class, _, fields) => {
                    let nested = crate::objective::bytecode::InjectorConstantDef {
                        class_name: nested_class.clone(),
                        fields: fields.clone(),
                    };
                    match self.materialize_injector_constant(&nested) {
                        Ok(id) => {
                        Some(id)
                        }
                        Err(_error) => {
                            None
                        }
                    }
                }
                _ => None,
            })
            .collect();

        // 元素为对象 ID 的数组项：无论推导出的元素类型如何都写入 ObjectArray
        let use_object_array = elements.iter().any(|f| matches!(
            f,
            crate::objective::bytecode::InjectorConstField::InjectObject(..)
        ));

        // 分配对象 ID 并写入对应类型的 native 载荷
        let array_id = self.next_object_id;
        self.next_object_id += 1;
        if use_object_array {
            let mut items: Vec<usize> = Vec::with_capacity(elements.len());
            for (i, f) in elements.iter().enumerate() {
                let item_id = match f {
                    crate::objective::bytecode::InjectorConstField::InjectObject(..) => {
                        element_ids[i].unwrap_or(0)
                    }
                    crate::objective::bytecode::InjectorConstField::Object(_, v) => *v,
                    _ => 0,
                };
                items.push(item_id);
            }
            self.native_payloads.insert(array_id, Box::new(ObjectArray { items }));
        } else if elem_type_name == "int" {
            let items: Vec<i64> = elements.iter().map(|f| match f {
                crate::objective::bytecode::InjectorConstField::Int(_, v) => *v,
                _ => 0,
            }).collect();
            self.native_payloads.insert(array_id, Box::new(IntArray { items }));
        } else if elem_type_name == "float" {
            let items: Vec<f64> = elements.iter().map(|f| match f {
                crate::objective::bytecode::InjectorConstField::Float(_, v) => *v,
                _ => 0.0,
            }).collect();
            self.native_payloads.insert(array_id, Box::new(FloatArray { items }));
        } else if elem_type_name == "bool" {
            let items: Vec<bool> = elements.iter().map(|f| match f {
                crate::objective::bytecode::InjectorConstField::Bool(_, v) => *v,
                _ => false,
            }).collect();
            self.native_payloads.insert(array_id, Box::new(BoolArray { items }));
        } else if elem_type_name == "string" {
            let items: Vec<String> = elements.iter().map(|f| match f {
                crate::objective::bytecode::InjectorConstField::String(_, v) => v.clone(),
                _ => String::new(),
            }).collect();
            self.native_payloads.insert(array_id, Box::new(StringArray { items }));
        } else {
            let items: Vec<usize> = elements.iter().map(|f| match f {
                crate::objective::bytecode::InjectorConstField::Object(_, v) => *v,
                _ => 0,
            }).collect();
            self.native_payloads.insert(array_id, Box::new(ObjectArray { items }));
        }

        // 编译层包装对象：int 字段 0 存放 length（对齐 InvokeArrayConstructor）
        let wrapper_id = self.next_object_id;
        self.next_object_id += 1;
        let mut wrapper = RuntimeObject::new_simple(
            array_class.clone(),
            &crate::objective::types::TypeCount { int_count: 1, ..crate::objective::types::TypeCount::zero() },
        );
        wrapper.set_int_field(0, elements.len() as i64);
        wrapper.native_object_id = Some(array_id);
        self.objects.insert(wrapper_id, wrapper);
        if let Some(native_obj) = self.objects.get_mut(&array_id) {
            native_obj.outer_compiled_id = Some(wrapper_id);
        }
        Ok(wrapper_id)
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

    /// 获取 string 类型的返回值
    pub fn get_return_string(&self) -> Option<String> {
        self.return_string.clone()
    }

    // ==================== 编辑期对象比较 / 哈希（谱面编辑器核心） ====================

    /// 编辑期比较两个对象是否等价（完整递归版，对齐 C# `GorgeObject.EditableEquals`）。
    ///
    /// 按运行时类型分派：
    /// - 两者都是注入器 → 递归比较（值类型字段 + object 字段逐一递归）；
    /// - 两者都是同种原生列表（Int/Float/Bool/String/Object List）→ 逐元素比较，
    ///   ObjectList 元素再次递归；
    /// - 其余情况 → 对象 ID 相等即等价。
    ///
    /// ID 为 0 视为「空对象」，双空相等、单空不等。
    pub fn editable_equals_objects(&self, a_id: usize, b_id: usize) -> bool {
        if a_id == 0 && b_id == 0 { return true; }
        if a_id == 0 || b_id == 0 { return false; }

        // 两者都是注入器：递归比较
        if let (Some(ia), Some(ib)) = (self.injectors.get(&a_id), self.injectors.get(&b_id)) {
            // 值类型字段先比较（含类名/字段数量/默认值标记）
            if !ia.editable_equals_values(ib) {
                return false;
            }
            // object 字段逐一递归比较
            for i in 0..ia.object_field_count() {
                let (av, a_def) = ia.object_field(i);
                let (bv, b_def) = ib.object_field(i);
                if a_def != b_def { return false; }
                if a_def { continue; }
                if !self.editable_equals_objects(av, bv) {
                    return false;
                }
            }
            return true;
        }

        // 两者都是原生列表：按具体列表类型逐元素比较
        if let (Some(pa), Some(pb)) = (self.native_payloads.get(&a_id), self.native_payloads.get(&b_id)) {
            if let (Some(la), Some(lb)) = (pa.downcast_ref::<IntList>(), pb.downcast_ref::<IntList>()) {
                return la.items == lb.items;
            }
            if let (Some(la), Some(lb)) = (pa.downcast_ref::<FloatList>(), pb.downcast_ref::<FloatList>()) {
                return la.items.len() == lb.items.len()
                    && la.items.iter().zip(lb.items.iter()).all(|(x, y)| x == y);
            }
            if let (Some(la), Some(lb)) = (pa.downcast_ref::<BoolList>(), pb.downcast_ref::<BoolList>()) {
                return la.items == lb.items;
            }
            if let (Some(la), Some(lb)) = (pa.downcast_ref::<StringList>(), pb.downcast_ref::<StringList>()) {
                return la.items == lb.items;
            }
            if let (Some(la), Some(lb)) = (pa.downcast_ref::<ObjectList>(), pb.downcast_ref::<ObjectList>()) {
                if la.items.len() != lb.items.len() { return false; }
                // 逐元素递归比较（元素可能是嵌套注入器/列表）
                let pairs: Vec<(usize, usize)> = la.items.iter().copied().zip(lb.items.iter().copied()).collect();
                return pairs.into_iter().all(|(x, y)| self.editable_equals_objects(x, y));
            }
        }

        // 其余对象：ID 相等即等价
        a_id == b_id
    }

    /// 编辑期计算对象哈希（完整递归版，对齐 C# `GorgeObject.EditableHashCode`）。
    ///
    /// 保证：`editable_equals_objects` 判定相等的两个对象产生相同哈希。
    pub fn editable_hash_code_object(&self, id: usize) -> u64 {
        let mut state = DefaultHasher::new();
        self.hash_object_into(id, &mut state);
        state.finish()
    }

    /// 将对象内容混入哈希器（供 `editable_hash_code_object` 递归调用）。
    fn hash_object_into<H: Hasher>(&self, id: usize, state: &mut H) {
        if id == 0 {
            0u8.hash(state);
            return;
        }
        // 注入器：混入值类型字段 + object 字段递归
        if let Some(inj) = self.injectors.get(&id) {
            1u8.hash(state);
            inj.hash_values(state);
            for i in 0..inj.object_field_count() {
                let (v, is_def) = inj.object_field(i);
                if is_def {
                    true.hash(state);
                } else {
                    self.hash_object_into(v, state);
                }
            }
            return;
        }
        // 原生列表：混入元素（ObjectList 递归）
        if let Some(p) = self.native_payloads.get(&id) {
            if let Some(l) = p.downcast_ref::<IntList>() {
                2u8.hash(state); l.items.hash(state); return;
            }
            if let Some(l) = p.downcast_ref::<FloatList>() {
                3u8.hash(state);
                for v in &l.items { v.to_bits().hash(state); }
                return;
            }
            if let Some(l) = p.downcast_ref::<BoolList>() {
                4u8.hash(state); l.items.hash(state); return;
            }
            if let Some(l) = p.downcast_ref::<StringList>() {
                5u8.hash(state); l.items.hash(state); return;
            }
            if let Some(l) = p.downcast_ref::<ObjectList>() {
                6u8.hash(state);
                let ids: Vec<usize> = l.items.clone();
                for eid in ids { self.hash_object_into(eid, state); }
                return;
            }
        }
        // 其余对象：混入 ID
        7u8.hash(state);
        id.hash(state);
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
            class_field_initializers: self.class_field_initializers.clone(),
            class_static_fields: self.class_static_fields.clone(),
            class_delegate_impls: self.class_delegate_impls.clone(),
            runtime_delegates: HashMap::new(),
            class_table: self.class_table.clone(),
            class_name_by_id: self.class_name_by_id.clone(),
            native_class_table: self.native_class_table.clone(),
            native_payloads: HashMap::new(),
            class_super_name: self.class_super_name.clone(),
            injector_constants: self.injector_constants.clone(),
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

    fn make_string_addr(index: usize) -> Address {
        Address::new(ValueType::String, index)
    }

    fn make_object_addr(index: usize) -> Address {
        Address::new(ValueType::Object, index)
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
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::GorgeType;

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
            injector_constructor_impl_id: vec![],
              method_annotations: std::collections::HashMap::new(),
              constructor_annotations: std::collections::HashMap::new(),
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
    fn test_invoke_instance_method_returns_float() {
        // 回归测试：实例方法返回 float，返回值应写入 float 结果槽（此前只写回 return_int 会丢失）
        use std::sync::Arc;
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::GorgeType;

        // 方法体：float 局部 0 = 3.5，然后 ReturnFloat
        let callee_method = CompiledMethod {
            name: "getValue".into(),
            codes: vec![
                CodeWithSpan::new(
                    IntermediateCode::assign(
                        Address::new(ValueType::Float, 0),
                        Operand::float(3.5),
                    ),
                    crate::diagnostics::Span::dummy(),
                ),
                CodeWithSpan::new(
                    IntermediateCode::return_value(ValueType::Float),
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
            injector_constructor_impl_id: vec![],
              method_annotations: std::collections::HashMap::new(),
              constructor_annotations: std::collections::HashMap::new(),
          };
        let mut cls = RuntimeClass::new(decl, None);
        cls.register_method(0, callee_method);
        let cls_arc = Arc::new(cls);

        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("Widget", cls_arc.clone());
        vm.register_class_field_counts("Widget", TypeCount::zero());

        let obj_id = vm.next_object_id;
        vm.next_object_id += 1;
        let obj = RuntimeObject::new_simple("Widget".into(), &TypeCount::zero());
        vm.objects.insert(obj_id, obj);
        vm.object_stack.push_frame(3);
        vm.int_stack.push_frame(3);
        vm.float_stack.push_frame(3);
        vm.bool_stack.push_frame(3);
        vm.string_stack.push_frame(3);

        // 结果地址为 float 槽 1
        let result_addr = Address::new(ValueType::Float, 1);
        let invoke_code = IntermediateCode::new(
            IntermediateOperator::InvokeInstance(0),
            Operand::Address(Address::new(ValueType::Object, 0)),
            None,
            Some(result_addr),
        );
        vm.object_stack.write(0, obj_id);
        let _advance = vm.execute_one(&invoke_code).unwrap();

        // float 返回值正确写入 float 结果槽
        assert!((*vm.float_stack.read(1) - 3.5).abs() < 1e-9);
    }

    #[test]
    fn test_invoke_constructor() {
        use std::sync::Arc;
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::GorgeType;

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
                    Operand::Address(Address::new(ValueType::Object, 0)), // left = this 对象
                    Some(Operand::Address(Address::new(ValueType::Int, 0))), // right = 值
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
            injector_constructor_impl_id: vec![],
              method_annotations: std::collections::HashMap::new(),
              constructor_annotations: std::collections::HashMap::new(),
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
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::GorgeType;

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
            injector_constructor_impl_id: vec![],
              method_annotations: std::collections::HashMap::new(),
              constructor_annotations: std::collections::HashMap::new(),
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
        // 在 object_stack[1] 设为当前注入器
        vm.object_stack.write(1, inj_id);
        let inj_addr = Operand::Address(Address::new(ValueType::Object, 1));

        // SetIntInjectorField(0, 77)
        let set_code = IntermediateCode::new(
            IntermediateOperator::SetIntInjectorField(0),
            Operand::int(77),
            Some(inj_addr.clone()),
            None,
        );
        let _ = vm.execute_one(&set_code).unwrap();

        // LoadIntInjectorField(0) → result int[2]
        let result = Address::new(ValueType::Int, 2);
        let load_code = IntermediateCode::new(
            IntermediateOperator::LoadIntInjectorField(0),
            inj_addr.clone(),
            None,
            Some(result),
        );
        let _ = vm.execute_one(&load_code).unwrap();
        assert_eq!(*vm.int_stack.read(2), 77, "注入器字段值应为 77");

        // 验证默认值标记：未设置的字段 1 应返回默认值
        let load_default = IntermediateCode::new(
            IntermediateOperator::LoadIntInjectorField(1),
            inj_addr.clone(),
            None,
            Some(Address::new(ValueType::Int, 3)),
        );
        let _ = vm.execute_one(&load_default).unwrap();
        assert_eq!(*vm.int_stack.read(3), 0, "未设置的注入器字段默认值应为 0");
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

    // ==================== Phase A: Native 互操作验收测试 ====================

    /// 手写的最小 native 类，用于验收 VM 的 native 分派链路
    ///
    /// 提供：
    /// - 0 号静态方法 `addOne(int) -> int`：参数 +1
    /// - 0 号构造方法：创建一个带 1 个 float 字段的对象，字段值取自参数池 float[0]
    /// - 0 号实例方法 `getValue() -> float`：读取对象 float 字段 0
    #[derive(Debug)]
    struct DemoNativeClass {
        name: String,
        counts: TypeCount,
    }

    impl crate::objective::native::NativeClass for DemoNativeClass {
        fn full_name(&self) -> &str {
            &self.name
        }

        fn field_type_count(&self) -> &TypeCount {
            &self.counts
        }

        fn invoke_native_method(
            &self,
            ctx: &mut crate::objective::native::NativeContext,
            obj_id: usize,
            method_id: usize,
        ) {
            // 0 号实例方法：getValue() -> float
            if method_id == 0 {
                let v = ctx.get_object_float_field(obj_id, 0);
                ctx.set_float_return(v);
            }
        }

        fn invoke_native_static(
            &self,
            ctx: &mut crate::objective::native::NativeContext,
            method_id: usize,
        ) {
            // 0 号静态方法：addOne(int) -> int
            if method_id == 0 {
                let a = ctx.get_int_param(0);
                ctx.set_int_return(a + 1);
            }
        }

        fn do_construct_native(
            &self,
            ctx: &mut crate::objective::native::NativeContext,
            target: Option<usize>,
            _ctor_id: usize,
        ) -> usize {
            // 从参数池读取 float[0] 作为字段初值
            let init = ctx.get_float_param(0);
            let id = match target {
                Some(id) => id,
                None => {
                    let obj = RuntimeObject::new_simple(self.name.clone(), &self.counts);
                    ctx.register_object(obj)
                }
            };
            ctx.set_object_float_field(id, 0, init);
            id
        }
    }

    /// 构造一个带单个 float 字段的 demo native 类
    fn make_demo_native() -> std::sync::Arc<DemoNativeClass> {
        std::sync::Arc::new(DemoNativeClass {
            name: "Demo.Native".into(),
            counts: TypeCount { float_count: 1, ..TypeCount::zero() },
        })
    }

    #[test]
    fn test_native_registration_supports_full_and_simple_names() {
        let mut vm = VirtualMachine::new();
        vm.register_native_class("Native", make_demo_native());

        assert!(vm.native_class_table.contains_key("Demo.Native"));
        assert!(vm.native_class_table.contains_key("Native"));
    }

    #[test]
    fn test_native_invoke_static_via_ir() {
        // 验收核心场景：一段 IR 调用 native 静态方法并取回返回值
        let mut vm = VirtualMachine::new();
        vm.register_native_class("Demo.Native", make_demo_native());
        vm.set_current_class("Demo.Native");
        vm.push_frame(4);

        // 准备参数：param[0] = 41（int）
        let arg = Address::new(ValueType::Int, 0);
        vm.int_stack.write(0, 41);
        let set_param = IntermediateCode::new(
            IntermediateOperator::SetIntParameter,
            Operand::Address(arg),
            None,
            Some(Address::new(ValueType::Int, 0)),
        );
        vm.execute_one(&set_param).unwrap();

        // InvokeStatic(0)，结果写入 int 地址 1
        let result = Address::new(ValueType::Int, 1);
        let invoke = IntermediateCode::new(
            IntermediateOperator::InvokeStatic(0),
            Operand::int(1),
            None,
            Some(result),
        );
        vm.execute_one(&invoke).unwrap();

        // native addOne(41) = 42
        assert_eq!(*vm.int_stack.read(1), 42);
    }

    #[test]
    fn test_native_construct_and_instance_method_via_ir() {
        // 验收：native 构造 + native 实例方法读字段
        let mut vm = VirtualMachine::new();
        vm.register_native_class("Demo.Native", make_demo_native());
        vm.set_current_class("Demo.Native");
        vm.push_frame(4);

        // 准备构造参数：float param[0] = 3.5
        vm.float_stack.write(0, 3.5);
        let set_fp = IntermediateCode::new(
            IntermediateOperator::SetFloatParameter,
            Operand::Address(Address::new(ValueType::Float, 0)),
            None,
            Some(Address::new(ValueType::Int, 0)),
        );
        vm.execute_one(&set_fp).unwrap();

        // InvokeConstructor(0)，对象 ID 写入 object 地址 1
        let obj_addr = Address::new(ValueType::Object, 1);
        let construct = IntermediateCode::new(
            IntermediateOperator::InvokeConstructor(0),
            Operand::int(1),
            None,
            Some(obj_addr),
        );
        vm.execute_one(&construct).unwrap();
        let obj_id = *vm.object_stack.read(1);
        assert!(obj_id != 0, "构造应返回有效对象 ID");
        assert!(vm.objects.contains_key(&obj_id), "对象应进入对象表");

        // 调用实例方法 getValue()：InvokeInstance(0)，left = 目标对象 ID
        let ret = Address::new(ValueType::Float, 2);
        let invoke = IntermediateCode::new(
            IntermediateOperator::InvokeInstance(0),
            Operand::Address(obj_addr),
            None,
            Some(ret),
        );
        vm.execute_one(&invoke).unwrap();

        // 字段初值 3.5 应被读回
        assert_eq!(*vm.float_stack.read(2), 3.5);
    }

    #[test]
    fn test_native_compiled_bidirectional_link() {
        // 验收 A5：native 对象与编译对象的双向引用与 RealObject 解析
        let mut vm = VirtualMachine::new();

        // 手工放入两个对象：一个编译对象、一个 native 子对象
        let compiled_id = 1usize;
        let native_id = 2usize;
        vm.objects.insert(
            compiled_id,
            RuntimeObject::new_simple("Compiled.Sub".into(), &TypeCount::zero()),
        );
        vm.objects.insert(
            native_id,
            RuntimeObject::new_simple("Demo.Native".into(), &TypeCount::zero()),
        );

        vm.link_native_and_compiled(compiled_id, native_id);

        // 双向引用建立
        assert_eq!(vm.objects.get(&compiled_id).unwrap().native_object_id, Some(native_id));
        assert_eq!(vm.objects.get(&native_id).unwrap().outer_compiled_id, Some(compiled_id));

        // native 子对象的真实对象应解析为外层编译对象
        assert_eq!(vm.resolve_real_object_id(native_id), compiled_id);
        // 普通对象的真实对象是自身
        assert_eq!(vm.resolve_real_object_id(compiled_id), compiled_id);
    }

    // ==================== Phase O: 非 this 对象字段读写测试 ====================

    #[test]
    fn test_non_this_field_load() {
        // 验证 LoadField 从 code.left 操作数读取对象引用（不再硬编码 object_stack[0]）
        let mut vm = VirtualMachine::new();
        vm.int_stack.push_frame(4);
        vm.object_stack.push_frame(4);

        let field_counts = TypeCount { int_count: 2, ..TypeCount::zero() };
        // 创建两个对象：obj_a 在 slot 2（字段值 10），obj_b 在 slot 3（字段值 20）
        let id_a = vm.next_object_id; vm.next_object_id += 1;
        let mut obj_a = RuntimeObject::new_simple("A".into(), &field_counts);
        obj_a.set_int_field(0, 10);
        vm.objects.insert(id_a, obj_a);

        let id_b = vm.next_object_id; vm.next_object_id += 1;
        let mut obj_b = RuntimeObject::new_simple("B".into(), &field_counts);
        obj_b.set_int_field(0, 20);
        vm.objects.insert(id_b, obj_b);

        // this (slot 0) 和 obj_a (slot 2) 不同
        vm.object_stack.write(0, id_a); // this = obj_a
        vm.object_stack.write(2, id_b); // 堆上另一个对象是 obj_b

        // LoadIntField(0) → left=object_stack[2] (obj_b), result=int[1]
        let result = Address::new(ValueType::Int, 1);
        let code = IntermediateCode::new(
            IntermediateOperator::LoadIntField(0),
            Operand::Address(Address::new(ValueType::Object, 2)), // left = obj_b
            None,
            Some(result),
        );
        let _ = vm.execute_one(&code).unwrap();
        // 应返回 obj_b 的字段值 20，而非 this (obj_a) 的字段值 10
        assert_eq!(*vm.int_stack.read(1), 20, "非 this 字段读取应使用 left 操作数指定的对象");
    }

    #[test]
    fn test_non_this_field_set() {
        // 验证 SetField 从 code.left 读对象引用、从 code.right 读值
        let mut vm = VirtualMachine::new();
        vm.int_stack.push_frame(4);
        vm.object_stack.push_frame(4);

        let field_counts = TypeCount { int_count: 2, float_count: 1, ..TypeCount::zero() };
        let id_a = vm.next_object_id; vm.next_object_id += 1;
        let obj_a = RuntimeObject::new_simple("A".into(), &field_counts);
        vm.objects.insert(id_a, obj_a);

        let id_b = vm.next_object_id; vm.next_object_id += 1;
        let obj_b = RuntimeObject::new_simple("B".into(), &field_counts);
        vm.objects.insert(id_b, obj_b);

        vm.object_stack.write(0, id_a); // this = obj_a
        vm.object_stack.write(2, id_b); // obj_b

        // SetFloatField(0): left=obj_b, right=值 3.14
        let code = IntermediateCode::new(
            IntermediateOperator::SetFloatField(0),
            Operand::Address(Address::new(ValueType::Object, 2)), // left = obj_b
            Some(Operand::float(3.14)), // right = 值
            None,
        );
        let _ = vm.execute_one(&code).unwrap();
        // obj_b.float[0] 应为 3.14
        assert!((vm.objects.get(&id_b).unwrap().get_float_field(0) - 3.14).abs() < 0.001);
        // this 的字段不应被改动
        assert!((vm.objects.get(&id_a).unwrap().get_float_field(0) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_non_this_string_field_set_and_load() {
        // 验证 String 字段的非 this 读写
        let mut vm = VirtualMachine::new();
        vm.string_stack.push_frame(4);
        vm.object_stack.push_frame(4);

        let field_counts = TypeCount { string_count: 1, ..TypeCount::zero() };
        let id = vm.next_object_id; vm.next_object_id += 1;
        let obj = RuntimeObject::new_simple("C".into(), &field_counts);
        vm.objects.insert(id, obj);
        vm.object_stack.write(2, id);

        // SetStringField(0): left=obj[2], right="hello"
        let set_code = IntermediateCode::new(
            IntermediateOperator::SetStringField(0),
            Operand::Address(Address::new(ValueType::Object, 2)),
            Some(Operand::string("hello")),
            None,
        );
        let _ = vm.execute_one(&set_code).unwrap();

        // LoadStringField(0): left=obj[2], result=string[0]
        let load_code = IntermediateCode::new(
            IntermediateOperator::LoadStringField(0),
            Operand::Address(Address::new(ValueType::Object, 2)),
            None,
            Some(Address::new(ValueType::String, 0)),
        );
        let _ = vm.execute_one(&load_code).unwrap();
        assert_eq!(vm.string_stack.read(0), "hello");
    }

    // ==================== Phase P: 字段初始化器测试 ====================

    #[test]
    fn test_field_initializer_executes_before_constructor() {
        // 验证字段初始化器在构造方法体前执行
        use std::sync::Arc;
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::GorgeType;
        use crate::objective::bytecode::CompiledFieldInitializer;

        let field_counts = TypeCount { int_count: 2, ..TypeCount::zero() };

        // 构造方法体：将字段 1（本类字段）设为 200
        let ctor_codes = vec![
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::LoadThis,
                    Operand::int(0), None,
                    Some(Address::new(ValueType::Object, 1)),
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::SetIntField(1),
                    Operand::Address(Address::new(ValueType::Object, 1)),
                    Some(Operand::int(200)),
                    None,
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(IntermediateCode::return_void(), crate::diagnostics::Span::dummy()),
        ];

        // 字段初始化器：将字段 0 设为 100（先于构造体执行）
        let init_codes = vec![
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::LoadThis,
                    Operand::int(0), None,
                    Some(Address::new(ValueType::Object, 0)),
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::SetIntField(0),
                    Operand::Address(Address::new(ValueType::Object, 0)),
                    Some(Operand::int(100)),
                    None,
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(IntermediateCode::return_void(), crate::diagnostics::Span::dummy()),
        ];

        let ctor = CompiledMethod { name: "MyClass".into(), codes: ctor_codes, local_count: 2 };
        let decl = ClassDeclaration {
            class_type: GorgeType::class("MyClass", None),
            is_native: false, annotations: vec![], fields: vec![],
            methods: vec![], static_methods: vec![],
            constructors: vec![], injector_fields: vec![],
            super_class: None, super_interfaces: vec![],
            field_type_count: field_counts.clone(),
            method_count: 0, static_method_count: 0, constructor_count: 1,
            injector_field_type_count: TypeCount::zero(),
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0, constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
            injector_constructor_impl_id: vec![],
              method_annotations: std::collections::HashMap::new(),
              constructor_annotations: std::collections::HashMap::new(),
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

        vm.register_runtime_class("MyClass", cls_arc.clone());
        vm.register_class_field_counts("MyClass", field_counts);
        // 注册字段初始化器：字段 0
        vm.register_class_field_initializers("MyClass", vec![
            CompiledFieldInitializer {
                field_index: 0,
                value_type: ValueType::Int,
                local_count: 2,
                codes: init_codes,
            },
        ]);
        vm.set_current_class("MyClass");

        // InvokeConstructor(0), 0 个参数
        let result_obj = Address::new(ValueType::Object, 2);
        let invoke_ctor = IntermediateCode::new(
            IntermediateOperator::InvokeConstructor(0),
            Operand::int(0),
            None,
            Some(result_obj),
        );
        let _ = vm.execute_one(&invoke_ctor).unwrap();

        let obj_id = *vm.object_stack.read(2);
        assert_ne!(obj_id, 0);
        let obj = vm.objects.get(&obj_id).expect("应有新对象");
        // 字段 0 由初始化器设为 100
        assert_eq!(obj.get_int_field(0), 100, "字段初始化器应设置字段 0=100");
        // 字段 1 由构造体设为 200
        assert_eq!(obj.get_int_field(1), 200, "构造方法体应设置字段 1=200");
    }

    // ==================== 编辑期对象比较 / 哈希 递归测试 ====================

    /// 构造一个仅含 `int_count` 个 int 注入器字段的注入器声明。
    fn injector_decl_with_ints(name: &str, int_count: usize) -> std::sync::Arc<crate::objective::declaration::ClassDeclaration> {
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::{GorgeType, TypeCount};
        std::sync::Arc::new(ClassDeclaration {
            class_type: GorgeType::class(name, None),
            is_native: false, annotations: vec![], fields: vec![],
            methods: vec![], static_methods: vec![], constructors: vec![],
            injector_fields: vec![], super_class: None, super_interfaces: vec![],
            field_type_count: TypeCount::zero(),
            method_count: 0, static_method_count: 0, constructor_count: 0,
            injector_field_type_count: TypeCount { int_count, ..TypeCount::zero() },
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0, constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
            injector_constructor_impl_id: vec![],
            method_annotations: HashMap::new(),
            constructor_annotations: HashMap::new(),
        })
    }

    /// 构造一个含 1 个 object 注入器字段的注入器声明（用于嵌套注入器）。
    fn injector_decl_with_objects(name: &str, object_count: usize) -> std::sync::Arc<crate::objective::declaration::ClassDeclaration> {
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::{GorgeType, TypeCount};
        std::sync::Arc::new(ClassDeclaration {
            class_type: GorgeType::class(name, None),
            is_native: false, annotations: vec![], fields: vec![],
            methods: vec![], static_methods: vec![], constructors: vec![],
            injector_fields: vec![], super_class: None, super_interfaces: vec![],
            field_type_count: TypeCount::zero(),
            method_count: 0, static_method_count: 0, constructor_count: 0,
            injector_field_type_count: TypeCount { object_count, ..TypeCount::zero() },
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0, constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
            injector_constructor_impl_id: vec![],
            method_annotations: HashMap::new(),
            constructor_annotations: HashMap::new(),
        })
    }

    #[test]
    fn test_editable_equals_flat_injectors() {
        let mut vm = VirtualMachine::new();
        let mut a = RuntimeInjector::new(injector_decl_with_ints("V", 1));
        let mut b = RuntimeInjector::new(injector_decl_with_ints("V", 1));
        a.set_injector_int(0, 5);
        b.set_injector_int(0, 5);
        vm.injectors.insert(10, a);
        vm.injectors.insert(11, b);
        assert!(vm.editable_equals_objects(10, 11));
        // 修改 b 的值 → 不等
        vm.injectors.get_mut(&11).unwrap().set_injector_int(0, 6);
        assert!(!vm.editable_equals_objects(10, 11));
    }

    #[test]
    fn test_editable_equals_nested_injectors() {
        // 外层注入器各含一个 object 字段指向内层注入器，内层相等则外层相等
        let mut vm = VirtualMachine::new();
        let mut inner_a = RuntimeInjector::new(injector_decl_with_ints("Inner", 1));
        let mut inner_b = RuntimeInjector::new(injector_decl_with_ints("Inner", 1));
        inner_a.set_injector_int(0, 9);
        inner_b.set_injector_int(0, 9);
        vm.injectors.insert(100, inner_a);
        vm.injectors.insert(101, inner_b);

        let mut outer_a = RuntimeInjector::new(injector_decl_with_objects("Outer", 1));
        let mut outer_b = RuntimeInjector::new(injector_decl_with_objects("Outer", 1));
        outer_a.set_injector_object(0, 100);
        outer_b.set_injector_object(0, 101);
        vm.injectors.insert(200, outer_a);
        vm.injectors.insert(201, outer_b);

        assert!(vm.editable_equals_objects(200, 201), "内层相等 → 外层相等");

        // 修改内层 b → 外层不等
        vm.injectors.get_mut(&101).unwrap().set_injector_int(0, 8);
        assert!(!vm.editable_equals_objects(200, 201), "内层不等 → 外层不等");
    }

    #[test]
    fn test_editable_equals_object_list() {
        // 两个 ObjectList 元素相等（元素是相等的注入器）→ 列表相等
        let mut vm = VirtualMachine::new();
        let mut ea = RuntimeInjector::new(injector_decl_with_ints("E", 1));
        let mut eb = RuntimeInjector::new(injector_decl_with_ints("E", 1));
        ea.set_injector_int(0, 1);
        eb.set_injector_int(0, 1);
        vm.injectors.insert(300, ea);
        vm.injectors.insert(301, eb);

        vm.native_payloads.insert(400, Box::new(ObjectList { items: vec![300] }));
        vm.native_payloads.insert(401, Box::new(ObjectList { items: vec![301] }));
        assert!(vm.editable_equals_objects(400, 401));

        // 元素不等 → 列表不等
        vm.injectors.get_mut(&301).unwrap().set_injector_int(0, 2);
        assert!(!vm.editable_equals_objects(400, 401));
    }

    #[test]
    fn test_editable_hash_code_equal_objects_same_hash() {
        let mut vm = VirtualMachine::new();
        let mut a = RuntimeInjector::new(injector_decl_with_ints("H", 1));
        let mut b = RuntimeInjector::new(injector_decl_with_ints("H", 1));
        a.set_injector_int(0, 77);
        b.set_injector_int(0, 77);
        vm.injectors.insert(500, a);
        vm.injectors.insert(501, b);
        assert!(vm.editable_equals_objects(500, 501));
        assert_eq!(
            vm.editable_hash_code_object(500),
            vm.editable_hash_code_object(501),
            "相等注入器哈希应相同"
        );
    }

    // === S1 委托测试 ===

    /// 委托返回 int 42 测试（1c）
    #[test]
    fn test_delegate_invoke_returns_int_42() {
        use crate::objective::delegate::RuntimeDelegate;
        use crate::objective::types::GorgeType;
        use crate::objective::types::BasicType;
        use crate::objective::value_pool::FixedFieldValuePool;

        // 构造一个返回 int 42 的委托方法体
        let result = make_int_addr(0);
        let method = CompiledMethod {
            name: "get42".into(),
            codes: vec![
                CodeWithSpan::new(
                    IntermediateCode::assign(
                        result,
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

        let delegate = RuntimeDelegate {
            delegate_type: GorgeType::new(BasicType::Delegate),
            method_impl: method,
            captured_values: FixedFieldValuePool::default(),
            param_types: vec![],
            captured_var_types: vec![],
            creator_this: None,
        };

        let mut vm = VirtualMachine::new();
        let obj_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.runtime_delegates.insert(obj_id, delegate);

        vm.invoke_delegate_object(obj_id).unwrap();

        assert_eq!(vm.param_pool.get_int_return(), 42);
    }

    /// 委托返回 float 3.14 测试（1c）
    #[test]
    fn test_delegate_invoke_returns_float_3_14() {
        use crate::objective::delegate::RuntimeDelegate;
        use crate::objective::types::GorgeType;
        use crate::objective::types::BasicType;
        use crate::objective::value_pool::FixedFieldValuePool;

        let result = make_float_addr(0);
        let method = CompiledMethod {
            name: "getPiPart".into(),
            codes: vec![
                CodeWithSpan::new(
                    IntermediateCode::assign(
                        result,
                        Operand::float(3.14),
                    ),
                    crate::diagnostics::Span::dummy(),
                ),
                CodeWithSpan::new(
                    IntermediateCode::return_value(ValueType::Float),
                    crate::diagnostics::Span::dummy(),
                ),
            ],
            local_count: 1,
        };

        let delegate = RuntimeDelegate {
            delegate_type: GorgeType::new(BasicType::Delegate),
            method_impl: method,
            captured_values: FixedFieldValuePool::default(),
            param_types: vec![],
            captured_var_types: vec![],
            creator_this: None,
        };

        let mut vm = VirtualMachine::new();
        let obj_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.runtime_delegates.insert(obj_id, delegate);

        vm.invoke_delegate_object(obj_id).unwrap();

        assert!((vm.param_pool.get_float_return() - 3.14).abs() < 0.001);
    }

    /// NativeContext.invoke_delegate 从 native 侧调用委托拿返回值 99（1d）
    #[test]
    fn test_native_context_invoke_delegate_returns_99() {
        use crate::objective::delegate::RuntimeDelegate;
        use crate::objective::types::GorgeType;
        use crate::objective::types::BasicType;
        use crate::objective::value_pool::FixedFieldValuePool;
        use crate::objective::native::NativeContext;

        let result = make_int_addr(0);
        let method = CompiledMethod {
            name: "get99".into(),
            codes: vec![
                CodeWithSpan::new(
                    IntermediateCode::assign(
                        result,
                        Operand::int(99),
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

        let delegate = RuntimeDelegate {
            delegate_type: GorgeType::new(BasicType::Delegate),
            method_impl: method,
            captured_values: FixedFieldValuePool::default(),
            param_types: vec![],
            captured_var_types: vec![],
            creator_this: None,
        };

        let mut vm = VirtualMachine::new();
        let obj_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.runtime_delegates.insert(obj_id, delegate);

        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.invoke_delegate(obj_id);
        }

        assert_eq!(vm.param_pool.get_int_return(), 99);
    }

    // ==================== S3 VM 测试 ====================

    /// invoke_method_by_id：注册类后按方法全局 ID 调用，获取返回值
    #[test]
    fn test_s3_invoke_method_by_id() {
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use std::sync::Arc;

        let mut decl = ClassDeclaration::dummy("S3Test".into());
        decl.method_count = 1;
        let mut cls = RuntimeClass::new(decl, None);
        let method = CompiledMethod {
            name: "getVal".into(),
            codes: vec![
                crate::virtual_machine::ir::CodeWithSpan::new(
                    IntermediateCode::new(
                        IntermediateOperator::ReturnInt,
                        Operand::int(42),
                        None, None,
                    ),
                    crate::diagnostics::Span::dummy(),
                ),
            ],
            local_count: 1,
        };
        cls.register_method(0, method);
        let cls = Arc::new(cls);

        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("S3Test", cls);
        vm.invoke_method_by_id("S3Test", None, 0).unwrap();
        assert_eq!(vm.return_int, Some(42));
    }

    /// instantiate_with_injector：创建对象并执行构造方法
    #[test]
    fn test_s3_instantiate_with_injector() {
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use std::sync::Arc;

        let mut decl = ClassDeclaration::dummy("S3Ctor".into());
        decl.constructor_count = 1;
        let mut cls = RuntimeClass::new(decl, None);
        let ctor = CompiledMethod {
            name: "constructor".into(),
            codes: vec![
                crate::virtual_machine::ir::CodeWithSpan::new(
                    IntermediateCode::new(
                        IntermediateOperator::ReturnVoid,
                        Operand::int(0), None, None,
                    ),
                    crate::diagnostics::Span::dummy(),
                ),
            ],
            local_count: 1,
        };
        cls.register_constructor(0, ctor);
        let cls = Arc::new(cls);

        let mut vm = VirtualMachine::new();
        vm.register_class_field_counts("S3Ctor", crate::objective::types::TypeCount::zero());
        vm.register_runtime_class("S3Ctor", cls);
        let obj_id = vm.instantiate_with_injector("S3Ctor", 0, 0).unwrap();
        assert!(obj_id > 0);
        assert!(vm.objects.contains_key(&obj_id));
        assert_eq!(vm.objects.get(&obj_id).unwrap().class_name, "S3Ctor");
    }

    #[test]
    fn test_invoke_injector_constructor_uses_dynamic_injector_class() {
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::system::native::injector::RuntimeInjector;
        use std::sync::Arc;

        fn empty_constructor() -> CompiledMethod {
            CompiledMethod {
                name: "constructor".into(),
                codes: vec![crate::virtual_machine::ir::CodeWithSpan::new(
                    IntermediateCode::new(IntermediateOperator::ReturnVoid, Operand::int(0), None, None),
                    crate::diagnostics::Span::dummy(),
                )],
                local_count: 1,
            }
        }

        let mut base_declaration = ClassDeclaration::dummy("BaseCurve".into());
        base_declaration.constructor_count = 1;
        base_declaration.injector_constructor_impl_id = vec![0];
        let mut base_class = RuntimeClass::new(base_declaration, None);
        base_class.register_constructor(0, empty_constructor());

        let mut concrete_declaration = ClassDeclaration::dummy("ConcreteCurve".into());
        concrete_declaration.constructor_start_id = 1;
        concrete_declaration.constructor_count = 1;
        concrete_declaration.injector_constructor_impl_id = vec![1];
        let injector_declaration = Arc::new(concrete_declaration.clone());
        let mut concrete_class = RuntimeClass::new(concrete_declaration, None);
        concrete_class.register_constructor(0, empty_constructor());

        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("BaseCurve", Arc::new(base_class));
        vm.register_runtime_class("ConcreteCurve", Arc::new(concrete_class));
        vm.register_class_field_counts("ConcreteCurve", TypeCount::zero());
        vm.push_frame(4);
        let injector_id = 42;
        vm.injectors.insert(injector_id, RuntimeInjector::new(injector_declaration));
        vm.current_injector = Some(injector_id);
        let result = Address::new(ValueType::Object, 1);

        vm.execute_one(&IntermediateCode::new(
            IntermediateOperator::InvokeInjectorConstructor(0),
            Operand::int(0),
            Some(Operand::string("BaseCurve")),
            Some(result),
        )).expect("动态注入器构造应成功");

        let object_id = *vm.object_stack.read(result.index);
        assert_eq!(vm.objects.get(&object_id).unwrap().class_name, "ConcreteCurve");
        assert_eq!(vm.current_injector, Some(injector_id));
    }

    #[test]
    fn test_invoke_injector_constructor_converts_native_global_id_to_local_id() {
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::native::{NativeClass, NativeContext};
        use crate::system::native::injector::RuntimeInjector;
        use std::sync::Arc;

        #[derive(Debug)]
        struct ConstructorIdNativeClass {
            field_counts: TypeCount,
        }

        impl NativeClass for ConstructorIdNativeClass {
            fn full_name(&self) -> &str {
                "ConcreteNativeCurve"
            }

            fn field_type_count(&self) -> &TypeCount {
                &self.field_counts
            }

            fn invoke_native_method(
                &self,
                _ctx: &mut NativeContext,
                _object_id: usize,
                _method_id: usize,
            ) {
            }

            fn invoke_native_static(
                &self,
                _ctx: &mut NativeContext,
                _method_id: usize,
            ) {
            }

            fn do_construct_native(
                &self,
                ctx: &mut NativeContext,
                _target: Option<usize>,
                constructor_id: usize,
            ) -> usize {
                let object = RuntimeObject::new_simple(
                    self.full_name().to_string(),
                    &self.field_counts,
                );
                let object_id = ctx.register_object(object);
                ctx.set_object_int_field(object_id, 0, constructor_id as i64);
                object_id
            }
        }

        let mut declaration = ClassDeclaration::dummy("ConcreteNativeCurve".into());
        declaration.is_native = true;
        declaration.constructor_start_id = 1;
        declaration.constructor_count = 1;
        declaration.injector_constructor_impl_id = vec![1];
        let injector_declaration = Arc::new(declaration.clone());

        let mut vm = VirtualMachine::new();
        vm.register_runtime_class(
            "ConcreteNativeCurve",
            Arc::new(RuntimeClass::new(declaration, None)),
        );
        vm.register_native_class(
            "ConcreteNativeCurve",
            Arc::new(ConstructorIdNativeClass {
                field_counts: TypeCount {
                    int_count: 1,
                    ..TypeCount::zero()
                },
            }),
        );
        vm.push_frame(4);
        let injector_id = 43;
        vm.injectors.insert(
            injector_id,
            RuntimeInjector::new(injector_declaration),
        );
        vm.current_injector = Some(injector_id);
        let result = Address::new(ValueType::Object, 1);

        vm.execute_one(&IntermediateCode::new(
            IntermediateOperator::InvokeInjectorConstructor(0),
            Operand::int(0),
            Some(Operand::string("BaseCurve")),
            Some(result),
        )).expect("native 动态注入器构造应成功");

        let object_id = *vm.object_stack.read(result.index);
        let object = vm.objects.get(&object_id).expect("native 构造应创建对象");
        assert_eq!(object.class_name, "ConcreteNativeCurve");
        assert_eq!(object.get_int_field(0), 0, "native 构造应接收本地构造 ID");
    }

    #[test]
    fn test_object_array_constructor_uses_get_set_and_length_abi() {
        use crate::system::native::array::{ObjectArray, ObjectArrayClass};
        use std::sync::Arc;

        let mut vm = VirtualMachine::new();
        vm.register_native_class("ObjectArray", Arc::new(ObjectArrayClass));
        vm.push_frame(4);
        let array_address = Address::new(ValueType::Object, 0);
        vm.execute_one(&IntermediateCode::new(
            IntermediateOperator::InvokeArrayConstructor,
            Operand::int(2),
            Some(Operand::string("GorgeFramework.Node")),
            Some(array_address),
        )).expect("对象数组构造应成功");

        let wrapper_id = *vm.object_stack.read(array_address.index);
        let wrapper = vm.objects.get(&wrapper_id).expect("应创建数组包装对象");
        assert_eq!(wrapper.class_name, "ObjectArray");
        assert_eq!(wrapper.get_int_field(0), 2);
        let native_id = wrapper.native_object_id.expect("包装对象应链接 native 载荷");
        assert_eq!(vm.native_payloads.get(&native_id)
            .and_then(|value| value.downcast_ref::<ObjectArray>())
            .unwrap().items, vec![0, 0]);

        vm.param_pool.set_int_param(0, 1);
        vm.param_pool.set_object_param(0, 123);
        vm.execute_one(&IntermediateCode::new(
            IntermediateOperator::InvokeInstance(1),
            Operand::Address(array_address),
            None,
            None,
        )).unwrap();
        vm.param_pool.set_int_param(0, 1);
        let item_address = Address::new(ValueType::Object, 1);
        vm.execute_one(&IntermediateCode::new(
            IntermediateOperator::InvokeInstance(0),
            Operand::Address(array_address),
            None,
            Some(item_address),
        )).unwrap();
        assert_eq!(*vm.object_stack.read(item_address.index), 123);

        let length_address = Address::new(ValueType::Int, 2);
        vm.execute_one(&IntermediateCode::new(
            IntermediateOperator::InvokeInstance(3),
            Operand::Address(array_address),
            None,
            Some(length_address),
        )).unwrap();
        assert_eq!(*vm.int_stack.read(length_address.index), 2);
    }

    /// 空对象数组常量物化：elements 为空时仍走 ObjectArray，不 panic，
    /// 包装对象 length=0 且载荷为空数组。
    #[test]
    fn test_materialize_injector_array_empty_object_array() {
        use crate::system::native::array::ObjectArray;

        let mut vm = VirtualMachine::new();
        vm.object_stack.push_frame(2);
        let wrapper_id = vm.materialize_injector_array(&[]).expect("空数组物化应成功");
        let wrapper = vm.objects.get(&wrapper_id).expect("空数组应创建包装对象");
        assert_eq!(wrapper.class_name, "ObjectArray");
        assert_eq!(wrapper.get_int_field(0), 0, "空数组 length 应为 0");
        let native_id = wrapper.native_object_id.expect("空数组包装应链接 native 载荷");
        let payload = vm.native_payloads.get(&native_id)
            .and_then(|value| value.downcast_ref::<ObjectArray>())
            .expect("空数组载荷应为 ObjectArray");
        assert!(payload.items.is_empty());
    }

    /// 多元素对象数组常量物化：InjectObject 项递归物化为注入器对象 ID，
    /// 统一写入 ObjectArray 载荷，包装对象 length 等于元素数。
    #[test]
    fn test_materialize_injector_array_object_items() {
        use crate::objective::bytecode::InjectorConstField;
        use crate::system::native::array::ObjectArray;

        let mut vm = VirtualMachine::new();
        vm.object_stack.push_frame(2);
        let elements = vec![
            InjectorConstField::InjectObject("Chart.Note".into(), String::new(), vec![]),
            InjectorConstField::InjectObject("Chart.Note".into(), String::new(), vec![]),
        ];
        let wrapper_id = vm.materialize_injector_array(&elements).expect("对象数组物化应成功");
        let wrapper = vm.objects.get(&wrapper_id).expect("应创建包装对象");
        assert_eq!(wrapper.class_name, "ObjectArray");
        assert_eq!(wrapper.get_int_field(0), 2, "对象数组 length 应为 2");
        let native_id = wrapper.native_object_id.expect("包装应链接 native 载荷");
        let payload = vm.native_payloads.get(&native_id)
            .and_then(|value| value.downcast_ref::<ObjectArray>())
            .expect("对象数组载荷应为 ObjectArray");
        assert_eq!(payload.items.len(), 2);
        assert!(payload.items.iter().all(|id| *id != 0), "对象数组元素应为非 0 对象 ID");
    }

    /// 注入器实例化失败路径：构造方法缺失时返回 Err，且 current_injector
    /// 必须恢复为调用前的值（不得泄漏中途设置的注入器）。
    #[test]
    fn test_instantiate_with_injector_restores_current_injector_on_error() {
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use std::sync::Arc;

        let mut decl = ClassDeclaration::dummy("BadCtor".into());
        decl.constructor_count = 1;
        decl.constructor_start_id = 0;
        let cls = Arc::new(RuntimeClass::new(decl, None));

        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("BadCtor", cls);
        vm.register_class_field_counts("BadCtor", crate::objective::types::TypeCount::zero());
        vm.current_injector = Some(7);
        let result = vm.instantiate_with_injector("BadCtor", 0, 9);
        assert!(result.is_err(), "缺失构造实现时应返回错误");
        assert_eq!(vm.current_injector, Some(7), "失败路径必须恢复 current_injector");
    }

    /// 嵌套编译构造不得破坏调用者 this 槽：外层构造内 new 内层对象后，
    /// LoadThis 必须仍读到外层对象，且外层帧 object[0] 在构造返回后恢复。
    #[test]
    fn test_invoke_constructor_restores_caller_this_slot() {
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::TypeCount;
        use std::sync::Arc;

        fn make_class(name: &str, ctor_codes: Vec<CodeWithSpan>) -> Arc<RuntimeClass> {
            let mut decl = ClassDeclaration::dummy(name.into());
            decl.constructor_count = 1;
            decl.constructor_start_id = 0;
            let mut cls = RuntimeClass::new(decl, None);
            cls.register_constructor(0, CompiledMethod {
                name: "constructor".into(),
                codes: ctor_codes,
                local_count: 4,
            });
            Arc::new(cls)
        }

        let inner_codes = vec![CodeWithSpan::new(
            IntermediateCode::new(
                IntermediateOperator::ReturnVoid,
                Operand::int(0),
                None,
                None,
            ),
            crate::diagnostics::Span::dummy(),
        )];
        let inner = make_class("Inner", inner_codes);

        let outer_codes = vec![
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::InvokeConstructor(0),
                    Operand::int(0),
                    Some(Operand::string("Inner")),
                    Some(Address::new(ValueType::Object, 1)),
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::LoadThis,
                    Operand::int(0),
                    None,
                    Some(Address::new(ValueType::Object, 2)),
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::SetObjectField(0),
                    Operand::Address(Address::new(ValueType::Object, 2)),
                    Some(Operand::Address(Address::new(ValueType::Object, 1))),
                    None,
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::ReturnVoid,
                    Operand::int(0),
                    None,
                    None,
                ),
                crate::diagnostics::Span::dummy(),
            ),
        ];
        let outer = make_class("Outer", outer_codes);

        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("Inner", inner);
        vm.register_runtime_class("Outer", outer);
        vm.register_class_field_counts("Inner", TypeCount::zero());
        vm.register_class_field_counts("Outer", TypeCount {
            object_count: 1,
            ..TypeCount::zero()
        });
        vm.push_frame(4);
        vm.object_stack.write(0, 777);
        let result_addr = Address::new(ValueType::Object, 1);

        vm.execute_one(&IntermediateCode::new(
            IntermediateOperator::InvokeConstructor(0),
            Operand::int(0),
            Some(Operand::string("Outer")),
            Some(result_addr),
        )).expect("嵌套构造应成功");

        assert_eq!(
            *vm.object_stack.read(0),
            777,
            "外层调用者 this 槽必须恢复，不得被嵌套构造覆盖",
        );
        let outer_id = *vm.object_stack.read(result_addr.index);
        let inner_id = vm.objects.get(&outer_id)
            .map(|obj| obj.get_object_field(0))
            .expect("外层对象字段 0 应写入内层对象 ID");
        assert_eq!(
            vm.objects.get(&inner_id).map(|obj| obj.class_name.as_str()),
            Some("Inner"),
        );
    }

    /// native 数组类名映射：标量按基本类型，对象/接口/注入器统一 ObjectArray。
    #[test]
    fn test_native_array_class_name_mapping() {
        assert_eq!(VirtualMachine::native_array_class_name("int"), "IntArray");
        assert_eq!(VirtualMachine::native_array_class_name("float"), "FloatArray");
        assert_eq!(VirtualMachine::native_array_class_name("bool"), "BoolArray");
        assert_eq!(VirtualMachine::native_array_class_name("string"), "StringArray");
        assert_eq!(VirtualMachine::native_array_class_name("GorgeFramework.Node"), "ObjectArray");
        assert_eq!(VirtualMachine::native_array_class_name("ITransformer"), "ObjectArray");
        assert_eq!(VirtualMachine::native_array_class_name(""), "ObjectArray");
    }

    /// class_methods_with_annotation：运行时查询带注解的方法
    #[test]
    fn test_s3_methods_with_annotation_query() {
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::{ClassDeclaration, MethodAnnotation, AnnotationValue};
        use std::sync::Arc;

        let mut decl = ClassDeclaration::dummy("S3Query".into());
        decl.method_count = 2;
        decl.method_annotations.insert(0, vec![MethodAnnotation {
            name: "MyAnnotation".into(),
            parameters: vec![("val".into(), AnnotationValue::Float(1.5))],
        }]);

        let mut cls = RuntimeClass::new(decl, None);
        cls.register_method(0, CompiledMethod { name: "a".into(), codes: vec![], local_count: 1 });
        cls.register_method(1, CompiledMethod { name: "b".into(), codes: vec![], local_count: 1 });
        let cls = Arc::new(cls);

        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("S3Query", cls);

        // 通过 NativeContext 查询
        let results = {
            let ctx = crate::objective::native::NativeContext::new(&mut vm);
            ctx.class_methods_with_annotation("S3Query", "MyAnnotation")
        };
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 0);
        let param = results[0].1.find_parameter("val").unwrap();
        assert!(matches!(param, AnnotationValue::Float(v) if (v - 1.5).abs() < 1e-9));

        // 不存在的注解返回空
        let empty = {
            let ctx = crate::objective::native::NativeContext::new(&mut vm);
            ctx.class_methods_with_annotation("S3Query", "Nonexistent")
        };
        assert!(empty.is_empty());
    }

    // ==================== P0: 字符串/对象比较操作码单测 ====================

    #[test]
    fn test_string_equality_true() {
        let mut vm = VirtualMachine::new();
        vm.string_stack.push_frame(4);
        vm.bool_stack.push_frame(4);

        let lhs = make_string_addr(0);
        let rhs = make_string_addr(1);
        let result = make_bool_addr(2);
        vm.string_stack.write(0, "hello".to_string());
        vm.string_stack.write(1, "hello".to_string());

        let code = IntermediateCode::binary(
            IntermediateOperator::StringEqual,
            Operand::Address(lhs),
            Operand::Address(rhs),
            result,
        );
        let _ = vm.execute_one(&code).unwrap();
        assert_eq!(*vm.bool_stack.read(2), true);
    }

    #[test]
    fn test_string_inequality_false() {
        let mut vm = VirtualMachine::new();
        vm.string_stack.push_frame(4);
        vm.bool_stack.push_frame(4);

        let lhs = make_string_addr(0);
        let rhs = make_string_addr(1);
        let result = make_bool_addr(2);
        vm.string_stack.write(0, "hello".to_string());
        vm.string_stack.write(1, "world".to_string());

        let code = IntermediateCode::binary(
            IntermediateOperator::StringNotEqual,
            Operand::Address(lhs),
            Operand::Address(rhs),
            result,
        );
        let _ = vm.execute_one(&code).unwrap();
        assert_eq!(*vm.bool_stack.read(2), true);
    }

    #[test]
    fn test_object_equality_same_id() {
        let mut vm = VirtualMachine::new();
        vm.object_stack.push_frame(4);
        vm.bool_stack.push_frame(4);

        let lhs = make_object_addr(0);
        let rhs = make_object_addr(1);
        let result = make_bool_addr(2);
        vm.object_stack.write(0, 42);
        vm.object_stack.write(1, 42);

        let code = IntermediateCode::binary(
            IntermediateOperator::ObjectEqual,
            Operand::Address(lhs),
            Operand::Address(rhs),
            result,
        );
        let _ = vm.execute_one(&code).unwrap();
        assert_eq!(*vm.bool_stack.read(2), true);
    }

    #[test]
    fn test_object_equality_null_null() {
        let mut vm = VirtualMachine::new();
        vm.object_stack.push_frame(4);
        vm.bool_stack.push_frame(4);

        // 0 表示 null，两个 null 应相等
        let lhs = make_object_addr(0);
        let rhs = make_object_addr(1);
        let result = make_bool_addr(2);
        vm.object_stack.write(0, 0);
        vm.object_stack.write(1, 0);

        let code = IntermediateCode::binary(
            IntermediateOperator::ObjectEqual,
            Operand::Address(lhs),
            Operand::Address(rhs),
            result,
        );
        let _ = vm.execute_one(&code).unwrap();
        assert_eq!(*vm.bool_stack.read(2), true);
    }

    #[test]
    fn test_object_inequality_diff_id() {
        let mut vm = VirtualMachine::new();
        vm.object_stack.push_frame(4);
        vm.bool_stack.push_frame(4);

        let lhs = make_object_addr(0);
        let rhs = make_object_addr(1);
        let result = make_bool_addr(2);
        vm.object_stack.write(0, 1);
        vm.object_stack.write(1, 2);

        let code = IntermediateCode::binary(
            IntermediateOperator::ObjectNotEqual,
            Operand::Address(lhs),
            Operand::Address(rhs),
            result,
        );
        let _ = vm.execute_one(&code).unwrap();
        assert_eq!(*vm.bool_stack.read(2), true);
    }

    // ==================== A-2 clone_object 测试 ====================

    /// clone_object 深拷贝注入器对象（值字段直拷、object 字段递归）
    #[test]
    fn test_a2_clone_injector_deep() {
        use crate::system::native::injector::RuntimeInjector;
        use crate::objective::types::TypeCount;
        use crate::objective::declaration::ClassDeclaration;
        use std::sync::Arc;

        let mut vm = VirtualMachine::new();
        vm.next_object_id = 100;

        // 先创建一个简单注入器作为 object 字段引用的目标
        let inner_decl = Arc::new(ClassDeclaration {
            injector_field_type_count: TypeCount { int_count: 1, ..TypeCount::zero() },
            ..ClassDeclaration::dummy("Inner".into())
        });
        let mut inner = RuntimeInjector::new(inner_decl);
        inner.set_injector_int(0, 77);
        let inner_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(inner_id, inner);

        // 创建外层注入器含 int+object 字段
        let decl = Arc::new(ClassDeclaration {
            injector_field_type_count: TypeCount { int_count: 1, object_count: 1, ..TypeCount::zero() },
            ..ClassDeclaration::dummy("TestInject".into())
        });
        let mut inj = RuntimeInjector::new(decl);
        inj.set_injector_int(0, 42);
        inj.set_injector_object(0, inner_id);
        let inj_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(inj_id, inj);

        let cloned_id = vm.clone_object(inj_id).unwrap();
        assert!(cloned_id != inj_id, "克隆应产生新的注入器 ID");
        let cloned = vm.injectors.get(&cloned_id).unwrap();
        assert_eq!(cloned.get_injector_int(0), 42);
        assert!(!cloned.get_injector_int_default_value(0)); // 值被显式设置
        // object 字段被递归克隆
        assert!(!cloned.get_injector_object_default_value(0));
        let cloned_inner_id = cloned.get_injector_object(0);
        assert!(cloned_inner_id != inner_id, "object 字段引用的注入器也应被克隆");
        assert_eq!(vm.injectors.get(&cloned_inner_id).unwrap().get_injector_int(0), 77);
    }

    /// clone_object 嵌套注入器递归克隆
    #[test]
    fn test_a2_clone_nested_injector() {
        use crate::system::native::injector::RuntimeInjector;
        use crate::objective::types::TypeCount;
        use crate::objective::declaration::ClassDeclaration;
        use std::sync::Arc;

        let mut vm = VirtualMachine::new();
        vm.next_object_id = 100;

        // 外层注入器含一个 object 字段，指向内层注入器
        let inner_decl = Arc::new(ClassDeclaration {
            injector_field_type_count: TypeCount { int_count: 1, ..TypeCount::zero() },
            ..ClassDeclaration::dummy("Inner".into())
        });
        let mut inner = RuntimeInjector::new(inner_decl);
        inner.set_injector_int(0, 7);
        let inner_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(inner_id, inner);

        let outer_decl = Arc::new(ClassDeclaration {
            injector_field_type_count: TypeCount { object_count: 1, ..TypeCount::zero() },
            ..ClassDeclaration::dummy("Outer".into())
        });
        let mut outer = RuntimeInjector::new(outer_decl);
        outer.set_injector_object(0, inner_id);
        let outer_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(outer_id, outer);

        let cloned_outer_id = vm.clone_object(outer_id).unwrap();
        assert!(cloned_outer_id != outer_id);
        let cloned_outer = vm.injectors.get(&cloned_outer_id).unwrap();
        let cloned_inner_id = cloned_outer.get_injector_object(0);
        assert!(!cloned_outer.get_injector_object_default_value(0));
        assert!(cloned_inner_id != inner_id, "嵌套注入器也应被递归克隆");
        assert!(vm.injectors.contains_key(&cloned_inner_id));
        assert_eq!(vm.injectors.get(&cloned_inner_id).unwrap().get_injector_int(0), 7);
    }

    /// clone_object 对 ObjectList 递归克隆每个元素
    #[test]
    fn test_a2_clone_objectlist_recursive() {
        use crate::system::native::list::ObjectList;
        use crate::system::native::injector::RuntimeInjector;
        use crate::objective::types::TypeCount;
        use crate::objective::declaration::ClassDeclaration;
        use std::sync::Arc;

        let mut vm = VirtualMachine::new();
        vm.next_object_id = 100;

        // 创建两个注入器对象作为列表元素
        let decl = Arc::new(ClassDeclaration {
            injector_field_type_count: TypeCount { int_count: 1, ..TypeCount::zero() },
            ..ClassDeclaration::dummy("Elem".into())
        });
        let mut e1 = RuntimeInjector::new(decl.clone());
        e1.set_injector_int(0, 101);
        let e1_id = vm.next_object_id; vm.next_object_id += 1; vm.injectors.insert(e1_id, e1);
        let mut e2 = RuntimeInjector::new(decl);
        e2.set_injector_int(0, 202);
        let e2_id = vm.next_object_id; vm.next_object_id += 1; vm.injectors.insert(e2_id, e2);

        // 创建 ObjectList
        let list_id = vm.next_object_id; vm.next_object_id += 1;
        vm.native_payloads.insert(list_id, Box::new(ObjectList { items: vec![e1_id, e2_id] }));

        let cloned_list_id = vm.clone_object(list_id).unwrap();
        assert!(cloned_list_id != list_id);
        let cloned = vm.native_payloads.get(&cloned_list_id)
            .and_then(|p| p.downcast_ref::<ObjectList>()).unwrap();
        assert_eq!(cloned.items.len(), 2);
        assert!(cloned.items[0] != e1_id, "列表元素应被递归克隆");
        assert!(cloned.items[1] != e2_id);
        assert_eq!(vm.injectors.get(&cloned.items[0]).unwrap().get_injector_int(0), 101);
        assert_eq!(vm.injectors.get(&cloned.items[1]).unwrap().get_injector_int(0), 202);
    }

    /// clone_object 超过深度上限应报错
    #[test]
    fn test_a2_clone_depth_limit() {
        use crate::system::native::injector::RuntimeInjector;
        use crate::objective::types::TypeCount;
        use crate::objective::declaration::ClassDeclaration;
        use std::sync::Arc;

        let mut vm = VirtualMachine::new();
        vm.next_object_id = 100;

        // 构造一个自引用的注入器（造成无限递归），验证深度限制返回错误。
        // 由于 clone_object_impl 按深度检查，自引用在第二次遇到同一对象时会再次递归
        // 直至超过 MAX_CLONE_DEPTH=64。
        let decl = Arc::new(ClassDeclaration {
            injector_field_type_count: TypeCount { object_count: 1, ..TypeCount::zero() },
            ..ClassDeclaration::dummy("SelfRef".into())
        });
        let mut inj = RuntimeInjector::new(decl);
        let inj_id = vm.next_object_id; vm.next_object_id += 1;
        inj.set_injector_object(0, inj_id); // 自引用
        vm.injectors.insert(inj_id, inj);

        let result = vm.clone_object(inj_id);
        assert!(result.is_err(), "自引用注入器应因超过深度上限而报错");
        assert!(result.unwrap_err().contains("64"));
    }

    // ==================== A-3 instantiate_with_injector_args 测试 ====================

    /// instantiate_with_injector_args：含参构造方法调用
    #[test]
    fn test_a3_instantiate_with_args() {
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use std::sync::Arc;

        let mut decl = ClassDeclaration::dummy("A3Ctor".into());
        decl.constructor_count = 1;
        let mut cls = RuntimeClass::new(decl, None);
        let ctor = CompiledMethod {
            name: "constructor".into(),
            codes: vec![
                crate::virtual_machine::ir::CodeWithSpan::new(
                    IntermediateCode::new(
                        IntermediateOperator::ReturnVoid,
                        Operand::int(0), None, None,
                    ),
                    crate::diagnostics::Span::dummy(),
                ),
            ],
            local_count: 1,
        };
        cls.register_constructor(0, ctor);
        let cls = Arc::new(cls);

        let mut vm = VirtualMachine::new();
        vm.register_class_field_counts("A3Ctor", crate::objective::types::TypeCount::zero());
        vm.register_runtime_class("A3Ctor", cls);

        let args = InstantiateArgs {
            ints: vec![10, 20],
            floats: vec![1.5],
            ..InstantiateArgs::default()
        };
        let obj_id = vm.instantiate_with_injector_args("A3Ctor", 0, 0, &args).unwrap();
        assert!(obj_id > 0);
        assert!(vm.objects.contains_key(&obj_id));
        // 参数已写入 param_pool
        assert_eq!(vm.param_pool.get_int_param(0), 10);
        assert_eq!(vm.param_pool.get_int_param(1), 20);
        assert!((vm.param_pool.get_float_param(0) - 1.5).abs() < 1e-9);
    }

    // ==================== 静默吞错回归测试 ====================

    /// InvokeInstance：调用不存在的方法（编译类和 native 祖先均无）应返回 Err
    #[test]
    fn test_invoke_instance_method_not_found_returns_err() {
        use std::sync::Arc;
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::GorgeType;

        // 注册一个无任何方法的编译类
        let decl = ClassDeclaration {
            class_type: GorgeType::class("Empty", None),
            is_native: false, annotations: vec![], fields: vec![],
            methods: vec![], static_methods: vec![],
            constructors: vec![], injector_fields: vec![],
            super_class: None, super_interfaces: vec![],
            field_type_count: TypeCount::zero(),
            method_count: 0, static_method_count: 0, constructor_count: 0,
            injector_field_type_count: TypeCount::zero(),
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0, constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
            injector_constructor_impl_id: vec![],
            method_annotations: HashMap::new(),
            constructor_annotations: HashMap::new(),
        };
        let cls = Arc::new(RuntimeClass::new(decl, None));

        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("Empty", cls.clone());
        vm.register_class_field_counts("Empty", TypeCount::zero());

        let obj_id = vm.next_object_id;
        vm.next_object_id += 1;
        let obj = RuntimeObject::new_simple("Empty".into(), &TypeCount::zero());
        vm.objects.insert(obj_id, obj);
        vm.object_stack.push_frame(3);
        vm.int_stack.push_frame(3);
        vm.float_stack.push_frame(3);
        vm.bool_stack.push_frame(3);
        vm.string_stack.push_frame(3);

        // 调用不存在的实例方法 method_id=0
        let result_addr = Address::new(ValueType::Int, 1);
        let invoke_code = IntermediateCode::new(
            IntermediateOperator::InvokeInstance(0),
            Operand::Address(Address::new(ValueType::Object, 0)),
            None,
            Some(result_addr),
        );
        vm.object_stack.write(0, obj_id);

        let result = vm.execute_one(&invoke_code);
        assert!(result.is_err(), "调用不存在的方法应返回 Err");
        assert!(result.unwrap_err().contains("未找到方法"), "错误信息应包含'未找到方法'");
    }

    /// InvokeInterface：类未实现指定接口的方法时返回 Err
    #[test]
    fn test_invoke_interface_method_not_implemented_returns_err() {
        use std::sync::Arc;
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::GorgeType;

        // 注册一个未实现任何接口的编译类
        let decl = ClassDeclaration {
            class_type: GorgeType::class("NoIface", None),
            is_native: false, annotations: vec![], fields: vec![],
            methods: vec![], static_methods: vec![],
            constructors: vec![], injector_fields: vec![],
            super_class: None, super_interfaces: vec![],
            field_type_count: TypeCount::zero(),
            method_count: 0, static_method_count: 0, constructor_count: 0,
            injector_field_type_count: TypeCount::zero(),
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0, constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
            injector_constructor_impl_id: vec![],
            method_annotations: HashMap::new(),
            constructor_annotations: HashMap::new(),
        };
        let cls = Arc::new(RuntimeClass::new(decl, None));

        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("NoIface", cls.clone());

        let obj_id = vm.next_object_id;
        vm.next_object_id += 1;
        let obj = RuntimeObject::new_simple("NoIface".into(), &TypeCount::zero());
        vm.objects.insert(obj_id, obj);
        vm.object_stack.push_frame(3);
        vm.int_stack.push_frame(3);
        vm.float_stack.push_frame(3);
        vm.bool_stack.push_frame(3);
        vm.string_stack.push_frame(3);

        // 接口调用：接口名为 "IShape"，接口方法 ID=0
        let result_addr = Address::new(ValueType::Int, 1);
        let invoke_code = IntermediateCode::new(
            IntermediateOperator::InvokeInterface(0),
            Operand::Address(Address::new(ValueType::Object, 0)),
            Some(Operand::Immediate(crate::virtual_machine::ir::ImmediateValue::String("IShape".into()))),
            Some(result_addr),
        );
        vm.object_stack.write(0, obj_id);

        let result = vm.execute_one(&invoke_code);
        assert!(result.is_err(), "调用未实现的接口方法应返回 Err");
        assert!(result.unwrap_err().contains("未实现接口"), "错误信息应包含'未实现接口'");
    }

    // ==================== P0-7 LoadInjectorConstant 嵌套物化 ====================

    /// LoadInjectorConstant 全链路：标量 + 嵌套注入器 + 数组递归物化
    #[test]
    fn test_p0_7_load_injector_constant_nested_materialize() {
        use crate::objective::bytecode::{InjectorConstantDef, InjectorConstField};
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::GorgeType;
        use std::sync::Arc;

        // 注册含 (int=1, object=1) 注入器字段的类（全名注册，验证双键解析）
        let decl = ClassDeclaration {
            class_type: GorgeType::class("Chart.Outer", None),
            injector_field_type_count: TypeCount { int_count: 1, object_count: 1, ..TypeCount::zero() },
            ..ClassDeclaration::dummy("Chart.Outer".into())
        };
        let cls = Arc::new(RuntimeClass::new(decl, None));

        // 嵌套注入器类同样注册（按简单名），验证嵌套类名也走双键解析
        let inner_decl = ClassDeclaration {
            class_type: GorgeType::class("Chart.Vector2", None),
            injector_field_type_count: TypeCount { float_count: 2, ..TypeCount::zero() },
            ..ClassDeclaration::dummy("Chart.Vector2".into())
        };
        let inner_cls = Arc::new(RuntimeClass::new(inner_decl, None));

        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("Outer", cls);
        vm.register_runtime_class("Vector2", inner_cls);
        vm.injector_constants = vec![InjectorConstantDef {
            class_name: "Chart.Outer".into(),
            fields: vec![
                InjectorConstField::Int("id".into(), 7),
                InjectorConstField::InjectObject(
                    "Chart.Vector2".into(),
                    String::new(),
                    vec![
                        InjectorConstField::Float("x".into(), 1.5),
                        InjectorConstField::Float("y".into(), -2.0),
                    ],
                ),
            ],
        }];

        vm.object_stack.push_frame(2);
        let code = IntermediateCode::new(
            IntermediateOperator::LoadInjectorConstant(0),
            Operand::int(0), None, Some(Address::new(ValueType::Object, 0)),
        );
        let result = vm.execute_one(&code);
        assert!(result.is_ok(), "物化失败: {:?}", result.err());
        let inj_id = *vm.object_stack.read(0);
        assert!(inj_id > 0, "应返回有效注入器 ID");

        // 外层注入器：int 字段 + 嵌套 object 字段均非默认
        let inj = vm.injectors.get(&inj_id).expect("外层应为注入器");
        assert_eq!(inj.get_injector_int(0), 7);
        assert!(!inj.get_injector_int_default_value(0));
        let nested_id = inj.get_injector_object(0);
        assert!(nested_id > 0, "嵌套注入器不应为 0（原占位实现填 0）");
        assert!(!inj.get_injector_object_default_value(0));

        // 嵌套注入器：类名按实际注册键解析（简单名 "Vector2"），标量值完整
        let nested = vm.injectors.get(&nested_id).expect("嵌套应为注入器");
        assert_eq!(
            nested.injection_class_declaration().class_type.full_name(),
            "Vector2",
            "嵌套注入器类名应解析为实际注册键（双键约定）"
        );
        assert_eq!(nested.get_injector_float(0), 1.5);
        assert_eq!(nested.get_injector_float(1), -2.0);
    }

    /// LoadInjectorConstant 数组字段：物化为 native 数组载荷 + 编译层包装
    #[test]
    fn test_p0_7_load_injector_constant_array_materialize() {
        use crate::objective::bytecode::{InjectorConstantDef, InjectorConstField};
        use crate::objective::class::RuntimeClass;
        use crate::objective::declaration::ClassDeclaration;
        use crate::objective::types::GorgeType;
        use std::sync::Arc;

        let decl = ClassDeclaration {
            class_type: GorgeType::class("Chart.Holder", None),
            injector_field_type_count: TypeCount { object_count: 1, ..TypeCount::zero() },
            ..ClassDeclaration::dummy("Chart.Holder".into())
        };
        let cls = Arc::new(RuntimeClass::new(decl, None));

        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("Holder", cls);
        vm.injector_constants = vec![InjectorConstantDef {
            class_name: "Chart.Holder".into(),
            fields: vec![InjectorConstField::Array(String::new(), vec![
                InjectorConstField::Float("".into(), 0.25),
                InjectorConstField::Float("".into(), 0.75),
            ])],
        }];

        vm.object_stack.push_frame(2);
        let code = IntermediateCode::new(
            IntermediateOperator::LoadInjectorConstant(0),
            Operand::int(0), None, Some(Address::new(ValueType::Object, 0)),
        );
        vm.execute_one(&code).unwrap();
        let inj_id = *vm.object_stack.read(0);
        let inj = vm.injectors.get(&inj_id).unwrap();
        let wrapper_id = inj.get_injector_object(0);
        assert!(wrapper_id > 0, "数组字段不应为 0");

        // 编译层包装：length 字段语义（对齐 InvokeArrayConstructor）
        let wrapper = vm.objects.get(&wrapper_id).expect("应为编译层包装对象");
        assert_eq!(wrapper.get_int_field(0), 2, "length 应写入 int 字段 0");
        let native_id = wrapper.native_object_id.expect("应链接 native 载荷");

        // native 载荷：FloatArray 元素完整
        use crate::system::native::array::FloatArray;
        let payload = vm.native_payloads.get(&native_id)
            .and_then(|p| p.downcast_ref::<FloatArray>())
            .expect("应为 FloatArray 载荷");
        assert_eq!(payload.items.len(), 2);
        assert!((payload.items[0] - 0.25).abs() < 1e-9);
        assert!((payload.items[1] - 0.75).abs() < 1e-9);
    }

    /// LoadInjectorConstant 常量索引越界返回 Err
    #[test]
    fn test_p0_7_load_injector_constant_index_out_of_bounds() {
        let mut vm = VirtualMachine::new();
        vm.object_stack.push_frame(2);
        let code = IntermediateCode::new(
            IntermediateOperator::LoadInjectorConstant(3),
            Operand::int(0), None, Some(Address::new(ValueType::Object, 0)),
        );
        let result = vm.execute_one(&code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("越界"));
    }
}
