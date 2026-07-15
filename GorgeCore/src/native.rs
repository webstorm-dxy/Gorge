use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::object::{GorgeObject, RuntimeObject};
use crate::param_pool::InvokeParameterPool;
use crate::types::TypeCount;

/// 空注入器表，供无注入器上下文的 `NativeContext::new` 借用（避免 Option 分支）。
static EMPTY_INJECTORS: std::sync::LazyLock<HashMap<usize, crate::injector::RuntimeInjector>> =
    std::sync::LazyLock::new(HashMap::new);

/// Native 方法执行上下文
///
/// 对应 C# 中 native 桥接层通过全局静态 `InvokeParameterPool` 与虚拟机通信的机制。
/// Rust 侧为避免全局可变状态，改为在调用 native 方法时把虚拟机的关键部分
/// （参数池、对象表、对象 ID 分配器）打包成一个上下文可变引用传入。
///
/// native 桥接函数通过本上下文：
/// - 读取调用参数（`get_*_param`）
/// - 写入返回值（`set_*_return`）
/// - 读写对象字段（`get_*_field` / `set_*_field`）
/// - 创建新对象（`register_object`）
///
/// # 生命周期
/// `'a` 绑定到虚拟机本次调用期间，上下文不拥有任何状态，仅借用虚拟机的字段。
pub struct NativeContext<'a> {
    /// 调用参数池（参数读取与返回值写入）
    pub param_pool: &'a InvokeParameterPool,
    /// 正式对象表：对象 ID → RuntimeObject
    pub objects: &'a mut HashMap<usize, RuntimeObject>,
    /// 对象 ID 分配器（下一个可用 ID）
    pub next_object_id: &'a mut usize,
    /// 原生集合对象载荷表（Phase H）：对象 ID → 类型的 Rust 数据
    pub native_payloads: &'a mut HashMap<usize, Box<dyn std::any::Any>>,
    /// 注入器对象表：注入器 ID → RuntimeInjector（供 native 构造读取注入器字段）
    pub injectors: &'a HashMap<usize, crate::injector::RuntimeInjector>,
    /// 当前注入器对象 ID（0 表示无，对应 C# `InvokeParameterPool.Injector`）
    pub current_injector: usize,
}

impl<'a> NativeContext<'a> {
    /// 创建上下文
    pub fn new(
        param_pool: &'a InvokeParameterPool,
        objects: &'a mut HashMap<usize, RuntimeObject>,
        next_object_id: &'a mut usize,
        native_payloads: &'a mut HashMap<usize, Box<dyn std::any::Any>>,
    ) -> Self {
        Self {
            param_pool,
            objects,
            next_object_id,
            native_payloads,
            injectors: &EMPTY_INJECTORS,
            current_injector: 0,
        }
    }

    /// 创建带注入器上下文的上下文（用于 native 构造读取注入器字段覆写）
    pub fn with_injector(
        param_pool: &'a InvokeParameterPool,
        objects: &'a mut HashMap<usize, RuntimeObject>,
        next_object_id: &'a mut usize,
        native_payloads: &'a mut HashMap<usize, Box<dyn std::any::Any>>,
        injectors: &'a HashMap<usize, crate::injector::RuntimeInjector>,
        current_injector: usize,
    ) -> Self {
        Self {
            param_pool,
            objects,
            next_object_id,
            native_payloads,
            injectors,
            current_injector,
        }
    }

    // ==================== 参数读取 ====================

    /// 读取整数参数
    pub fn get_int_param(&self, index: usize) -> i64 {
        self.param_pool.get_int_param(index)
    }

    /// 读取浮点参数
    pub fn get_float_param(&self, index: usize) -> f64 {
        self.param_pool.get_float_param(index)
    }

    /// 读取布尔参数
    pub fn get_bool_param(&self, index: usize) -> bool {
        self.param_pool.get_bool_param(index)
    }

    /// 读取字符串参数
    pub fn get_string_param(&self, index: usize) -> String {
        self.param_pool.get_string_param(index)
    }

    /// 读取对象参数（对象 ID）
    pub fn get_object_param(&self, index: usize) -> usize {
        self.param_pool.get_object_param(index)
    }

