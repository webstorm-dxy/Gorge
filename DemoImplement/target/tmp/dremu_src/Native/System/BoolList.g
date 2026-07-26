namespace Gorge;

native class BoolList
{
    @Inject
    int length;
    
    BoolList();
    
    bool Get(int index);
    void Set(int index, bool value);
    void Add(bool value);
    void RemoveAt(int index);
}