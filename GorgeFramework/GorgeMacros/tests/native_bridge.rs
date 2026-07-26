//! GorgeMacros 集成测试
//!
//! 用两个真实 native 类（`Math` 纯静态、`Vector2` 含字段/构造/实例方法）验证
//! 宏生成的桥接层能被 `gorge_core` 虚拟机正确调用。

use std::collections::HashMap;

use gorge_core::objective::native::{NativeClass, NativeContext};
use gorge_core::objective::object::GorgeObject;
use gorge_core::objective::object::RuntimeObject;
use gorge_core::virtual_machine::vm::VirtualMachine;

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

/// 构造上下文的测试脚手架（1d 重构后持 VM）
struct Fixture {
    vm: VirtualMachine,
}

impl Fixture {
    fn new() -> Self {
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        Self { vm }
    }

    fn ctx(&mut self) -> NativeContext<'_> {
        NativeContext::new(&mut self.vm)
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
    fx.vm.param_pool.set_float_param(0, -3.5);
    {
        let mut ctx = fx.ctx();
        math.invoke_native_static(&mut ctx, 0); // abs
    }
    assert_eq!(fx.vm.param_pool.get_float_return() as f32, 3.5);
}

#[test]
fn test_math_static_add_one() {
    let math = Math {};
    let mut fx = Fixture::new();
    fx.vm.param_pool.set_int_param(0, 41);
    {
        let mut ctx = fx.ctx();
        math.invoke_native_static(&mut ctx, 1); // add_one
    }
    assert_eq!(fx.vm.param_pool.get_int_return(), 42);
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
    fx.vm.param_pool.set_float_param(0, 3.0);
    fx.vm.param_pool.set_float_param(1, 4.0);
    let obj_id = {
        let mut ctx = fx.ctx();
        v.do_construct_native(&mut ctx, None, 0)
    };
    assert!(obj_id != 0);
    assert!(fx.vm.objects.contains_key(&obj_id));

    // 实例方法 get_x（编号 1，与 distance 共享混合编号空间）
    {
        let mut ctx = fx.ctx();
        v.invoke_native_method(&mut ctx, obj_id, 1);
    }
    assert_eq!(fx.vm.param_pool.get_float_return() as f32, 3.0);
}

#[test]
fn test_vector2_static_distance() {
    let v = Vector2 { x: 0.0, y: 0.0 };
    let mut fx = Fixture::new();

    // 构造两个点：(0,0) 和 (3,4)
    fx.vm.param_pool.set_float_param(0, 0.0);
    fx.vm.param_pool.set_float_param(1, 0.0);
    let p1 = {
        let mut ctx = fx.ctx();
        v.do_construct_native(&mut ctx, None, 0)
    };
    fx.vm.param_pool.set_float_param(0, 3.0);
    fx.vm.param_pool.set_float_param(1, 4.0);
    let p2 = {
        let mut ctx = fx.ctx();
        v.do_construct_native(&mut ctx, None, 0)
    };

    // distance 是静态方法编号 0
    fx.vm.param_pool.set_object_param(0, p1);
    fx.vm.param_pool.set_object_param(1, p2);
    {
        let mut ctx = fx.ctx();
        v.invoke_native_static(&mut ctx, 0);
    }
    assert_eq!(fx.vm.param_pool.get_float_return() as f32, 5.0);
}

// ==================== M-1 回归测试 ====================

/// 单方法 impl（仅 1 个 #[gorge_method]，无 ctor/static）— 回归参数计数 bug
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct SingleMethodTest {
    #[gorge_field]
    pub _placeholder: bool,
}

#[gorge_native_impl]
impl SingleMethodTest {
    /// 实例方法 0：接收 float 参数，返回 float
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let _ = ctx;
        let _ = this;
        x * 2.0
    }
}

/// 多方法不同参数数测试（2 个 #[gorge_method] 分别 1 个/0 个值参数）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct MultiMethodTest {
    #[gorge_field]
    pub _placeholder: bool,
}

#[gorge_native_impl]
impl MultiMethodTest {
    /// 实例方法 0：接收 float 参数，返回 float
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let _ = ctx;
        let _ = this;
        x * 2.0
    }

    /// 实例方法 1：无值参数，返回 int
    #[gorge_method]
    pub fn get_value(ctx: &mut NativeContext, this: usize) -> i32 {
        let _ = ctx;
        let _ = this;
        42
    }
}

#[test]
fn test_single_method_impl_evaluate() {
    let obj = SingleMethodTest { _placeholder: false };
    let mut fx = Fixture::new();

    // 构造
    fx.vm.param_pool.set_bool_param(0, false);
    let obj_id = {
        let mut ctx = fx.ctx();
        obj.do_construct_native(&mut ctx, None, 0)
    };

    // 调 evaluate(x=3.0)
    fx.vm.param_pool.set_float_param(0, 3.0);
    {
        let mut ctx = fx.ctx();
        obj.invoke_native_method(&mut ctx, obj_id, 0);
    }
    assert_eq!(fx.vm.param_pool.get_float_return() as f32, 6.0);
}

