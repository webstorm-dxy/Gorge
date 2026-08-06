//! GorgeFramework —— Gorge 音乐游戏框架的 native 类库（Rust 移植）。
//!
//! 本 crate 用 `gorge_macros` 提供的桥接宏，将框架的 native 类以纯 Rust
//! 结构体 + 业务实现的形式暴露给 Gorge 虚拟机。
//! 通过 [`register_native`] 把全部 native 类一次性注册进 [`gorge_core::objective::runtime::GorgeRuntime`]。

pub mod system;
pub mod adaptor;
pub mod automaton;
pub mod chart;
pub mod input;
pub mod runtime;
pub mod signal;
pub mod simulators;
pub mod stage;
pub mod utilities;

pub use system::native::math::Math;
pub use system::native::vector2::Vector2;
pub use system::native::vector3::Vector3;
pub use system::native::vector3::Quaternion;
pub use system::native::random::Random;
pub use system::native::float_signal::FloatSignal;
pub use system::native::bool_signal::BoolSignal;
pub use system::native::touch_signal::TouchSignal;
pub use system::native::color_argb::ColorArgb;
pub use system::native::priority::Priority;
pub use system::native::period_config::PeriodConfig;
pub use system::native::graph::Graph;
pub use system::native::element_line_point::ElementLinePoint;
pub use system::native::element_line::ElementLine;
pub use system::native::logger::Logger;
pub use system::native::append_signal_command::AppendSignalCommand;
pub use system::native::derive_element_command::DeriveElementCommand;
pub use system::native::destroy_element_command::DestroyElementCommand;
pub use system::native::constant_function_curve::ConstantFunctionCurve;
pub use system::native::linear_function_curve::LinearFunctionCurve;
pub use system::native::quadratic_function_curve::QuadraticFunctionCurve;
pub use system::native::linear_curve::LinearCurve;
pub use system::native::arc_function_curve::ArcFunctionCurve;
pub use system::native::cubic_hermite_spline::CubicHermiteSpline;
pub use system::native::time_item::TimeItem;
pub use system::native::float_signal_filter::FloatSignalFilter;
pub use system::native::input_graph_edge::InputGraphEdge;
pub use system::native::note_linkage::NoteLinkage;
pub use system::native::variable_float::VariableFloat;
pub use system::native::function_curve_combinators::FunctionPiece;
pub use system::native::function_curve_combinators::CompositeFunctionCurve;
pub use system::native::function_curve_combinators::AdditionFunctionCurve;
pub use system::native::function_curve_combinators::MultiplicationFunctionCurve;
pub use system::native::function_curve_combinators::PeriodicFunctionCurve;
pub use system::native::function_curve_combinators::AxialSymmetricFunctionCurve;
pub use system::native::function_curve_combinators::PiecewiseFunctionCurve;
pub use system::native::node_native::Node;
pub use system::native::element_native::Element;
pub use system::native::note::Note;
pub use system::native::signal_filter_native::SignalFilter;
pub use system::native::input_signal_filter::InputSignalFilter;
pub use system::native::input_graph::InputGraph;
pub use system::native::input_graph_state::InputGraphState;
pub use system::native::history::HistoryStack;
pub use system::native::time_stack::TimeStack;
pub use system::native::element_simulator::ElementSimulator;
pub use system::native::lerp_color_curve::LerpColorCurve;
pub use system::native::annulus_mesh_transformer::AnnulusMeshTransformer;
pub use system::native::transform::CurveMeshTransformer;
pub use system::native::curve_warp_transformer::CurveWarpTransformer;
// S6: 资产族 + 精灵族 + 音频/视频 + 环境
pub use system::native::asset::Asset;
pub use system::native::graph_asset::GraphAsset;
pub use system::native::image_asset::ImageAsset;
pub use system::native::native_audio_asset::NativeAudioAsset;
pub use system::native::native_video_asset::NativeVideoAsset;
pub use system::native::audio_asset::AudioAsset;
pub use system::native::wav_audio_asset::WavAudioAsset;
pub use system::native::video_asset::VideoAsset;
pub use system::native::audio::AudioNative;
pub use system::native::video::VideoNative;
pub use system::native::sprite::Sprite;
pub use system::native::nine_slice_sprite::NineSliceSprite;
pub use system::native::curve_sprite::CurveSprite;
pub use system::native::environment::EnvironmentNative;
// S7: SignalTsiga 自动机
pub use system::native::signal_tsiga::SignalTsiga;
// 阶段1-C: 曲线分派基类
pub use system::native::function_curve::FunctionCurveNative;
pub use system::native::color_curve::ColorCurve;

