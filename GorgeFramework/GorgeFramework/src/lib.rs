//! GorgeFramework —— Gorge 音乐游戏框架的 native 类库（Rust 移植）。
//!
//! 本 crate 用 `gorge_macros` 提供的桥接宏，将框架的 native 类以纯 Rust
//! 结构体 + 业务实现的形式暴露给 Gorge 虚拟机。本轮（Phase C）实现两个
//! 示范类：
//! - [`Math`]：纯静态数学工具类，验证无字段/纯静态方法路径。
//! - [`Vector2`]：二维向量，验证字段、构造、实例方法、静态方法、注入器字段、
//!   以及「返回新对象」的完整桥接。
//!
//! 通过 [`register_native`] 把全部 native 类一次性注册进 [`gorge_core::runtime::GorgeRuntime`]。

pub mod math;
pub mod vector2;

pub use math::Math;
pub use vector2::Vector2;

use gorge_core::native::NativeClass;
use gorge_core::runtime::GorgeRuntime;
use std::sync::Arc;

/// 返回框架全部 native 类实例（`Arc<dyn NativeClass>`）。
///
/// 供调用方按需注册（如按全名或简单名）。每个元素是一个 native 类的共享句柄。
pub fn native_classes() -> Vec<Arc<dyn NativeClass>> {
    vec![
        Arc::new(Math {}),
        Arc::new(Vector2 { x: 0.0, y: 0.0 }),
    ]
}

