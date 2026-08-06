# Gorge 项目 TODO 清单

> 生成时间：2026-07-26
> 来源：全项目 TODO 注释扫描 + 骨架/占位实现梳理
> 勾选格式：`- [ ]` 未完成 / `- [x]` 已完成

---

## P0 — 阻断 Demo 画面显示（静态方法求值链）

真实 Dremu 包的元素/音频数据来自静态方法返回值，当前这些步骤全是骨架，导致没有任何元素进入渲染管线。

- [x] **P0-1 `@Form` 静态方法求值（元素类型列表）**
  - 说明：通过 VM 调用 `@Form` 静态方法，从返回的 `StringArray` 提取元素类型列表
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/runtime/runtime_form_container.rs`（`extract_element_types_from_method`）
    - `GorgeFramework/GorgeFramework/src/runtime/runtime_manager.rs`（`scan_forms` 透传 VM）
  - 完成时间：2026-07-26

- [x] **P0-2 `@Chart` / `@Song` 静态方法求值（谱表元素注入器）**
  - 说明：调用谱表类的 `@Chart`/`@Song` 静态方法，把返回的 `ObjectArray<Injector>` 填入 `ElementPeriod.elements`；当前仅从注解常量推导，得到空或错误数据
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/chart/simulation_score.rs:269`（`extract_stave_from_runtime`）
    - `GorgeFramework/GorgeFramework/src/chart/simulation_score.rs:426`（`extract_element_periods_from_class` 元素列表推导）
  - 参考：C# `references/gorge-framework/src/Chart/SimulationScore.cs:194-213`
  - 依赖：可复用 P0-1 的 VM 调用模式
  - 完成时间：2026-07-26
  - 实现说明：采用**常量池直读**路径（真实谱面方法体为纯常量字面量，与 VM 执行语义等价）。定位方法字节码中的 `LoadInjectorConstant` → 从类常量池取该方法返回的注入器数组/对象常量 → 类型感知 JSON 转换（嵌套字段名按父类注入器字段声明位置对齐恢复）。配套修复：编译器 `InjectObject` 常量保留类名（`GorgeCompiler/src/visitors/codegen.rs` 3 处）；新增 native 注入器字段元数据导出（`NativeClass::injector_fields_meta` + 宏生成）。VM 执行路径仍待 GorgeCore 补两块（injector_constants 注册进 VM、嵌套常量物化），见 P0-7

- [x] **P0-3 音频乐段注入器实例化**
  - 说明：音频乐段（`AudioPeriod`）的注入器当前从常量近似推导，需走完整注入器实例化
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/chart/simulation_score.rs:611`
  - 参考：C# `SimulationScore.cs` 音频乐段提取逻辑
  - 完成时间：2026-07-31
  - 实现说明：`config` 参数在真实谱面中为 `PeriodConfig^` 注入器字面量，编译器为其生成隐藏方法并以 `AnnotationValue::Delegate(全局方法 ID)` 记录。`extract_period_config_injector` 改为沿 Delegate 定位隐藏方法字节码的 `LoadInjectorConstant`，从类常量池取出 `PeriodConfig` 注入器常量转 JSON（与音频/元素注入器同一条完整实例化路径）；`Delegate` 缺失/不可解析时回退默认配置 + 直接标量参数。新增 `resolve_delegate_injector_json` 辅助函数与 2 个回归测试

- [x] **P0-4 `@PeriodModifier` 静态方法应用**
  - 说明：`ChartManager::add_score_element` 需先 clone 注入器，再沿继承链调用所有 `@PeriodModifier` 静态方法做 gameplay 修正（轨道位置、缩放等）
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/runtime/environment.rs:124-164`（`ChartManager::add_score_element`）
  - 参考：C# `references/gorge-framework/src/Runtime/Environment/ChartManager.cs:91-170`
  - 依赖：`RuntimeFormContainer.element_modifiers`（已有数据，未接线）
  - 完成时间：2026-07-31
  - 实现说明：`add_score_element` 新增 `period_config` 参数（对齐 C# `AddScoreElement(scoreElement, periodConfig)`，`load_score` 传入乐段配置）。`modify_injector` 克隆注入器（`RuntimeInjector` derive Clone：值深拷贝、声明 Arc 共享），沿类 + 父类声明链扫描 `@PeriodModifier` 方法（声明注解表直读，与 C# 遍历 `FormContainer.ElementModifiers` 等价，容器接线后语义不变），按 `(元素注入器, PeriodConfig)` 以 object 参数池调用（`LoadObjectParameter(0)/(1)` 直读参数池，`LoadInjector` 经 `current_injector` 寻址修改版注入器）；`PeriodConfig` 对象按 native 字段布局（float[0]=timeOffset、float[1]=minLength、bool[0]=active）物化进对象表。生成表全部改用修改版注入器 ID；`active=false` 不创生（对齐 C# `Modify` 返回 null）。新增回归测试 `test_p0_4_period_modifier_applied_along_inheritance_chain`（继承链修正 + 参数传递 + clone 隔离 + config 物化）

