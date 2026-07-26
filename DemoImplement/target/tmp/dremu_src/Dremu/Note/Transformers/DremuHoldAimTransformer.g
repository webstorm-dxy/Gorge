using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuHoldAimTransformer :: ITransformer
{
    DremuHold note;
    
    DremuHoldAimTransformer(DremuHold note)
    {
        this.note = note;
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        // 实时计算hold当前的瞄准点
        float holdingProgress = now - note.hitTime;
        if (holdingProgress < 0)
        {
            holdingProgress = 0;
        }
        else if (holdingProgress > note.holdTime)
        {
            holdingProgress = note.holdTime;
        }
        
        float mainX = note.position.EvaluateAdd(holdingProgress);
        
        float targetProjectionFix = 1;
        if (note.targetProjectionFix)
        {
            targetProjectionFix = Math.SinDeg(note.lane.EvaluateNormalVectorAngle(mainX, now));
        }
        
        float aimX = mainX + note.holdLine.Evaluate(holdingProgress) * targetProjectionFix;
        note.aimPosition = note.lane.EvaluatePointPosition(aimX, 0, now);
        return null;
    }
}