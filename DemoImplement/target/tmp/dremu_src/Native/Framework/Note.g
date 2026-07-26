using Gorge;
namespace GorgeFramework;

native class Note : Element
{
    SignalTsiga automaton;
    
    Note();
    
    // 执行响应
    // 返回值为自动机指令表
    IAutomatonCommand[] DoRespond(string respondMode, float respondChartTime);
}