using Gorge;
namespace GorgeFramework;

native class ColorCurve
{
    injector ColorCurve();
    
    ColorArgb Evaluate(float x);
}