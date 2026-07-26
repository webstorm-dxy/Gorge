using Gorge;
namespace GorgeFramework;

native class LinearFunctionCurve : FunctionCurve
{
    [auto defaultValue]
    @Inject<float>
    float k;
    
    [auto defaultValue]
    @Inject<float>
    float b;
    
    LinearFunctionCurve();
    
    LinearFunctionCurve(float k, float b);
    
    float Evaluate(float x);
}