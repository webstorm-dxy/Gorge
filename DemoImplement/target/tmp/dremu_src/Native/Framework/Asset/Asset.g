using Gorge;
namespace GorgeFramework;

native class Asset
{
    @Inject
    string name;
    
    Asset();
    
    // 加载资源
    bool LoadAsset();
}