using Gorge;
namespace GorgeFramework;

// 由Note向自动机发出的，销毁element的指令
native class DestroyElementCommand :: IAutomatonCommand
{
    Element element;
    
    bool changeAutomaton;
    
    DestroyElementCommand(Element element, bool changeAutomaton);
}