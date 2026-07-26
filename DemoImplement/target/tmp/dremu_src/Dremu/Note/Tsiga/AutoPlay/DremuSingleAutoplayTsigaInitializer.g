using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuSingleAutoplayTsigaInitializer
{
    TimeStack timeStack;
    InputGraph inputGraph;
    HistoryStack historyStack;
    
    DremuSingleAutoplayTsigaInitializer(DremuNote note, bool isReverse)
    {
        historyStack = new HistoryStack();
        timeStack = new TimeStack(true, "BestPerfect");
        
        // 在hitTIme时间点上响应，并且将状态切换为Accepted
        InputSignalFilter filter1 = new InputSignalFilter(
            Priority[]:() ->
            {
                return new Priority[0]{,};
            },
            null,
            new TouchType[0]{,},
            bool:(int signalId)->
            {
                return true;
            },
            bool:(TouchSignal signal) ->
            {
                return false;
            },
            float:() ->
            {
                return note.hitTime;
            },
            TimeMode.CatchBefore,
            false,
            false
        );
        
        InputGraphEdge acceptedEdge1 = new InputGraphEdge(false, 1, null, true, false, true, "Accepted");
        
        InputGraphEdge deniedEdge1 = new InputGraphEdge(false, 1, null, true, false, true, "Accepted");
        
        InputGraphState state1 = new InputGraphState(filter1, acceptedEdge1, deniedEdge1);
        
        InputGraphState[] states = new InputGraphState[1];
        states[0] = state1;
        
        inputGraph = new InputGraph(states, false, false, 0, "Waiting");
        
        if (isReverse)
        {
            inputGraph.GoAcceptEdge(note.hitTime, historyStack);
        }
    }
}