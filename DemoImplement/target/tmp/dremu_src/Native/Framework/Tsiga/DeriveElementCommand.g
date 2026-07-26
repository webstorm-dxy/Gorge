using Gorge;
namespace GorgeFramework;

// 由Note向自动机发出的，派生element的指令
native class DeriveElementCommand :: IAutomatonCommand
{
    Element element;
    
    bool changeAutomaton;
    
    DeriveElementCommand(Element element, bool changeAutomaton);
}