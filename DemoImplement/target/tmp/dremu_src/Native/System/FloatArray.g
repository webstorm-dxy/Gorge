namespace Gorge;

native class FloatArray
{
    @Inject
    int length;
    
    FloatArray();
    
    float Get(int index);
    void Set(int index, float value);
}