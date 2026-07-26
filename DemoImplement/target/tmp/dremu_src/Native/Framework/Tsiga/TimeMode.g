using Gorge;
namespace GorgeFramework;

// 输入项的时间类型
native enum TimeMode
{
    // 在结束时间前捕获一次，捕获成功则跳转
    CatchBefore,
    // 在结束时间前始终存在，时间结束时跳转
    KeepUntil
}