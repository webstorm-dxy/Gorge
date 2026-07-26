using Gorge;
namespace GorgeFramework;

// 曲线变换器，在纵向或横向上生效，在指定方向上按曲线扭曲，另一方向不变形
native class CurveMeshTransformer :: IMeshTransformer
{
    // 形状曲线
    FunctionCurve curve;
    
    // 是否在横向上变形
    bool isHorizontal;
    
    CurveMeshTransformer();
    
    Vector3 Transform(Vector3 vertex);
}