using Gorge;
using GorgeFramework;
namespace Dremu;

[
    delegate<float:DremuHold^> display = string:(DremuHold^ noteInjector) ->
    {
        return noteInjector.^hitTime + "";
    },
    ColorArgb^ color = ColorArgb : {a : 1.0, r : 0.8, g : 0.2392157, b : 0.6224183},
    delegate<ElementLine:DremuHold^> elementLine = ElementLine:(DremuHold^ noteInjector) ->
    {
        ElementLinePoint[] points = new ElementLinePoint[2];
        points[0] = new ElementLinePoint(noteInjector.^hitTime, 0.3, 0.2);
        points[1] = new ElementLinePoint(noteInjector.^hitTime + noteInjector.^holdTime, 0.3, 0.2);
        return new ElementLine(new ColorArgb(1, 0.8, 0.447059, 0.682353), points);
    },
    string displayName = "Hold"
]
@EditableElement(type = "Note", editUpdateMode = EditUpdateMode.RePlay)
class DremuHold : DremuNote
{
    [
        auto defaultValue = 1.0,
        string type = "基本",
        int order = 4,
        string displayName = "保持时间",
        string information = "单位秒，>0",
        delegate<bool:float> check = bool:(float holdTime) -> { return holdTime > 0; },
        string timePointName = "HoldEnd",
        string timePointEarlyAnchor = "HitTime",
        string timePointLateAnchor = null,
        string timePointRegion = "RhythmDuration"
    ]
    @Inject
    float holdTime = ^holdTime;
    
    [
        auto defaultValue = null,
        string type = "基本",
        int order = 5,
        string displayName = "内部打击时刻",
        string information = "内部打击时刻列表",
        delegate<bool:DremuHoldInnerNote^[]^> check = bool:(DremuHoldInnerNote^[]^ innerNotes) -> { return true; }
    ]
    @Inject<DremuHoldInnerNote^[]^>
    DremuHoldInnerNote^[] innerNotes = (^innerNotes == null) ? null : (new (^innerNotes)[^innerNotes.length]);
    
    [
        auto defaultValue = LinearFunctionCurve : {k : -8.0, b : 0.0},
        string type = "基本",
        int order = 6,
        string displayName = "尾部距离曲线",
        string information = "标准坐标系|横轴为以释放时刻为0点的时间，单位秒；纵轴为距离轨道判定线的距离",
        delegate<bool:VariableFloat^> check = bool:(VariableFloat^ endDistance) -> { return true; },
        float scaleMax = 11.5,
        float scaleMin = -11.5,
        string baseAt = "HoldEnd"
    ]
    @Inject<FunctionCurve^>
    FunctionCurve endDistance = new ^endDistance();
    
    [
        auto defaultValue = ConstantFunctionCurve : {value : 0.0},
        string type = "基本",
        int order = 7,
        string displayName = "Hold轨迹",
        string information = "标准坐标系|横轴以打击时刻为0点的时间，单位秒；纵轴为距离在轨道上的相对位置",
        delegate<bool:VariableFloat^> check = bool:(VariableFloat^ holdLine) -> { return true; }
    ]
    @Inject<FunctionCurve^>
    FunctionCurve holdLine = new ^holdLine();
    
    [
        auto defaultValue = true,
        string type = "基本",
        int order = 8,
        string displayName = "轨迹截断",
        string information = "将尾部外的轨迹截断，否则压缩",
        delegate<bool:bool> check = bool:(bool truncation) -> { return true; }
    ]
    @Inject
    bool truncation = ^truncation;
    
    [
        auto defaultValue = 1,
        string type = "基本",
        int order = 9,
        string displayName = "标准轨迹长度",
        string information = "在轨迹截断模式下holdTime长度的轨迹对应的实际距离，不为0",
        delegate<bool:float> check = bool:(float holdLength) -> { return holdLength != 0; },
        float scaleMax = 20.0,
        float scaleMin = 0.0
    ]
    @Inject
    float holdLength = ^holdLength;
    
    [
        auto defaultValue = 100,
        string type = "效果",
        int order = 2005,
        string displayName = "绘制精度",
        string information = "采样点数，>1",
        delegate<bool:int> check = bool:(int pointCount) -> { return pointCount > 1; },
        float scaleMax = 1050.0,
        float scaleMin = 0.0
    ]
    @Inject
    int pointCount = ^pointCount;
    
    [
        auto defaultValue = true,
        string type = "效果",
        int order = 2006,
        string displayName = "切线投影修正",
        string information = "是否将曲线横向变化修正到主轨道判定线的切线上，否则与主轨道x方向平行",
        delegate<bool:bool> check = bool:(bool targetProjectionFix) -> { return true; }
    ]
    @Inject
    bool targetProjectionFix = ^targetProjectionFix;
    
    CurveSprite holdLineNode;
    
    DremuHoldInnerNote[] innerNoteConfigs;
    
