using Gorge;
namespace GorgeFramework;

// 网格变换器
native interface IMeshTransformer
{
    Vector3 Transform(Vector3 vertex);
}