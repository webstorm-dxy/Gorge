using Gorge;
namespace GorgeFramework;

native class ImageAsset : GraphAsset
{
    [auto defaultValue]
    @Inject
    Graph texture;
    
    ImageAsset();
    
    string DescriptorDisplayString();
    
    Graph GetAsset();
    
    bool LoadAsset();
}