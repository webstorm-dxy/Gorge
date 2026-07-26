using Gorge;
using GorgeFramework;
namespace Dremu;

[
    delegate<float:DremuLane^> display = string:(DremuLane^ laneInjector) ->
    {
        return laneInjector.^name;
    },
    ColorArgb^ color = ColorArgb : {a : 1.0, r : 0.2396693, g : 0.6370158, b : 0.8},
    delegate<ElementLine:DremuLane^> elementLine = ElementLine:(DremuLane^ lane) ->
    {
        ElementLinePoint[] points = new ElementLinePoint[2];
        points[0] = new ElementLinePoint(lane.^generateTime, 0.5, 0.8);
        points[1] = new ElementLinePoint(lane.^generateTime + lane.^keepTime, 0.5, 0.8);
        return new ElementLine(new ColorArgb(0.3, 1, 1, 1), points);
    },
    string displayName = "Note"
]
@EditableElement(type = "Note", editUpdateMode = EditUpdateMode.RePlay)
class DremuNote : Note
{
    [
        string type = "基本",
        int order = 0,
        string displayName = "轨道名",
        string information = "所属轨道的名称",
        delegate<bool:string> check = bool:(string laneName) -> { return true; }
    ]
    @Inject
    string laneName = ^laneName;
    
    [
        auto defaultValue = 0.0,
        string type = "基本",
        int order = 1,
        string displayName = "打击时刻",
        string information = "单位秒，>=0",
        delegate<bool:float> check = bool:(float hitTime) -> { return hitTime >= 0; },
        string timePointName = "HitTime",
        string timePointEarlyAnchor = null,
        string timePointLateAnchor = null,
        string timePointRegion = "RhythmDuration"
    ]
    @Inject
    float hitTime = ^hitTime;
    
    [
        auto defaultValue = 1.5,
        string type = "生命周期",
        int order = 1000,
        string displayName = "超前生成时间",
        string information = "单位秒，>=0",
        delegate<bool:float> check = bool:(float leadTime) -> { return leadTime >= 0; },
        string timePointName = "GenerateTime",
        string timePointEarlyAnchor = null,
        string timePointLateAnchor = "HitTime",
        bool majorTimePoint = false
    ]
    @Inject
    float leadTime = ^leadTime;
    
    [
        auto defaultValue = 1.5,
        string type = "生命周期",
        int order = 1001,
        string displayName = "滞后销毁时间",
        string information = "单位秒，>=0",
        delegate<bool:float> check = bool:(float lagTime) -> { return lagTime >= 0; },
        string timePointName = "DestroyTime",
        string timePointEarlyAnchor = "RhythmDuration.End",
        string timePointLateAnchor = null,
        bool majorTimePoint = false
    ]
    @Inject
    float lagTime = ^lagTime;
    
    [
        auto defaultValue = VariableFloat : {baseValue : 0.0},
        string type = "基本",
        int order = 2,
        string displayName = "位置",
        string information = "标准坐标系|横轴为以打击时刻为0点的时间，单位秒；纵轴为相对于轨道的横向位置，取基础值加曲线纵轴",
        delegate<bool:VariableFloat^> check = bool:(VariableFloat^ position) -> { return true; },
        float scaleMax = 9.5,
        float scaleMin = -9.5,
        string baseAt = "HitTime"
    ]
    @Inject<VariableFloat^>
    VariableFloat position = new ^position();
    
    [
        auto defaultValue = LinearFunctionCurve : {k : -8.0, b : 0.0},
        string type = "基本",
        int order = 3,
        string displayName = "距离曲线",
        string information = "标准坐标系|横轴为以打击时刻为0点的时间，单位秒；纵轴为距离轨道判定线的距离",
        delegate<bool:VariableFloat^> check = bool:(VariableFloat^ positionX) -> { return true; },
        float scaleMax = 11.5,
        float scaleMin = -11.5,
        string baseAt = "HitTime"
    ]
    @Inject<FunctionCurve^>
    FunctionCurve distance = new ^distance();
    
    [
        auto defaultValue = null,
        string type = "效果",
        int order = 2001,
        string displayName = "颜色",
        string information = "横轴为以打击时刻为0的时间，单位秒",
        delegate<bool:ColorCurve^> check = bool:(ColorCurve^ color) -> { return true; }
    ]
    @Inject<ColorCurve^>
    ColorCurve color = (^color == null) ? null : (new ^color());
    
    [
        auto defaultValue = false,
        string type = "效果",
        int order = 2002,
        string displayName = "打击效果跟随判定线",
        string information = "打击效果是否跟随判定线移动",
        delegate<bool:bool> check = bool:(bool hintReference) -> { return true; }
    ]
    @Inject<bool>
    bool hintReference = ^hintReference;
    
    [
        auto defaultValue = null,
        string type = "效果",
        int order = 2003,
        string displayName = "打击效果颜色1",
        string information = "主要颜色",
        delegate<bool:ColorArgb^> check = bool:(ColorArgb^ respondHintColor1) -> { return true; }
    ]
    @Inject<ColorArgb^>
    ColorArgb respondHintColor1 = (^respondHintColor1 == null) ? new ColorArgb(1, 1, 1, 1) : (new ^respondHintColor1());
    
    [
        auto defaultValue = null,
        string type = "效果",
        int order = 2004,
        string displayName = "打击效果颜色2",
        string information = "闪动颜色",
        delegate<bool:ColorArgb^> check = bool:(ColorArgb^ respondHintColor1) -> { return true; }
    ]
    @Inject<ColorArgb^>
    ColorArgb respondHintColor2 = (^respondHintColor2 == null) ? new ColorArgb(1, 1, 1, 1) : (new ^respondHintColor2());
    
    [
        auto defaultValue = -2.0,
        string type = "效果",
        int order = 2005,
        string displayName = "叠放次序",
        string information = "标准坐标系",
        delegate<bool:float> check = bool:(float positionZ) -> { return true; },
        float scaleMax = 11.5,
        float scaleMin = -11.5
    ]
    @Inject<float>
    float positionZ = ^positionZ;
    
    DremuLane lane;
    
    NineSliceSprite graphNode;
    
    // 判定目标位置，轨道本地
    Vector2 aimPosition = new Vector2(0, 0);
    
    DremuNote()
    {
        // 查找对应的判定线，没有判定线回退到引导线
        lane = (DremuLane) Environment.FindAliveLane("Dremu.DremuMainLane", laneName);
        if (lane == null)
        {
            lane = (DremuLane) Environment.FindAliveLane("Dremu.DremuGuideLane", laneName);
        }
    }
    
    
    
    @PeriodModifier
    static void PeriodModifier(DremuNote^ noteInjector, PeriodConfig periodConfig)
    {
        noteInjector.^hitTime = noteInjector.^hitTime + periodConfig.timeOffset;
    }
    
    // 获取触控信号和瞄准点间的距离
    float GetAimDistance(TouchSignal signal)
    {
        return Vector2.Distance(signal.position, lane.noteReferenceNode.LocalPositionToGlobalPosition(aimPosition.ToVector3()).ToVector2());
    }
}