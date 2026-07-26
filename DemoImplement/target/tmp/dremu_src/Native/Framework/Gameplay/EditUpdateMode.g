using Gorge;
namespace GorgeFramework;

// 编辑更新方式
native enum EditUpdateMode
{
    // 不做任何动态更新
    Static,
    // 如果元素已经生成，则调用@EditReInject注解标记的方法进行更新
    // 如果元素没有生成，则调用@EditTryGenerate注解标记的方法判断是否生成，如是则生成
    // 处理后使用零推进刷新状态
    ReInject,
    // 如果元素已生成，则销毁该元素
    // 调用@EditTryGenerate注解标记的方法判断是否生成，如是则生成
    // 处理后使用零推进刷新状态
    ReGenerate,
    // 重新创建仿真环境，并推进到当前谱面时间
    RePlay
    // TODO 可能考虑提供一些基于时间倒回的方案，倒回到当前元素创生前？
}