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
    string displayName = "轨道"
]
@EditableElement(type = "轨道", editUpdateMode = EditUpdateMode.RePlay)
class DremuLane : Element
{
    [
        string type = "基本",
        int order = 0,
        string displayName = "轨道名",
        string information = "不能为空",
        delegate<bool:string> check = bool:(string name) -> { return name != ""; }
    ]
    @Inject
    string name = ^name;
    
    [
        auto defaultValue = 0.0,
        string type = "生命周期",
        int order = 1000,
        string displayName = "生成时刻",
        string information = "单位秒",
        delegate<bool:float> check = bool:(float generateTime) -> { return true; },
        string timePointName = "GenerateTime",
        string timePointEarlyAnchor = null,
        string timePointLateAnchor = null
    ]
    @Inject
    float generateTime = ^generateTime;
    
    [
        auto defaultValue = 10.0,
        string type = "生命周期",
        int order = 1001,
        string displayName = "保持时长",
        string information = "单位秒，>0",
        delegate<bool:float> check = bool:(float keepTime) -> { return keepTime > 0; },
        string timePointName = "DestroyTime",
        string timePointEarlyAnchor = "GenerateTime",
        string timePointLateAnchor = null
    ]
    @Inject
    float keepTime = ^keepTime;
    
    [
        auto defaultValue = FunctionCurve^ : {ConstantFunctionCurve : {value : 0.0}},
        string type = "基本",
        int order = 4,
        string displayName = "判定线形状",
        string information = "形状曲线",
        delegate<bool:FunctionCurve^[]^> check = bool:(FunctionCurve^[]^ laneLines) -> { return true; }
    ]
    @Inject<FunctionCurve^[]^>
    FunctionCurve^[] laneLines = (^laneLines == null) ? null : (new (^laneLines)[^laneLines.length]);
    
    [
        auto defaultValue = LinearFunctionCurve : {k : 1.0, b : 0.0},
        string type = "基本",
        int order = 5,
        string displayName = "判定线动画进度",
        string information = "进度坐标系|横轴为以打击时刻为0点的时间，单位秒；纵轴为距离轨道判定线的距离",
        delegate<bool:VariableFloat^> check = bool:(VariableFloat^ animation) -> { return true; },
        float scaleMax = 5.5,
        float scaleMin = -0.5,
        string baseAt = "GenerateTime"
    ]
    @Inject<FunctionCurve^>
    FunctionCurve animation = new ^animation();
    
    [
        auto defaultValue = VariableFloat : {baseValue : -10.0},
        string type = "效果",
        int order = 2000,
        string displayName = "绘制起点",
        string information = "标准坐标系|横轴为以生成时刻为0的时间，单位秒；纵轴为坐标加值，实时点为基值加上纵轴值",
        delegate<bool:VariableFloat^> check = bool:(VariableFloat^ drawStartX) -> { return true; },
        float scaleMax = 9.5,
        float scaleMin = -9.5,
        string baseAt = "GenerateTime"
    ]
    @Inject<VariableFloat^>
    VariableFloat drawStartX = new ^drawStartX();
    
    [
        auto defaultValue = VariableFloat : {baseValue : 10.0},
        string type = "效果",
        int order = 2001,
        string displayName = "绘制终点",
        string information = "标准坐标系|横轴为以生成时刻为0的时间，单位秒；纵轴为当前动画进度在哪两个关键帧曲线间",
        delegate<bool:VariableFloat^> check = bool:(VariableFloat^ drawEndX) -> { return true; },
        float scaleMax = 9.5,
        float scaleMin = -9.5,
        string baseAt = "GenerateTime"
    ]
    @Inject<VariableFloat^>
    VariableFloat drawEndX = new ^drawEndX();
    
    [
        auto defaultValue = 100,
        string type = "效果",
        int order = 2002,
        string displayName = "绘制精度",
        string information = "采样点数，>1",
        delegate<bool:int> check = bool:(int pointCount) -> { return pointCount > 1; },
        float scaleMax = 1050.0,
        float scaleMin = 0.0
    ]
    @Inject
    int pointCount = ^pointCount;
    
    [
        auto defaultValue = 0.01,
        string type = "效果",
        int order = 2003,
        string displayName = "估算邻域宽度",
        string information = "标准坐标系，>0",
        delegate<bool:float> check = bool:(float evaluateDelta) -> { return evaluateDelta > 0; }
    ]
    @Inject
    float evaluateDelta = ^evaluateDelta;
    
    [
        auto defaultValue = null,
        string type = "效果",
        int order = 2004,
        string displayName = "线条颜色",
        string information = "横轴为以生成时刻为0的时间，单位秒",
        delegate<bool:ColorCurve^> check = bool:(ColorCurve^ color) -> { return true; }
    ]
    @Inject<ColorCurve^>
    ColorCurve color = (^color == null) ? null : (new ^color());
    
    [
        auto defaultValue = -1.0,
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
    
    Node positionNode;
    
    Node noteReferenceNode;
    
    FunctionCurve[] laneLineCurves;
    
    CurveSprite lineGraphNode;
    
    float now;
    
    DremuLane()
    {
        if (laneLines == null)
        {
            laneLineCurves = null;
        }
        else
        {
            laneLineCurves = new FunctionCurve[laneLines.length];
            for (int i = 0; i < laneLines.length; i = i + 1)
            {
                if (laneLines[i] == null)
                {
                    laneLineCurves[i] = new ConstantFunctionCurve(){value : 0.0};
                }
                else
                {
                    laneLineCurves[i] = new laneLines[i]();
                }
            }
        }
    }
    
    // 计算曲线点
    float EvaluateLaneLine(float x, float now)
    {
        this.now = now;
        int length = laneLineCurves.length;
        if (length == 0)
        {
            return 0;
        }
        int maxIndex = length - 1;
        float animationProgress = animation.Evaluate(now - generateTime);
        int curveIndex1 = Math.ClampInt(0, maxIndex, Math.Floor(animationProgress));
        int curveIndex2 = Math.ClampInt(0, maxIndex, Math.Ceil(animationProgress));
        if (curveIndex1 == curveIndex2)
        {
            return laneLineCurves[curveIndex1].Evaluate(x);
        }
        
        float y1 = laneLineCurves[curveIndex1].Evaluate(x);
        float y2 = laneLineCurves[curveIndex2].Evaluate(x);
        return Math.Lerp(y1, y2, animationProgress - curveIndex1);
    }
    
    // 计算曲线点，时间为上次调用EvaluateLaneLine(float, float)时传入的时间
    float EvaluateLaneLine(float x)
    {
        EvaluateLaneLine(x, now);
    }
    
    Vector2 EvaluatePointPosition(float x, float distance, float now)
    {
        // virtual
        return null;
    }
    
    float EvaluatePointRotation(float x, float distance, float now)
    {
        // virtual
        return 0;
    }
    
    float EvaluateNormalVectorAngle(float x, float now)
    {
        // virtual
        return 0;
    }
}