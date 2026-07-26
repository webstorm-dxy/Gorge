using Gorge;
namespace GorgeFramework;

// 网格化Sprite
native class MeshedSprite : Node
{
    // 图像
    Graph graph;
    
    // 中心点x
    float centerX;
    
    // 中心点y
    float centerY;
    
    // 宽度
    float width;
    
    // 高度
    float height;
    
    // 颜色
    ColorArgb color;
    
    // 水平网格段数
    int horizontalSegments;
    
    // 垂直网格段数
    int verticalSegments;
    
    MeshedSprite(Graph graph);
    
    void AddMeshTransformer(IMeshTransformer transformer);
    
    void ForceUpdate();
}