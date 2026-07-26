using Gorge;
namespace GorgeFramework;

native class Math
{
    static float Abs(float f);
    
    static float Sqrt(float f);
    
    static float Max(float f1, float f2);
    
    static float Max(float f1, float f2, float f3, float f4);
    
    static float Min(float f1, float f2);
    
    static float Atan(float f);
    
    static float Sin(float f);
    
    static float Cos(float f);
    
    static float CosDeg(float f);
    
    static float SinDeg(float f);
    
    static float Pi();
    
    static float FloatPositiveInfinity();
    
    static float FloatNegativeInfinity();
    
    static int Floor(float f);
    
    static int Ceil(float f);
    
    static int ClampInt(int a, int b, int value);
    
    static float Lerp(float a, float b, float t);
    
    static float InverseLerp(float a, float b, float v);
}