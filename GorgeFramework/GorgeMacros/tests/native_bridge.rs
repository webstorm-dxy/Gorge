//! GorgeMacros 集成测试
//!
//! 用两个真实 native 类（`Math` 纯静态、`Vector2` 含字段/构造/实例方法）验证
//! 宏生成的桥接层能被 `gorge_core` 虚拟机正确调用。

use std::collections::HashMap;

use gorge_core::native::{NativeClass, NativeContext};
use gorge_core::object::RuntimeObject;
use gorge_core::param_pool::InvokeParameterPool;

use gorge_macros::{gorge_native_class, gorge_native_impl};

// ==================== 测试类 1：Math（纯静态方法，无字段）====================

#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Math {}

#[gorge_native_impl]
impl Math {
    /// 静态方法 0：绝对值
    #[gorge_static]
    pub fn abs(_ctx: &mut NativeContext, f: f32) -> f32 {
        f.abs()
    }

    /// 静态方法 1：整数加一
    #[gorge_static]
    pub fn add_one(_ctx: &mut NativeContext, n: i32) -> i32 {
        n + 1
    }
}

// ==================== 测试类 2：Vector2（字段+构造+实例方法）====================

#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Vector2 {
    #[gorge_field]
    #[inject(default = 0.0)]
    pub x: f32,
    #[gorge_field]
    #[inject(default = 0.0)]
    pub y: f32,
}

#[gorge_native_impl]
impl Vector2 {
    /// 构造方法 0：从 x、y 初始化
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, x: f32, y: f32) {
        ctx.set_object_float_field(this, Vector2::FIELD_INDEX_x, x as f64);
        ctx.set_object_float_field(this, Vector2::FIELD_INDEX_y, y as f64);
    }

    /// 静态方法 0：两点距离
    #[gorge_static]
    pub fn distance(ctx: &mut NativeContext, v1: usize, v2: usize) -> f32 {
        let x1 = ctx.get_object_float_field(v1, Vector2::FIELD_INDEX_x);
        let y1 = ctx.get_object_float_field(v1, Vector2::FIELD_INDEX_y);
        let x2 = ctx.get_object_float_field(v2, Vector2::FIELD_INDEX_x);
        let y2 = ctx.get_object_float_field(v2, Vector2::FIELD_INDEX_y);
        let dx = x1 - x2;
        let dy = y1 - y2;
        (dx * dx + dy * dy).sqrt() as f32
    }

    /// 实例方法 1：读取 x 分量
    #[gorge_method]
    pub fn get_x(ctx: &mut NativeContext, this: usize) -> f32 {
        ctx.get_object_float_field(this, Vector2::FIELD_INDEX_x) as f32
    }
}

/// 构造上下文的测试脚手架
struct Fixture {
    pool: InvokeParameterPool,
    objects: HashMap<usize, RuntimeObject>,
    next_id: usize,
    native_payloads: HashMap<usize, Box<dyn std::any::Any>>,
}

impl Fixture {
    fn new() -> Self {
        Self {
            pool: InvokeParameterPool::new(),
            objects: HashMap::new(),
            next_id: 1,
            native_payloads: HashMap::new(),
        }
    }

    fn ctx(&mut self) -> NativeContext<'_> {
        NativeContext::new(&self.pool, &mut self.objects, &mut self.next_id, &mut self.native_payloads)
    }
}

#[test]
fn test_math_full_name_and_no_fields() {
    assert_eq!(Math::GORGE_FULL_NAME, "GorgeFramework.Math");
    let tc = Math::gorge_field_type_count();
    assert_eq!(tc.float_count, 0);
    assert_eq!(tc.int_count, 0);
}

#[test]
fn test_math_static_abs() {
    let math = Math {};
    let mut fx = Fixture::new();
    fx.pool.set_float_param(0, -3.5);
    {
        let mut ctx = fx.ctx();
        math.invoke_native_static(&mut ctx, 0); // abs
    }
    assert_eq!(fx.pool.get_float_return() as f32, 3.5);
}

#[test]
fn test_math_static_add_one() {
    let math = Math {};
    let mut fx = Fixture::new();
    fx.pool.set_int_param(0, 41);
    {
        let mut ctx = fx.ctx();
        math.invoke_native_static(&mut ctx, 1); // add_one
    }
    assert_eq!(fx.pool.get_int_return(), 42);
}

#[test]
fn test_vector2_metadata() {
    assert_eq!(Vector2::GORGE_FULL_NAME, "GorgeFramework.Vector2");
    let tc = Vector2::gorge_field_type_count();
    assert_eq!(tc.float_count, 2);
    // 字段索引按 float 组分配：x=0, y=1
    assert_eq!(Vector2::FIELD_INDEX_x, 0);
    assert_eq!(Vector2::FIELD_INDEX_y, 1);
    // 注入器字段索引同样按 float 组分配
    assert_eq!(Vector2::INJECTOR_INDEX_x, 0);
    assert_eq!(Vector2::INJECTOR_INDEX_y, 1);
    // 注入器默认值
    assert_eq!(Vector2::gorge_injector_default_x(), 0.0);
    assert_eq!(Vector2::gorge_injector_default_y(), 0.0);
}

