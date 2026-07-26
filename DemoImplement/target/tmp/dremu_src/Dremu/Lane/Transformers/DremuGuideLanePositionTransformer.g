using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuGuideLanePositionTransformer :: ITransformer
{
    DremuGuideLane lane;
    
    DremuGuideLanePositionTransformer(DremuGuideLane lane)
    {
        this.lane = lane;
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        float t = now - lane.generateTime;
        float positionBaseX = lane.position.EvaluateAdd(t);
        float positionBaseY = lane.mainLane.EvaluateLineY(positionBaseX, now);
        
        Vector2 normalVector = lane.mainLane.EvaluateLineNormalVector(positionBaseX, now);
        lane.positionNode.position.x = positionBaseX;
        lane.positionNode.position.y = positionBaseY;
        lane.positionNode.rotation.z = Vector2.Angle(normalVector) - 90;
        
        return null;
    }
}