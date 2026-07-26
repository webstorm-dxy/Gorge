using Gorge;
namespace GorgeFramework;

native class Element
{
    ElementSimulator simulator;
    
    ElementSimulator lateIndependentSimulator;
    
    Node[] nodes;
    
    Element[] derivedElements;

    Element();
}