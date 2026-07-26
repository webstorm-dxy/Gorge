using Gorge;
namespace GorgeFramework;

native class VideoAsset : Asset
{
    VideoAsset();
    
    // 获取音频
    Video GetAsset();
    
    // 加载资源
    bool LoadAsset();
}