using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuHoldAutoplayTsigaInitializer
{
    TimeStack timeStack;
    InputGraph inputGraph;
    HistoryStack historyStack;
    
    DremuHoldAutoplayTsigaInitializer(DremuHold note, bool isReverse)
    {
        historyStack = new HistoryStack();
        
        // 在HitTime时间点上响应，并且将状态切换为Holding
        InputGraphState state0 = new InputGraphState(
            new InputSignalFilter(
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
            ),
            new InputGraphEdge(false, 1, null, false, true, false, "Holding"),
            new InputGraphEdge(false, 1, null, false, true, false, "Holding")
        );
        
        // 在FinishMoment时间点上响应，并且将状态切换为Accepted
        InputGraphState state1 = new InputGraphState(
            new InputSignalFilter(
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
                    return note.hitTime + note.holdTime;
                },
                TimeMode.CatchBefore,
                false,
                false
            ),
            new InputGraphEdge(false, 1, null, true, true, false, "Accepted"),
            new InputGraphEdge(false, 1, null, true, true, false, "Accepted")
        );
        
        InputGraphState[] states = new InputGraphState[2];
        states[0] = state0;
        states[1] = state1;
        
        inputGraph = new InputGraph(states, true, true, 0, "Waiting");
        
        timeStack = new TimeStack(false, null);
        
        // 中间响应倒序压栈
        if (note.innerNoteConfigs != null)
        {
            for (int i = note.innerNoteConfigs.length - 1; i >= 0; i = i - 1)
            {
                if (note.innerNoteConfigs[i] != null)
                {
                    DremuHoldInnerNote innerNote = note.innerNoteConfigs[i];
                    string respond = "BestPerfect";
                    timeStack.InitPush(
                        new TimeItem(
                            float:() ->
                            {
                                return note.hitTime + innerNote.hitTime;
                            },
                            false,
                            respond
                        )
                    );
                }
            }
        }
        
        timeStack.InitPush(
            new TimeItem(
                float:() ->
                {
                    return note.hitTime;
                },
                true,
                "BestPerfect"
            )
        );
        
        if (isReverse)
        {
            timeStack.Pop(note.hitTime, historyStack);
            inputGraph.GoAcceptEdge(note.hitTime, historyStack);
            if (note.innerNoteConfigs != null)
            {
                for (int i = 0; i < note.innerNoteConfigs.length; i = i + 1)
                {
                    if (note.innerNoteConfigs[i] != null)
                    {
                        timeStack.Pop(note.innerNoteConfigs[i].hitTime, historyStack);
                    }
                }
            }
            
            inputGraph.GoAcceptEdge(note.hitTime + note.holdTime, historyStack);
        }
    }
}