use gorge_core::objective::native::NativeClass;
use gorge_core::objective::runtime::GorgeRuntime;
use gorge_core::system::native::list::{IntListClass, FloatListClass, BoolListClass, StringListClass, ObjectListClass};
use gorge_core::system::native::array::{IntArrayClass, FloatArrayClass, BoolArrayClass, StringArrayClass, ObjectArrayClass};
use std::sync::Arc;

/// 返回框架全部 native 类实例（`Arc<dyn NativeClass>`）。
///
/// 供调用方按需注册（如按全名或简单名）。每个元素是一个 native 类的共享句柄。
pub fn native_classes() -> Vec<Arc<dyn NativeClass>> {
    vec![
        Arc::new(Math {}),
        Arc::new(Vector2 { x: 0.0, y: 0.0 }),
        Arc::new(Vector3 { x: 0.0, y: 0.0, z: 0.0 }),
        Arc::new(Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }),
        Arc::new(Random {}),
        Arc::new(FloatSignal { value: 0.0 }),
        Arc::new(BoolSignal { value: false }),
        Arc::new(TouchSignal { is_touching: false, position: 0 }),
        Arc::new(ColorArgb { a: 1.0, r: 0.0, g: 0.0, b: 0.0 }),
        Arc::new(Priority { get_priority: 0 }),
        Arc::new(PeriodConfig { time_offset: 0.0, min_length: 0.0, active: false }),
        Arc::new(Graph { width: 0, height: 0 }),
        Arc::new(ElementLinePoint { time: 0.0, position: 0.0, width: 0.0 }),
        Arc::new(ElementLine { color_r: 0, color_g: 0, color_b: 0, color_a: 0 }),
        Arc::new(Logger {}),
        Arc::new(AppendSignalCommand { signal_id: 0, priority: 0 }),
        Arc::new(DeriveElementCommand { element_spec: 0 }),
        Arc::new(DestroyElementCommand { target_type: 0 }),
        // 简单函数曲线
        Arc::new(ConstantFunctionCurve { value: 0.0 }),
        Arc::new(LinearFunctionCurve { k: 0.0, b: 0.0 }),
        Arc::new(QuadraticFunctionCurve { a: 0.0, b: 0.0, c: 0.0 }),
        Arc::new(LinearCurve { time_start: 0.0, value_start: 0.0, time_end: 0.0, value_end: 0.0 }),
        Arc::new(ArcFunctionCurve { chord_start: 0.0, chord_end: 0.0, angle: 0.0 }),
        Arc::new(CubicHermiteSpline {
            start_point: 0, start_tangent: 0.0, start_weight: 0.33333,
            end_point: 0, end_tangent: 0.0, end_weight: 0.33333,
        }),
        // 函数曲线组合器
        Arc::new(AdditionFunctionCurve { first: 0, second: 0 }),
        Arc::new(MultiplicationFunctionCurve { first: 0, second: 0 }),
        Arc::new(CompositeFunctionCurve { outer: 0, inner: 0 }),
        Arc::new(PeriodicFunctionCurve { curve: 0, start_x: 0.0, end_x: 0.0, left_closed: true }),
        Arc::new(AxialSymmetricFunctionCurve { curve: 0, axis: 0.0, keep_left: true }),
        Arc::new(FunctionPiece {
            curve: 0, start_x: 0.0, end_x: 0.0, left_closed: true, right_closed: false,
        }),
        Arc::new(PiecewiseFunctionCurve { pieces: 0 }),
        Arc::new(TimeItem { time: 0, accept: false, respond_mode: String::new() }),
        Arc::new(FloatSignalFilter { priority: 0, condition_types: 0, end_time: 0, time_mode: 0, accept_consume: true, deny_consume: false, channel_name: String::new(), filter_range: 0 }),
        Arc::new(InputGraphEdge { deny: false, jump: 0, stack_respond: false, edge_respond: false, accept: false, export_state: String::new() }),
        // S2: 7 个核心 native 类
        Arc::new(SignalFilter { priority: 0, condition_types: 0, end_time: 0, time_mode: 0, accept_consume: true, deny_consume: false }),
        Arc::new(InputSignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0, accept_consume: true, deny_consume: false,
            on_detected: 0, signal_id_filter: 0, touch_area: 0,
        }),
        Arc::new(InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() }),
        Arc::new(InputGraphState { filter: 0, accepted_edge: 0, denied_edge: 0 }),
        Arc::new(HistoryStack { _placeholder: false }),
        Arc::new(TimeStack { accept: true, respond_mode: String::new() }),
        Arc::new(ElementSimulator { transformers: 0 }),
        // S5: 曲线/变换/工具类
        Arc::new(LerpColorCurve { color_points: 0, progress_curve: 0 }),
        Arc::new(AnnulusMeshTransformer { x_angle: 0, y_radius: 0 }),
        Arc::new(CurveMeshTransformer { curve: 0, is_horizontal: false }),
        Arc::new(CurveWarpTransformer { curve: 0, preserve_proportions: true, curvature_influence: 0.1, transformed_axis: 0, curve_value_axis: 1 }),
        // Phase H: 内建集合类型
        Arc::new(IntListClass), Arc::new(FloatListClass), Arc::new(BoolListClass),
        Arc::new(StringListClass), Arc::new(ObjectListClass),
        Arc::new(IntArrayClass), Arc::new(FloatArrayClass), Arc::new(BoolArrayClass),
        Arc::new(StringArrayClass), Arc::new(ObjectArrayClass),
        // S6 批 A: 纯数据 5 类
        Arc::new(Asset { name: String::new() }),
        Arc::new(GraphAsset { name: String::new() }),
        Arc::new(ImageAsset { name: String::new(), texture: 0 }),
        Arc::new(NativeAudioAsset { name: String::new(), audio: 0 }),
        Arc::new(NativeVideoAsset { name: String::new(), video: 0 }),
        // S6 批 B: 资源查找 3 类
        Arc::new(AudioAsset { name: String::new() }),
        Arc::new(VideoAsset { name: String::new() }),
        Arc::new(WavAudioAsset { name: String::new(), wav_file_path: String::new() }),
        // S6 批 C: 渲染/播放 5 类
        Arc::new(AudioNative {}),
        Arc::new(VideoNative {}),
        Arc::new(Sprite {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
            graph: 0, color: 0,
        }),
        Arc::new(NineSliceSprite {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
            graph: 0, slice_left_top: 0, slice_right_bottom: 0, base_size: 0, color: 0, hsl: 0,
        }),
        Arc::new(CurveSprite {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
            points: 0, color: 0, width: 0.1,
        }),
        Arc::new(EnvironmentNative {}),
        // S6 缺漏补全（5 类）：之前在 use 中声明但未加入 native_classes() vec
        Arc::new(NoteLinkage { json: String::new() }),
        Arc::new(VariableFloat { base_value: 0.0, variation_curve: 0 }),
        Arc::new(Node {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
        }),
        Arc::new(Element { nodes: 0, derived_elements: 0, simulator: 0, late_independent_simulator: 0 }),
        Arc::new(Note { automaton: 0 }),
        // S7: SignalTsiga 自动机
        Arc::new(SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 }),
        // 阶段1-C: 曲线分派基类（evaluate 方法待宏修复后补充）
        Arc::new(FunctionCurveNative { _placeholder: false }),
        Arc::new(ColorCurve { _placeholder: false }),
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
    use gorge_core::objective::native::{NativeClass, NativeContext};
    
    use gorge_core::virtual_machine::vm::VirtualMachine;

    /// 测试脚手架：持 VirtualMachine，通过 NativeContext 驱动 native 类
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
    fn test_register_native_into_runtime() {
        let mut runtime = GorgeRuntime::new();
        register_native(&mut runtime);
        assert!(runtime.is_native_class("GorgeFramework.Math"));
        assert!(runtime.is_native_class("GorgeFramework.Vector2"));
    }

    #[test]
    fn test_native_classes_count() {
        let classes = native_classes();
        assert!(classes.len() >= 68, "应有至少 68 个 native 类（含 S6 补齐 5 + S7 1），实际 {}", classes.len());
    }

    #[test]
    fn test_math_sqrt_via_native() {
        let math = Math {};
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_float_param(0, 16.0);
        {
            let mut ctx = fx.ctx();
            math.invoke_native_static(&mut ctx, 1); // sqrt
        }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 4.0);
    }

    #[test]
    fn test_vector2_construct_and_magnitude() {
        let v = Vector2 { x: 0.0, y: 0.0 };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_float_param(0, 3.0);
        fx.vm.param_pool.set_float_param(1, 4.0);
        let id = {
            let mut ctx = fx.ctx();
            v.do_construct_native(&mut ctx, None, 0)
        };
        {
            let mut ctx = fx.ctx();
            v.invoke_native_method(&mut ctx, id, 2);
        }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 5.0);
    }

    #[test]
    fn test_vector3_construct_and_magnitude() {
        let v = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_float_param(0, 3.0);
        fx.vm.param_pool.set_float_param(1, 4.0);
        fx.vm.param_pool.set_float_param(2, 0.0);
        let id = {
            let mut ctx = fx.ctx();
            v.do_construct_native(&mut ctx, None, 1)
        };
        {
            let mut ctx = fx.ctx();
            v.invoke_native_method(&mut ctx, id, 1);
        }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 5.0);
    }

    #[test]
    fn test_register_native_has_signals() {
        let mut runtime = GorgeRuntime::new();
        register_native(&mut runtime);
        assert!(runtime.is_native_class("GorgeFramework.FloatSignal"));
        assert!(runtime.is_native_class("GorgeFramework.BoolSignal"));
        assert!(runtime.is_native_class("GorgeFramework.TouchSignal"));
    }

    // ========== S5 曲线/变换/工具类测试 ==========

    /// 辅助：向 Fixture 注册指定 native 类
    fn register_native_class_to_vm(fx: &mut Fixture, cls: std::sync::Arc<dyn NativeClass>) {
        let name = cls.full_name().to_string();
        fx.vm.register_native_class(&name, cls);
    }

    /// 辅助：创建注册好的 CurveMeshTransformer
    #[allow(dead_code)]
    fn make_curve_mesh_trans(fx: &mut Fixture, curve_id: usize, is_h: bool) -> usize {
        let ctm = CurveMeshTransformer { curve: 0, is_horizontal: false };
        fx.vm.param_pool.set_object_param(0, curve_id);
        fx.vm.param_pool.set_bool_param(0, is_h);
        let mut ctx = fx.ctx();
        ctm.do_construct_native(&mut ctx, None, 0)
    }

    /// 辅助：创建简单 Vector3
    fn make_v3(fx: &mut Fixture, x: f32, y: f32, z: f32) -> usize {
        use gorge_core::objective::object::RuntimeObject;
        use gorge_core::objective::types::TypeCount;
        let obj = RuntimeObject::new_simple(
            "GorgeFramework.Vector3".to_string(),
            &TypeCount { float_count: 3, ..Default::default() },
        );
        let mut ctx = fx.ctx();
        let id = ctx.register_object(obj);
        ctx.set_object_float_field(id, 0, x as f64);
        ctx.set_object_float_field(id, 1, y as f64);
        ctx.set_object_float_field(id, 2, z as f64);
        id
    }

    /// 辅助：读取 Vector3 的 (x, y, z)
    fn read_v3(fx: &mut Fixture, id: usize) -> (f32, f32, f32) {
        let ctx = fx.ctx();
        let x = ctx.get_object_float_field(id, 0) as f32;
        let y = ctx.get_object_float_field(id, 1) as f32;
        let z = ctx.get_object_float_field(id, 2) as f32;
        (x, y, z)
    }

    /// 辅助：创建 ColorArgb
    fn make_color_argb(fx: &mut Fixture, a: f32, r: f32, g: f32, b: f32) -> usize {
        let c = ColorArgb { a: 1.0, r: 0.0, g: 0.0, b: 0.0 };
        fx.vm.param_pool.set_float_param(0, a as f64);
        fx.vm.param_pool.set_float_param(1, r as f64);
        fx.vm.param_pool.set_float_param(2, g as f64);
        fx.vm.param_pool.set_float_param(3, b as f64);
        let mut ctx = fx.ctx();
        c.do_construct_native(&mut ctx, None, 0)
    }

    /// 辅助：创建 LinearFunctionCurve(k, b)
    fn make_linear_curve(fx: &mut Fixture, k: f32, b: f32) -> usize {
        use gorge_core::objective::object::RuntimeObject;
        let obj = RuntimeObject::new_simple(
            LinearFunctionCurve::GORGE_FULL_NAME.to_string(),
            &LinearFunctionCurve::gorge_field_type_count(),
        );
        let mut ctx = fx.ctx();
        let id = ctx.register_object(obj);
        ctx.set_object_float_field(id, LinearFunctionCurve::FIELD_INDEX_k, k as f64);
        ctx.set_object_float_field(id, LinearFunctionCurve::FIELD_INDEX_b, b as f64);
        id
    }

    /// 辅助：创建 ConstantFunctionCurve(value)
    fn make_constant_curve(fx: &mut Fixture, value: f32) -> usize {
        use gorge_core::objective::object::RuntimeObject;
        let obj = RuntimeObject::new_simple(
            ConstantFunctionCurve::GORGE_FULL_NAME.to_string(),
            &ConstantFunctionCurve::gorge_field_type_count(),
        );
        let mut ctx = fx.ctx();
        let id = ctx.register_object(obj);
        ctx.set_object_float_field(id, ConstantFunctionCurve::FIELD_INDEX_value, value as f64);
        id
    }

    /// 辅助：创建 ObjectArray 包含两个颜色对象
    fn make_color_points_array(fx: &mut Fixture, c0_id: usize, c1_id: usize) -> usize {
        use gorge_core::system::native::array::ObjectArrayClass;
        let cls = ObjectArrayClass;
        let arr_id = { let mut ctx = fx.ctx(); cls.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = fx.ctx(); ctx.object_array_add(arr_id, c0_id); }
        { let mut ctx = fx.ctx(); ctx.object_array_add(arr_id, c1_id); }
        arr_id
    }

    #[test]
    fn test_s5_lerp_color_curve_black_to_white() {
        let mut fx = Fixture::new();
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(LerpColorCurve { color_points: 0, progress_curve: 0 }));
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(ColorArgb { a: 1.0, r: 0.0, g: 0.0, b: 0.0 }));
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(LinearFunctionCurve { k: 0.0, b: 0.0 }));
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));

        // 创建两点：黑(1,0,0,0)在索引0，白(1,1,1,1)在索引1
        let black = make_color_argb(&mut fx, 1.0, 0.0, 0.0, 0.0);
        let white = make_color_argb(&mut fx, 1.0, 1.0, 1.0, 1.0);
        let arr = make_color_points_array(&mut fx, black, white);

        // 进度曲线 f(x)=x，所以 x=0.5 → progress=0.5，在点0和点1之间插值
        let pc = make_linear_curve(&mut fx, 1.0, 0.0);

        // 构造 LerpColorCurve
        let lcc = LerpColorCurve { color_points: 0, progress_curve: 0 };
        fx.vm.param_pool.set_object_param(0, arr);
        fx.vm.param_pool.set_object_param(1, pc);
        let lcc_id = { let mut ctx = fx.ctx(); lcc.do_construct_native(&mut ctx, None, 1) };

        // evaluate(0.5)
        fx.vm.param_pool.set_float_param(0, 0.5f64);
        { let mut ctx = fx.ctx(); lcc.invoke_native_method(&mut ctx, lcc_id, 0); }
        let result_id = fx.vm.param_pool.get_object_return();
        assert!(result_id > 0, "应返回新 ColorArgb 对象");
        let r = { let ctx = fx.ctx(); ctx.get_object_float_field(result_id, ColorArgb::FIELD_INDEX_r) as f32 };
        let g = { let ctx = fx.ctx(); ctx.get_object_float_field(result_id, ColorArgb::FIELD_INDEX_g) as f32 };
        let b = { let ctx = fx.ctx(); ctx.get_object_float_field(result_id, ColorArgb::FIELD_INDEX_b) as f32 };
        // 黑(0)到白(1)的中间点，预期 ≈0.5
        assert!((r - 0.5).abs() < 0.01, "r 应在 0.5 附近，实际 {r}");
        assert!((g - 0.5).abs() < 0.01, "g 应在 0.5 附近，实际 {g}");
        assert!((b - 0.5).abs() < 0.01, "b 应在 0.5 附近，实际 {b}");
    }

    #[test]
    fn test_s5_annulus_mesh_constant_zero_angle() {
        let mut fx = Fixture::new();
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(AnnulusMeshTransformer { x_angle: 0, y_radius: 0 }));
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));

        // angle=0, radius=1 常量曲线
        let angle_curve = make_constant_curve(&mut fx, 0.0);
        let radius_curve = make_constant_curve(&mut fx, 1.0);

        let at = AnnulusMeshTransformer { x_angle: 0, y_radius: 0 };
        fx.vm.param_pool.set_object_param(0, angle_curve);
        fx.vm.param_pool.set_object_param(1, radius_curve);
        let at_id = { let mut ctx = fx.ctx(); at.do_construct_native(&mut ctx, None, 0) };

        // transform(vertex) — 任意顶点，角度和半径由曲线覆盖
        let v_id = make_v3(&mut fx, 0.0, 0.0, 0.0);
        fx.vm.param_pool.set_object_param(0, v_id);
        { let mut ctx = fx.ctx(); at.invoke_native_method(&mut ctx, at_id, 0); }
        let result_id = fx.vm.param_pool.get_object_return();
        let (rx, ry, _rz) = read_v3(&mut fx, result_id);
        // angle=0, radius=1 → (cos0*1, sin0*1) = (1, 0)
        assert!((rx - 1.0).abs() < 0.01, "x 应为 1.0，实际 {rx}");
        assert!((ry - 0.0).abs() < 0.01, "y 应为 0.0，实际 {ry}");
    }

    #[test]
    fn test_s5_annulus_mesh_pi_half_angle() {
        let mut fx = Fixture::new();
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(AnnulusMeshTransformer { x_angle: 0, y_radius: 0 }));
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));

        use std::f32::consts::FRAC_PI_2;
        let angle_curve = make_constant_curve(&mut fx, FRAC_PI_2);
        let radius_curve = make_constant_curve(&mut fx, 1.0);

        let at = AnnulusMeshTransformer { x_angle: 0, y_radius: 0 };
        fx.vm.param_pool.set_object_param(0, angle_curve);
        fx.vm.param_pool.set_object_param(1, radius_curve);
        let at_id = { let mut ctx = fx.ctx(); at.do_construct_native(&mut ctx, None, 0) };

        let v_id = make_v3(&mut fx, 0.0, 0.0, 0.0);
        fx.vm.param_pool.set_object_param(0, v_id);
        { let mut ctx = fx.ctx(); at.invoke_native_method(&mut ctx, at_id, 0); }
        let result_id = fx.vm.param_pool.get_object_return();
        let (rx, ry, _) = read_v3(&mut fx, result_id);
        // angle=π/2, radius=1 → (cosπ/2, sinπ/2) = (≈0, 1)
        assert!((rx - 0.0).abs() < 0.01, "x 应 ≈0.0，实际 {rx}");
        assert!((ry - 1.0).abs() < 0.01, "y 应 = 1.0，实际 {ry}");
    }

    #[test]
    fn test_s5_curve_warp_constant_curve() {
        let mut fx = Fixture::new();
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(CurveWarpTransformer {
            curve: 0, preserve_proportions: true, curvature_influence: 0.1,
            transformed_axis: 0, curve_value_axis: 1,
        }));
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));

        // f(x)=5 常量曲线 → 切线(1,0)，法线(0,1)，曲率=0
        let curve = make_constant_curve(&mut fx, 5.0);

        let cwt = CurveWarpTransformer {
            curve: 0, preserve_proportions: true, curvature_influence: 0.1,
            transformed_axis: 0, curve_value_axis: 1,
        };
        fx.vm.param_pool.set_object_param(0, curve);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_float_param(0, 0.1f64);
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_int_param(1, 1);
        let cwt_id = { let mut ctx = fx.ctx(); cwt.do_construct_native(&mut ctx, None, 0) };

        // transform(1,2,3) → 对于 f(x)=5:
        // curveX=1, curvePoint=(1,5), tangent=(1,0), normal=(0,1), curvature=0
        // distortion=1.0, curveValue=2 (y), adjustedY=2
        // warped=(1+0*2, 5+1*2)=(1,7), result=(1,7,3)
        let v_id = make_v3(&mut fx, 1.0, 2.0, 3.0);
        fx.vm.param_pool.set_object_param(0, v_id);
        { let mut ctx = fx.ctx(); cwt.invoke_native_method(&mut ctx, cwt_id, 0); }
        let result_id = fx.vm.param_pool.get_object_return();
        let (rx, ry, rz) = read_v3(&mut fx, result_id);
        assert!((rx - 1.0).abs() < 0.1, "x 应 ≈1.0，实际 {rx}");
        assert!((ry - 7.0).abs() < 0.1, "y 应 ≈7.0，实际 {ry}");
        assert!((rz - 3.0).abs() < 0.01, "z 应 =3.0，实际 {rz}");
    }

    #[test]
    fn test_s5_curve_warp_no_curve_identity() {
        let mut fx = Fixture::new();
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(CurveWarpTransformer {
            curve: 0, preserve_proportions: true, curvature_influence: 0.1,
            transformed_axis: 0, curve_value_axis: 1,
        }));

        let cwt = CurveWarpTransformer {
            curve: 0, preserve_proportions: true, curvature_influence: 0.1,
            transformed_axis: 0, curve_value_axis: 1,
        };
        fx.vm.param_pool.set_object_param(0, 0); // no curve
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_float_param(0, 0.1f64);
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_int_param(1, 1);
        let cwt_id = { let mut ctx = fx.ctx(); cwt.do_construct_native(&mut ctx, None, 0) };

        let v_id = make_v3(&mut fx, 1.0, 2.0, 3.0);
        fx.vm.param_pool.set_object_param(0, v_id);
        { let mut ctx = fx.ctx(); cwt.invoke_native_method(&mut ctx, cwt_id, 0); }
        let result_id = fx.vm.param_pool.get_object_return();
        let (rx, ry, rz) = read_v3(&mut fx, result_id);
        assert!((rx - 1.0).abs() < 0.01);
        assert!((ry - 2.0).abs() < 0.01);
        assert!((rz - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_s5_native_context_object_f() {
        // 验证 call_native_method_object_f 可用（通过 LerpColorCurve.evaluate 间接测试）
        let mut fx = Fixture::new();
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(LerpColorCurve { color_points: 0, progress_curve: 0 }));
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(ColorArgb { a: 1.0, r: 0.0, g: 0.0, b: 0.0 }));
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(LinearFunctionCurve { k: 0.0, b: 0.0 }));

        let white = make_color_argb(&mut fx, 1.0, 1.0, 1.0, 1.0);
        let black = make_color_argb(&mut fx, 1.0, 0.0, 0.0, 0.0);
        let arr = make_color_points_array(&mut fx, black, white);
        let pc = make_linear_curve(&mut fx, 1.0, 0.0);

        let lcc = LerpColorCurve { color_points: 0, progress_curve: 0 };
        fx.vm.param_pool.set_object_param(0, arr);
        fx.vm.param_pool.set_object_param(1, pc);
        let lcc_id = { let mut ctx = fx.ctx(); lcc.do_construct_native(&mut ctx, None, 1) };

        // 验证外部可通过 evaluate(0.0) 拿到对象 ID
        fx.vm.param_pool.set_float_param(0, 0.0f64);
        { let mut ctx = fx.ctx(); lcc.invoke_native_method(&mut ctx, lcc_id, 0); }
        let result_id = fx.vm.param_pool.get_object_return();
        assert!(result_id > 0, "evaluate(0) 应返回 ColorArgb 对象");
    }

    #[test]
    fn test_s5_float_extension_bit_int() {
        use crate::utilities::float_extension::bit_int;
        // 1.0f32 → IEEE 754 位模式 0x3F800000
        assert_eq!(bit_int(1.0f32), 0x3F800000u32 as i32);
        assert_eq!(bit_int(0.0f32), 0);
        // -1.0f32 → 0xBF800000
        assert_eq!(bit_int(-1.0f32), 0xBF800000u32 as i32);
    }

    // ========== 多态分派测试 ==========

    /// 测试 can_detect 虚方法分派：基类 SignalFilter 返回 false，
    /// 子类 InputSignalFilter 返回 true，FloatSignalFilter 按 channel 匹配。
    #[test]
    fn test_polymorphic_can_detect_dispatch() {
        let mut fx = Fixture::new();

        // 注册 SignalFilter 基类
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(SignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
        }));
        // 注册 InputSignalFilter 子类
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(InputSignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
            on_detected: 0, signal_id_filter: 0, touch_area: 0,
        }));
        // 注册 FloatSignalFilter 子类
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(FloatSignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
            channel_name: String::new(), filter_range: 0,
        }));

        // 构造基类 SignalFilter 对象
        for i in 0..3 { fx.vm.param_pool.set_object_param(i, 0); }
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        let base_id = {
            let sf = SignalFilter {
                priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
                accept_consume: true, deny_consume: false,
            };
            let mut ctx = fx.ctx();
            sf.do_construct_native(&mut ctx, None, 0)
        };

        // 构造 InputSignalFilter 子类对象
        for i in 0..6 { fx.vm.param_pool.set_object_param(i, 0); }
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        let input_id = {
            let isf = InputSignalFilter {
                priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
                accept_consume: true, deny_consume: false,
                on_detected: 0, signal_id_filter: 0, touch_area: 0,
            };
            let mut ctx = fx.ctx();
            isf.do_construct_native(&mut ctx, None, 0)
        };

        // 构造 FloatSignalFilter 子类对象（channel="speed"）
        for i in 0..4 { fx.vm.param_pool.set_object_param(i, 0); }
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_string_param(0, "speed".to_string());
        let float_id = {
            let fsf = FloatSignalFilter {
                priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
                accept_consume: true, deny_consume: false,
                channel_name: String::new(), filter_range: 0,
            };
            let mut ctx = fx.ctx();
            fsf.do_construct_native(&mut ctx, None, 0)
        };

        // 验证：基类 SignalFilter.can_detect 返回 false
        fx.vm.param_pool.set_string_param(0, "Touch".to_string());
        { let mut ctx = fx.ctx(); ctx.invoke_native_method_on("GorgeFramework.SignalFilter", base_id, 0); }
        assert!(!fx.vm.param_pool.get_bool_return(), "SignalFilter.can_detect 应返回 false");

        // 验证：子类 InputSignalFilter.can_detect 返回 true
        fx.vm.param_pool.set_string_param(0, "Touch".to_string());
        { let mut ctx = fx.ctx(); ctx.invoke_native_method_on("GorgeFramework.InputSignalFilter", input_id, 0); }
        assert!(fx.vm.param_pool.get_bool_return(), "InputSignalFilter.can_detect 应返回 true");

        // 验证：子类 FloatSignalFilter.can_detect 按 channel 匹配
        fx.vm.param_pool.set_string_param(0, "speed".to_string());
        { let mut ctx = fx.ctx(); ctx.invoke_native_method_on("GorgeFramework.FloatSignalFilter", float_id, 0); }
        assert!(fx.vm.param_pool.get_bool_return(), "FloatSignalFilter.can_detect(\"speed\") 应返回 true");

        fx.vm.param_pool.set_string_param(0, "other".to_string());
        { let mut ctx = fx.ctx(); ctx.invoke_native_method_on("GorgeFramework.FloatSignalFilter", float_id, 0); }
        assert!(!fx.vm.param_pool.get_bool_return(), "FloatSignalFilter.can_detect(\"other\") 应返回 false");
    }

    /// 测试 evaluate 方法重写：基类 FunctionCurve.evaluate 返回 0.0，
    /// 子类 ConstantFunctionCurve.evaluate 返回构造时指定的 value。
    #[test]
    fn test_polymorphic_function_curve_evaluate_dispatch() {
        let mut fx = Fixture::new();

        // 注册基类 FunctionCurveNative
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(FunctionCurveNative { _placeholder: false }));
        // 注册子类 ConstantFunctionCurve
        register_native_class_to_vm(&mut fx, std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));

        // 构造基类 FunctionCurveNative 对象
        fx.vm.param_pool.set_bool_param(0, false);
        let base_id = {
            let fcn = FunctionCurveNative { _placeholder: false };
            let mut ctx = fx.ctx();
            fcn.do_construct_native(&mut ctx, None, 0)
        };

        // 构造子类 ConstantFunctionCurve 对象（value=42.0，值参 ctor 为 1 号）
        fx.vm.param_pool.set_float_param(0, 42.0);
        let const_id = {
            let cfc = ConstantFunctionCurve { value: 0.0 };
            let mut ctx = fx.ctx();
            cfc.do_construct_native(&mut ctx, None, 1)
        };

        // 验证：基类 evaluate 返回 0.0
        fx.vm.param_pool.set_float_param(0, 10.0);
        { let mut ctx = fx.ctx(); ctx.invoke_native_method_on("GorgeFramework.FunctionCurve", base_id, 0); }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 0.0, "FunctionCurve.evaluate 应返回 0.0");

        // 验证：子类 evaluate 返回 value（42.0），与输入 x 无关
        fx.vm.param_pool.set_float_param(0, 10.0);
        { let mut ctx = fx.ctx(); ctx.invoke_native_method_on("GorgeFramework.ConstantFunctionCurve", const_id, 0); }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 42.0, "ConstantFunctionCurve.evaluate 应返回 42.0");
    }
}
