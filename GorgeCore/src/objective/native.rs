use std::fmt::Debug;
use std::sync::Arc;

use crate::objective::object::{GorgeObject, RuntimeObject};
use crate::virtual_machine::vm::VirtualMachine;
use crate::objective::types::TypeCount;

/// Native 方法执行上下文
///
/// 对应 C# 中 native 桥接层通过全局静态 `InvokeParameterPool` 与虚拟机通信的机制。
/// 重构为持有 `vm: &mut VirtualMachine`，所有 API 通过 `self.vm.xxx` 访问（1d）。
///
/// native 桥接函数通过本上下文：
/// - 读取调用参数（`get_*_param`）
/// - 写入返回值（`set_*_return`）
/// - 读写对象字段（`get_*_field` / `set_*_field`）
/// - 创建新对象（`register_object`）
/// - 调用委托（`invoke_delegate`）
///
/// # 生命周期
/// `'a` 绑定到虚拟机本次调用期间。
pub struct NativeContext<'a> {
    /// 虚拟机引用（含参数池、对象表、注入器表、native 表等）
    pub vm: &'a mut VirtualMachine,
    /// 当前注入器对象 ID（0 表示无，对应 C# `InvokeParameterPool.Injector`）
    pub current_injector: usize,
}

impl<'a> NativeContext<'a> {
    /// 从虚拟机创建上下文
    pub fn new(vm: &'a mut VirtualMachine) -> Self {
        Self {
            vm,
            current_injector: 0,
        }
    }

    /// 创建带注入器上下文的上下文（用于 native 构造读取注入器字段覆写）
    pub fn with_injector(vm: &'a mut VirtualMachine, current_injector: usize) -> Self {
        Self {
            vm,
            current_injector,
        }
    }

    // ==================== 参数读取 ====================

    /// 读取整数参数
    pub fn get_int_param(&self, index: usize) -> i64 {
        self.vm.param_pool.get_int_param(index)
    }

    /// 读取浮点参数
    pub fn get_float_param(&self, index: usize) -> f64 {
        self.vm.param_pool.get_float_param(index)
    }

    /// 读取布尔参数
    pub fn get_bool_param(&self, index: usize) -> bool {
        self.vm.param_pool.get_bool_param(index)
    }

    /// 读取字符串参数
    pub fn get_string_param(&self, index: usize) -> String {
        self.vm.param_pool.get_string_param(index)
    }

    /// 读取对象参数（对象 ID）
    pub fn get_object_param(&self, index: usize) -> usize {
        self.vm.param_pool.get_object_param(index)
    }

    /// 读取注入器专用位（注入器对象 ID，0 表示无）
    pub fn get_injector(&self) -> usize {
        self.vm.param_pool.get_injector()
    }

    // ==================== 注入器字段读取（native 构造用） ====================

    /// 读取当前注入器的 float 字段值（`inj_index` 为该字段在 float 分组内的索引）。
    pub fn injector_float(&self, inj_index: usize) -> Option<f64> {
        use crate::system::native::injector::Injector;
        let inj = self.vm.injectors.get(&self.current_injector)?;
        if inj_index >= inj.float_field_count() || inj.get_injector_float_default_value(inj_index) {
            None
        } else {
            Some(inj.get_injector_float(inj_index))
        }
    }

    /// 读取当前注入器的 int 字段值。
    pub fn injector_int(&self, inj_index: usize) -> Option<i64> {
        use crate::system::native::injector::Injector;
        let inj = self.vm.injectors.get(&self.current_injector)?;
        if inj_index >= inj.int_field_count() || inj.get_injector_int_default_value(inj_index) {
            None
        } else {
            Some(inj.get_injector_int(inj_index))
        }
    }

    /// 读取当前注入器的 bool 字段值。
    pub fn injector_bool(&self, inj_index: usize) -> Option<bool> {
        use crate::system::native::injector::Injector;
        let inj = self.vm.injectors.get(&self.current_injector)?;
        if inj_index >= inj.bool_field_count() || inj.get_injector_bool_default_value(inj_index) {
            None
        } else {
            Some(inj.get_injector_bool(inj_index))
        }
    }

