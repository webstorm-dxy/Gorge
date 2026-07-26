namespace Gorge;

native class ObjectArray<TItem>
{
    @Inject
    int length;
    
    ObjectArray();
    
    TItem Get(int index);
    void Set(int index, TItem value);
}