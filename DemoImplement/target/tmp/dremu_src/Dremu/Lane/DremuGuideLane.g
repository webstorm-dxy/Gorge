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
        points[0] = new ElementLinePoint(lane.^generateTime, 0.1, 0.2);
        points[1] = new ElementLinePoint(lane.^generateTime + lane.^keepTime, 0.1, 0.2);
        return new ElementLine(new ColorArgb(0.3, 1, 1, 1), points);
    },
    string displayName = "引导线"
]
@EditableElement(type = "轨道", editUpdateMode = EditUpdateMode.RePlay)
class DremuGuideLane : DremuLane
{
    [
        string type = "基本",
        int order = 1,
        string displayName = "主轨道",
        string information = "不能为空",
        delegate<bool:string> check = bool:(string mainLaneName) -> { return mainLaneName != ""; }
    ]
    @Inject
    string mainLaneName = ^mainLaneName;
    
    [
        auto defaultValue = VariableFloat : {baseValue : 0.0},
        string type = "基本",
        int order = 2,
        string displayName = "位置",
        string information = "主轨道X轴|横轴为以生成时刻为0的时间，单位秒；纵轴为坐标加值，实时坐标为基值加上纵轴值",
        delegate<bool:VariableFloat^> check = bool:(VariableFloat^ position) -> { return true; },
        float scaleMax = 9.5,
        float scaleMin = -9.5,
        string baseAt = "GenerateTime"
    ]
    @Inject<VariableFloat^>
    VariableFloat position = new ^position();
    
    [
        auto defaultValue = true,
        string type = "效果",
        int order = 2004,
        string displayName = "切线投影修正",
        string information = "是否将曲线横向变化修正到主轨道判定线的切线上，否则与主轨道x方向平行",
        delegate<bool:int> check = bool:(bool targetProjectionFix) -> { return true; }
    ]
    @Inject
    bool targetProjectionFix = ^targetProjectionFix;
    
    DremuMainLane mainLane;
    
    [
        delegate<float:DremuLane^> time = float:(DremuLane^ laneInjector) ->
        {
            return laneInjector.^generateTime;
        }
    ]
    @ForwardTimedGenerate
    [
        delegate<float:DremuLane^> time = float:(DremuLane^ laneInjector) ->
        {
            return laneInjector.^generateTime + laneInjector.^keepTime;
        }
    ]
    @BackwardTimedGenerate
    @EditGenerate
    DremuGuideLane() : super()
    {
        mainLane = (DremuMainLane) Environment.FindAliveLane("Dremu.DremuMainLane", mainLaneName);
        if (mainLane == null)
        {
            return null;
        }
        
        noteReferenceNode = mainLane.noteReferenceNode;
        
        // GorgeGraphics
        positionNode = new Node();
        positionNode.positionReference = mainLane.positionNode;
        positionNode.rotationReference = mainLane.positionNode;
        
        lineGraphNode = new CurveSprite(null);
        lineGraphNode.positionReference = positionNode;
        lineGraphNode.rotationReference = positionNode;
        lineGraphNode.size.x = 1;
        lineGraphNode.position.z = positionZ;
        
        nodes = new Node[2];
        nodes[0] = positionNode;
        nodes[1] = lineGraphNode;
        
        ITransformer[] transformers = new ITransformer[1];
        transformers[0] = new DremuGuideLanePositionTransformer(this);
        
        ITransformer[] lateTransformers = new ITransformer[2];
        lateTransformers[0] = new DremuGuideLaneLineTransformer(this);
        lateTransformers[1] = new DremuLaneColorTransformer(this);
        
        simulator = new ElementSimulator(transformers);
        lateIndependentSimulator = new ElementSimulator(lateTransformers);
    }
    
    // 计算指定x位置下按法线方向离线指定距离的点相对坐标
    Vector2 EvaluatePointPosition(float x, float distance, float now)
    {
        float mainX = x + position.EvaluateAdd(now - generateTime) + EvaluateLaneLine(distance, now);
        return mainLane.EvaluatePointPosition(mainX, distance, now);
    }
    
    // 计算指定x位置下按法线方向离线指定距离的点相对角度
    float EvaluatePointRotation(float x, float distance, float now)
    {
        Vector2 point1 = EvaluatePointPosition(0, distance - evaluateDelta, now);
        Vector2 point2 = EvaluatePointPosition(0, distance + evaluateDelta, now);
        
        return Vector2.Angle(new Vector2(point2.x - point1.x, point2.y - point1.y)) - 90;
    }
    
    // 计算指定x位置下的主判定线切线角度
    float EvaluateNormalVectorAngle(float x, float now)
    {
        return mainLane.EvaluateNormalVectorAngle(x, now);
    }
    
    @ForwardTimedDestroy
    float ForwardDestroyTime()
    {
        return generateTime + keepTime;
    }
    
    @BackwardTimedDestroy
    float BackwardDestroyTime()
    {
        return generateTime;
    }
    
    @EditTryGenerate
    static bool TryGenerate(DremuLane^ newInjector, float chartTime)
    {
        return chartTime >= generateTime && chartTime <= generateTime + keepTime;
    }
    
    @PeriodModifier
    static void PeriodModifier(DremuLane^ laneInjector, PeriodConfig periodConfig)
    {
        laneInjector.^generateTime = laneInjector.^generateTime + periodConfig.timeOffset;
    }
}