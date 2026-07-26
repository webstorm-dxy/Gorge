using Gorge;
namespace GorgeFramework;

native class NativeVideoAsset : VideoAsset
{
    [auto defaultValue]
    @Inject
    Video video;
    
    NativeVideoAsset();
    
    // 获取图像
    Video GetAsset();
    
    bool LoadAsset();
}