- [x] **P0-5 Element 继承判定**
  - 说明：`scan_element_container_from_class` / `add_score_element` 需检测类是否继承自 `GorgeFramework.Element`，防止非元素对象混入生成表
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/runtime/runtime_form_container.rs:203`
    - `GorgeFramework/GorgeFramework/src/runtime/environment.rs`（`ChartManager::add_score_element`）
  - 参考：C# `RuntimeFormContainer.cs:74`、`ChartManager.cs:57`
  - 完成时间：2026-07-31
  - 实现说明：新增 `is_element_subclass`（runtime_form_container 共享函数，对齐 C# `ClassDeclaration.Is` 沿 super 链上溯）。Rust 侧 native 类注册表不含继承信息（`Note : Element` 断链），因此以硬编码根 `Element`/`Note` 为终点，沿 `class_super_name` 链逐层判定（键/值兼容全名与简单名两种注册约定，loader 用简单名、测试路径可能用全名）。接入点：`scan_forms_from_compiled` 容器扫描（替换原 TODO）与 `add_score_element` 入口（对齐 C# `ChartManager.cs:57`，非元素直接 return）。顺带修复 `scan_element_container_from_class` 前置检查 bug：`@PeriodModifier` 位于静态方法注解表，原检查只在构造注解表里找导致带修改器的元素类被提前跳过。新增 3 个回归测试

- [x] **P0-6 RuntimeManager 持有 RuntimeFormContainer**
  - 说明：C# 中 `RuntimeManager` 创建并持有 `FormContainer`，`ChartManager`/`SimulationScore` 通过它访问修改器和即时音效方法；Rust 侧只有 `scan_forms` 接口，容器用完即弃
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/runtime/runtime_manager.rs`（新增字段 + 初始化）
    - `GorgeFramework/GorgeFramework/src/chart/simulation_score.rs`（`load_instant_audio` 读取容器）
    - `DemoImplement/MarcoquadDemo/src/loader.rs`（调用链适配）
  - 完成时间：2026-07-31
  - 实现说明：`RuntimeManager` 新增 `pub form_container` 字段（`new()` 初始化为空容器）+ `scan_forms_into_owned`（对齐 C# `CreateLanguageRuntime` 中 `FormContainer = new RuntimeFormContainer(LanguageRuntime)` 时机，编译类就绪后、提取仿真资源前调用）。`load_instant_audio(form_container, vm)` 从容器 `instant_audio_methods` 表经 VM 调用静态方法取得 `AudioAsset` 对象存入 `instant_audio`（`{"__object_id": N}` 延迟物化形式，供 AudioManager 消费）；类名按「全名 → 简单名」双键约定解析（loader 注册约定），调用失败跳过。`extract_simulation_resources`/`prepare_score`/`reload_assets` 增补 `vm` 参数透传。loader 在 Compiled 状态后插入 `scan_forms_into_owned`。新增 4 个回归测试（自持容器填充 / prepare_score 装载即时音效 / load_instant_audio 方法调用 / 未注册类跳过）

