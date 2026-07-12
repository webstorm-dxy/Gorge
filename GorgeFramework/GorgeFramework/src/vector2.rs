//! `GorgeFramework.Vector2` —— 二维向量 native 类。
//!
//! 移植自 C# 参考实现 `System/Native/Vector2.cs` 的代表性子集，
//! 用于验证 `#[gorge_native_class]` / `#[gorge_native_impl]` 在含字段、
//! 构造方法、实例方法、静态方法、注入器字段、以及「返回新对象」场景下的桥接。

use gorge_core::native::NativeContext;
use gorge_core::object::RuntimeObject;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 二维向量，含 x、y 两个 float 字段
///
/// x、y 同时是注入器字段（默认值 0.0），对应 C# 的 `@Inject float x = ^x`。
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
    /// 构造方法 0：从 x、y 初始化对象字段
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, x: f32, y: f32) {
        ctx.set_object_float_field(this, Vector2::FIELD_INDEX_x, x as f64);
        ctx.set_object_float_field(this, Vector2::FIELD_INDEX_y, y as f64);
    }

    /// 静态方法 0：两点欧氏距离
    #[gorge_static]
    pub fn distance(ctx: &mut NativeContext, v1: usize, v2: usize) -> f32 {
        let (x1, y1) = read_xy(ctx, v1);
        let (x2, y2) = read_xy(ctx, v2);
        let dx = x1 - x2;
        let dy = y1 - y2;
        (dx * dx + dy * dy).sqrt()
    }

    /// 静态方法 1：分量相乘，返回新 Vector2（对象 ID）
    ///
    /// 演示「native 方法创建并返回新对象」：在方法体内经 `ctx` 分配对象、
    /// 写入字段，返回其对象 ID。
    #[gorge_static]
    pub fn scale(ctx: &mut NativeContext, v1: usize, v2: usize) -> usize {
        let (x1, y1) = read_xy(ctx, v1);
        let (x2, y2) = read_xy(ctx, v2);
        make_vector2(ctx, x1 * x2, y1 * y2)
    }

    /// 实例方法 2：向量模长
    #[gorge_method]
    pub fn magnitude(ctx: &mut NativeContext, this: usize) -> f32 {
        let (x, y) = read_xy(ctx, this);
        (x * x + y * y).sqrt()
    }

    /// 实例方法 3：读取 x 分量
    #[gorge_method]
    pub fn get_x(ctx: &mut NativeContext, this: usize) -> f32 {
        ctx.get_object_float_field(this, Vector2::FIELD_INDEX_x) as f32
    }

    /// 实例方法 4：读取 y 分量
    #[gorge_method]
    pub fn get_y(ctx: &mut NativeContext, this: usize) -> f32 {
        ctx.get_object_float_field(this, Vector2::FIELD_INDEX_y) as f32
    }

    /// 静态方法 5：线性插值（混合类型参数：Vector2, Vector2, float）
    ///
    /// 用于验证 B-2——参数按值类型分组编号（两个 object 参数 + 一个 float 参数），
    /// 返回插值得到的新 Vector2 对象 ID。
    #[gorge_static]
    pub fn lerp(ctx: &mut NativeContext, a: usize, b: usize, t: f32) -> usize {
        let (ax, ay) = read_xy(ctx, a);
        let (bx, by) = read_xy(ctx, b);
        let t = t.clamp(0.0, 1.0);
        make_vector2(ctx, ax + (bx - ax) * t, ay + (by - ay) * t)
    }
}

/// 读取某 Vector2 对象的 (x, y) 分量
///
/// 内部辅助函数，未标注 Gorge 属性，原样保留。
fn read_xy(ctx: &NativeContext, obj_id: usize) -> (f32, f32) {
    let x = ctx.get_object_float_field(obj_id, Vector2::FIELD_INDEX_x) as f32;
    let y = ctx.get_object_float_field(obj_id, Vector2::FIELD_INDEX_y) as f32;
    (x, y)
}

/// 在对象表中创建一个新的 Vector2 并写入 x、y，返回其对象 ID
///
/// 内部辅助函数，供「返回新对象」的静态方法复用。
fn make_vector2(ctx: &mut NativeContext, x: f32, y: f32) -> usize {
    let obj = RuntimeObject::new_simple(
        Vector2::GORGE_FULL_NAME.to_string(),
        &Vector2::gorge_field_type_count(),
    );
    let id = ctx.register_object(obj);
    ctx.set_object_float_field(id, Vector2::FIELD_INDEX_x, x as f64);
    ctx.set_object_float_field(id, Vector2::FIELD_INDEX_y, y as f64);
    id
}
