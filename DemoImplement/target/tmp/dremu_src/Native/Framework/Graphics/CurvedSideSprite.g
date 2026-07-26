using Gorge;
namespace GorgeFramework;

// 曲边四边形
native class CurvedSideSprite : Node
{
    // 图像
    Graph graph;
    
    // 左下顶点位置
    Vector2 vertexLeftBottom;
    
    // 右下顶点位置
    Vector2 vertexRightBottom;
    
    // 左上顶点位置
    Vector2 vertexLeftTop;
    
    // 右上顶点位置
    Vector2 vertexRightTop;
    
    // 左边曲线
    FunctionCurve leftCurve;
    
    // 左边曲线
    FunctionCurve topCurve;
    
    // 左边曲线
    FunctionCurve rightCurve;
    
    // 左边曲线
    FunctionCurve bottomCurve;
    
    // 颜色
    ColorArgb color;
    
    // 水平网格段数
    int horizontalSegments;
    
    // 垂直网格段数
    int verticalSegments;
    
    CurvedSideSprite(Graph graph);
}