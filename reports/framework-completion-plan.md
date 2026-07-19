# Framework 补全分步实现方案（2026-07-18 确认）

> 背景：C# 66 native 类 → Rust 已注册 41，缺 25；8 个模块骨架逻辑未实现；三大基础设施阻塞点。
> 本方案经用户确认四项关键决策后定稿，执行顺序：**S1→S2→S3→S4→S7 主线，S5/S6 穿插**。

## 已确认的关键决策

| 决策项 | 选定方案 | 说明 |
|--------|---------|------|
| native→VM 委托重入 | **NativeContext 持 `vm: &mut VirtualMachine`** | 其余 API 内部经 vm 派生访问；外部签名不变、宏生成代码零改动；无 unsafe |
| 执行顺序 | **基础设施优先** | S1→S2→S3→S4→S7 主线推进，S5/S6 间隙穿插 |
| wav 解码 | **平台 trait 后置** | CreateAudio 归入 PlatformBase trait，headless 实现只记录路径不解码 |
| SignalDetectionCondition | **数据化结构体** | 存 filter/tsiga 对象 ID + 上下文数据，调用点解释执行（替代 C# 闭包） |

## 依赖关系

```
S1 委托执行引擎 ──┬─→ S2 集合/委托字段模式 ──→ S7 自动机全链路
                  ├─→ S3 方法注解+Injector反射 ──→ S4 Runtime核心逻辑 ──→ S7
                  └─────────────────────────────────────────────────────↗
S5 曲线/变换/工具类（无依赖，可并行穿插）
S6 平台后端 trait（独立，可并行穿插）
快赢：S4a ScoringV1 / S4b AutomatonManager 三方法（纯逻辑，可随时插入）
```

---

## S1 委托执行引擎（阻塞点①，~1 周）

| 子步 | 内容 |
|------|------|
| 1a | `vm.rs` 提取统一辅助 `call_compiled_method(method, params, ret_type, result_addr)`：保存栈+5 类 return 寄存器 → 布置参数 → 内联 `execute_one` 循环 → 写回返回值 → 恢复。重构 8 处重复内联点（InvokeInstance/Static/Interface/Delegate/Constructor/Super/InjectorCtor/字段初始化器，vm.rs 1232/1314/1415/1503/1657/1764/1844/1612） |
| 1b | 修复 `ConstructDelegate`（vm.rs:871 创建 RuntimeDelegate 后丢弃）：新增 `vm.runtime_delegates: HashMap<对象ID, RuntimeDelegate>`；捕获值从当前栈真实填充 `captured_values`（delegate.rs from_def 现忽略 outer_values） |
| 1c | 新增 `vm.invoke_delegate_object(delegate_obj_id)`：按对象 ID 查委托 → call_compiled_method（现状只能按类名查 class_delegate_impls 编译时 Lambda） |
| 1d | NativeContext 重构为持 `vm: &mut VirtualMachine`：全部现有 API（参数/返回值/字段/对象/注入器/跨对象调用）内部改经 vm 字段实现，外部签名不变；`invoke_delegate` 真实现 |

**验收**：`.g` 定义 Lambda 赋给 native 类委托字段（usize），native 方法内同步 Invoke 拿 float/object 返回值；全量回归零 warning。

## S2 集合/委托字段模式（阻塞点②，~5 天）

核心洞察：委托字段、ObjectArray 字段在 Gorge 类型系统里都是 object → `usize` 对象 ID，**宏已支持**（先例 element_native.rs nodes）；泛型集合是 C# 私有实现细节 → 用现成 `native_payloads: HashMap<usize, Box<dyn Any>>` 存 per-object 内部状态。**无需大改宏**。

| 子步 | 内容 |
|------|------|
| 2a | NativeContext 集合便捷 API：`object_array_items(id) -> Vec<usize>`、`object_array_len/get/add` |
| 2b | payload 模式打通：构造时 `native_payloads.insert(this, Box::new(内部状态))`，方法内 downcast 读写 |
| 2c | 注册 7 类：SignalFilter、InputSignalFilter（委托字段=usize：priority/endTime/onDetected/signalIdFilter/touchArea）、InputGraph、InputGraphState、HistoryStack、TimeStack（payload 集合）、ElementSimulator（transformers ObjectArray 构造时拷贝） |
| 2d | CanDetect/Detect 虚分派：调用点按对象实际类名 `invoke_native_method_on` 分派（Step 0a 已有） |

**验收**：InputGraph 状态机（DoTimeout/GoAccept/GoDeny/RevertGoEdge）、HistoryStack PopUntil、TimeStack Push/Pop/Revert 与 C# 语义对齐单测。

