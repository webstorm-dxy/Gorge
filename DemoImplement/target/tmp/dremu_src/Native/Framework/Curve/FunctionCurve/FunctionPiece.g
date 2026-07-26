using Gorge;
namespace GorgeFramework;

native class FunctionPiece
{
    [auto defaultValue]
    @Inject<FunctionCurve^>
    FunctionCurve functionCurve;
    
    [auto defaultValue]
    @Inject
    float startX;
    
    [auto defaultValue]
    @Inject
    float endX;
    
    [auto defaultValue]
    @Inject
    bool leftClosed;
    
    [auto defaultValue]
    @Inject
    bool rightClosed;
    
    FunctionPiece();
}