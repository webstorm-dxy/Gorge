using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuRespondHintHeartTransformer :: ITransformer
{
    DremuRespondHint respondHint;
    
    FunctionCurve progressCurve = new CubicHermiteSpline()
    {
        startPoint : Vector2 : {x : 0.0, y : 0.0},
        startTangent : 0.0,
        startWeight : 0.0,
        endPoint : Vector2 : {x : 1.0, y : 1.0},
        endTangent : 0.1,
        endWeight : 0.8,
    };
    
    Vector2[] directions = new Vector2[16];
    float[] distances = new float[16];
    float[] finalSizes = new float[16];
    float[] keepTimes = new float[16];
    
    DremuRespondHintHeartTransformer(DremuRespondHint respondHint)
    {
        this.respondHint = respondHint;
        
        for (int i = 0; i < 16; i = i + 1)
        {
            directions[i] = Random.RandomNormalized();
            distances[i] = Random.RandomFloat(1.2, 2.5);
            finalSizes[i] = Random.RandomFloat(0.0, 0.2);
            keepTimes[i] = Random.RandomFloat(0.3, 0.8);
        }
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        float t = now - respondHint.startTime;
        for (int i = 0; i < 16; i = i + 1)
        {
            float curveTime = t / keepTimes[i];
            Vector2 direction = directions[i];
            float distance = distances[i];
            float progress = progressCurve.Evaluate(curveTime);
            float finalSize = finalSizes[i];
            Sprite nowSprite = respondHint.heartSprites[i];
            nowSprite.position.x = direction.x * distance * progress + respondHint.respondPosition.x;
            nowSprite.position.y = direction.y * distance * progress + respondHint.respondPosition.y;
            nowSprite.size.x = Math.Lerp(0.3, finalSize, curveTime);
            nowSprite.size.y = Math.Lerp(0.225, finalSize * 0.75, curveTime);
            
            if (curveTime >= 1)
            {
                ColorArgb color = nowSprite.color;
                if (color.a != 0.0)
                {
                    nowSprite.color = new ColorArgb(0.0, color.r, color.g, color.b);
                }
            }
            else if (curveTime > 0.93)
            {
                nowSprite.color = respondHint.flashColor;
            }
        }
        
        return null;
    }
}