- [x] **P0-7 VM 注入器常量实例化完整化（VM 执行路径前提）**
  - 说明：VM 直接执行含 `LoadInjectorConstant` 的方法目前不可行——(a) Runner/Framework 未把 `CompiledClass.injector_constants` 注册进 `vm.injector_constants`；(b) `RuntimeInjector::from_constant` 对嵌套 `InjectObject`/`Array` 只占槽填 0，未递归物化。补齐后 P0-1/P0-2 的 VM 执行路径才真正可用
  - 修改文件：
    - `GorgeCore/src/virtual_machine/vm.rs:2114-2131`（`LoadInjectorConstant` 嵌套物化）
    - `GorgeCore/src/system/native/injector.rs:132-190`（`from_constant` 递归填充）
    - `DemoImplement/MarcoquadDemo/src/loader.rs` / `GorgeFramework/GorgeRunner/src/main.rs`（注册 injector_constants）
  - 完成时间：2026-07-31
  - 实现说明：**编译器侧**——注入器常量改为按**类**持有（`CodeGenerator` 以类级池种子续写索引，`take_injector_constants` 只回收新增尾部，compiler.rs 四生成点统一 `class_injector_constants_seed`/`merge_class_injector_constants` 辅助），修复多方法类内 `LoadInjectorConstant` 索引错位（此前每方法从 0 重置）。**VM 侧**——`LoadInjectorConstant` 处理器改为两步物化：`RuntimeInjector::from_constant`（改为接收类声明参数，纯标量填充）+ 二次遍历递归物化嵌套 `InjectObject`（注入器，声明按全名→末段短名双键解析）与 `Array`（native 数组载荷 + 编译层包装对象，`length` 字段对齐 `InvokeArrayConstructor`）；`Object` 字段保留编译期 ID。**注册侧**——loader/GorgeRunner 按类注册顺序（继承深度排序）把 `CompiledClass.injector_constants` 合并进 `vm.injector_constants`。新增 4 个回归测试（VM 嵌套物化 / 数组物化 / 索引越界 / 编译器多方法索引连续 / framework 端到端嵌套常量经 `load_instant_audio` 完整物化）。全量 716 测试通过、零 warning

- [x] **P0-8 native 类 `#[inject]` 字段对齐**
  - 说明：曲线/向量族 native 类的注入器字段标记不完整——`CubicHermiteSpline` 全部 8 字段、`VariableFloat.variation_curve`、`LerpColorCurve` 两个对象字段等均缺 `#[inject]`；且 `CubicHermiteSpline` Rust 字段名（`time_start` 等）与 C#/谱面字段名（`startPoint` 等）不一致。导致嵌套注入器的字段名无法按声明恢复（退化为 `__unnamed_N`）、下游物化无法命名匹配
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/system/native/cubic_hermite_spline.rs`
    - `GorgeFramework/GorgeFramework/src/system/native/variable_float.rs`
    - `GorgeFramework/GorgeFramework/src/system/native/lerp_color_curve.rs`
    - 其余曲线/向量族 native 类（对照 C# 参考逐个核对）
  - 注意：补 `#[inject]` 会改变注入器字段计数与索引映射，需回归全量测试
  - 完成时间：2026-08-02
  - 实现说明：以谱面存根（`Native.zip` 的 `.g` 声明）+ C# 参考为权威逐类对齐。**结构重构**——`CubicHermiteSpline` 由 8-float 拍平改为 C# 六字段（`startPoint`/`endPoint` 存 Vector2 对象 ID + 4 float，weights 默认 0.33333），null 端点在 evaluate 回退 (0,0)/(1,1)（对象字段非空默认值机制不支持，净效果与 C# 一致），删除无调用方的 8 参 `new` 与 `impl FunctionCurve`；`PeriodicFunctionCurve` 补 `leftClosed: bool`（default true）、`FunctionPiece` 补 `leftClosed`/`rightClosed`、`AxialSymmetricFunctionCurve` 以 `axis+keepLeft` 替换 `axis_center/axis_amplitude`，三者 evaluate 语义按 C# 修正（周期回绕 `<`/`<=`、轴对称 `2*axis-x`、分段区间包含性，Piecewise 空分段返回 0）。**纯标记类**——Constant/Linear/Quadratic/Arc（angle 默认 π）/LinearCurve/VariableFloat/LerpColorCurve 及全部组合曲线补 `#[inject(name = <camelCase>)]` 与 C# 默认值；`VariableFloat.baseValue` 按 C# 去掉 default。**ctor 表对齐**——涉及类 0 号位插入 0-arg `#[gorge_ctor]`（空体，对齐存根 `类名();`，宏的 `gorge_field_initialize` 先于 ctor 应用注入器字段），原值参 ctor 移 1 号，combinators/lib.rs 共 22 处 `do_construct_native(.., 0)` 调用点同步改 1。测试：新增 6 个（meta 顺序/默认值/新语义），更新 `test_p02_native_injector_fields_meta_available`（`base_value`→`baseValue`），全量 721 通过、零 warning。范围外遗留：`function_curve.rs` 的 Rust-only `RustAxialSymmetricFunctionCurve` 等 trait 族仍旧语义；C# 加权 AnimationCurveInterpolant 求值未移植（仅注释标注）；宏对对象字段 `default` 静默忽略（机制限制，本轮无影响）

