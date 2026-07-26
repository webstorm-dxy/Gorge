using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuSingleAimTransformer :: ITransformer
{
    DremuNote note;
    
    DremuSingleAimTransformer(DremuNote note)
    {
        this.note = note;
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        float x = note.position.EvaluateAdd(0);
        note.aimPosition = note.lane.EvaluatePointPosition(x, 0, now);
        return null;
    }
}