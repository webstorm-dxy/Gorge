using Gorge;
namespace GorgeFramework;

native class Vector3
{
    [auto defaultValue]
    @Inject
    float x;
    [auto defaultValue]
    @Inject
    float y;
    [auto defaultValue]
    @Inject
    float z;
    
    Vector3();
    
    Vector3(float x, float y, float z);
    
    Vector2 ToVector2();
}