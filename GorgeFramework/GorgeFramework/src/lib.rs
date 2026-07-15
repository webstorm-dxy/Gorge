//! GorgeFramework —— Gorge 音乐游戏框架的 native 类库（Rust 移植）。
//!
//! 本 crate 用 `gorge_macros` 提供的桥接宏，将框架的 native 类以纯 Rust
//! 结构体 + 业务实现的形式暴露给 Gorge 虚拟机。
//! - [`Math`]：纯静态数学工具类
//! - [`Vector2`]：二维向量
//! - [`Vector3`]：三维向量（N1）
//! - [`Random`]：随机数工具（N1）
//!
//! 通过 [`register_native`] 把全部 native 类一次性注册进 [`gorge_core::runtime::GorgeRuntime`]。

pub mod math;
pub mod vector2;
pub mod vector3;
pub use vector3::Quaternion;
pub mod random;
pub mod float_signal;
pub mod bool_signal;
pub mod touch_signal;
pub mod color_argb;
pub mod priority;
pub mod period_config;
pub mod function_curve;
pub mod resource;
pub mod node;
pub mod time;
pub mod signal_filter;
pub mod input_graph;
pub mod history;
pub mod element;
pub mod signal_tsiga;
pub mod transform;

pub use math::Math;
pub use vector2::Vector2;
pub use vector3::Vector3;
pub use random::Random;
pub use float_signal::FloatSignal;
pub use bool_signal::BoolSignal;
pub use touch_signal::TouchSignal;
pub use color_argb::ColorArgb;
pub use priority::Priority;
pub use period_config::PeriodConfig;
pub use resource::Graph;
pub use element::ElementLinePoint;
pub use element::ElementLine;
pub use logger::Logger;
pub use function_curve::ConstantFunctionCurve;
pub use function_curve::LinearFunctionCurve;
pub use function_curve::QuadraticFunctionCurve;
pub use function_curve::LinearCurve;
pub use function_curve::ArcFunctionCurve;
pub use function_curve::CubicHermiteSpline;
pub use time::TimeItem;
pub use signal_filter::FloatSignalFilter;
pub use input_graph::InputGraphEdge;

pub mod logger;
pub mod commands;

use gorge_core::native::NativeClass;
use gorge_core::runtime::GorgeRuntime;
use gorge_core::list::{IntListClass, FloatListClass, BoolListClass, StringListClass, ObjectListClass};
use gorge_core::array::{IntArrayClass, FloatArrayClass, BoolArrayClass, StringArrayClass, ObjectArrayClass};
use std::sync::Arc;

