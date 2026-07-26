namespace Gorge;

native class BoolArray
{
    @Inject
    int length;
    
    BoolArray();
    
    bool Get(int index);
    void Set(int index, bool value);
}