using Gorge;
namespace GorgeFramework;

native class ConstantFunctionCurve : FunctionCurve
{
    [auto defaultValue]
    @Inject<float>
    float value;
    
    ConstantFunctionCurve();
    
    ConstantFunctionCurve(float value);
    
    float Evaluate(float x);
}