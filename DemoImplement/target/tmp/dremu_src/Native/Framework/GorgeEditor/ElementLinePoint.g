using Gorge;
namespace GorgeFramework;

native class ElementLinePoint
{
    // 该点所在时间
    float time;
    
    // 该点绘制的纵向位置，从下向上，0-1
    float position;
    
    // 该点绘制的纵向宽度，0-1
    float width;
    
    ElementLinePoint(float time, float position, float width);
}