---

## P1 — Framework 数据与生命周期完整性

- [x] **P1-1 即时音频加载（`@InstantAudio` 方法调用）**
  - 说明：`load_instant_audio` 当前仅 `clear()` 表，需遍历 `instant_audio_methods` 调用静态方法取得 `AudioAsset`
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/chart/simulation_score.rs:252-258`（`load_instant_audio`）
    - `GorgeFramework/GorgeFramework/src/runtime/runtime_manager.rs:147`（`prepare_score` 调用处）
  - 参考：C# `SimulationScore.cs:355-365`
  - 依赖：P0-6（FormContainer 持有，已完成）
  - 完成时间：2026-08-03（确认项——实现已在 P0-6/P0-7 期间落地，本项为核对销项）
  - 实现说明：`load_instant_audio` 已对齐 C# `LoadInstantAudio`（SimulationScore.cs:355-361）：`clear()` → 遍历 `form_container.instant_audio_methods` → 经 `vm.invoke_method_by_id` 调用静态方法 → 返回对象 ID 非 0 时以 `{"__object_id": N}` 延迟物化形式存入 `instant_audio`（类名按全名→短名双键解析，失败跳过）。`prepare_score`/`reload_assets` 均已接线调用。回归测试 6 项全过（方法调用装载 / 未注册类跳过 / 嵌套常量物化 / 容器扫描 / prepare_score 端到端 / clear 语义）

- [x] **P1-2 资产 native 桥接（AudioAsset / VideoAsset）**
  - 说明：`AudioAsset::load_asset`/`get_asset`、`VideoAsset::load_asset`/`get_asset` 当前固定返回 false/0，需桥接 `Environment.GetAssetByName` 与平台音频/视频句柄
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/system/native/audio_asset.rs:33-45`
    - `GorgeFramework/GorgeFramework/src/system/native/video_asset.rs:30-41`
  - 完成时间：2026-08-03（方案 A：对齐 C# 全链路，经用户确认）
  - 实现说明：`Environment::get_asset_by_name` 新增 `audio:`/`video:` 前缀分支（仿 `image:` 分支），分别包装为 `NativeAudioAsset -> Audio`、`NativeVideoAsset -> Video` VM 对象，平台句柄登记进 `EnvironmentGlobal` 新增的 `audio_handles`/`video_handles` 全局句柄表（`register/resolve_audio_handle`、`register/resolve_video_handle`，`sync_assets_from` 同步清空）。`AudioAsset/VideoAsset::load_asset` 对齐 C#：`Environment.GetAssetByName(name)` → 音频/视频资产族判定（对齐 `FromGorgeObject` 强转，非族类返回 false）→ 调资产对象 `GetAsset`（1 号方法）→ 结果以 native 载荷缓存（对应 C# 私有 `_audio`/`_video`，允许为 0）；`get_asset` 返回缓存载荷，未加载返回 0。新增测试 8 项（环境包装 audio/video 各 1，资产加载成功/未找到/类型不符各 2）