## S3 方法注解序列化 + Injector 反射（阻塞点③，~1 周）

| 子步 | 内容 |
|------|------|
| 3a | 编译器：方法/构造方法新增 `annotations: Vec<AnnotationInfo>`（名字+参数键值）序列化进字节码（现状仅类级 CompiledClass.annotations） |
| 3b | metadata 委托：`@ForwardTimedGenerate(time=...)` 的表达式编译成隐藏静态方法，注解引用其方法 ID（对齐 C# TryGetParameter 返回 GorgeDelegate） |
| 3c | 运行时查询 API：`methods_with_annotation(class, name) -> Vec<(method_id, 参数)>` + `invoke_method_by_id` |
| 3d | `Injector.Instantiate`：VM 新增 `instantiate_with_injector(class, ctor_id, injector_id)`（复用 InvokeInjectorConstructor 链路），经 NativeContext 暴露（依赖 S1d） |

**验收**：`.g` 类方法带 `@ForwardTimedDestroy` → Rust 扫描到注解、调用方法取 time；Injector 实例化端到端。

## S4 Runtime 核心逻辑（依赖 S1+S3，~1 周）

| 子步 | 内容 |
|------|------|
| 4a | **ScoringV1 公式**（快赢零依赖）：`clamp(sqrt(700000*(comboBonus/maxComboBonus) + 300000*(accBonus/maxAccBonus)^10)*1000, 0, 10^6) + bestPerfect数*1`；判定奖励 Miss=0/Good=50/Perfect=100/BestPerfect=100；maxComboBonus=(n+1)n/2；Accuracy=accBonus/(100*总判定数) |
| 4b | **AutomatonManager 三方法**（快赢纯逻辑）：AddSignalEdge（无信道建 ChannelSplit；null 值终止信号；同值未过期不追加/过期延续；异值追加 Edge）、SplitInputSignals（遍历 Fragment.Split(from,to)）、GetInputSignalEarliestEdgeTimeAfter（min 大于 t 的边沿时间，无则 f32::MAX） |
| 4c | `do_action` 签名重构：`&dyn SimulationContext` → `&mut GorgeSimulationRuntime`（C# 先收集 actionQueue 再执行，规避借用冲突） |
| 4d | GameplayAction 三 DoAction（Injector.Instantiate + AutowiredArguments(isAutoPlay/isReverse) + 注解扫描 ForwardTimedDestroy/BackwardTimedDestroy/DeriveGenerate + AliveElements/模拟器/自动机/图形节点登记）；ChartManager AddScoreElement（Modify→Clone Injector→InvokeStaticMethod(PeriodModifier)→填 Initialize/ForwardTimed/BackwardTimedGenerateList，TimedGenerateElement.Time 惰性 Invoke）；SimulationMachine 真实分派（复合步/零步长两阶段收集 actions 后执行）+ LateIndependentSimulators（每次 Drive 结束后 InstantSimulate，返回值丢弃） |

**验收**：集成测试"谱面加载 → 定时生成元素 → 推进 → 定时销毁"。

## S5 曲线/变换/工具类（无依赖穿插，~3 天）

- 扩展跨对象便捷方法：`call_native_method_object/int/bool`（现仅 float）
- **LerpColorCurve**（ColorArgb.Lerp + colorPoints ObjectArray + progressCurve）
- **AnnulusMeshTransformer**（xAngle/yRadius 曲线 + sin/cos 极坐标→Vector3）
- **CurveMeshTransformer**（curve+isHorizontal，顶点沿曲线偏移）
- **CurveWarpTransformer**（切线 Vector2::normalize/曲率 SignedAngle/法线偏移，需补 Vector2 数学函数）
- **FloatExtension**（f32::to_bits as i32）、**StackExtension**（Rust Option 天然）→ utilities 模块

**验收**：固定输入输出与 C# 数值对齐单测。

## S6 平台后端（独立穿插，~5 天）

- `adaptor/` trait 族：`ISprite`（SetPosition/SetRotation/SetScale/SetColor/SetGraph/Destroy）、`INineSliceSprite`（+SetHsl）、`ICurveSprite`（+SetLine/SetWidth）、`IAudioPlayer`、`PlatformBase`（CreateAudio/CreateSprite/CreateNineSliceSprite/CreateCurveSprite）+ **Headless 实现**（记录调用序列供断言）
- 三批注册 13 类：纯数据 5 类（Asset/GraphAsset/ImageAsset/NativeAudioAsset/NativeVideoAsset）→ 资源查找 3 类（AudioAsset/VideoAsset 走 Environment.GetAssetByName；WavAudioAsset 的解码后置到平台 trait）→ 渲染包装 5 类（Audio/Video/Sprite/NineSliceSprite/CurveSprite）

