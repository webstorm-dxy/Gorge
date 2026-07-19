//! `GorgeFramework.CurveMeshTransformer` —— 曲线网格变形 native 类。
//!
//! 移植自 C# 参考实现 `System/Native/CurveMeshTransformer.cs`。
//! 将网格顶点沿函数曲线方向偏移：
//! - `isHorizontal=true`：x += curve.evaluate(y)
//! - `isHorizontal=false`：y += curve.evaluate(x)

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 曲线网格变形器
///
/// 字段：
/// - `curve` (对象 ID)：FunctionCurve
/// - `is_horizontal` (bool)：true 则沿水平方向（x += curve(y)），false 则沿垂直方向（y += curve(x)）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct CurveMeshTransformer {
    #[gorge_field]
    pub curve: usize,
    #[gorge_field]
    pub is_horizontal: bool,
}

#[gorge_native_impl]
impl CurveMeshTransformer {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, curve: usize, is_horizontal: bool) {
        ctx.set_object_object_field(this, CurveMeshTransformer::FIELD_INDEX_curve, curve);
        ctx.set_object_bool_field(this, CurveMeshTransformer::FIELD_INDEX_is_horizontal, is_horizontal);
    }

    /// 对顶点应用曲线变形，返回新 Vector3 对象 ID
    ///
    /// 对齐 C# `Transform(Vector3 vertex)`：
    /// - isHorizontal → x += curve.Evaluate(y)
    /// - !isHorizontal → y += curve.Evaluate(x)
    /// z 坐标保持不变。
    #[gorge_method]
    pub fn transform(ctx: &mut NativeContext, this: usize, vertex: usize) -> usize {
        let curve = ctx.get_object_object_field(this, CurveMeshTransformer::FIELD_INDEX_curve);
        let is_h = ctx.get_object_bool_field(this, CurveMeshTransformer::FIELD_INDEX_is_horizontal);

        let vx = ctx.get_object_float_field(vertex, 0) as f32;
        let vy = ctx.get_object_float_field(vertex, 1) as f32;
        let vz = ctx.get_object_float_field(vertex, 2) as f32;

        if curve == 0 {
            return make_vector3(ctx, vx, vy, vz);
        }

        if is_h {
            let offset = ctx.call_native_method_float_f(curve, 0, vy as f64) as f32;
            make_vector3(ctx, vx + offset, vy, vz)
        } else {
            let offset = ctx.call_native_method_float_f(curve, 0, vx as f64) as f32;
            make_vector3(ctx, vx, vy + offset, vz)
        }
    }
}

