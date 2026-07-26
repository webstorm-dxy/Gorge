using Gorge;
using GorgeFramework;
namespace Dremu;

@LateIndependent
class DremuGuideLaneLineTransformer :: ITransformer
{
    DremuGuideLane lane;
    
    DremuGuideLaneLineTransformer(DremuGuideLane lane)
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
        
        float targetProjectionFix = 1;
        if (lane.targetProjectionFix)
        {
            float mainX = lane.position.EvaluateAdd(t);
            targetProjectionFix = Math.SinDeg(lane.mainLane.EvaluateNormalVectorAngle(mainX, now));
        }
        
        for (int i = 0; i < pointCount; i = i + 1)
        {
            // 根据实际表现可能可以考虑投射到切线上，但这是额外的计算量，并且没有直观的道理
            // 可能直观道理在于从曲线形状做较轻微的形变
            float mainBaseX = lane.EvaluateLaneLine(x, now) * targetProjectionFix;
            curvePoints[i] = lane.mainLane.EvaluatePointPosition(mainBaseX, x, now);
            x = x + step;
        }
        
        lane.lineGraphNode.points = curvePoints;
        return null;
    }
}