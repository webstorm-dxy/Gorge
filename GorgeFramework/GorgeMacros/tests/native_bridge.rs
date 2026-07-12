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
}

impl Fixture {
    fn new() -> Self {
        Self {
            pool: InvokeParameterPool::new(),
            objects: HashMap::new(),
            next_id: 1,
        }
    }

    fn ctx(&mut self) -> NativeContext<'_> {
        NativeContext::new(&self.pool, &mut self.objects, &mut self.next_id)
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