#[test]
fn test_multi_method_different_arity() {
    let obj = MultiMethodTest { _placeholder: false };
    let mut fx = Fixture::new();

    // 构造
    fx.vm.param_pool.set_bool_param(0, false);
    let obj_id = {
        let mut ctx = fx.ctx();
        obj.do_construct_native(&mut ctx, None, 0)
    };

    // 调 evaluate(x=5.0) — 有值参数
    fx.vm.param_pool.set_float_param(0, 5.0);
    {
        let mut ctx = fx.ctx();
        obj.invoke_native_method(&mut ctx, obj_id, 0);
    }
    assert_eq!(fx.vm.param_pool.get_float_return() as f32, 10.0);

    // 调 get_value() — 无值参数
    {
        let mut ctx = fx.ctx();
        obj.invoke_native_method(&mut ctx, obj_id, 1);
    }
    assert_eq!(fx.vm.param_pool.get_int_return(), 42);
}

#[test]
fn test_vector2_field_initialize_applies_injector_override() {
    // 验证 native 构造时注入器字段覆写生效（对齐 C# FieldInitialize）。
    use gorge_core::system::native::injector::{RuntimeInjector, Injector};
    use gorge_core::objective::declaration::ClassDeclaration;
    use gorge_core::objective::object::GorgeObject;
    use gorge_core::objective::types::{GorgeType, TypeCount};
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
        method_annotations: std::collections::HashMap::new(),
        constructor_annotations: std::collections::HashMap::new(),
    });
    let mut inj = RuntimeInjector::new(decl);
    inj.set_injector_float(Vector2::INJECTOR_INDEX_x, 7.0); // x 显式覆写
    // y 保持默认（default_value 标记为 true）

    let mut vm = VirtualMachine::new();
    vm.next_object_id = 1;
    vm.injectors.insert(100, inj);

    // 直接调用 gorge_field_initialize：在新对象上应用注入器覆写
    let obj = RuntimeObject::new_simple(
        Vector2::GORGE_FULL_NAME.to_string(),
        &Vector2::gorge_field_type_count(),
    );
    let obj_id;
    {
        let mut ctx = NativeContext::with_injector(&mut vm, 100);
        obj_id = ctx.register_object(obj);
        Vector2::gorge_field_initialize(&mut ctx, obj_id);
    }
    // x 被注入器覆写为 7.0；y 用默认值 0.0
    assert_eq!(vm.objects.get(&obj_id).unwrap().get_float_field(Vector2::FIELD_INDEX_x) as f32, 7.0);
    assert_eq!(vm.objects.get(&obj_id).unwrap().get_float_field(Vector2::FIELD_INDEX_y) as f32, 0.0);
}

// ==================== 注入器构造方法测试（Fix 2）====================

/// 含注入器构造方法与普通构造方法的类
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct InjectorCtorTest {
    #[gorge_field]
    pub value: i32,
}

#[gorge_native_impl]
impl InjectorCtorTest {
    /// 注入器构造方法 0：从注入器参数初始化
    #[gorge_injector_ctor]
    pub fn from_injector(ctx: &mut NativeContext, this: usize, v: i32) {
        ctx.set_object_int_field(this, InjectorCtorTest::FIELD_INDEX_value, v as i64);
    }

    /// 普通构造方法 0：标准构造
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, v: i32) {
        ctx.set_object_int_field(this, InjectorCtorTest::FIELD_INDEX_value, v as i64);
    }
}

#[test]
fn test_injector_ctor_count() {
    assert_eq!(InjectorCtorTest::gorge_injector_constructor_count(), 1);
}

#[test]
fn test_injector_ctor_dispatch() {
    let obj = InjectorCtorTest { value: 0 };
    let mut fx = Fixture::new();

    // 先构造对象框架
    fx.vm.param_pool.set_int_param(0, 0);
    let obj_id = {
        let mut ctx = fx.ctx();
        obj.do_construct_native(&mut ctx, None, 0) // 调普通构造方法 0
    };
    assert!(obj_id > 0);

    // 再通过注入器构造方法覆写字段（模拟 VM 在注入器场景下的调用）
    fx.vm.param_pool.set_int_param(0, 99);
    {
        let mut ctx = fx.ctx();
        InjectorCtorTest::gorge_invoke_injector_constructor(&mut ctx, obj_id, 0);
    }
    assert_eq!(fx.vm.objects.get(&obj_id).unwrap().get_int_field(0), 99);
}

// ==================== 类注解测试（Fix 3）====================

/// 无注解类：验证默认返回空列表
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct NoAnnotationTest {
    #[gorge_field]
    pub _placeholder: bool,
}

/// 含注解类：验证 `annotations = [...]` 语法
#[gorge_native_class(namespace = "GorgeFramework", annotations = ["Serialize", "Export"])]
pub struct WithAnnotationTest {
    #[gorge_field]
    pub _placeholder: bool,
}

#[test]
fn test_no_class_annotations_returns_empty() {
    let anns = NoAnnotationTest::gorge_class_annotations();
    assert!(anns.is_empty());
}

#[test]
fn test_class_annotations() {
    let anns = WithAnnotationTest::gorge_class_annotations();
    assert_eq!(anns.len(), 2);
    assert_eq!(anns[0].name, "Serialize");
    assert!(anns[0].generic_type.is_none());
    assert!(anns[0].arguments.is_empty());
    assert_eq!(anns[1].name, "Export");
    assert!(anns[1].generic_type.is_none());
    assert!(anns[1].arguments.is_empty());
}
