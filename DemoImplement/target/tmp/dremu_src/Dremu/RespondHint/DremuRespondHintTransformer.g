using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuRespondHintTransformer :: ITransformer
{
    DremuRespondHint respondHint;
    
    float expandSpeed = 5.0;
    
    float alphaChangeSpeed = 0.2;
    
    DremuRespondHintTransformer(DremuRespondHint respondHint)
    {
        this.respondHint = respondHint;
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        float curveTime = (now - respondHint.startTime) / respondHint.keepTime;
        float process = respondHint.respondHintProcessCurve.Evaluate(curveTime);
        float scale = respondHint.respondHintSize;
        
        float newScale = Math.Atan(process * respondHint.keepTime * expandSpeed) * 2 * scale / Math.Pi();
        
        respondHint.graphNode.size.x = newScale;
        respondHint.graphNode.size.y = newScale;
        
        ColorArgb color = respondHint.color;
        respondHint.graphNode.color = new ColorArgb(Math.Atan(((1 - process) * respondHint.keepTime) / alphaChangeSpeed) * 2 / Math.Pi(), color.r, color.g, color.b);
        
        return null;
    }
}