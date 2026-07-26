using Gorge;
namespace GorgeFramework;

native class AudioAsset : Asset
{
    AudioAsset();
    
    // 获取音频
    Audio GetAsset();
    
    // 加载资源
    bool LoadAsset();
}