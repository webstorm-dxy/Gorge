using Gorge;
namespace GorgeFramework;

native class WavAudioAsset : AudioAsset
{
    [auto defaultValue]
    @Inject
    string wavFilePath;
    
    NativeAudioAsset();
    
    // 获取图像
    Audio GetAsset();
    
    bool LoadAsset();
}