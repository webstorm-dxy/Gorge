using Gorge;
namespace GorgeFramework;

native class ElementSimulator
{
    ITransformer[] transformers;
    
    ElementSimulator(ITransformer[] transformers);
}