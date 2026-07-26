using Gorge;
namespace GorgeFramework;

// 二次函数 y = ax^2 + bx + c
native class QuadraticFunctionCurve : FunctionCurve
{
    [auto defaultValue]
    @Inject<float>
    float a;
    
    [auto defaultValue]
    @Inject<float>
    float b;
    
    [auto defaultValue]
    @Inject<float>
    float c;
    
    QuadraticFunctionCurve();
    
    QuadraticFunctionCurve(float a, float b, float c);
    
    float Evaluate(float x);
}