    /// 读取当前注入器的 string 字段值。
    pub fn injector_string(&self, inj_index: usize) -> Option<String> {
        use crate::system::native::injector::Injector;
        let inj = self.vm.injectors.get(&self.current_injector)?;
        if inj_index >= inj.string_field_count() || inj.get_injector_string_default_value(inj_index) {
            None
        } else {
            Some(inj.get_injector_string(inj_index))
        }
    }

    /// 读取当前注入器的 object 字段值（对象 ID）。
    pub fn injector_object(&self, inj_index: usize) -> Option<usize> {
        use crate::system::native::injector::Injector;
        let inj = self.vm.injectors.get(&self.current_injector)?;
        if inj_index >= inj.object_field_count() || inj.get_injector_object_default_value(inj_index) {
            None
        } else {
            Some(inj.get_injector_object(inj_index))
        }
    }

    // ==================== 返回值写入 ====================

    /// 写入整数返回值
    pub fn set_int_return(&self, value: i64) {
        self.vm.param_pool.set_int_return(value);
    }

    /// 写入浮点返回值
    pub fn set_float_return(&self, value: f64) {
        self.vm.param_pool.set_float_return(value);
    }

    /// 写入布尔返回值
    pub fn set_bool_return(&self, value: bool) {
        self.vm.param_pool.set_bool_return(value);
    }

    /// 写入字符串返回值
    pub fn set_string_return(&self, value: String) {
        self.vm.param_pool.set_string_return(value);
    }

    /// 写入对象返回值（对象 ID）
    pub fn set_object_return(&self, value: usize) {
        self.vm.param_pool.set_object_return(value);
    }

    // ==================== 对象字段访问 ====================

    /// 读取对象的浮点字段
    pub fn get_object_float_field(&self, obj_id: usize, index: usize) -> f64 {
        self.vm.objects
            .get(&obj_id)
            .map(|o| o.get_float_field(index))
            .unwrap_or(0.0)
    }

    /// 读取对象的整数字段
    pub fn get_object_int_field(&self, obj_id: usize, index: usize) -> i64 {
        self.vm.objects
            .get(&obj_id)
            .map(|o| o.get_int_field(index))
            .unwrap_or(0)
    }

    /// 读取对象的布尔字段
    pub fn get_object_bool_field(&self, obj_id: usize, index: usize) -> bool {
        self.vm.objects
            .get(&obj_id)
            .map(|o| o.get_bool_field(index))
            .unwrap_or(false)
    }

    /// 读取对象的字符串字段
    pub fn get_object_string_field(&self, obj_id: usize, index: usize) -> String {
        self.vm.objects
            .get(&obj_id)
            .map(|o| o.get_string_field(index))
            .unwrap_or_default()
    }

    /// 读取对象的对象字段（对象 ID）
    pub fn get_object_object_field(&self, obj_id: usize, index: usize) -> usize {
        self.vm.objects
            .get(&obj_id)
            .map(|o| o.get_object_field(index))
            .unwrap_or(0)
    }

    /// 写入对象的浮点字段
    pub fn set_object_float_field(&mut self, obj_id: usize, index: usize, value: f64) {
        if let Some(o) = self.vm.objects.get_mut(&obj_id) {
            o.set_float_field(index, value);
        }
    }

    /// 写入对象的整数字段
    pub fn set_object_int_field(&mut self, obj_id: usize, index: usize, value: i64) {
        if let Some(o) = self.vm.objects.get_mut(&obj_id) {
            o.set_int_field(index, value);
        }
    }

    /// 写入对象的布尔字段
    pub fn set_object_bool_field(&mut self, obj_id: usize, index: usize, value: bool) {
        if let Some(o) = self.vm.objects.get_mut(&obj_id) {
            o.set_bool_field(index, value);
        }
    }

    /// 写入对象的字符串字段
    pub fn set_object_string_field(&mut self, obj_id: usize, index: usize, value: String) {
        if let Some(o) = self.vm.objects.get_mut(&obj_id) {
            o.set_string_field(index, value);
        }
    }

