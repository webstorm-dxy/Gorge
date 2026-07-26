using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuNativeResources
{
    @InstantAudio(name = "RespondA")
    static AudioAsset DremuNativeResourcesTap()
    {
        return (AudioAsset) Environment.GetAssetByName("audio:Dremu/FormAsset/HitSong0");
    }
    
    @InstantAudio(name = "RespondB")
    static AudioAsset DremuNativeResourcesDrag()
    {
        return (AudioAsset) Environment.GetAssetByName("audio:Dremu/FormAsset/HitSong1");
    }
    
    @InstantAudio(name = "RespondC")
    static AudioAsset DremuNativeResourcesTaplik()
    {
        return (AudioAsset) Environment.GetAssetByName("audio:Dremu/FormAsset/HitSong2");
    }
}