using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuMainLanePositionTransformer :: ITransformer
{
    DremuMainLane lane;
    
    DremuMainLanePositionTransformer(DremuMainLane lane)
    {
        this.lane = lane;
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        float t = now - lane.generateTime;
        
        lane.positionNode.position = new Vector3(
            lane.positionX.EvaluateAdd(t),
            lane.positionY.EvaluateAdd(t),
            -1.0
        );
        
        lane.positionNode.rotation = new Vector3(
            0.0,
            0.0,
            lane.rotationZ.EvaluateAdd(t)
        );
        
        lane.positionNode.size = new Vector3(
            1.0,
            1.0,
            1.0
        );
        
        return null;
    }
}