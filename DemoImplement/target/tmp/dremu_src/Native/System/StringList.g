namespace Gorge;

native class StringList
{
    @Inject
    int length;
    
    StringList();
    
    string Get(int index);
    void Set(int index, string value);
    void Add(string value);
    void RemoveAt(int index);
}