using Gorge;
namespace GorgeFramework;

// 环形变换器，将方形网格变换为扇环
native class AnnulusMeshTransformer :: IMeshTransformer
{
    // 将x坐标映射到角度（弧度制）的函数
    FunctionCurve xAngle;
    
    // 将y坐标映射到半径的函数
    FunctionCurve yRadius;
    
    AnnulusMeshTransformer();
    
    Vector3 Transform(Vector3 vertex);
}