using Gorge;
namespace GorgeFramework;

native class TimeItem
{
    // 是否接收
    bool accept;
    
    // 响应结果，为null代表不响应
    string respondMode;
    
    // 出栈时间
    delegate<float> time;
    
    TimeItem(delegate<float> time, bool accept, string respondMode);
}