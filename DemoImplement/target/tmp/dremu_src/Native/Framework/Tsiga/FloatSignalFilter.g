using Gorge;
namespace GorgeFramework;

// 信号过滤器
// 目前只包含触控信号
native class FloatSignalFilter : SignalFilter
{
    // 触控区
    delegate<bool:FloatSignal> filterRange;
    
    // 过滤的频道名
    string channelName;
    
    FloatSignalFilter(string channelName, delegate<Priority[]> priority, int[] conditionType, delegate<bool:FloatSignal> filterRange,
        delegate<float> endTime, TimeMode timeMode, bool acceptConsume, bool denyConsume);
    
    bool CanDetect(string channelName);
    
    bool Detect(string channelName, int signalId, int conditionType, ISignal signalValue, ISignal lastSignalValue);
}