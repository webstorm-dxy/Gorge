using Gorge;
namespace GorgeFramework;

native class InputGraph
{
    InputGraph(InputGraphState[] states, bool accept, bool stackRespond, int inputPointer, string exportState);
    
    // 进入接收边，只执行状态修改，不执行动作
    // 返回值为接受边对象
    InputGraphEdge GoAcceptEdge(float chartTime, HistoryStack historyStack);
    
    // 进入拒绝边，只执行状态修改，不执行动作
    // 返回值为接受边对象
    InputGraphEdge GoDenyEdge(float chartTime, HistoryStack historyStack);
}