    /// 写入对象的对象字段（对象 ID）
    pub fn set_object_object_field(&mut self, obj_id: usize, index: usize, value: usize) {
        if let Some(o) = self.vm.objects.get_mut(&obj_id) {
            o.set_object_field(index, value);
        }
    }

    // ==================== 对象创建 ====================

    /// 分配一个新的对象 ID
    pub fn alloc_object_id(&mut self) -> usize {
        let id = self.vm.next_object_id;
        self.vm.next_object_id += 1;
        id
    }

    /// 将一个 RuntimeObject 注册进对象表，返回其对象 ID
    pub fn register_object(&mut self, obj: RuntimeObject) -> usize {
        let id = self.alloc_object_id();
        self.vm.objects.insert(id, obj);
        id
    }

    // ==================== 参数写入（供 native 方法设置调用参数） ====================

    /// 设置整数参数
    pub fn set_int_param(&self, index: usize, value: i64) {
        self.vm.param_pool.set_int_param(index, value);
    }

    /// 设置浮点参数
    pub fn set_float_param(&self, index: usize, value: f64) {
        self.vm.param_pool.set_float_param(index, value);
    }

    /// 设置布尔参数
    pub fn set_bool_param(&self, index: usize, value: bool) {
        self.vm.param_pool.set_bool_param(index, value);
    }

    /// 设置字符串参数
    pub fn set_string_param(&self, index: usize, value: String) {
        self.vm.param_pool.set_string_param(index, value);
    }

    /// 设置对象参数（对象 ID）
    pub fn set_object_param(&self, index: usize, value: usize) {
        self.vm.param_pool.set_object_param(index, value);
    }

    // ==================== 返回值读取 ====================

    /// 读取整数返回值
    pub fn get_int_return(&self) -> i64 {
        self.vm.param_pool.get_int_return()
    }

    /// 读取浮点返回值
    pub fn get_float_return(&self) -> f64 {
        self.vm.param_pool.get_float_return()
    }

    /// 读取布尔返回值
    pub fn get_bool_return(&self) -> bool {
        self.vm.param_pool.get_bool_return()
    }

    /// 读取字符串返回值
    pub fn get_string_return(&self) -> String {
        self.vm.param_pool.get_string_return()
    }

    /// 读取对象返回值（对象 ID）
    pub fn get_object_return(&self) -> usize {
        self.vm.param_pool.get_object_return()
    }

    // ==================== 返回值保存/恢复 ====================

    /// 保存当前全部返回值，返回快照
    pub fn save_returns(&self) -> ReturnSnapshot {
        ReturnSnapshot {
            int: self.vm.param_pool.get_int_return(),
            float: self.vm.param_pool.get_float_return(),
            bool: self.vm.param_pool.get_bool_return(),
            string: self.vm.param_pool.get_string_return(),
            object: self.vm.param_pool.get_object_return(),
        }
    }

    /// 恢复保存的返回值快照
    pub fn restore_returns(&self, snap: &ReturnSnapshot) {
        self.vm.param_pool.set_int_return(snap.int);
        self.vm.param_pool.set_float_return(snap.float);
        self.vm.param_pool.set_bool_return(snap.bool);
        self.vm.param_pool.set_string_return(snap.string.clone());
        self.vm.param_pool.set_object_return(snap.object);
    }

    // ==================== 委托调用 ====================

    /// 调用委托（1d 真实实现：经 VM invoke_delegate_object 执行）
    pub fn invoke_delegate(&mut self, delegate_id: usize) {
        let _ = self.vm.invoke_delegate_object(delegate_id);
    }

    // ==================== Native 方法交互调用 ====================

    // --- 跨对象实例方法便捷调用（S5 新增） ---
    //
    // 提供两种调用模式：
    // 1. 便捷方法：适合单参数常见组合（如 call_native_method_float_f），自动保存/恢复返回值
    // 2. 手动模式：调用方先 set_*_param 布置参数，再 invoke_native_method_on 分派，
    //    最后 get_*_return 读取结果 —— 适合多参数或非标准返回类型

