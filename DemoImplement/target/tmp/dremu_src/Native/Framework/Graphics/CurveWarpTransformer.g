using Gorge;
namespace GorgeFramework;

// 曲线扭曲变换器，将x轴扭曲为目标曲线，同时保持法线做纵向映射
native class CurveWarpTransformer :: IMeshTransformer
{
    // 形状曲线
    FunctionCurve curve;
    
    // 是否关闭曲率畸变
    bool preserveProportions;

    // 曲率畸变强度
    float curvatureInfluence;

    // 待变换轴向，对应曲线x轴
    Axis transformedAxis;

    // 曲线值轴向，对应曲线y轴
    Axis curveValueAxis;

    CurveWarpTransformer();
    
    Vector3 Transform(Vector3 vertex);
}