using Gorge;
namespace GorgeFramework;

// 输入图的一条边
native class InputGraphEdge
{
    // 是否直接进入失败停机
    bool deny;
    
    // 跳转步数
    // deny为true则不跳转
    int jump;
    
    // 跳转时栈操作
    // deny为true则不执行
    // 第三个参数是谱面时间
    delegate<void:Note,TimeStack,float,HistoryStack> stackAction;
    
    // 是否进入接收模式
    bool accept;
    
    // 是否进入弹栈响应模式
    bool stackRespond;
    
    // 是否在跳转时尝试响应，具体发生的响应内容由时间栈确定
    bool edgeRespond;
    
    // 进入的导出状态，如果为null则保持原状
    string exportState;
    
    InputGraphEdge(bool deny, int jump, delegate<void:Note,TimeStack,float,HistoryStack> stackAction, bool accept, bool stackRespond, bool edgeRespond, string exportState);
}