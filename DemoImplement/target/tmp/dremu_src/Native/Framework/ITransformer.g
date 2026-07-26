using Gorge;
namespace GorgeFramework;

native interface ITransformer
{
    IAutomatonCommand[] Transform(float now);
}