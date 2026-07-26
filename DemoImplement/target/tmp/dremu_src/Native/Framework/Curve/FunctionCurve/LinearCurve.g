using Gorge;
namespace GorgeFramework;

native class LinearCurve : FunctionCurve
{
    [auto defaultValue]
    @Inject
    float timeStart;
    
    [auto defaultValue]
    @Inject
    float valueStart;
    
    [auto defaultValue]
    @Inject
    float timeEnd;
    
    [auto defaultValue]
    @Inject
    float valueEnd;
    
    LinearCurve();
    
    LinearCurve(float timeStart, float valueStart, float timeEnd, float valueEnd);
    
    float Evaluate(float time);
}