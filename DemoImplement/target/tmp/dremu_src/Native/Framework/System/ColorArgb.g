using Gorge;
namespace GorgeFramework;

native class ColorArgb
{
    [auto defaultValue]
    @Inject
    float a;
    [auto defaultValue]
    @Inject
    float r;
    [auto defaultValue]
    @Inject
    float g;
    [auto defaultValue]
    @Inject
    float b;
    
    ColorArgb();
    
    ColorArgb(float a, float r, float g, float b);
}