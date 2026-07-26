using Gorge;
namespace GorgeFramework;

native class CompositeFunctionCurve : FunctionCurve
{
    [auto defaultValue]
    @Inject<FunctionCurve^>
    FunctionCurve outerFunctionCurve;
    
    [auto defaultValue]
    @Inject<FunctionCurve^>
    FunctionCurve innerFunctionCurve;
    
    CompositeFunctionCurve();
    
    CompositeFunctionCurve(FunctionCurve outerFunctionCurve, FunctionCurve innerFunctionCurve);
    
    float Evaluate(float x);
}