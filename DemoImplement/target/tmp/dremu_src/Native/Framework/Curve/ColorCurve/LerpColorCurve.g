using Gorge;
namespace GorgeFramework;

// 补间颜色曲线
native class LerpColorCurve : ColorCurve
{
    // 颜色点
    [auto defaultValue]
    @Inject<ColorArgb^[]^>
    ColorArgb[] colorPoints;
    
    // 进度曲线，0代表第0点
    [auto defaultValue]
    @Inject<FunctionCurve^>
    FunctionCurve progressCurve;
    
    LerpColorCurve();
    
    ColorArgb Evaluate(float x);
}