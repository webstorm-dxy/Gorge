namespace Gorge;

native class FloatList
{
    @Inject
    int length;
    
    FloatList();
    
    float Get(int index);
    void Set(int index, float value);
    void Add(float value);
    void RemoveAt(int index);
}