#[test]
fn test_vector2_construct_and_get() {
    let v = Vector2 { x: 0.0, y: 0.0 };
    let mut fx = Fixture::new();

    // 构造：param float[0]=3.0, float[1]=4.0
    fx.pool.set_float_param(0, 3.0);
    fx.pool.set_float_param(1, 4.0);
    let obj_id = {
        let mut ctx = fx.ctx();
        v.do_construct_native(&mut ctx, None, 0)
    };
    assert!(obj_id != 0);
    assert!(fx.objects.contains_key(&obj_id));

    // 实例方法 get_x（编号 1，与 distance 共享混合编号空间）
    {
        let mut ctx = fx.ctx();
        v.invoke_native_method(&mut ctx, obj_id, 1);
    }
    assert_eq!(fx.pool.get_float_return() as f32, 3.0);
}

#[test]
fn test_vector2_static_distance() {
    let v = Vector2 { x: 0.0, y: 0.0 };
    let mut fx = Fixture::new();

    // 构造两个点：(0,0) 和 (3,4)
    fx.pool.set_float_param(0, 0.0);
    fx.pool.set_float_param(1, 0.0);
    let p1 = {
        let mut ctx = fx.ctx();
        v.do_construct_native(&mut ctx, None, 0)
    };
    fx.pool.set_float_param(0, 3.0);
    fx.pool.set_float_param(1, 4.0);
    let p2 = {
        let mut ctx = fx.ctx();
        v.do_construct_native(&mut ctx, None, 0)
    };

    // distance 是静态方法编号 0
    fx.pool.set_object_param(0, p1);
    fx.pool.set_object_param(1, p2);
    {
        let mut ctx = fx.ctx();
        v.invoke_native_static(&mut ctx, 0);
    }
    assert_eq!(fx.pool.get_float_return() as f32, 5.0);
}

#[test]
fn test_vector2_field_initialize_applies_injector_override() {
    // 验证 native 构造时注入器字段覆写生效（对齐 C# FieldInitialize）。
    use gorge_core::injector::{RuntimeInjector, Injector};
    use gorge_core::declaration::ClassDeclaration;
    use gorge_core::object::GorgeObject;
    use gorge_core::types::{GorgeType, TypeCount};
    use std::sync::Arc;

    // 构造一个含 2 个 float 注入器字段的注入器，x 显式设为 7.0，y 保持默认
    let decl = Arc::new(ClassDeclaration {
        class_type: GorgeType::class("GorgeFramework.Vector2", None),
        is_native: true, annotations: vec![], fields: vec![],
        methods: vec![], static_methods: vec![], constructors: vec![],
        injector_fields: vec![], super_class: None, super_interfaces: vec![],
        field_type_count: TypeCount::zero(),
        method_count: 0, static_method_count: 0, constructor_count: 0,
        injector_field_type_count: TypeCount { float_count: 2, ..TypeCount::zero() },
        injector_field_default_value_type_count: TypeCount::zero(),
        method_start_id: 0, constructor_start_id: 0,
        interface_method_impl_id: HashMap::new(),
        method_override_id: HashMap::new(),
        injector_constructor_impl_id: vec![],
    });
    let mut inj = RuntimeInjector::new(decl);
    inj.set_injector_float(Vector2::INJECTOR_INDEX_x, 7.0); // x 显式覆写
    // y 保持默认（default_value 标记为 true）

    let mut injectors: HashMap<usize, RuntimeInjector> = HashMap::new();
    injectors.insert(100, inj);

    let mut objects: HashMap<usize, RuntimeObject> = HashMap::new();
    let mut next_id = 1usize;
    let mut payloads: HashMap<usize, Box<dyn std::any::Any>> = HashMap::new();
    let pool = InvokeParameterPool::new();

    // 直接调用 gorge_field_initialize：在新对象上应用注入器覆写
    let obj = RuntimeObject::new_simple(
        Vector2::GORGE_FULL_NAME.to_string(),
        &Vector2::gorge_field_type_count(),
    );
    let obj_id;
    {
        let mut ctx = NativeContext::with_injector(
            &pool, &mut objects, &mut next_id, &mut payloads, &injectors, 100,
        );
        obj_id = ctx.register_object(obj);
        Vector2::gorge_field_initialize(&mut ctx, obj_id);
    }
    // x 被注入器覆写为 7.0；y 用默认值 0.0
    assert_eq!(objects.get(&obj_id).unwrap().get_float_field(Vector2::FIELD_INDEX_x) as f32, 7.0);
    assert_eq!(objects.get(&obj_id).unwrap().get_float_field(Vector2::FIELD_INDEX_y) as f32, 0.0);
}

