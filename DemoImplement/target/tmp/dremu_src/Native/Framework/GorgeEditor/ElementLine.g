using Gorge;
namespace GorgeFramework;

native class ElementLine
{
    // 该线的颜色
    ColorArgb color;
    
    // 该线的各点
    ElementLinePoint[] points;
    
    ElementLine(ColorArgb color, ElementLinePoint[] points);
}