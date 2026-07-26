using Gorge;
namespace GorgeFramework;

native class TouchSignal :: ISignal
{
    bool isTouching;
    Vector2 position;
    
    TouchSignal(bool isTouching, Vector2 position);
}