using Gorge;
namespace GorgeFramework;

native class VariableFloat
{
    @Inject
    float baseValue;
    
    [auto defaultValue]
    @Inject<FunctionCurve^>
    FunctionCurve variationCurve;
    
    VariableFloat();
    
    float EvaluateAdd(float curveTime);
    
    float EvaluateDoubleLerp(float curveTime, float min, float max);
}