    /// 在目标对象上调用 native 实例方法：一个 float 参数，返回 float
    pub fn call_native_method_float_f(&mut self, obj_id: usize, method_id: usize, arg: f64) -> f64 {
        let saved = self.save_returns();
        self.vm.param_pool.set_float_param(0, arg);
        let cls_name = self.vm.objects.get(&obj_id)
            .map(|o| o.class_name.clone())
            .unwrap_or_default();
        if let Some(cls) = self.vm.native_class_table.get(&cls_name).cloned() {
            cls.invoke_native_method(self, obj_id, method_id);
        }
        let result = self.vm.param_pool.get_float_return();
        self.restore_returns(&saved);
        result
    }

    /// 在目标对象上调用 native 实例方法：一个 float 参数，返回 object（对象 ID）
    pub fn call_native_method_object_f(&mut self, obj_id: usize, method_id: usize, arg: f64) -> usize {
        let saved = self.save_returns();
        self.vm.param_pool.set_float_param(0, arg);
        let cls_name = self.vm.objects.get(&obj_id)
            .map(|o| o.class_name.clone())
            .unwrap_or_default();
        if let Some(cls) = self.vm.native_class_table.get(&cls_name).cloned() {
            cls.invoke_native_method(self, obj_id, method_id);
        }
        let result = self.vm.param_pool.get_object_return();
        self.restore_returns(&saved);
        result
    }

    /// 在目标对象上调用 native 实例方法：一个 float 参数，返回 int
    pub fn call_native_method_int_f(&mut self, obj_id: usize, method_id: usize, arg: f64) -> i64 {
        let saved = self.save_returns();
        self.vm.param_pool.set_float_param(0, arg);
        let cls_name = self.vm.objects.get(&obj_id)
            .map(|o| o.class_name.clone())
            .unwrap_or_default();
        if let Some(cls) = self.vm.native_class_table.get(&cls_name).cloned() {
            cls.invoke_native_method(self, obj_id, method_id);
        }
        let result = self.vm.param_pool.get_int_return();
        self.restore_returns(&saved);
        result
    }

    /// 在目标对象上调用 native 实例方法：一个 float 参数，返回 bool
    pub fn call_native_method_bool_f(&mut self, obj_id: usize, method_id: usize, arg: f64) -> bool {
        let saved = self.save_returns();
        self.vm.param_pool.set_float_param(0, arg);
        let cls_name = self.vm.objects.get(&obj_id)
            .map(|o| o.class_name.clone())
            .unwrap_or_default();
        if let Some(cls) = self.vm.native_class_table.get(&cls_name).cloned() {
            cls.invoke_native_method(self, obj_id, method_id);
        }
        let result = self.vm.param_pool.get_bool_return();
        self.restore_returns(&saved);
        result
    }

    /// 在目标对象上调用 native 实例方法：无参数，返回 object
    pub fn call_native_method_object(&mut self, obj_id: usize, method_id: usize) -> usize {
        let saved = self.save_returns();
        let cls_name = self.vm.objects.get(&obj_id)
            .map(|o| o.class_name.clone())
            .unwrap_or_default();
        if let Some(cls) = self.vm.native_class_table.get(&cls_name).cloned() {
            cls.invoke_native_method(self, obj_id, method_id);
        }
        let result = self.vm.param_pool.get_object_return();
        self.restore_returns(&saved);
        result
    }

    /// 在目标对象上调用 native 实例方法：无参数，返回 float
    pub fn call_native_method_float(&mut self, obj_id: usize, method_id: usize) -> f64 {
        let saved = self.save_returns();
        let cls_name = self.vm.objects.get(&obj_id)
            .map(|o| o.class_name.clone())
            .unwrap_or_default();
        if let Some(cls) = self.vm.native_class_table.get(&cls_name).cloned() {
            cls.invoke_native_method(self, obj_id, method_id);
        }
        let result = self.vm.param_pool.get_float_return();
        self.restore_returns(&saved);
        result
    }

