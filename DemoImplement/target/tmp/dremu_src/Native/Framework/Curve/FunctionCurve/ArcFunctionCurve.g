using Gorge;
namespace GorgeFramework;

// 弧函数曲线，按圆心角在位于x轴的指定弦上生成一个弧
native class ArcFunctionCurve : FunctionCurve
{
    // 弦起点坐标
    [auto defaultValue]
    @Inject<float>
    float chordStart;
    
    // 弦终点坐标
    [auto defaultValue]
    @Inject<float>
    float chordEnd;
    
    // 圆心角，弧度制
    [auto defaultValue]
    @Inject<float>
    float angle;
    
    ArcFunctionCurve();
    
    ArcFunctionCurve(float chordStart, float chordEnd, float angle);
    
    float Evaluate(float x);
}