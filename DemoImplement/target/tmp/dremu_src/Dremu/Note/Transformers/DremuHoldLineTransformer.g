using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuHoldLineTransformer :: ITransformer
{
    DremuHold note;
    
    DremuHoldLineTransformer(DremuHold note)
    {
        this.note = note;
    }
    
    IAutomatonCommand[] Transform(float now)
    {
        int pointCount = note.pointCount;
        Vector2[] curvePoints = new Vector2[pointCount];
        string automatonState = note.automaton.GetState();
        bool isWaiting = automatonState == "Waiting";
        float startDistance;
        if (isWaiting)
        {
            startDistance = note.distance.Evaluate(now - note.hitTime);
        }
        else
        {
            if (now >= note.hitTime)
            {
                startDistance = 0;
            }
            else
            {
                startDistance = note.distance.Evaluate(now - note.hitTime);
            }
        }
        float endDistance = note.endDistance.Evaluate(now - note.hitTime - note.holdTime);
        float dStep = (endDistance - startDistance) / (pointCount - 1);
        float d = startDistance;
        float startTime;
        if (isWaiting)
        {
            startTime = 0;
        }
        else
        {
            startTime = now - note.hitTime;
            if (startTime < 0)
            {
                startTime = 0;
            }
        }
        float endTime = note.holdTime;
        float tStep;
        if (note.truncation)
        {
            tStep = ((endDistance - startDistance) / note.holdLength * note.holdTime) / (pointCount - 1);
        }
        else
        {
            tStep = (endTime - startTime) / (pointCount - 1);
        }
        float t = startTime;
        float mainX = note.position.EvaluateAdd(now - note.hitTime);
        
        float targetProjectionFix = 1;
        if (note.targetProjectionFix)
        {
            targetProjectionFix = Math.SinDeg(note.lane.EvaluateNormalVectorAngle(mainX, now));
        }
        
        for (int i = 0; i < pointCount; i = i + 1)
        {
            float x = mainX + note.holdLine.Evaluate(t) * targetProjectionFix;
            curvePoints[i] = note.lane.EvaluatePointPosition(x, d, now);
            
            d = d + dStep;
            t = t + tStep;
        }
        
        note.holdLineNode.points = curvePoints;
        return null;
    }
}