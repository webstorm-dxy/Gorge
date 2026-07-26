using Gorge;
namespace GorgeFramework;

native class MultiplicationFunctionCurve : FunctionCurve
{
    [auto defaultValue]
    @Inject<FunctionCurve^>
    FunctionCurve firstFunctionCurve;
    
    [auto defaultValue]
    @Inject<FunctionCurve^>
    FunctionCurve secondFunctionCurve;
    
    MultiplicationFunctionCurve();
    
    MultiplicationFunctionCurve(FunctionCurve firstFunctionCurve, FunctionCurve secondFunctionCurve);
    
    float Evaluate(float x);
}