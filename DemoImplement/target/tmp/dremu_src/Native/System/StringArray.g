namespace Gorge;

native class StringArray
{
    @Inject
    int length;
    
    StringArray();
    
    string Get(int index);
    void Set(int index, string value);
}