    [
        delegate<float:DremuHold^> time = float:(DremuHold^ noteInjector) ->
        {
            return noteInjector.^hitTime - noteInjector.^leadTime;
        }
    ]
    @ForwardTimedGenerate
    [
        delegate<float:DremuHold^> time = float:(DremuHold^ noteInjector) ->
        {
            return noteInjector.^hitTime + noteInjector.^holdTime + noteInjector.^lagTime;
        }
    ]
    @BackwardTimedGenerate
    @EditGenerate
    DremuHold(bool isAutoPlay, bool isReverse) : super()
    {
        if (lane == null)
        {
            return null;
        }
        
        if (innerNotes == null)
        {
            innerNoteConfigs = null;
        }
        else
        {
            innerNoteConfigs = new DremuHoldInnerNote[innerNotes.length];
            for (int i = 0; i < innerNotes.length; i = i + 1)
            {
                if (innerNotes[i] == null)
                {
                    innerNoteConfigs[i] = null;
                }
                else
                {
                    innerNoteConfigs[i] = new innerNotes[i]();
                }
            }
        }
        
        ITransformer[] transformers = new ITransformer[2];
        
        transformers[0] = new DremuSingleNotePositionTransformer(this);
        transformers[1] = new DremuHoldAimTransformer(this);
        
        ITransformer[] lateTransformers = new ITransformer[3];
        lateTransformers[0] = new DremuHoldLineTransformer(this);
        lateTransformers[1] = new DremuNoteColorTransformer(this);
        lateTransformers[2] = new DremuHoldLineColorTransformer(this);
        
        // GorgeGraphics
        ImageAsset lineImage = (ImageAsset) Environment.GetAssetByName("image:Dremu/FormAsset/Tap");
        graphNode = new NineSliceSprite(lineImage.texture, new Vector2(80, 0), new Vector2(80, 0), new Vector2(2, 2));
        graphNode.size.y = 0.4;
        graphNode.size.x = 2;
        graphNode.position.z = positionZ;
        graphNode.positionReference = lane.noteReferenceNode;
        graphNode.rotationReference = lane.noteReferenceNode;
        
        holdLineNode = new CurveSprite(null);
        holdLineNode.positionReference = lane.noteReferenceNode;
        holdLineNode.rotationReference = lane.noteReferenceNode;
        holdLineNode.size.x = 1;
        holdLineNode.size.z = positionZ;
        
        nodes = new Node[2];
        nodes[0] = graphNode;
        nodes[1] = holdLineNode;
        
        if (isAutoPlay)
        {
            DremuHoldAutoplayTsigaInitializer initializer = new DremuHoldAutoplayTsigaInitializer(this, isReverse);
            automaton = new SignalTsiga(this, initializer.timeStack, initializer.inputGraph, initializer.historyStack);
        }
        else
        {
            DremuHoldTsiga initializer = new DremuHoldTsiga(this, isReverse);
            automaton = new SignalTsiga(this, initializer.timeStack, initializer.inputGraph, initializer.historyStack);
        }
        
        simulator = new ElementSimulator(transformers);
        lateIndependentSimulator = new ElementSimulator(lateTransformers);
    }

    int holdCount = 0;
    
    IAutomatonCommand[] DoRespond(string respondMode, float respondChartTime)
    {
        RespondResult respondResult;
        bool playEffect;
        
        switch (respondMode)
        {
            case "BestPerfect":
                respondResult = RespondResult.BestPerfect;
                playEffect = true;
                break;
            case "Perfect":
                respondResult = RespondResult.Perfect;
                playEffect = true;
                break;
            case "Good":
                respondResult = RespondResult.Good;
                playEffect = true;
                break;
            case "Miss":
                respondResult = RespondResult.Miss;
                playEffect = false;
                break;
        }
        
        Environment.Scoring(respondResult);
        
        if (!playEffect)
        {
            return new IAutomatonCommand[0];
        }
        if (holdCount == 0) {
            Environment.PlayRespondEffect("RespondA");
            holdCount = holdCount + 1;
        }
        else {
            Environment.PlayRespondEffect("RespondB");
        }
        DremuRespondHint respondHint;
        
        float t = respondChartTime - hitTime;
        
        float targetProjectionFixScale = 1;
        
        float mainX = position.EvaluateAdd(t);
        
        if (targetProjectionFix)
        {
            targetProjectionFixScale = Math.SinDeg(lane.EvaluateNormalVectorAngle(mainX, respondChartTime));
        }
        
        float x = mainX + holdLine.Evaluate(t) * targetProjectionFixScale;
        Vector2 point = lane.EvaluatePointPosition(x, 0, respondChartTime);
        
        if (hintReference)
        {
            respondHint = new DremuRespondHint(point, respondChartTime, lane.noteReferenceNode, respondHintColor1, respondHintColor2);
        }
        else
        {
            respondHint = new DremuRespondHint(lane.noteReferenceNode.LocalPositionToGlobalPosition(point.ToVector3()).ToVector2(), respondChartTime, null, respondHintColor1, respondHintColor2);
        }
        
        IAutomatonCommand[] commands = new IAutomatonCommand[1];
        commands[0] = new DeriveElementCommand(respondHint, false);
        return commands;
    }
    
    @ForwardTimedDestroy
    float ForwardDestroyTime()
    {
        return hitTime + holdTime + lagTime;
    }
    
    @BackwardTimedDestroy
    float BackwardDestroyTime()
    {
        return hitTime - leadTime;
    }
    
    @EditTryGenerate
    static bool TryGenerate(DremuHold^ newInjector, float chartTime)
    {
        return chartTime >= (newInjector.^hitTime - newInjector.^leadTime) && chartTime <= (newInjector.^hitTime + newInjector.^holdTime + newInjector.^lagTime);
    }
}