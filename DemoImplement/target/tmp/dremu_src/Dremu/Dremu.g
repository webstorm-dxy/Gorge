using Gorge;
using GorgeFramework;
namespace Dremu;

class Dremu
{
    @Form(name = "Dremu", version = "0.1")
    static string[] ElementTypeList()
    {
        return new string[6]{"Dremu.DremuTap", "Dremu.DremuDrag", "Dremu.DremuTaplik", "Dremu.DremuHold", "Dremu.DremuMainLane", "Dremu.DremuGuideLane"};
    }
}