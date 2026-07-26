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
    string displayName = "判定线"
]
@EditableElement(type = "轨道", editUpdateMode = EditUpdateMode.RePlay)
class DremuMainLane : DremuLane
{
    [
        auto defaultValue = VariableFloat : {baseValue : 0.0},
        string type = "基本",
        int order = 1,
        string displayName = "X坐标",
        string information = "标准坐标系|横轴为谱面时间，单位秒；纵轴为坐标加值，实时坐标为基值加上纵轴值",
        delegate<bool:VariableFloat^> check = bool:(VariableFloat^ positionX) -> { return true; },
        float scaleMax = 9.5,
        float scaleMin = -9.5,
        string baseAt = "GenerateTime"
    ]
    @Inject<VariableFloat^>
    VariableFloat positionX = new ^positionX();
    
    [
        auto defaultValue = VariableFloat : {baseValue : 0.0},
        string type = "基本",
        int order = 2,
        string displayName = "Y坐标",
        string information = "标准坐标系|横轴为谱面时间，单位秒；纵轴为坐标加值，实时坐标为基值加上纵轴值",
        delegate<bool:VariableFloat^> check = bool:(VariableFloat^ positionY) -> { return true; },
        float scaleMax = 9.5,
        float scaleMin = -9.5,
        string baseAt = "GenerateTime"
    ]
    @Inject<VariableFloat^>
    VariableFloat positionY = new ^positionY();
    
    [
        auto defaultValue = VariableFloat : {baseValue : 0.0},
        string type = "基本",
        int order = 3,
        string displayName = "角度",
        string information = "标准坐标系|横轴为谱面时间，单位秒；纵轴为角度加值，实时角度为基值加上纵轴值",
        delegate<bool:VariableFloat^> check = bool:(VariableFloat^ rotationZ) -> { return true; },
        float scaleMax = 9.5,
        float scaleMin = -9.5,
        string baseAt = "GenerateTime"
    ]
    @Inject<VariableFloat^>
    VariableFloat rotationZ = new ^rotationZ();
    
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
    DremuMainLane() : super()
    {
        // GorgeGraphics
        positionNode = new Node();
        noteReferenceNode = positionNode;
        
        lineGraphNode = new CurveSprite(null);
        lineGraphNode.positionReference = positionNode;
        lineGraphNode.rotationReference = positionNode;
        lineGraphNode.size.x = 1;
        lineGraphNode.position.z = positionZ;
        
        nodes = new Node[2];
        nodes[0] = positionNode;
        nodes[1] = lineGraphNode;
        
        ITransformer[] transformers = new ITransformer[1];
        transformers[0] = new DremuMainLanePositionTransformer(this);
        
        ITransformer[] lateTransformers = new ITransformer[2];
        lateTransformers[0] = new DremuMainLaneLineTransformer(this);
        lateTransformers[1] = new DremuLaneColorTransformer(this);
        
        simulator = new ElementSimulator(transformers);
        lateIndependentSimulator = new ElementSimulator(lateTransformers);
    }
    
    // 计算曲线值
    float EvaluateLineY(float x, float now)
    {
        return EvaluateLaneLine(x, now);
    }
    
    // 计算切向量，不归一化
    Vector2 EvaluateLineTangentVectorNotNormalized(float x, float now)
    {
        float x1 = x - evaluateDelta;
        float x2 = x + evaluateDelta;
        float y1 = EvaluateLineY(x1, now);
        float y2 = EvaluateLineY(x2, now);
        
        return new Vector2(x2 - x1, y2 - y1);
    }
    
    // 计算法向量，归一化
    Vector2 EvaluateLineNormalVector(float x, float now)
    {
        Vector2 tangentVector = EvaluateLineTangentVectorNotNormalized(x, now);
        Vector2 normalVector = new Vector2(-tangentVector.y, tangentVector.x);
        return Vector2.Normalize(normalVector);
    }
    
    // 计算指定x位置下按法线方向离线指定距离的点相对坐标
    Vector2 EvaluatePointPosition(float x, float distance, float now)
    {
        float y = EvaluateLineY(x, now);
        Vector2 normalVector = EvaluateLineNormalVector(x, now);
        return new Vector2(x + normalVector.x * distance, y + normalVector.y * distance);
    }
    
    // 计算指定x位置下按法线方向离线指定距离的点相对角度
    float EvaluatePointRotation(float x, float distance, float now)
    {
        return EvaluateNormalVectorAngle(x, now) - 90;
    }
    
    // 计算指定x位置下的主判定线切线角度
    float EvaluateNormalVectorAngle(float x, float now)
    {
        return Vector2.Angle(EvaluateLineNormalVector(x, now));
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