/// 创建新 Vector3 对象
fn make_vector3(ctx: &mut NativeContext, x: f32, y: f32, z: f32) -> usize {
    use gorge_core::objective::object::RuntimeObject;
    use gorge_core::objective::types::TypeCount;
    let obj = RuntimeObject::new_simple(
        "GorgeFramework.Vector3".to_string(),
        &TypeCount { float_count: 3, ..Default::default() },
    );
    let id = ctx.register_object(obj);
    ctx.set_object_float_field(id, 0, x as f64);
    ctx.set_object_float_field(id, 1, y as f64);
    ctx.set_object_float_field(id, 2, z as f64);
    id
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::object::RuntimeObject;
    use gorge_core::virtual_machine::vm::VirtualMachine;
    use crate::system::native::linear_function_curve::LinearFunctionCurve;

    fn make_vm() -> VirtualMachine {
        VirtualMachine::new()
    }

    fn register_class(vm: &mut VirtualMachine, cls: std::sync::Arc<dyn NativeClass>) {
        let name = cls.full_name().to_string();
        vm.register_native_class(&name, cls.clone());
    }

    /// 创建 Vector3 对象并返回 ID
    fn make_v3(ctx: &mut NativeContext, x: f32, y: f32, z: f32) -> usize {
        make_vector3(ctx, x, y, z)
    }

    /// 创建 LinearFunctionCurve(k, b) 并返回对象 ID
    fn make_linear_curve(ctx: &mut NativeContext, k: f32, b: f32) -> usize {
        let obj = RuntimeObject::new_simple(
            LinearFunctionCurve::GORGE_FULL_NAME.to_string(),
            &LinearFunctionCurve::gorge_field_type_count(),
        );
        let id = ctx.register_object(obj);
        ctx.set_object_float_field(id, LinearFunctionCurve::FIELD_INDEX_k, k as f64);
        ctx.set_object_float_field(id, LinearFunctionCurve::FIELD_INDEX_b, b as f64);
        id
    }

    #[test]
    fn test_curve_mesh_horizontal() {
        let ctm = CurveMeshTransformer { curve: 0, is_horizontal: false };
        let mut vm = make_vm();
        register_class(&mut vm, std::sync::Arc::new(LinearFunctionCurve { k: 0.0, b: 0.0 }));
        register_class(&mut vm, std::sync::Arc::new(CurveMeshTransformer { curve: 0, is_horizontal: false }));
        register_class(&mut vm, std::sync::Arc::new(crate::system::native::vector3::Vector3 { x: 0.0, y: 0.0, z: 0.0 }));

        let (_curve_id, vertex_id, trans_id) = {
            let mut ctx = NativeContext::new(&mut vm);
            let cid = make_linear_curve(&mut ctx, 1.0, 0.0);
            let vid = make_v3(&mut ctx, 1.0, 2.0, 3.0);
            ctx.set_object_param(0, cid);
            ctx.set_bool_param(0, true);
            let tid = ctm.do_construct_native(&mut ctx, None, 0);
            (cid, vid, tid)
        };

        // isHorizontal=true, curve k=1,b=0, vertex(1,2,3) → x += curve(2) = x + 2 = 3, result(3,2,3)
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_param(0, vertex_id);
            ctm.invoke_native_method(&mut ctx, trans_id, 0);
            let result_id = ctx.get_object_return();
            let rx = ctx.get_object_float_field(result_id, 0) as f32;
            let ry = ctx.get_object_float_field(result_id, 1) as f32;
            let rz = ctx.get_object_float_field(result_id, 2) as f32;
            assert!((rx - 3.0).abs() < 0.01, "水平变形 x 应为 3.0，实际 {rx}");
            assert!((ry - 2.0).abs() < 0.01, "y 不变");
            assert!((rz - 3.0).abs() < 0.01, "z 不变");
        }
    }

    #[test]
    fn test_curve_mesh_vertical() {
        let ctm = CurveMeshTransformer { curve: 0, is_horizontal: true };
        let mut vm = make_vm();
        register_class(&mut vm, std::sync::Arc::new(LinearFunctionCurve { k: 0.0, b: 0.0 }));
        register_class(&mut vm, std::sync::Arc::new(CurveMeshTransformer { curve: 0, is_horizontal: false }));
        register_class(&mut vm, std::sync::Arc::new(crate::system::native::vector3::Vector3 { x: 0.0, y: 0.0, z: 0.0 }));

        let (_curve_id, vertex_id, trans_id) = {
            let mut ctx = NativeContext::new(&mut vm);
            let cid = make_linear_curve(&mut ctx, 1.0, 0.0);
            let vid = make_v3(&mut ctx, 1.0, 2.0, 3.0);
            ctx.set_object_param(0, cid);
            ctx.set_bool_param(0, false);
            let tid = ctm.do_construct_native(&mut ctx, None, 0);
            (cid, vid, tid)
        };

        // isHorizontal=false, curve k=1,b=0, vertex(1,2,3) → y += curve(1) = y + 1 = 3, result(1,3,3)
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_param(0, vertex_id);
            ctm.invoke_native_method(&mut ctx, trans_id, 0);
            let result_id = ctx.get_object_return();
            let rx = ctx.get_object_float_field(result_id, 0) as f32;
            let ry = ctx.get_object_float_field(result_id, 1) as f32;
            let rz = ctx.get_object_float_field(result_id, 2) as f32;
            assert!((rx - 1.0).abs() < 0.01, "x 不变");
            assert!((ry - 3.0).abs() < 0.01, "垂直变形 y 应为 3.0，实际 {ry}");
            assert!((rz - 3.0).abs() < 0.01, "z 不变");
        }
    }

    #[test]
    fn test_curve_mesh_no_curve_returns_identity() {
        let ctm = CurveMeshTransformer { curve: 0, is_horizontal: false };
        let mut vm = make_vm();
        register_class(&mut vm, std::sync::Arc::new(CurveMeshTransformer { curve: 0, is_horizontal: false }));

        let (vertex_id, trans_id) = {
            let mut ctx = NativeContext::new(&mut vm);
            let vid = make_v3(&mut ctx, 5.0, 6.0, 7.0);
            ctx.set_object_param(0, 0); // curve = 0
            ctx.set_bool_param(0, true);
            let tid = ctm.do_construct_native(&mut ctx, None, 0);
            (vid, tid)
        };

        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_param(0, vertex_id);
            ctm.invoke_native_method(&mut ctx, trans_id, 0);
            let result_id = ctx.get_object_return();
            let rx = ctx.get_object_float_field(result_id, 0) as f32;
            let ry = ctx.get_object_float_field(result_id, 1) as f32;
            let rz = ctx.get_object_float_field(result_id, 2) as f32;
            assert!((rx - 5.0).abs() < 0.01);
            assert!((ry - 6.0).abs() < 0.01);
            assert!((rz - 7.0).abs() < 0.01);
        }
    }
}
