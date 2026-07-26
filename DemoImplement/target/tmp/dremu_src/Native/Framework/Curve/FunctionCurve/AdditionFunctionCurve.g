using Gorge;
namespace GorgeFramework;

native class AdditionFunctionCurve : FunctionCurve
{
    [auto defaultValue]
    @Inject<FunctionCurve^>
    FunctionCurve firstFunctionCurve;
    
    [auto defaultValue]
    @Inject<FunctionCurve^>
    FunctionCurve secondFunctionCurve;
    
    AdditionFunctionCurve();
    
    float Evaluate(float x);
}