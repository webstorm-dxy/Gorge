using Gorge;
namespace GorgeFramework;

// 追加信号边沿的指令
native class AppendSignalCommand :: IAutomatonCommand
{
    string channelName;
    
    int id;
    
    float delaySimulateTime;
    
    ISignal value;
    
    AppendSignalCommand(string channelName, int id, float delaySimulateTime, ISignal value);
}