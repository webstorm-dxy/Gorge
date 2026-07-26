using Gorge;
namespace GorgeFramework;

native class Vector2
{
    [auto defaultValue]
    @Inject
    float x;
    [auto defaultValue]
    @Inject
    float y;
    
    Vector2();
    
    Vector2(float x, float y);
    
    Vector3 ToVector3();
    
    static Vector2 Scale(Vector2 v1, Vector2 v2);
    
    static float Distance(Vector2 v1, Vector2 v2);
    
    static Vector2 Normalize(Vector2 v);
    
    // 计算向量的角度，角度制
    static float Angle(Vector2 v);
    
    static Vector2 Lerp(Vector2 a, Vector2 b, float t);
}