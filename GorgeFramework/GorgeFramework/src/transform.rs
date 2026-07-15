//! `GorgeFramework` — 曲线网格变形器。
//!
//! 移植自 C# 参考实现 `CurveMeshTransformer`。将网格顶点沿
//! 函数曲线方向变形，用于实现轨道弯曲等视觉效果。

use glam::Vec3;
use crate::function_curve::FunctionCurve;

/// 曲线网格变形器
///
/// 对齐 C# `CurveMeshTransformer`。给定一条函数曲线和一个变形方向，
/// 对三维坐标进行曲线变形：`Transform(v) = (x, y, z ± curve(x))`。
#[derive(Debug)]
pub struct CurveMeshTransformer {
    /// 变形曲线：y = f(x)
    pub curve: Box<dyn FunctionCurve>,
    /// 是否水平变形（true: 沿 Y 轴变形, false: 沿 Z 轴变形）
    pub is_horizontal: bool,
}

impl CurveMeshTransformer {
    pub fn new(curve: Box<dyn FunctionCurve>, is_horizontal: bool) -> Self {
        Self { curve, is_horizontal }
    }

    /// 对输入坐标进行变形
    pub fn transform(&self, v: Vec3) -> Vec3 {
        let curve_val = self.curve.evaluate(v.x);
        if self.is_horizontal {
            Vec3::new(v.x, v.y + curve_val, v.z)
        } else {
            Vec3::new(v.x, v.y, v.z + curve_val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::function_curve::ConstantFunctionCurve;

    #[test]
    fn test_curve_transform_horizontal() {
        let c = Box::new(ConstantFunctionCurve::new(5.0));
        let t = CurveMeshTransformer::new(c, true);
        let result = t.transform(Vec3::new(1.0, 2.0, 3.0));
        assert!((result.y - 7.0).abs() < 0.01); // y + curve(1) = 2 + 5
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_curve_transform_vertical() {
        let c = Box::new(ConstantFunctionCurve::new(10.0));
        let t = CurveMeshTransformer::new(c, false);
        let result = t.transform(Vec3::new(1.0, 2.0, 3.0));
        assert!((result.z - 13.0).abs() < 0.01); // z + curve(1) = 3 + 10
    }
}