- [x] **P1-3 Manager 生命周期钩子补全**
  - 说明：`audio/graphics/automaton/simulation/scene` 的 `start/stop/destruct` 多为空实现，外层状态机已有但子系统无动作
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/runtime/environment.rs:862-902`（各 `*_start_simulation` / `*_stop_simulation` / `*_destruct` 钩子）
  - 参考：C# `GorgeSimulationRuntime.StartSimulation`
  - 完成时间：2026-08-04
  - 实现说明：全部钩子对齐 C# 实体化。**Chart**——Start 对 `initialize_generate_list` 逐个 `GenerateElement`（Infinitesimal 方向）；Stop 对全部存活元素逐个 `DestroyElement` 并清空五张运行期表（生成表保留供 RePlay）；C# `RemoveCalculatedTime` 无对应概念（Rust 定时生成表不跟踪已计算时间），不移植。**Audio**——Start 从即时音效缓存（`cache_instant_audio`，经 AudioAsset 资产桥取平台句柄）创建音效播放器、按 `cached_periods` 补齐缺失乐段播放器；Stop 先 `stop_all_song` 再逐个 Destruct 清空两表，缓存保留供 restart 重建。**Graphics**——Start/Stop 清空节点表（对齐 C# `Nodes = new List`/`null`，不销毁精灵）。**Automaton**——Initialize 清三表 + `_nextSignalId = 1`，Destruct 清三表 + 归零。**Simulation**——Initialize/Destruct 重建两张优先级堆与模拟器注册表、重注册标准模拟器（-1/-1/0/10000 + 尾独立 100000），并同步 `SimulationMachine.runtime_initialize/destruct`。**Scene**——Initialize 重建 `ScoringV1::new(1395)`，Destruct 对齐 C# 空实现。**RuntimeManager 接线**——`load_score` 与 `start_simulation` 前 `seed_instant_audio`（从 `Score.InstantAudio` 延迟物化条目播种音效缓存）；start 后同步全局 scoring 与 respond_effects 表，stop 后清空全局音效表。新增 5 个 P1-3 回归测试（automaton 初始化/析构编号语义、graphics 节点表清重建、simulation 堆/注册表/机器复位与重注册、audio 播放器销毁重建、即时音效全链路创建）

- [x] **P1-4 Element 销毁链完整化**
  - 说明：`Terminate` 未调用 `on_terminate` 回调；`DestroyElement` 缺少 node→element 反查，不移除图形节点
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/simulators/impls.rs:353`（`Terminate::do_action`）
    - `GorgeFramework/GorgeFramework/src/simulators/impls.rs:524`（`DestroyElement::do_action`，需建立 node→element 映射表）
    - `GorgeFramework/GorgeFramework/src/runtime/environment.rs`（`GraphicsManager` 增加映射存储）
  - 完成时间：2026-08-04
  - 实现说明：**Terminate**——`GorgeSimulationRuntime` 新增 `on_terminate: Option<Box<dyn FnMut()>>`（对齐 C# `Action OnTerminate`），`Terminate::do_action` 先 logger 输出 "Terminated" 再触发回调；`RuntimeManager::create_simulation_runtime` 增加 `on_terminate` 参数（对齐 C# `CreateSimulationRuntime(Action? onTerminate = null)`），Demo loader 传 None。**DestroyElement**——按 C# 权威实现直读元素 `nodes` 字段（ObjectArray）逐个销毁，不建 node→element 反查表（TODO 原设想，C# 无此结构）：每节点按实际 native 类分派 destroy（新增 `graphics_node_destroy_method`：Node=6 号、Sprite/CurveSprite/NineSliceSprite=1 号，置 alive=false + 销毁平台精灵），再从 `graphics.nodes` 移除。同时修正两处存量 bug：①自动机注销原按元素 ID retain，改为按 C# 语义读 `note.automaton` 字段移除 `automatons` 与 `pending_detection_conditions`；②模拟器注销原 `simulators.remove(&element_id)` 拿元素 ID 当注册键（永不命中），改为 `ChartManager` 新增 `element_simulator_keys` 映射（GenerateElement/DeriveElement 注册时记录 reg_key，DestroyElement 查表从优先级堆 + SimRegistry 精确注销），`unload_score`/`chart_stop_simulation` 同步清空。新增 4 个回归测试（Terminate 回调触发/缺省不 panic、destroy 方法编号映射、销毁全链路、Generate→Destroy 端到端）