/// 返回框架全部 native 类实例（`Arc<dyn NativeClass>`）。
///
/// 供调用方按需注册（如按全名或简单名）。每个元素是一个 native 类的共享句柄。
pub fn native_classes() -> Vec<Arc<dyn NativeClass>> {
    vec![
        Arc::new(Math {}),
        Arc::new(Vector2 { x: 0.0, y: 0.0 }),
        Arc::new(Vector3 { x: 0.0, y: 0.0, z: 0.0 }),
        Arc::new(Random {}),
        Arc::new(FloatSignal { value: 0.0 }),
        Arc::new(BoolSignal { value: false }),
        Arc::new(TouchSignal { is_touching: false, position: 0 }),
        Arc::new(ColorArgb { a: 255, r: 0, g: 0, b: 0 }),
        Arc::new(Priority { value: 0 }),
        Arc::new(PeriodConfig { start_time: 0.0, end_time: 0.0 }),
        // F2: 四元数
        Arc::new(Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }),
        // Phase P3-1: 资源标记类型
        Arc::new(Graph { width: 0, height: 0 }),
        // Phase P3-1: 元素连线类型
        Arc::new(ElementLinePoint { time: 0.0, position: 0.0, width: 0.0 }),
        Arc::new(ElementLine { color_r: 0, color_g: 0, color_b: 0, color_a: 0 }),
        // Phase P3-1: 日志工具类
        Arc::new(Logger {}),
        // Phase P3-1: Automaton 指令类
        Arc::new(commands::AppendSignalCommand { signal_id: 0, priority: 0 }),
        Arc::new(commands::DeriveElementCommand { element_spec: 0 }),
        Arc::new(commands::DestroyElementCommand { target_type: 0 }),
        // Phase P3-2: 函数曲线族（简单字段曲线）
        Arc::new(ConstantFunctionCurve { value: 0.0 }),
        Arc::new(LinearFunctionCurve { k: 0.0, b: 0.0 }),
        Arc::new(QuadraticFunctionCurve { a: 0.0, b: 0.0, c: 0.0 }),
        Arc::new(LinearCurve { time_start: 0.0, value_start: 0.0, time_end: 0.0, value_end: 0.0 }),
        Arc::new(ArcFunctionCurve { chord_start: 0.0, chord_end: 0.0, angle: 0.0 }),
        Arc::new(CubicHermiteSpline { time_start: 0.0, value_start: 0.0, m0: 0.0, w0: 0.0, time_end: 0.0, value_end: 0.0, m1: 0.0, w1: 0.0 }),
        // Phase P3-2: 时序类
        Arc::new(TimeItem { time: 0.0, accept: false, respond_mode: String::new() }),
        // Phase P3-3: 信号过滤器
        Arc::new(FloatSignalFilter { channel_name: String::new(), min_value: 0.0, max_value: 0.0, time_mode: 0, accept_consume: true, deny_consume: false, end_time: 0.0 }),
        // Phase P3-3: 输入图边
        Arc::new(InputGraphEdge { deny: false, jump: 0, stack_respond: false, edge_respond: false, accept: false, export_state: String::new() }),
        // Phase H: 内建集合类型
        Arc::new(IntListClass), Arc::new(FloatListClass), Arc::new(BoolListClass),
        Arc::new(StringListClass), Arc::new(ObjectListClass),
        Arc::new(IntArrayClass), Arc::new(FloatArrayClass), Arc::new(BoolArrayClass),
        Arc::new(StringArrayClass), Arc::new(ObjectArrayClass),
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

    // ==================== N1: Vector3 + Random 测试 ====================

    #[test]
    fn test_register_native_has_vector3_and_random() {
        let mut runtime = GorgeRuntime::new();
        register_native(&mut runtime);
        assert!(runtime.is_native_class("GorgeFramework.Vector3"));
        assert!(runtime.is_native_class("GorgeFramework.Random"));
    }

    #[test]
    fn test_vector3_construct_and_magnitude() {
        let v = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        let mut fx = Fixture::new();
        // 构造 (3, 4, 0)
        fx.pool.set_float_param(0, 3.0);
        fx.pool.set_float_param(1, 4.0);
        fx.pool.set_float_param(2, 0.0);
        let id = {
            let mut ctx = fx.ctx();
            v.do_construct_native(&mut ctx, None, 1) // ctor 1
        };
        // magnitude 是混合编号 1
        {
            let mut ctx = fx.ctx();
            v.invoke_native_method(&mut ctx, id, 1);
        }
        assert_eq!(fx.pool.get_float_return() as f32, 5.0);
    }

    #[test]
    fn test_vector3_get_components() {
        let v = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        let mut fx = Fixture::new();
        fx.pool.set_float_param(0, 1.0);
        fx.pool.set_float_param(1, 2.0);
        fx.pool.set_float_param(2, 3.0);
        let id = {
            let mut ctx = fx.ctx();
            v.do_construct_native(&mut ctx, None, 1)
        };
        // get_x=2, get_y=3, get_z=4
        for (method_idx, expected) in [(2, 1.0), (3, 2.0), (4, 3.0)] {
            let mut ctx = fx.ctx();
            v.invoke_native_method(&mut ctx, id, method_idx);
            assert_eq!(fx.pool.get_float_return() as f32, expected);
        }
    }

    #[test]
    fn test_vector3_distance() {
        let v = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        let mut fx = Fixture::new();
        let a = make_v3(&v, &mut fx, 0.0, 0.0, 0.0);
        let b = make_v3(&v, &mut fx, 3.0, 4.0, 0.0);
        // distance 是混合编号 5
        fx.pool.set_object_param(0, a);
        fx.pool.set_object_param(1, b);
        {
            let mut ctx = fx.ctx();
            v.invoke_native_static(&mut ctx, 5);
        }
        assert_eq!(fx.pool.get_float_return() as f32, 5.0);
    }

    #[test]
    fn test_vector3_to_vector2() {
        let v = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        let mut fx = Fixture::new();
        let id = make_v3(&v, &mut fx, 10.0, 20.0, 30.0);
        // to_vector2 是混合编号 0
        {
            let mut ctx = fx.ctx();
            v.invoke_native_method(&mut ctx, id, 0);
        }
        let result_id = fx.pool.get_object_return();
        assert!(result_id != 0 && result_id != id);
        let rx = fx.objects.get(&result_id).unwrap().get_float_field(0);
        let ry = fx.objects.get(&result_id).unwrap().get_float_field(1);
        assert_eq!(rx as f32, 10.0);
        assert_eq!(ry as f32, 20.0);
    }

    #[test]
    fn test_random_random_float() {
        let r = Random {};
        let mut fx = Fixture::new();
        {
            let mut ctx = fx.ctx();
            r.invoke_native_static(&mut ctx, 0); // random_float
        }
        let val = fx.pool.get_float_return();
        assert!(val >= 0.0 && val < 1.0);
    }

    fn make_v3(v: &Vector3, fx: &mut Fixture, x: f32, y: f32, z: f32) -> usize {
        fx.pool.set_float_param(0, x as f64);
        fx.pool.set_float_param(1, y as f64);
        fx.pool.set_float_param(2, z as f64);
        let mut ctx = fx.ctx();
        v.do_construct_native(&mut ctx, None, 1)
    }

    // ==================== N2: 信号系统测试 ====================

    #[test]
    fn test_register_native_has_signals() {
        let mut runtime = GorgeRuntime::new();
        register_native(&mut runtime);
        assert!(runtime.is_native_class("GorgeFramework.FloatSignal"));
        assert!(runtime.is_native_class("GorgeFramework.BoolSignal"));
        assert!(runtime.is_native_class("GorgeFramework.TouchSignal"));
    }

    #[test]
    fn test_float_signal_construct() {
        let s = FloatSignal { value: 0.0 };
        let mut fx = Fixture::new();
        fx.pool.set_float_param(0, 3.14);
        let id = {
            let mut ctx = fx.ctx();
            s.do_construct_native(&mut ctx, None, 0)
        };
        let val = fx.objects.get(&id).unwrap().get_float_field(0);
        assert!((val - 3.14).abs() < 0.01);
    }

    #[test]
    fn test_bool_signal_construct() {
        let s = BoolSignal { value: false };
        let mut fx = Fixture::new();
        fx.pool.set_bool_param(0, true);
        let id = {
            let mut ctx = fx.ctx();
            s.do_construct_native(&mut ctx, None, 0)
        };
        assert!(fx.objects.get(&id).unwrap().get_bool_field(0));
    }

    #[test]
    fn test_touch_signal_construct() {
        let s = TouchSignal { is_touching: false, position: 0 };
        let mut fx = Fixture::new();
        // 先构造一个 Vector2 作为位置
        let v2 = Vector2 { x: 0.0, y: 0.0 };
        fx.pool.set_float_param(0, 100.0);
        fx.pool.set_float_param(1, 200.0);
        let pos_id = {
            let mut ctx = fx.ctx();
            v2.do_construct_native(&mut ctx, None, 0)
        };
        // 构造 TouchSignal(is_touching=true, position=pos_id)
        // 参数按值类型分组：bool 参数 is_touching, object 参数 position
        fx.pool.set_bool_param(0, true);
        fx.pool.set_object_param(0, pos_id);
        let id = {
            let mut ctx = fx.ctx();
            s.do_construct_native(&mut ctx, None, 0)
        };
        assert!(fx.objects.get(&id).unwrap().get_bool_field(0));
        let stored_pos = fx.objects.get(&id).unwrap().get_object_field(0);
        assert_eq!(stored_pos, pos_id);
    }

    // ==================== N3: ColorArgb 测试 ====================

    #[test]
    fn test_register_color_argb() {
        let mut runtime = GorgeRuntime::new();
        register_native(&mut runtime);
        assert!(runtime.is_native_class("GorgeFramework.ColorArgb"));
    }

    #[test]
    fn test_color_argb_construct() {
        let c = ColorArgb { a: 255, r: 0, g: 0, b: 0 };
        let mut fx = Fixture::new();
        fx.pool.set_int_param(0, 128);
        fx.pool.set_int_param(1, 255);
        fx.pool.set_int_param(2, 0);
        fx.pool.set_int_param(3, 0);
        let id = {
            let mut ctx = fx.ctx();
            c.do_construct_native(&mut ctx, None, 0)
        };
        // a=128, r=255, g=0, b=0 → 半透明红色
        assert_eq!(fx.objects.get(&id).unwrap().get_int_field(0), 128);
        assert_eq!(fx.objects.get(&id).unwrap().get_int_field(1), 255);
        assert_eq!(fx.objects.get(&id).unwrap().get_int_field(2), 0);
        assert_eq!(fx.objects.get(&id).unwrap().get_int_field(3), 0);
    }

    // ==================== N3: Priority + PeriodConfig 测试 ====================

    #[test]
    fn test_priority_construct() {
        let p = Priority { value: 0 };
        let mut fx = Fixture::new();
        fx.pool.set_int_param(0, 999);
        let id = {
            let mut ctx = fx.ctx();
            p.do_construct_native(&mut ctx, None, 0)
        };
        assert_eq!(fx.objects.get(&id).unwrap().get_int_field(0), 999);
    }

    #[test]
    fn test_period_config_construct() {
        let pc = PeriodConfig { start_time: 0.0, end_time: 0.0 };
        let mut fx = Fixture::new();
        fx.pool.set_float_param(0, 10.0);
        fx.pool.set_float_param(1, 20.0);
        let id = {
            let mut ctx = fx.ctx();
            pc.do_construct_native(&mut ctx, None, 0)
        };
        assert!((fx.objects.get(&id).unwrap().get_float_field(0) - 10.0).abs() < 0.01);
        assert!((fx.objects.get(&id).unwrap().get_float_field(1) - 20.0).abs() < 0.01);
    }

    // ==================== P3 Phase 1: Graph 注册测试 ====================

    #[test]
    fn test_register_native_has_graph() {
        let mut runtime = GorgeRuntime::new();
        register_native(&mut runtime);
        assert!(runtime.is_native_class("GorgeFramework.Graph"));
    }

    #[test]
    fn test_graph_construct() {
        let g = Graph { width: 0, height: 0 };
        let mut fx = Fixture::new();
        fx.pool.set_int_param(0, 640);
        fx.pool.set_int_param(1, 480);
        let id = {
            let mut ctx = fx.ctx();
            g.do_construct_native(&mut ctx, None, 0)
        };
        assert_eq!(fx.objects.get(&id).unwrap().get_int_field(0), 640);
        assert_eq!(fx.objects.get(&id).unwrap().get_int_field(1), 480);
    }

    #[test]
    fn test_element_line_point_construct() {
        let p = ElementLinePoint { time: 0.0, position: 0.0, width: 0.0 };
        let mut fx = Fixture::new();
        fx.pool.set_float_param(0, 1.5);
        fx.pool.set_float_param(1, 100.0);
        fx.pool.set_float_param(2, 3.0);
        let id = {
            let mut ctx = fx.ctx();
            p.do_construct_native(&mut ctx, None, 0)
        };
        assert!((fx.objects.get(&id).unwrap().get_float_field(0) - 1.5).abs() < 0.01);
        assert!((fx.objects.get(&id).unwrap().get_float_field(1) - 100.0).abs() < 0.01);
        assert!((fx.objects.get(&id).unwrap().get_float_field(2) - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_element_line_construct() {
        let l = ElementLine { color_r: 0, color_g: 0, color_b: 0, color_a: 0 };
        let mut fx = Fixture::new();
        fx.pool.set_int_param(0, 255);
        fx.pool.set_int_param(1, 128);
        fx.pool.set_int_param(2, 64);
        fx.pool.set_int_param(3, 32);
        let id = {
            let mut ctx = fx.ctx();
            l.do_construct_native(&mut ctx, None, 0)
        };
        assert_eq!(fx.objects.get(&id).unwrap().get_int_field(0), 255);
        assert_eq!(fx.objects.get(&id).unwrap().get_int_field(1), 128);
        assert_eq!(fx.objects.get(&id).unwrap().get_int_field(2), 64);
        assert_eq!(fx.objects.get(&id).unwrap().get_int_field(3), 32);
    }

    #[test]
    fn test_logger_log_int() {
        let logger = Logger {};
        let mut fx = Fixture::new();
        fx.pool.set_int_param(0, 42);
        {
            let mut ctx = fx.ctx();
            logger.invoke_native_static(&mut ctx, 0); // log_int
        }
        assert_eq!(fx.pool.get_int_return(), 42);
    }

    // ==================== P3 Phase 2: 函数曲线测试 ====================

    #[test]
    fn test_constant_curve_evaluate() {
        let c = ConstantFunctionCurve { value: 0.0 };
        let mut fx = Fixture::new();
        fx.pool.set_float_param(0, 7.0);
        let c_obj = {
            let mut ctx = fx.ctx();
            c.do_construct_native(&mut ctx, None, 0)
        };
        // evaluate(x) 是实例方法编号 0
        fx.pool.set_float_param(0, 100.0); // x
        {
            let mut ctx = fx.ctx();
            c.invoke_native_method(&mut ctx, c_obj, 0);
        }
        assert_eq!(fx.pool.get_float_return() as f32, 7.0);
    }

    #[test]
    fn test_linear_curve_evaluate() {
        let c = LinearFunctionCurve { k: 0.0, b: 0.0 };
        let mut fx = Fixture::new();
        fx.pool.set_float_param(0, 2.0); // k
        fx.pool.set_float_param(1, 5.0); // b
        let c_obj = {
            let mut ctx = fx.ctx();
            c.do_construct_native(&mut ctx, None, 0)
        };
        fx.pool.set_float_param(0, 3.0); // x
        {
            let mut ctx = fx.ctx();
            c.invoke_native_method(&mut ctx, c_obj, 0);
        }
        // f(3) = 2*3 + 5 = 11
        assert_eq!(fx.pool.get_float_return() as f32, 11.0);
    }
}
