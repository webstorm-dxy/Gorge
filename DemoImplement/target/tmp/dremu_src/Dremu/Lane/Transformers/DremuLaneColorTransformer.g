using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuLaneColorTransformer :: ITransformer
{
    DremuLane lane;
    
    DremuLaneColorTransformer(DremuLane lane)
    {
        this.lane = lane;
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        if (lane.color == null)
        {
            lane.lineGraphNode.color = new ColorArgb(1, 1, 1, 1);
            return null;
        }
        float t = now - lane.generateTime;
        lane.lineGraphNode.color = lane.color.Evaluate(t);
        return null;
    }
}