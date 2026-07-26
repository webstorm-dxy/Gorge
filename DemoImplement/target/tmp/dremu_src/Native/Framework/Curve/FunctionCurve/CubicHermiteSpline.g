using Gorge;
namespace GorgeFramework;

// 加权三次埃尔米特曲线
native class CubicHermiteSpline : FunctionCurve
{
    [auto defaultValue]
    @Inject<Vector2^>
    Vector2 startPoint;
    
    [auto defaultValue]
    @Inject
    float startTangent;
    
    [auto defaultValue]
    @Inject
    float startWeight;
    
    [auto defaultValue]
    @Inject<Vector2^>
    Vector2 endPoint;
    
    [auto defaultValue]
    @Inject
    float endTangent;
    
    [auto defaultValue]
    @Inject
    float endWeight;
    
    CubicHermiteSpline();
    
    float Evaluate(float x);
}