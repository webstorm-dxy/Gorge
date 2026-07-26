using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuHoldLineColorTransformer :: ITransformer
{
    DremuHold note;
    
    DremuHoldLineColorTransformer(DremuHold note)
    {
        this.note = note;
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        string automatonState = note.automaton.GetState();
        if (automatonState == "Accepted" || automatonState == "Denied")
        {
            ColorArgb c = note.holdLineNode.color;
            note.holdLineNode.color = new ColorArgb(0, c.r, c.g, c.b);
            return null;
        }
        
        if (note.color == null)
        {
            note.holdLineNode.color = new ColorArgb(1, 1, 1, 1);
            return null;
        }
        float t = now - note.hitTime;
        note.holdLineNode.color = note.color.Evaluate(t);
        return null;
    }
}