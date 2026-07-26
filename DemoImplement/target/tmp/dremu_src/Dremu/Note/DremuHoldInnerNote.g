using Gorge;
using GorgeFramework;
namespace Dremu;

[
    string displayName = "内部Note"
]
@Editable
class DremuHoldInnerNote
{
    [
        auto defaultValue = 1.0,
        string displayName = "打击时间",
        string information = "单位秒，相对于所在Note的打击时间>0"
    ]
    @Inject<float>
    float hitTime = ^hitTime;
    
    [
        auto defaultValue = true,
        string displayName = "打击音效",
        string information = "是否播放打击音效"
    ]
    @Inject<bool>
    bool audioEffect = ^audioEffect;
    
    [
        auto defaultValue = true,
        string displayName = "打击效果",
        string information = "是否显示打击效果"
    ]
    @Inject<bool>
    bool respondHint = ^respondHint;
    
    DremuHoldInnerNote()
    {
    }

    
}