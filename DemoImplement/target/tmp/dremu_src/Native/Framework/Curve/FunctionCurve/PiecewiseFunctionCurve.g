using Gorge;
namespace GorgeFramework;

native class PiecewiseFunctionCurve : FunctionCurve
{
    [auto defaultValue]
    @Inject<FunctionPiece^[]^>
    FunctionPiece[] functionPieces;
    
    PiecewiseFunctionCurve();
    
    float Evaluate(float x);
}