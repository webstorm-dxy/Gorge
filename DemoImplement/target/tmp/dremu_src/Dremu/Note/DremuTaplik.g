using Gorge;
using GorgeFramework;
namespace Dremu;

[
    delegate<float:DremuTaplik^> display = string:(DremuTaplik^ noteInjector) ->
    {
        return noteInjector.^hitTime + "";
    },
    ColorArgb^ color = ColorArgb : {a : 1.0, r : 0.0117647, g : 0.7647059, b : 0.6},
    delegate<ElementLine:DremuTaplik^> elementLine = ElementLine:(DremuTaplik^ noteInjector) ->
    {
        ElementLinePoint[] points = new ElementLinePoint[1];
        points[0] = new ElementLinePoint(noteInjector.^hitTime, 0.5, 0.2);
        return new ElementLine(new ColorArgb(1, 0.0117647, 0.7647059, 0.6), points);
    },
    string displayName = "Taplik"
]
@EditableElement(type = "Note", editUpdateMode = EditUpdateMode.RePlay)
class DremuTaplik : DremuNote
{
    // 滑动基点，标准坐标
    Vector2 flickBase;
    
    [
        delegate<float:DremuTaplik^> time = float:(DremuTaplik^ noteInjector) ->
        {
            return noteInjector.^hitTime - noteInjector.^leadTime;
        }
    ]
    @ForwardTimedGenerate
    [
        delegate<float:DremuTaplik^> time = float:(DremuTaplik^ noteInjector) ->
        {
            return noteInjector.^hitTime + noteInjector.^lagTime;
        }
    ]
    @BackwardTimedGenerate
    @EditGenerate
    DremuTaplik(bool isAutoPlay, bool isReverse) : super()
    {
        if (lane == null)
        {
            return null;
        }
        
        ITransformer[] transformers = new ITransformer[2];
        
        transformers[0] = new DremuSingleNotePositionTransformer(this);
        transformers[1] = new DremuSingleAimTransformer(this);
        
        ITransformer[] lateTransformers = new ITransformer[1];
        lateTransformers[0] = new DremuNoteColorTransformer(this);
        
        // GorgeGraphics
        ImageAsset lineImage = (ImageAsset) Environment.GetAssetByName("image:Dremu/FormAsset/Taplik");
        graphNode = new NineSliceSprite(lineImage.texture, new Vector2(80, 0), new Vector2(80, 0), new Vector2(2, 2));
        graphNode.size.y = 0.4;
        graphNode.size.x = 2;
        graphNode.position.z = positionZ;
        graphNode.positionReference = lane.noteReferenceNode;
        graphNode.rotationReference = lane.noteReferenceNode;
        
        nodes = new Node[1];
        nodes[0] = graphNode;
        
        if (isAutoPlay)
        {
            DremuSingleAutoplayTsigaInitializer initializer = new DremuSingleAutoplayTsigaInitializer(this, isReverse);
            automaton = new SignalTsiga(this, initializer.timeStack, initializer.inputGraph, initializer.historyStack);
        }
        else
        {
            DremuTaplikTsiga initializer = new DremuTaplikTsiga(this, isReverse);
            automaton = new SignalTsiga(this, initializer.timeStack, initializer.inputGraph, initializer.historyStack);
        }
        
        simulator = new ElementSimulator(transformers);
        lateIndependentSimulator = new ElementSimulator(lateTransformers);
    }
    
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
        
        Environment.PlayRespondEffect("RespondC");
        
        DremuRespondHint respondHint;
        
        if (hintReference)
        {
            respondHint = new DremuRespondHint(graphNode.position.ToVector2(), respondChartTime, lane.noteReferenceNode, respondHintColor1, respondHintColor2);
        }
        else
        {
            respondHint = new DremuRespondHint(graphNode.GlobalPosition().ToVector2(), respondChartTime, null, respondHintColor1, respondHintColor2);
        }
        
        IAutomatonCommand[] commands = new IAutomatonCommand[1];
        commands[0] = new DeriveElementCommand(respondHint, false);
        return commands;
    }

    @ForwardTimedDestroy
    float ForwardDestroyTime()
    {
        return hitTime + lagTime;
    }
    
    @BackwardTimedDestroy
    float BackwardDestroyTime()
    {
        return hitTime - leadTime;
    }
    
    @EditTryGenerate
    static bool TryGenerate(DremuTaplik^ newInjector, float chartTime)
    {
        return chartTime >= (newInjector.^hitTime - newInjector.^leadTime) && chartTime <= (newInjector.^hitTime + newInjector.^lagTime);
    }
    
    // 计算滑动距离
    float GetFlickDistance(TouchSignal signal)
    {
        if (flickBase == null)
        {
            return 0;
        }
        
        return Vector2.Distance(signal.position, flickBase);
    }
}