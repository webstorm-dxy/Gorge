namespace Gorge;

native class IntArray
{
    @Inject
    int length;
    
    IntArray();
    
    int Get(int index);
    void Set(int index, int value);
}