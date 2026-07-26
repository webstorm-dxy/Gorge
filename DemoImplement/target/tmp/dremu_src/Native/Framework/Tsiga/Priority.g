using Gorge;
namespace GorgeFramework;

native class Priority
{
    delegate<float:ISignal> getPriority;
    
    Priority(delegate<float:ISignal> getPriority);
}