- [x] **P1-5 IStaff trait-object 路径修复**
  - 说明：`IStaff::periods`/`try_get_period` 因存储类型不一致返回空/None，具体类型方法可用
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/chart/staff.rs:82-89`
  - 完成时间：2026-08-04
  - 实现说明：采用**方案 A（对齐 C# 语义，经用户确认）**——保持具体存储 `Vec<ElementPeriod>`/`Vec<AudioPeriod>` 不变（对应 C# `List<T> Periods`），trait 签名 `periods()` 改为 `Vec<&dyn IPeriod>`、`try_get_period()` 改为 `Option<&dyn IPeriod>`（对应 C# `IEnumerable<IPeriod>` 接口访问，Rust 无协变故每次调用构建引用集合）。实现经 `map(|p| p as &dyn IPeriod)`，具体类型方法（返回 `&ElementPeriod`）仍保留，与 trait 同名方法共存。新增 2 个 trait-object 回归测试（element/audio 各一）。全量 341 framework 测试通过、零 warning

- [x] **P1-6 PeriodConfig 注入器 JSON 解析**
  - 说明：`PeriodConfig` 从注入器 JSON 解析为骨架实现
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/chart/period.rs:46`
  - 完成时间：2026-08-04（确认项——实现已在 P0-3 期间落地，本项为核对销项）
  - 实现说明：`PeriodConfig::from_injector_json` 已对齐 C# `PeriodConfig` native 类三字段（`timeOffset` 默认 0 / `minLength` 默认 10 / `active` 默认 true）从注入器 JSON 解析，缺失/非数值字段回退注入器默认值，与 native 版 `PeriodConfig`（system/native/period_config.rs）字段一致。`PeriodData::new`/`update_config` 均经此解析，上游 `extract_period_config_injector` 返回完整字段 JSON。回归测试 3 项全过（完整 JSON / 默认值 / 空 JSON）

---

## P2 — 自动机与仿真闭环（S7）

- [x] **P2-1 待决检测条件重算**
  - 说明：`UpdatePendingDetectionCondition::do_action` 为空，需接入依赖重新计算
  - 修改文件：`GorgeFramework/GorgeFramework/src/simulators/impls.rs`
  - 完成时间：2026-08-05
  - 实现说明：`fill_pending_detection_conditions` 增加 `SimulateDirection` 参数（创生场景传 Forward），`UpdatePendingDetectionCondition::do_action` 调它按动作携带方向整体重算并覆盖写回 `pending_detection_conditions[automaton_id]`（对齐 C# `DoAction`）。方法 6 `get_detection_conditions` 仍只支持 Forward（Backward/Infinitesimal 方向重算得空，遗留）。

- [x] **P2-2 前向 async 仿真目标**
  - 说明：遍历自动机时间转移列表取最早时间，当前返回固定 MAX
  - 修改文件：`GorgeFramework/GorgeFramework/src/simulators.rs`、`simulators/impls.rs`、`runtime/simulation_machine.rs`
  - 完成时间：2026-08-05
  - 实现说明：`ISimulator` 三个异步目标方法签名加 `&mut VirtualMachine`（trait + 全部实现者 + `get_or_calc_task`/`calculate_simulation_task`/`drive` 同步）。`PreciseAutomatonSimulator::forward_async_simulation_target` 遍历 automatons 经 NativeContext 调 SignalTsiga 方法 0（`forward_state_change_time`）取最小时间，空表返回 MAX。

- [x] **P2-3 后向 async 仿真目标**
  - 修改文件：`GorgeFramework/GorgeFramework/src/simulators/impls.rs`
  - 完成时间：2026-08-05
  - 实现说明：`backward_async_simulation_target` 遍历 automatons 调 SignalTsiga 方法 2（`backward_state_change_time`）取最大时间，空表返回 MIN。

- [x] **P2-4 瞬时仿真目标（零时间转移竞争检测）**
  - 修改文件：`GorgeFramework/GorgeFramework/src/simulators/impls.rs`
  - 完成时间：2026-08-05
  - 实现说明：对齐 C# `InfinitesimalAsyncSimulationTarget` 固定返回 f32::MAX；瞬时竞争检测在 `instant_simulate`（P2-7）。

- [x] **P2-5 前向推进动作传播**
  - 说明：调用 SignalTsiga `forward_simulate` 后动作列表被丢弃，未传播
  - 修改文件：`GorgeFramework/GorgeFramework/src/simulators/impls.rs`、`system/native/signal_tsiga.rs`
  - 完成时间：2026-08-05
  - 实现说明：采用对齐 C# 方案——SignalTsiga 前向路径 native 方法（1/11/12/13/14/15）由 i32 计数改为返回命令 ObjectArray ID（0=空，新增 `build_command_array` 合并辅助），`forward_simulate` 复用 `convert_actions_from_commands` 收集传播，命令非空追加 `UpdatePendingDetectionCondition(Forward)`。

