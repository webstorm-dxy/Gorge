using Gorge;
namespace GorgeFramework;

native class PeriodConfig
{
    [auto defaultValue]
    @Inject
    float timeOffset;
    
    [auto defaultValue]
    @Inject
    float minLength;

    [auto defaultValue]
    @Inject
    bool active;
    
    PeriodConfig();
}