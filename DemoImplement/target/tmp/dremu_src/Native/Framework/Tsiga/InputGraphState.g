using Gorge;
namespace GorgeFramework;

// 输入图的一个状态
native class InputGraphState
{
    // 输入信号过滤器
    SignalFilter filter;
    
    // 过滤成功出边
    InputGraphEdge acceptedEdge;
    
    // 过滤失败出边
    InputGraphEdge deniedEdge;
    
    InputGraphState(SignalFilter filter, InputGraphEdge acceptedEdge, InputGraphEdge deniedEdge);
}