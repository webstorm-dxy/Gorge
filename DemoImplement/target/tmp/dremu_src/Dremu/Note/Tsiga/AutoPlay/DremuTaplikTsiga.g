using Gorge;
using GorgeFramework;
namespace Dremu;

class DremuTaplikTsiga
{
    TimeStack timeStack;
    InputGraph inputGraph;
    HistoryStack historyStack;
    
    DremuTaplikTsiga(DremuTaplik note, bool isReverse)
    {
        historyStack = new HistoryStack();
        
        DremuLane lane = note.lane;
        
        float missStart = note.hitTime - 0.24;
        float goodStart = note.hitTime - 0.18;
        float perfectStart = note.hitTime - 0.12;
        float bestPerfectStart = note.hitTime - 0.06;
        float bestPerfectEnd = note.hitTime + 0.06;
        float perfectEnd = note.hitTime + 0.12;
        float goodEnd = note.hitTime + 0.18;
        float missEnd = note.hitTime + 0.24;
        float radius = 4;
        float flickDistance = 0.3;
        
        // 第一阶段 -MissStart，无视任何输入（不过滤任何输入，不消耗）直到时间结束，接收或拒绝都进入下一输入
        InputGraphState state0 = new InputGraphState(
            new InputSignalFilter(
                Priority[]:() ->
                {
                    return new Priority[0];
                },
                null,
                new TouchType[0],
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
                    return missStart;
                },
                TimeMode.CatchBefore,
                false,
                false
            ),
            new InputGraphEdge(false, 1, null, false, false, false, null),
            new InputGraphEdge(false, 1, null, false, false, false, null)
        );
        
        // 第二阶段 MissStart-GreatStart，时间内捕获Begin，消耗，接收进入失败，拒绝进入下一输入
        // 处理早点产生的miss
        InputGraphState state1 = new InputGraphState(
            new InputSignalFilter(
                Priority[]:() ->
                {
                    Priority[] priorities = new Priority[2];
                    priorities[0] = new Priority(float:(ISignal signal) -> { return note.hitTime; });
                    priorities[1] = new Priority(float:(ISignal signal) -> { return note.GetAimDistance((TouchSignal) signal); });
                    return priorities;
                },
                null,
                new TouchType[1]{TouchType.Begin},
                bool:(int signalId)->
                {
                    return true;
                },
                bool:(TouchSignal signal) ->
                {
                    return note.GetAimDistance(signal) < radius;
                },
                float:() ->
                {
                    return goodStart;
                },
                TimeMode.CatchBefore,
                true,
                false
            ),
            new InputGraphEdge(true, 0, null, false, false, false, null),
            new InputGraphEdge(false, 1, null, false, false, false, null)
        );
        
        // 第三阶段 GreatStart-GreatEnd，时间内捕获Begin，消耗，接收进入位移，拒绝进入失败
        // 非Miss点击
        InputGraphState state2 = new InputGraphState(
            new InputSignalFilter(
                Priority[]:() ->
                {
                    Priority[] priorities = new Priority[2];
                    priorities[0] = new Priority(float:(ISignal signal) -> { return note.hitTime; });
                    priorities[1] = new Priority(float:(ISignal signal) -> { return note.GetAimDistance((TouchSignal) signal); });
                    return priorities;
                },
                null,
                new TouchType[1]{TouchType.Begin},
                bool:(int signalId)->
                {
                    return true;
                },
                bool:(TouchSignal signal) ->
                {
                    float distance = note.GetAimDistance(signal);
                    if (distance < radius)
                    {
                        note.flickBase = signal.position;
                        return true;
                    }
                    return false;
                },
                float:() ->
                {
                    return goodEnd;
                },
                TimeMode.CatchBefore,
                true,
                false
            ),
            new InputGraphEdge(false, 1, null, false, false, false, null),
            new InputGraphEdge(true, 0, null, true, false, true, "Timeout")
        );
        
        // 第四阶段 -GreatEnd，时间内在滑动目标范围捕获Keep，不消耗，接收进入成功，拒绝进入失败
        // 非Miss点击
        InputGraphState state3 = new InputGraphState(
            new InputSignalFilter(
                Priority[]:() ->
                {
                    Priority[] priorities = new Priority[2];
                    priorities[0] = new Priority(float:(ISignal signal) -> { return note.hitTime; });
                    priorities[1] = new Priority(float:(ISignal signal) -> { return note.GetAimDistance((TouchSignal) signal); });
                    return priorities;
                },
                null,
                new TouchType[1]{TouchType.Keep},
                bool:(int signalId)->
                {
                    return true;
                },
                bool:(TouchSignal signal) ->
                {
                    return note.GetFlickDistance(signal) >= flickDistance;
                },
                float:() ->
                {
                    return goodEnd;
                },
                TimeMode.CatchBefore,
                false,
                false
            ),
            new InputGraphEdge(false, 1, null, true, false, true, "Accepted"),
            new InputGraphEdge(true, 0, null, true, false, true, "Timeout")
        );
        
        InputGraphState[] states = new InputGraphState[4];
        states[0] = state0;
        states[1] = state1;
        states[2] = state2;
        states[3] = state3;
        
        inputGraph = new InputGraph(states, false, false, 0, "Waiting");
        
        timeStack = new TimeStack(false, null);
        
        // Miss由输入图实现
        // Perfect窗口过后，响应变为Good
        timeStack.InitPush(
            new TimeItem(
                float:() ->
                {
                    return perfectEnd;
                },
                true,
                "Good"
            )
        );
        
        // BestPerfect窗口过后，响应变为Perfect
        timeStack.InitPush(
            new TimeItem(
                float:() ->
                {
                    return bestPerfectEnd;
                },
                true,
                "Perfect"
            )
        );
        
        // BestPerfect窗口到来，响应变为BestPerfect
        timeStack.InitPush(
            new TimeItem(
                float:() ->
                {
                    return bestPerfectStart;
                },
                true,
                "BestPerfect"
            )
        );
        
        // Perfect窗口到来，响应变为Perfect
        timeStack.InitPush(
            new TimeItem(
                float:() ->
                {
                    return perfectStart;
                },
                true,
                "Perfect"
            )
        );
        
        // Good窗口到来，响应变为Good
        timeStack.InitPush(
            new TimeItem(
                float:() ->
                {
                    return goodStart;
                },
                true,
                "Good"
            )
        );
        
        // 暂不考虑反转
        /*
            反转情况对应正转路径
            MissStart : 0->1
            GoodStart : Good弹栈
            GoodStart : 1->2 deny
            PerfectStart : Perfect弹栈
            BestPerfectStart : BestPerfect弹栈
            RespondMoment : 2->3
            Accept
        */
        /*
        if(isReverse)
        {
            DeentyLane lane = (DeentyLane)Environment.FindAliveLane(note.laneType, note.laneName);
            inputGraph.GoAcceptEdge(lane.MissRespondStartMoment(note.respondMoment), historyStack);
            timeStack.Pop(lane.GoodRespondStartMoment(note.respondMoment), historyStack);
            inputGraph.GoDenyEdge(lane.GoodRespondStartMoment(note.respondMoment), historyStack);
            timeStack.Pop(lane.PerfectRespondStartMoment(note.respondMoment), historyStack);
            timeStack.Pop(lane.BestPerfectRespondStartMoment(note.respondMoment), historyStack);
            inputGraph.GoAcceptEdge(note.respondMoment, historyStack);
        }
        */
    }
}