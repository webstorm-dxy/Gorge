using Gorge;
namespace GorgeFramework;

native class PeriodicFunctionCurve : FunctionCurve
{
    [auto defaultValue]
    @Inject<FunctionCurve^>
    FunctionCurve functionCurve;
    
    [auto defaultValue]
    @Inject
    float startX;
    
    [auto defaultValue]
    @Inject
    float endX;
    
    // 左包含，否则为右包含
    [auto defaultValue]
    @Inject
    bool leftClosed;
    
    PeriodicFunction();
    
    float Evaluate(float x);
}