using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuSingleNotePositionTransformer :: ITransformer
{
    DremuNote note;
    
    DremuSingleNotePositionTransformer(DremuNote note)
    {
        this.note = note;
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        float t = now - note.hitTime;
        float positionBaseX = note.position.EvaluateAdd(t);
        float distance = note.distance.Evaluate(t);
        
        Vector2 position = note.lane.EvaluatePointPosition(positionBaseX, distance, now);
        float rotation = note.lane.EvaluatePointRotation(positionBaseX, distance, now);
        
        note.graphNode.position.x = position.x;
        note.graphNode.position.y = position.y;
        note.graphNode.rotation.z = rotation;
        
        return null;
    }
}