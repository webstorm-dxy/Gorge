using Gorge;
namespace GorgeFramework;

// 信号过滤器
// 目前只包含触控信号
native class InputSignalFilter : SignalFilter
{
    // 触控区
    delegate<bool:TouchSignal> touchArea;

    // 信号编号过滤器
    delegate<bool:int> signalIdFilter;

    // 信号检查调用，无论过滤是否成功，都会调用
    delegate<void:int,TouchSignal> onDetected;
    
    InputSignalFilter(delegate<Priority[]> priority, delegate<void:int,TouchSignal> onDetected, int[] touchType, delegate<bool:int> signalIdFilter, delegate<bool:TouchSignal> touchArea, delegate<float> endTime, TimeMode timeMode, bool acceptConsume, bool denyConsume);
    
    bool CanDetect(string channelName);
    
    bool Detect(string channelName, int signalId, int conditionType, ISignal signalValue, ISignal lastSignalValue);
}