    /// 读取注入器专用位（注入器对象 ID，0 表示无）
    pub fn get_injector(&self) -> usize {
        self.param_pool.get_injector()
    }

    // ==================== 注入器字段读取（native 构造用） ====================
    //
    // 用于 native 类构造时应用注入器字段覆写（对齐 C# `FieldInitialize(injector)`）。
    // 每个方法返回 `Some(值)` 当且仅当：存在当前注入器，且该注入器对应类型字段
    // 未使用默认值标记（即被显式赋值）；否则返回 `None`，调用方应回退到字段默认值。

    /// 读取当前注入器的 float 字段值（`inj_index` 为该字段在 float 分组内的索引）。
    pub fn injector_float(&self, inj_index: usize) -> Option<f64> {
        use crate::injector::Injector;
        let inj = self.injectors.get(&self.current_injector)?;
        if inj_index >= inj.float_field_count() || inj.get_injector_float_default_value(inj_index) {
            None
        } else {
            Some(inj.get_injector_float(inj_index))
        }
    }

    /// 读取当前注入器的 int 字段值。
    pub fn injector_int(&self, inj_index: usize) -> Option<i64> {
        use crate::injector::Injector;
        let inj = self.injectors.get(&self.current_injector)?;
        if inj_index >= inj.int_field_count() || inj.get_injector_int_default_value(inj_index) {
            None
        } else {
            Some(inj.get_injector_int(inj_index))
        }
    }

    /// 读取当前注入器的 bool 字段值。
    pub fn injector_bool(&self, inj_index: usize) -> Option<bool> {
        use crate::injector::Injector;
        let inj = self.injectors.get(&self.current_injector)?;
        if inj_index >= inj.bool_field_count() || inj.get_injector_bool_default_value(inj_index) {
            None
        } else {
            Some(inj.get_injector_bool(inj_index))
        }
    }

    /// 读取当前注入器的 string 字段值。
    pub fn injector_string(&self, inj_index: usize) -> Option<String> {
        use crate::injector::Injector;
        let inj = self.injectors.get(&self.current_injector)?;
        if inj_index >= inj.string_field_count() || inj.get_injector_string_default_value(inj_index) {
            None
        } else {
            Some(inj.get_injector_string(inj_index))
        }
    }

    /// 读取当前注入器的 object 字段值（对象 ID）。
    pub fn injector_object(&self, inj_index: usize) -> Option<usize> {
        use crate::injector::Injector;
        let inj = self.injectors.get(&self.current_injector)?;
        if inj_index >= inj.object_field_count() || inj.get_injector_object_default_value(inj_index) {
            None
        } else {
            Some(inj.get_injector_object(inj_index))
        }
    }

    // ==================== 返回值写入 ====================

    /// 写入整数返回值
    pub fn set_int_return(&self, value: i64) {
        self.param_pool.set_int_return(value);
    }

    /// 写入浮点返回值
    pub fn set_float_return(&self, value: f64) {
        self.param_pool.set_float_return(value);
    }

    /// 写入布尔返回值
    pub fn set_bool_return(&self, value: bool) {
        self.param_pool.set_bool_return(value);
    }

    /// 写入字符串返回值
    pub fn set_string_return(&self, value: String) {
        self.param_pool.set_string_return(value);
    }

    /// 写入对象返回值（对象 ID）
    pub fn set_object_return(&self, value: usize) {
        self.param_pool.set_object_return(value);
    }

    // ==================== 对象字段访问 ====================

    /// 读取对象的浮点字段
    ///
    /// `obj_id` 为对象表中的对象 ID，`index` 为字段在其值类型分组内的下标。
    pub fn get_object_float_field(&self, obj_id: usize, index: usize) -> f64 {
        self.objects
            .get(&obj_id)
            .map(|o| o.get_float_field(index))
            .unwrap_or(0.0)
    }

    /// 读取对象的整数字段
    pub fn get_object_int_field(&self, obj_id: usize, index: usize) -> i64 {
        self.objects
            .get(&obj_id)
            .map(|o| o.get_int_field(index))
            .unwrap_or(0)
    }