/// 把 GorgeFramework 的全部 native 类注册进运行时。
///
/// 注册后，虚拟机即可通过全名（如 `GorgeFramework.Math`、
/// `GorgeFramework.Vector2`）分派这些类的静态方法、实例方法与构造方法。
///
/// # 参数
/// - `runtime`：目标运行时，注册会同时写入其内部虚拟机的 native 类表。
pub fn register_native(runtime: &mut GorgeRuntime) {
    for cls in native_classes() {
        runtime.register_native_class(cls);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::native::{NativeClass, NativeContext};
    use gorge_core::object::{GorgeObject, RuntimeObject};
    use gorge_core::param_pool::InvokeParameterPool;
    use std::collections::HashMap;

    /// 测试脚手架：独立于虚拟机，直接用 NativeContext 驱动 native 类
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
    fn test_register_native_into_runtime() {
        // 验证 register_native 把两个类注册进运行时
        let mut runtime = GorgeRuntime::new();
        register_native(&mut runtime);
        assert!(runtime.is_native_class("GorgeFramework.Math"));
        assert!(runtime.is_native_class("GorgeFramework.Vector2"));
    }

    #[test]
    fn test_math_sqrt_via_native() {
        let math = Math {};
        let mut fx = Fixture::new();
        fx.pool.set_float_param(0, 16.0);
        {
            let mut ctx = fx.ctx();
            math.invoke_native_static(&mut ctx, 1); // sqrt
        }
        assert_eq!(fx.pool.get_float_return() as f32, 4.0);
    }

    #[test]
    fn test_math_lerp_and_clamp() {
        let math = Math {};
        let mut fx = Fixture::new();
        // lerp(10, 20, 0.5) = 15
        fx.pool.set_float_param(0, 10.0);
        fx.pool.set_float_param(1, 20.0);
        fx.pool.set_float_param(2, 0.5);
        {
            let mut ctx = fx.ctx();
            math.invoke_native_static(&mut ctx, 6); // lerp
        }
        assert_eq!(fx.pool.get_float_return() as f32, 15.0);

        // clamp(5, 0, 3) = 3
        fx.pool.set_float_param(0, 5.0);
        fx.pool.set_float_param(1, 0.0);
        fx.pool.set_float_param(2, 3.0);
        {
            let mut ctx = fx.ctx();
            math.invoke_native_static(&mut ctx, 7); // clamp
        }
        assert_eq!(fx.pool.get_float_return() as f32, 3.0);
    }

    #[test]
    fn test_vector2_construct_and_magnitude() {
        let v = Vector2 { x: 0.0, y: 0.0 };
        let mut fx = Fixture::new();
        // 构造 (3, 4)
        fx.pool.set_float_param(0, 3.0);
        fx.pool.set_float_param(1, 4.0);
        let id = {
            let mut ctx = fx.ctx();
            v.do_construct_native(&mut ctx, None, 0)
        };
        // magnitude 是实例方法编号 2（distance=0, scale=1 为静态，共享混合编号）
        {
            let mut ctx = fx.ctx();
            v.invoke_native_method(&mut ctx, id, 2);
        }
        assert_eq!(fx.pool.get_float_return() as f32, 5.0);
    }

    #[test]
    fn test_vector2_scale_returns_new_object() {
        let v = Vector2 { x: 0.0, y: 0.0 };
        let mut fx = Fixture::new();

        // 构造两个向量 (2,3) 和 (4,5)
        fx.pool.set_float_param(0, 2.0);
        fx.pool.set_float_param(1, 3.0);
        let a = {
            let mut ctx = fx.ctx();
            v.do_construct_native(&mut ctx, None, 0)
        };
        fx.pool.set_float_param(0, 4.0);
        fx.pool.set_float_param(1, 5.0);
        let b = {
            let mut ctx = fx.ctx();
            v.do_construct_native(&mut ctx, None, 0)
        };

        // scale 是静态方法编号 1，返回新对象 ID
        fx.pool.set_object_param(0, a);
        fx.pool.set_object_param(1, b);
        {
            let mut ctx = fx.ctx();
            v.invoke_native_static(&mut ctx, 1);
        }
        let result_id = fx.pool.get_object_return();
        assert!(result_id != 0);
        assert!(result_id != a && result_id != b, "应是新对象");

        // 校验结果字段 (2*4, 3*5) = (8, 15)
        let rx = fx.objects.get(&result_id).unwrap().get_float_field(Vector2::FIELD_INDEX_x);
        let ry = fx.objects.get(&result_id).unwrap().get_float_field(Vector2::FIELD_INDEX_y);
        assert_eq!(rx as f32, 8.0);
        assert_eq!(ry as f32, 15.0);
    }

    #[test]
    fn test_vector2_lerp_mixed_params() {
        // B-2 验证：lerp(Vector2, Vector2, float) 混合类型参数按值类型分组读取
        let v = Vector2 { x: 0.0, y: 0.0 };
        let mut fx = Fixture::new();

        // 构造 a=(0,0), b=(10,20)
        fx.pool.set_float_param(0, 0.0);
        fx.pool.set_float_param(1, 0.0);
        let a = {
            let mut ctx = fx.ctx();
            v.do_construct_native(&mut ctx, None, 0)
        };
        fx.pool.set_float_param(0, 10.0);
        fx.pool.set_float_param(1, 20.0);
        let b = {
            let mut ctx = fx.ctx();
            v.do_construct_native(&mut ctx, None, 0)
        };

        // lerp 是静态方法编号 5：object 参数 a=obj[0], b=obj[1]；float 参数 t=float[0]
        fx.pool.set_object_param(0, a);
        fx.pool.set_object_param(1, b);
        fx.pool.set_float_param(0, 0.5);
        {
            let mut ctx = fx.ctx();
            v.invoke_native_static(&mut ctx, 5);
        }
        let result_id = fx.pool.get_object_return();
        assert!(result_id != 0 && result_id != a && result_id != b);

        // lerp((0,0),(10,20),0.5) = (5,10)
        let rx = fx.objects.get(&result_id).unwrap().get_float_field(Vector2::FIELD_INDEX_x);
        let ry = fx.objects.get(&result_id).unwrap().get_float_field(Vector2::FIELD_INDEX_y);
        assert_eq!(rx as f32, 5.0);
        assert_eq!(ry as f32, 10.0);
    }
}
