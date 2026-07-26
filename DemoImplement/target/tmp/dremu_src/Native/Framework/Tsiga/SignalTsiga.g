using Gorge;
namespace GorgeFramework;

native class SignalTsiga
{
    SignalTsiga(Note note, TimeStack timeStack, InputGraph inputGraph, HistoryStack historyStack);
    
    string GetState();
}