    /// 在类上调用 native 静态方法（按类名分派）
    ///
    /// 调用方需先通过 set_*_param 布置参数，调用后通过 get_*_return 读取返回值。
    pub fn invoke_native_static_on(&mut self, class_name: &str, method_id: usize) {
        if let Some(cls) = self.vm.native_class_table.get(class_name).cloned() {
            cls.invoke_native_static(self, method_id);
        }
    }

    // ==================== 数组/集合访问 ====================

    /// 获取对象数组的各元素对象 ID 列表
    pub fn object_array_items(&self, obj_id: usize) -> Vec<usize> {
        use crate::system::native::array::ObjectArray;
        self.vm.native_payloads
            .get(&obj_id)
            .and_then(|p| p.downcast_ref::<ObjectArray>())
            .map(|a| a.items.clone())
            .unwrap_or_default()
    }

    /// 获取对象数组长度
    pub fn object_array_len(&self, obj_id: usize) -> usize {
        self.object_array_items(obj_id).len()
    }

    /// 从对象数组获取指定下标元素
    pub fn object_array_get(&self, obj_id: usize, index: usize) -> usize {
        self.object_array_items(obj_id).get(index).copied().unwrap_or(0)
    }

    /// 获取整数数组的各元素
    pub fn int_array_items(&self, obj_id: usize) -> Vec<i64> {
        use crate::system::native::array::IntArray;
        self.vm.native_payloads
            .get(&obj_id)
            .and_then(|p| p.downcast_ref::<IntArray>())
            .map(|a| a.items.clone())
            .unwrap_or_default()
    }

    /// 获取浮点数组的各元素
    pub fn float_array_items(&self, obj_id: usize) -> Vec<f64> {
        use crate::system::native::array::FloatArray;
        self.vm.native_payloads
            .get(&obj_id)
            .and_then(|p| p.downcast_ref::<FloatArray>())
            .map(|a| a.items.clone())
            .unwrap_or_default()
    }

    /// 向对象数组追加元素
    pub fn object_array_add(&mut self, obj_id: usize, value: usize) {
        use crate::system::native::array::ObjectArray;
        if let Some(payload) = self.vm.native_payloads.get_mut(&obj_id) {
            if let Some(arr) = payload.downcast_mut::<ObjectArray>() {
                arr.items.push(value);
            }
        }
    }

    // ==================== 载荷存储 ====================

    /// 为对象绑定 native 载荷数据
    pub fn insert_payload<T: std::any::Any>(&mut self, obj_id: usize, payload: Box<T>) {
        self.vm.native_payloads.insert(obj_id, payload);
    }

    /// 检查对象是否含有 native 载荷
    pub fn has_payload(&self, obj_id: usize) -> bool {
        self.vm.native_payloads.contains_key(&obj_id)
    }

    /// 获取对象的 native 载荷数据
    pub fn get_payload<T: std::any::Any>(&self, obj_id: usize) -> Option<&T> {
        self.vm.native_payloads.get(&obj_id).and_then(|p| p.downcast_ref::<T>())
    }

    /// 获取对象的 native 载荷数据（可变引用）
    pub fn get_payload_mut<T: std::any::Any>(&mut self, obj_id: usize) -> Option<&mut T> {
        self.vm.native_payloads.get_mut(&obj_id).and_then(|p| p.downcast_mut::<T>())
    }

    // ==================== 跨对象方法调用 ====================

    /// 在目标对象上按类名调用 native 实例方法
    ///
    /// 参数通过 ctx 设置，方法返回值通过 ctx 的 return 位读取。
    pub fn invoke_native_method_on(&mut self, class_name: &str, obj_id: usize, method_id: usize) {
        if let Some(cls) = self.vm.native_class_table.get(class_name).cloned() {
            cls.invoke_native_method(self, obj_id, method_id);
        }
    }

    // ==================== S3c 注解查询与调用 ====================

