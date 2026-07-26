using Gorge;
namespace GorgeFramework;

// 轴对称函数
native class AxialSymmetricFunctionCurve : FunctionCurve
{
    [auto defaultValue]
    @Inject<FunctionCurve^>
    FunctionCurve functionCurve;
    
    // 对称轴
    [auto defaultValue]
    @Inject<float>
    float axis;
    
    // 是否保留左侧而对称到右侧
    [auto defaultValue]
    @Inject<bool>
    bool keepLeft;
    
    AxialSymmetricFunctionCurve();
    
    float Evaluate(float x);
}