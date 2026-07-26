using Gorge;
namespace GorgeFramework;

native class Sprite : Node
{
    // 图像
    Graph graph;
    
    // 颜色
    ColorArgb color;
    
    Sprite(Graph graph);
}