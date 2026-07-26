namespace Gorge;

native class ObjectList<TItem>
{
    @Inject
    int length;
    
    ObjectList();
    
    TItem Get(int index);
    void Set(int index, TItem value);
    void Add(TItem value);
    void RemoveAt(int index);
}