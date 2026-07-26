using Gorge;
namespace GorgeFramework;

native class CurveSprite : Node
{
    // 曲线点
    Vector2[] points;
    
    // 颜色
    ColorArgb color;

    // 宽度
    float width;
    
    Curve2D(Vector2[] points);
}