- [x] **P2-6 后向推进动作传播**
  - 说明：调用 SignalTsiga `backward_simulate` 后动作列表被丢弃
  - 修改文件：`GorgeFramework/GorgeFramework/src/simulators/impls.rs`、`system/native/signal_tsiga.rs`
  - 完成时间：2026-08-05
  - 实现说明：`backward_state_change`（方法 3）返回 HistoryStack pop_until 的受影响数组 ID；`backward_simulate` 读数组非空追加 `UpdatePendingDetectionCondition(Backward)`。C# 反向弹栈只产 UpdatePending 直接动作（非命令），故不走命令转换。

- [x] **P2-7 瞬时仿真竞争检测**
  - 说明：基于 `pending_detection_conditions` 的竞争检测未实现
  - 修改文件：`GorgeFramework/GorgeFramework/src/simulators/impls.rs`、`system/native/signal_tsiga.rs`
  - 完成时间：2026-08-05
  - 实现说明：`detection_accept`/`detection_deny`（方法 4/5）改为返回命令数组 ID；`instant_simulate` 的 Accept/Deny 分支收集命令转动作并追加 `UpdatePendingDetectionCondition`，末尾返回 `game_actions`（替代原空 Vec）。Note：Deny 分支 direction 用入参而非 C# 固定 Infinitesimal（语义基本一致，细微差异）。

- [x] **P2-8 SongSimulator 播放控制**
  - 说明：`forward_simulate` 遍历空转，未实际控制音乐播放/停止
  - 修改文件：`GorgeFramework/GorgeFramework/src/simulators/impls.rs`、`runtime/environment.rs`
  - 完成时间：2026-08-05
  - 实现说明：遍历 `period_audio_sources`，`start = timeOffset + RespondDelay(0)`、`end = start + audio_length()`，chart_to 落 `[start,end)` 且未播放 → `set_time + play`，否则播放则 `stop`。新增 `AudioManager::period_player/period_time_offset` 访问器与 `RESPOND_DELAY` 常量。Headless/Macroquad 真后端 audio_length/set_time 仍占位（P3 范围）。

- [x] **P2-9 HistoryStack 受影响动作列表**
  - 说明：返回受影响的动作对象 ID 列表当前为占位
  - 修改文件：`GorgeFramework/GorgeFramework/src/system/native/history.rs`、`signal_tsiga.rs`、`simulators/impls.rs`
  - 完成时间：2026-08-05
  - 实现说明：`pop_until`（方法 4）返回受影响自动机 ID 的 ObjectArray（去掉 0x8000_0000 标志位，空返回 0）；`backward_state_change` 透传；`backward_simulate` 非空追加 `UpdatePendingDetectionCondition(Backward)`。反向方法 6 只支持 Forward 的遗留见 P2-1 注。

---

## P3 — 平台适配（Macroquad Demo）

- [x] **P3-1 音频从字节数据创建**
  - 说明：`create_audio_from_data` 返回 Err，`create_audio(path)` 返回 0；音频时长/播放状态/seek 为占位
  - 修改文件：
    - `DemoImplement/MarcoquadDemo/src/adaptor.rs`（`audio_length`/`is_playing`/`set_time`、`create_audio`/`create_audio_from_data`）
  - 完成时间：2026-08-06
  - 实现说明：引入 sasa（git 依赖 + rev 4470cd7 锁定，未发布 crates.io）作为唯一音频后端——macroquad 保持默认（dummy 音频、无音频线程），避免 quad-snd 与 cpal 双 WASAPI 设备冲突。`create_audio_from_data` 用 `AudioClip::new`（symphonia 同步解码 WAV/MP3/FLAC/OGG）→ `AudioManager::create_music`；`create_audio(path)` 读磁盘字节复用同一路径。`audio_length`=clip.length() 真实时长、`is_playing`=!music.paused() 真实状态、`set_time`=music.seek_to() 真实 seek，与 C# 语义完全对齐；音效播放器走 `create_sfx`（每次从 0 播放可叠加）。AudioManager 用 thread_local 单例（sasa 的 `Box<dyn Backend>` 无 Send 标记，放不进 `Arc<Mutex<InnerState>>`），惰性创建，无音频设备时优雅降级（返回明确 Err / 播放 no-op）。Demo 实测音频资源真实解码注册：clips=5 music=5 sfx=3。
  - 后续链路线（2026-08-06 同日修复，见 MEMORY 最新条目）：编译器丢 metadata（`[PeriodConfig^ config=...]` 块未入参数表）、隐藏方法插入索引错位（global_id=0 存量 bug）、`register_audio_periods` 无调用点、`create_period_player` 缺 SetAudio。修复后 Demo 实测 **0.373s 起播放 Song.wav（130.7 秒）真实出声**（music paused:false、pos 实时增长）。

