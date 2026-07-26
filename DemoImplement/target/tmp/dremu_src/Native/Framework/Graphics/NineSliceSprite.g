using Gorge;
namespace GorgeFramework;

native class NineSliceSprite : Node
{
    // 图像
    Graph graph;
    
    // 左上切分点(像素数)
    Vector2 sliceLeftTop;
    
    // 右下切分点(像素数)
    Vector2 sliceRightBottom;
    
    // 基本尺寸(不做任何9slice拉伸的情况下的原始大小)
    Vector2 baseSize;
    
    // 颜色
    ColorArgb color;
    
    // 颜色偏移
    Vector3 hsl;
    
    NineSliceSprite(Graph graph, Vector2 sliceLeftTop, Vector2 sliceRightBottom, Vector2 baseSize);
}