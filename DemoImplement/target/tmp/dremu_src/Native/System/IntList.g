namespace Gorge;

native class IntList
{
    @Inject
    int length;
    
    IntList();
    
    int Get(int index);
    void Set(int index, int value);
    void Add(int value);
    void RemoveAt(int index);
}