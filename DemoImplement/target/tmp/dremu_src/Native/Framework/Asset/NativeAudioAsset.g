using Gorge;
namespace GorgeFramework;

native class NativeAudioAsset : AudioAsset
{
    [auto defaultValue]
    @Inject
    Audio audio;
    
    NativeAudioAsset();
    
    // 获取图像
    Audio GetAsset();
    
    bool LoadAsset();
}