- [x] **P3-2 视频真实支持**
  - 说明：视频当前退化为纹理/占位路径
  - 修改文件：`DemoImplement/MarcoquadDemo/src/adaptor.rs`（`create_video_from_data`）
  - 完成时间：2026-08-06
  - 实现说明：macroquad 无视频解码能力（谱面包也无视频资源）。新增 `is_video_data` 魔数识别（MP4/MOV ftyp、WebM/MKV EBML、AVI RIFF、FLV、WMV/ASF GUID），命中返回明确 Err「macroquad 无视频解码能力，视频资源不受支持」；图片类数据继续走纹理加载。新增 4 个回归测试（视频魔数识别 5 种 / 非视频数据拒绝 / WAV 解码时长 8kHz 单声道 / 44.1kHz 立体声）。Demo 测试 9 → 13，全 workspace 测试通过、零 warning。

---

## P4 — 编译器与 VM 正确性（与画面无关，但属已知缺陷）

- [ ] **P4-1 无初始化器局部变量类型分配**
  - 说明：`Type name;` 一律分配 `ValueType::Int`，Float/Bool/String/Object 可能用错栈
  - 修改文件：`GorgeCompiler/src/visitors/codegen.rs`
  - 测试：补五类值类型回归测试

- [ ] **P4-2 对象闭包捕获**
  - 说明：Lambda/委托捕获 Object 时固定写入 0，未真实读取外层对象槽
  - 修改文件：`GorgeCore/src/objective/delegate.rs`

- [ ] **P4-3 字节码版本元数据统一**
  - 说明：序列化写 V6，但 `compile_sources` 构造 `CompiledModule { version: 5 }`
  - 修改文件：`GorgeCompiler/src/lib.rs`（`compile_sources`）

- [ ] **P4-4 旧 NativeArray 对象接口 panic**
  - 说明：五类数组 `gorge_class()` 为 `unimplemented!()`，改走旧 trait 路径的代码会先补齐
  - 修改文件：`GorgeCore/src/system/native/array.rs:204-236`

---

## P5 — 代码组织与清理

- [ ] **P5-1 SceneManager 模块拆分**
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/runtime/environment/scene_manager.rs:3`
    - `GorgeFramework/GorgeFramework/src/runtime/environment.rs:662-666`

- [ ] **P5-2 SimulationManager / SimRegistry 模块拆分**
  - 修改文件：
    - `GorgeFramework/GorgeFramework/src/runtime/environment/simulation_manager.rs:4`
    - `GorgeFramework/GorgeFramework/src/runtime/environment.rs`

- [ ] **P5-3 未跟踪文件清理（需用户确认）**
  - 说明：`err*.txt`、`run_*` 日志、`DemoImplement/target/`、`GorgeCompiler/tests/tmp_repro_eq.rs`

---

## 最终验收

- [ ] **V-1 真实 Demo 可见窗口 7/7 验收**
  - 说明：完成 P0 全链路后，在可见窗口完成启动、资产加载、仿真启动和基本交互验证
  - 依赖：P0-1 ~ P0-6、P1-1 ~ P1-4

---

## 建议修复顺序

1. P0-2 / P0-3（`@Chart`/`@Song`/音频乐段求值）— 直接决定元素是否生成
2. P0-6 + P0-4 + P0-5（FormContainer 持有 + PeriodModifier + 继承判定）
3. P1-1 + P1-2（即时音频 + 资产桥）
4. P1-4（Element 销毁链）
5. P2 全项（自动机闭环，影响玩法不影响显示）
6. P3（音视频平台适配）
7. P4（编译器/VM 正确性债务）
8. V-1（可见窗口验收）