**验收**：headless 后端断言 UpdateNode 产生的 SetPosition/SetColor 调用序列。

## S7 自动机全链路（依赖 S1+S2，~1 周）

- SignalTsiga 注册为 native（现纯 Rust struct，字段 input_graph/time_stack/history_stack 改为对象 ID + payload）
- GetDetectionConditions 完整化：priority.Invoke → ObjectArray of Priority；Detect 包装维护信号记录（value/lastValue）；Accept 按 timeMode==CatchBefore 走 GoAcceptEdge→DoEdgeRespond
- SignalDetectionCondition 数据化结构体（决策已定）：持 filter/tsiga 对象 ID + 方向等上下文，PreciseAutomatonSimulator 调用点解释
- PreciseAutomatonSimulator：InstantSimulate（CanDetect 过滤→Detect→接受/拒绝→consume 标记→DetectionDeny）+ Forward/Backward/Infinitesimal 四方向（经 runtime.Automaton.Automatons 间接访问，不持有 SignalTsiga 字段）
- **PendingDetectionConditions**：`Dictionary<SignalTsiga, List<SignalDetectionCondition>>` → Rust `HashMap<usize, Vec<SignalDetectionCondition>>`

**验收**：端到端"输入信号 → 检测 → 判定 → ScoringV1 计分"。

---

## C# 参考实现关键情报（探索报告摘要）

### 委托字段清单（S2 注册用）
- SignalFilter：priority/endTime（GorgeDelegate）+ conditionTypes(IntArray)/timeMode(int)/acceptConsume/denyConsume(bool)
- InputSignalFilter 额外：onDetected `(int,TouchSignal)->void`、signalIdFilter `(int)->bool`、touchArea `(TouchSignal)->bool`
- FloatSignalFilter 额外：filterRange(GorgeDelegate)+channelName(string)
- TimeItem.time 是 GorgeDelegate（当前 Rust 注册版是 f32 字段，**需改**）

### 集合方法签名（S2 实现用）
- InputGraph：State/StateTimeout（endTime.Invoke）/Accept/StackRespond/ExportState/InputPointer/StateCount + DoTimeout/GoAcceptEdge/GoDenyEdge(chartTime, HistoryStack)->InputGraphEdge + RevertGoEdge
- HistoryStack：RevertTime + Push(IHistoryItem) + PopUntil(targetTime, automaton, direction, inputGraph, timeStack)->IGameplayAction[]
- TimeStack：Accept/RespondMode/PopTime + TryPop/Pop/Push(chartTime, TimeItem, HistoryStack) + InitPush/RevertPop/RevertPush

### 注解清单（S3 实现用）
- 构造注解：@InitializeGenerate、@ForwardTimedGenerate(metadata time 委托)、@BackwardTimedGenerate
- 方法注解：@ForwardTimedDestroy、@BackwardTimedDestroy、@DeriveGenerate
- 静态方法注解：@Form(name, version)（返回 StringArray 元素类型表）、@PeriodModifier（签名 (Injector, PeriodConfig)->void）、@InstantAudio(name)
- C# 全部经 `Declaration.Methods[i].Annotations` 遍历，不用 .NET 反射

### 必须注册 native / 保留 Rust 内部的分界
- **必须 native**：被 .g 直接 new/extends/字段引用的类（Element/Note/Node/Sprite 族/ElementSimulator/SignalFilter 族/InputGraph 族/TimeStack/HistoryStack/SignalTsiga/曲线族/Transformer 族/Asset 族/Command 族/Vector/Color 等）
- **保留 Rust 内部**：SimulationMachine/各 Manager/PreciseAutomatonSimulator/IGameplayAction 实现/SignalDetectionCondition/ScoringV1/RuntimeFormContainer/信号数据结构（Fragment/Edge/ChannelSplit）/工具扩展

### Rust 现状要点（第二份探索报告）
- NativeContext.invoke_delegate 空占位（native.rs:365）；invoke_native_method_on/call_native_method_float_f 已可用
- VM 无统一 call_compiled_method；8 处内联重复
- ConstructDelegate 创建 RuntimeDelegate 后丢弃（vm.rs:871）
- 宏字段仅支持 i32/i64/f32/f64/bool/String/usize，非法类型直接报错；不支持也不需要支持 Vec/HashMap
- do_action 签名只有 &dyn SimulationContext，接不到各 Manager（S4c 重构）
- run_all_simulators/instant_simulate_all/late_independent_simulate 遍历但不调用（simulation_machine.rs 159-206）
- ChartManager 四个定时列表已声明无填充；AutomatonManager.add_signal_edge 占位返回 false
- ScoringV1.score() 硬编码返回 0
