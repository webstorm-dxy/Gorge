using Gorge;
namespace GorgeFramework;

// 信号过滤器
// 目前只包含触控信号
native class SignalFilter
{
    // 优先级
    delegate<Priority[]> priority;
    
    // 过滤情况集合
    int[] conditionTypes;
    
    // 结束时间
    delegate<float> endTime;
    
    // 时间类型
    TimeMode timeMode;
    
    // 接收时是否消耗
    bool acceptConsume;
    
    // 拒绝时是否消耗
    // 危险逻辑，使用时注意
    // 这里的消耗是在对单个信号的判断后，如果拒绝则立刻消耗，所以实际效果有点类似于“无条件消耗”
    // 如果后续或关系中的其他检测条件被接受，不会回溯解除消耗，并且由于没有对其他或关系项目的检测，所以不会触发消耗
    bool denyConsume;
    
    InputSignalFilter(delegate<Priority[]> priority, int[] conditionTypes, delegate<float> endTime, TimeMode timeMode, bool acceptConsume, bool denyConsume);
    
    // 过滤信道
    // 根据信道名判断是否是否进入检测
    bool CanDetect(string channelName);
    
    // 执行检测
    // 返回值为是否可接受
    bool Detect(string channelName, int signalId, int conditionType, ISignal signalValue, ISignal lastSignalValue);
}