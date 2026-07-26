using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuNoteColorTransformer :: ITransformer
{
    DremuNote note;
    
    DremuNoteColorTransformer(DremuNote note)
    {
        this.note = note;
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        string automatonState = note.automaton.GetState();
        if (automatonState == "Accepted" || automatonState == "Denied" || automatonState == "Holding")
        {
            ColorArgb c = note.graphNode.color;
            note.graphNode.color = new ColorArgb(0, c.r, c.g, c.b);
            return null;
        }
        
        if (note.color == null)
        {
            note.graphNode.color = new ColorArgb(1, 1, 1, 1);
            return null;
        }
        float t = now - note.hitTime;
        note.graphNode.color = note.color.Evaluate(t);
        return null;
    }
}