    /// 查询类中带指定注解的方法列表（S3c）
    ///
    /// 返回克隆后的 (方法全局ID, MethodAnnotation) 列表，避免借用纠缠。
    pub fn class_methods_with_annotation(
        &self,
        class_name: &str,
        annotation_name: &str,
    ) -> Vec<(usize, crate::objective::declaration::MethodAnnotation)> {
        self.vm.class_table
            .get(class_name)
            .map(|cls| {
                cls.declaration.methods_with_annotation(annotation_name)
                    .into_iter()
                    .map(|(id, ann)| (id, ann.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 查询类中带指定注解的构造方法列表（S3c）
    pub fn class_constructors_with_annotation(
        &self,
        class_name: &str,
        annotation_name: &str,
    ) -> Vec<(usize, crate::objective::declaration::MethodAnnotation)> {
        self.vm.class_table
            .get(class_name)
            .map(|cls| {
                cls.declaration.constructors_with_annotation(annotation_name)
                    .into_iter()
                    .map(|(id, ann)| (id, ann.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 按方法全局 ID 调用编译方法（S3c）
    ///
    /// 无参、静态调用。返回值通过 get_*_return 读取。
    pub fn invoke_method_by_id(
        &mut self,
        class_name: &str,
        obj_id: Option<usize>,
        method_id: usize,
    ) {
        if self.vm.invoke_method_by_id(class_name, obj_id, method_id).is_ok() {
            // VM return_* 寄存器已被写入，此处无需额外操作
        }
    }

    /// 通过注入器实例化对象（S3d）
    ///
    /// 创建对象 → 设注入器 → 执行字段初始化器+构造 → 恢复注入器 → 返回对象 ID。
    pub fn instantiate_with_injector(
        &mut self,
        class_name: &str,
        ctor_id: usize,
        injector_id: usize,
    ) -> usize {
        self.vm.instantiate_with_injector(class_name, ctor_id, injector_id).unwrap_or(0)
    }

    /// 深拷贝对象（A-2）
    ///
    /// 由 VM clone_object 提供完整克隆逻辑，本方法为便捷入口。
    pub fn clone_object(&mut self, obj_id: usize) -> Result<usize, String> {
        self.vm.clone_object(obj_id)
    }
}

/// 返回值快照（供 save_returns / restore_returns 使用）
#[derive(Debug, Clone)]
pub struct ReturnSnapshot {
    pub int: i64,
    pub float: f64,
    pub bool: bool,
    pub string: String,
    pub object: usize,
}

/// Native 类的运行时契约
///
/// 对应 C# `AutoGenerated` 中每个 native 类生成的 `Implementation : GorgeClass`。
/// 由 `GorgeMacros`（Phase B）为每个 native 类自动实现，也可手写。
///
/// 方法分派通过 `NativeContext` 访问参数池与对象表，与虚拟机解耦。
pub trait NativeClass: Debug + Send + Sync {
    /// 类的全名（含命名空间，如 `GorgeFramework.Vector2`）
    fn full_name(&self) -> &str;

    /// 字段各值类型的数量（用于为对象分配字段存储）
    fn field_type_count(&self) -> &TypeCount;

    /// 调用实例方法
    ///
    /// `obj_id` 为目标对象 ID，`method_id` 为方法在混合方法表中的下标。
    /// 参数从 `ctx` 的参数池读取，返回值写回参数池。
    fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize);

    /// 调用静态方法
    ///
    /// `method_id` 为方法在混合方法表中的下标。
    fn invoke_native_static(&self, ctx: &mut NativeContext, method_id: usize);

    /// 执行构造方法，返回新对象 ID
    ///
    /// `target` 为已存在的对象框架（外部编译类继承本 native 类时传入其 ID），
    /// 若为 `None` 则由本方法创建新对象。`ctor_id` 为构造方法下标。
    /// 返回构造完成的对象 ID。
    fn do_construct_native(
        &self,
        ctx: &mut NativeContext,
        target: Option<usize>,
        ctor_id: usize,
    ) -> usize;

    /// 获取 native 层的空白 RuntimeObject（字段全为默认值）
    ///
    /// 供构造流程创建对象框架使用。默认按 `field_type_count` 分配字段存储。
    fn make_empty_object(self: Arc<Self>) -> RuntimeObject {
        RuntimeObject::new_simple(self.full_name().to_string(), self.field_type_count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 一个手写的最小 native 类：模拟只有一个静态方法 `double(int) -> int`
    #[derive(Debug)]
    struct DoublerClass {
        name: String,
        field_counts: TypeCount,
    }

    impl NativeClass for DoublerClass {
        fn full_name(&self) -> &str {
            &self.name
        }

        fn field_type_count(&self) -> &TypeCount {
            &self.field_counts
        }

        fn invoke_native_method(&self, _ctx: &mut NativeContext, _obj_id: usize, _method_id: usize) {}

        fn invoke_native_static(&self, ctx: &mut NativeContext, method_id: usize) {
            if method_id == 0 {
                let arg = ctx.get_int_param(0);
                ctx.set_int_return(arg * 2);
            }
        }

        fn do_construct_native(
            &self,
            ctx: &mut NativeContext,
            target: Option<usize>,
            _ctor_id: usize,
        ) -> usize {
            match target {
                Some(id) => id,
                None => {
                    let obj = RuntimeObject::new_simple(self.name.clone(), &self.field_counts);
                    ctx.register_object(obj)
                }
            }
        }
    }

    fn make_vm() -> VirtualMachine {
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        vm
    }

    #[test]
    fn test_native_static_via_context() {
        let mut vm = make_vm();
        vm.param_pool.set_int_param(0, 21);

        let cls = DoublerClass {
            name: "Test.Doubler".into(),
            field_counts: TypeCount::zero(),
        };

        {
            let mut ctx = NativeContext::new(&mut vm);
            cls.invoke_native_static(&mut ctx, 0);
        }

        assert_eq!(vm.param_pool.get_int_return(), 42);
    }

    #[test]
    fn test_native_construct_via_context() {
        let mut vm = make_vm();

        let cls = DoublerClass {
            name: "Test.Doubler".into(),
            field_counts: TypeCount::zero(),
        };

        let obj_id = {
            let mut ctx = NativeContext::new(&mut vm);
            cls.do_construct_native(&mut ctx, None, 0)
        };

        assert_eq!(obj_id, 1);
        assert!(vm.objects.contains_key(&obj_id));
    }

    #[test]
    fn test_float_array_items_empty() {
        use crate::system::native::array::FloatArray;
        let mut vm = make_vm();
        let arr_id = 1;
        vm.native_payloads.insert(arr_id, Box::new(FloatArray { items: vec![] }));
        let ctx = NativeContext::new(&mut vm);
        let items = ctx.float_array_items(arr_id);
        assert!(items.is_empty());
    }

    #[test]
    fn test_float_array_items_with_data() {
        use crate::system::native::array::FloatArray;
        let mut vm = make_vm();
        let arr_id = 1;
        vm.native_payloads.insert(arr_id, Box::new(FloatArray { items: vec![1.5, 2.5, 3.5] }));
        let ctx = NativeContext::new(&mut vm);
        let items = ctx.float_array_items(arr_id);
        assert_eq!(items, vec![1.5, 2.5, 3.5]);
    }

    #[test]
    fn test_int_array_items_empty() {
        use crate::system::native::array::IntArray;
        let mut vm = make_vm();
        let arr_id = 1;
        vm.native_payloads.insert(arr_id, Box::new(IntArray { items: vec![] }));
        let ctx = NativeContext::new(&mut vm);
        let items = ctx.int_array_items(arr_id);
        assert!(items.is_empty());
    }

    #[test]
    fn test_int_array_items_with_data() {
        use crate::system::native::array::IntArray;
        let mut vm = make_vm();
        let arr_id = 1;
        vm.native_payloads.insert(arr_id, Box::new(IntArray { items: vec![10, 20, 30] }));
        let ctx = NativeContext::new(&mut vm);
        let items = ctx.int_array_items(arr_id);
        assert_eq!(items, vec![10, 20, 30]);
    }
}
