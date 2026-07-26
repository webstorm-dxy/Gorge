using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuMainLaneLineTransformer :: ITransformer
{
    DremuMainLane lane;
    
    DremuMainLaneLineTransformer(DremuMainLane lane)
    {
        this.lane = lane;
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        float t = now - lane.generateTime;
        int pointCount = lane.pointCount;
        Vector2[] curvePoints = new Vector2[pointCount];
        float startX = lane.drawStartX.EvaluateAdd(t);
        float endX = lane.drawEndX.EvaluateAdd(t);
        float step = (endX - startX) / (pointCount - 1);
        float x = startX;
        for (int i = 0; i < pointCount; i = i + 1)
        {
            curvePoints[i] = new Vector2(x, lane.EvaluateLaneLine(x, now));
            x = x + step;
        }
        
        lane.lineGraphNode.points = curvePoints;
        return null;
    }
}