    /// 读取对象的布尔字段
    pub fn get_object_bool_field(&self, obj_id: usize, index: usize) -> bool {
        self.objects
            .get(&obj_id)
            .map(|o| o.get_bool_field(index))
            .unwrap_or(false)
    }

    /// 读取对象的字符串字段
    pub fn get_object_string_field(&self, obj_id: usize, index: usize) -> String {
        self.objects
            .get(&obj_id)
            .map(|o| o.get_string_field(index))
            .unwrap_or_default()
    }

    /// 读取对象的对象字段（对象 ID）
    pub fn get_object_object_field(&self, obj_id: usize, index: usize) -> usize {
        self.objects
            .get(&obj_id)
            .map(|o| o.get_object_field(index))
            .unwrap_or(0)
    }

    /// 写入对象的浮点字段
    pub fn set_object_float_field(&mut self, obj_id: usize, index: usize, value: f64) {
        if let Some(o) = self.objects.get_mut(&obj_id) {
            o.set_float_field(index, value);
        }
    }

    /// 写入对象的整数字段
    pub fn set_object_int_field(&mut self, obj_id: usize, index: usize, value: i64) {
        if let Some(o) = self.objects.get_mut(&obj_id) {
            o.set_int_field(index, value);
        }
    }

    /// 写入对象的布尔字段
    pub fn set_object_bool_field(&mut self, obj_id: usize, index: usize, value: bool) {
        if let Some(o) = self.objects.get_mut(&obj_id) {
            o.set_bool_field(index, value);
        }
    }

    /// 写入对象的字符串字段
    pub fn set_object_string_field(&mut self, obj_id: usize, index: usize, value: String) {
        if let Some(o) = self.objects.get_mut(&obj_id) {
            o.set_string_field(index, value);
        }
    }

    /// 写入对象的对象字段（对象 ID）
    pub fn set_object_object_field(&mut self, obj_id: usize, index: usize, value: usize) {
        if let Some(o) = self.objects.get_mut(&obj_id) {
            o.set_object_field(index, value);
        }
    }

    // ==================== 对象创建 ====================

    /// 分配一个新的对象 ID
    pub fn alloc_object_id(&mut self) -> usize {
        let id = *self.next_object_id;
        *self.next_object_id += 1;
        id
    }

    /// 将一个 RuntimeObject 注册进对象表，返回其对象 ID
    ///
    /// 用于 native 构造方法创建新对象实例。
    pub fn register_object(&mut self, obj: RuntimeObject) -> usize {
        let id = self.alloc_object_id();
        self.objects.insert(id, obj);
        id
    }
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
            // 0 号静态方法：double(int) -> int
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

    #[test]
    fn test_native_static_via_context() {
        let pool = InvokeParameterPool::new();
        let mut objects: HashMap<usize, RuntimeObject> = HashMap::new();
        let mut next_id = 1usize;

        pool.set_int_param(0, 21);

        let cls = DoublerClass {
            name: "Test.Doubler".into(),
            field_counts: TypeCount::zero(),
        };

        {
            let mut native_payloads: HashMap<usize, Box<dyn std::any::Any>> = HashMap::new();
            let mut ctx = NativeContext::new(&pool, &mut objects, &mut next_id, &mut native_payloads);
            cls.invoke_native_static(&mut ctx, 0);
        }

        assert_eq!(pool.get_int_return(), 42);
    }

    #[test]
    fn test_native_construct_via_context() {
        let pool = InvokeParameterPool::new();
        let mut objects: HashMap<usize, RuntimeObject> = HashMap::new();
        let mut next_id = 1usize;

        let cls = DoublerClass {
            name: "Test.Doubler".into(),
            field_counts: TypeCount::zero(),
        };

        let obj_id = {
            let mut native_payloads: HashMap<usize, Box<dyn std::any::Any>> = HashMap::new();
            let mut ctx = NativeContext::new(&pool, &mut objects, &mut next_id, &mut native_payloads);
            cls.do_construct_native(&mut ctx, None, 0)
        };

        assert_eq!(obj_id, 1);
        assert!(objects.contains_key(&obj_id));
    }
}
