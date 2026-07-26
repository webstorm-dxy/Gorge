using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuRespondHint : Element
{
    float startTime;
    
    float keepTime;
    
    float respondHintSize;
    
    FunctionCurve respondHintProcessCurve;
    
    ColorArgb color;
    
    ColorArgb flashColor;
    
    Sprite[] heartSprites;
    
    NineSliceSprite graphNode;
    
    Vector2 respondPosition;
    
    DremuRespondHint(Vector2 respondPosition, float startTime, Node reference, ColorArgb color1, ColorArgb color2)
    {
        this.startTime = startTime;
        this.keepTime = 0.8;
        this.respondPosition = respondPosition;
        this.respondHintSize = 2.5;
        this.respondHintProcessCurve = new CubicHermiteSpline()
        {
            startPoint : Vector2 : {x : 0.0, y : 0.0},
            startTangent : 0.0,
            startWeight : 0.0,
            endPoint : Vector2 : {x : 1.0, y : 1.0},
            endTangent : 0.1,
            endWeight : 0.8,
        };
        //color = new ColorArgb(1, 0.0196078, 0.7607843, 0.5882353);
        color = color1;
        flashColor = color2;
        
        ITransformer[] transformers = new ITransformer[2];
        transformers[0] = new DremuRespondHintTransformer(this);
        transformers[1] = new DremuRespondHintHeartTransformer(this);
        
        // GorgeGraphics
        ImageAsset lineImage = (ImageAsset) Environment.GetAssetByName("image:Dremu/FormAsset/RespondHint");
        graphNode = new NineSliceSprite(lineImage.texture, new Vector2(169, 169), new Vector2(169, 169), new Vector2(0.2, 0.2));
        graphNode.position.x = respondPosition.x;
        graphNode.position.y = respondPosition.y;
        graphNode.position.z = -1.5;
        graphNode.rotation.z = 45;
        graphNode.size.x = 1;
        graphNode.size.y = 1;
        graphNode.color = color1;
        graphNode.positionReference = reference;
        
        nodes = new Node[17];
        nodes[16] = graphNode;
        
        ImageAsset heartImage = (ImageAsset) Environment.GetAssetByName("image:Dremu/FormAsset/Heart");
        heartSprites = new Sprite[16];
        
        for (int i = 0; i < 16; i = i + 1)
        {
            Sprite heartSprite = new Sprite(heartImage.texture);
            heartSprite.position.x = respondPosition.x;
            heartSprite.position.y = respondPosition.y;
            heartSprite.position.z = -1;
            heartSprite.size.x = 0.3;
            heartSprite.size.y = 0.225;
            heartSprite.color = color1;
            heartSprites[i] = heartSprite;
            nodes[i] = heartSprite;
            heartSprite.positionReference = reference;
        }
        
        simulator = new ElementSimulator(transformers);
    }
    
    @ForwardTimedDestroy
    float ForwardDestroyTime()
    {
        return startTime + keepTime;
    }
    
    @BackwardTimedDestroy
    float BackwardDestroyTime()
    {
        return startTime;
    }
}