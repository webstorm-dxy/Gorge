using Gorge;
namespace GorgeFramework;

native class TimeStack
{
    TimeStack(bool accept, string respondMode);
    
    void InitPush(TimeItem timeItem);
    
    TimeItem Pop(float chartTime, HistoryStack historyStack);
    
    void Push(float chartTime, TimeItem timeItem, HistoryStack historyStack);
}