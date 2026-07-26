# Gorge 项目记忆

> 最后核对：2026-07-22。
> 本文件只记录当前有效状态、稳定约定和真实遗留项；阶段流水与已解决故障不再保留。
> 代码与测试是事实来源。`reports/` 中部分计划和缺口清单已过期，只能作为历史资料。

> 2026-07-24 补充：真实 Dremu 谱面含背景、轨道和音符，Macroquad 画面空白的首要原因是 Score 到 Runtime 创生、Node/Sprite 更新及资产对象链路未接通。用户已选择完整修复方向；在确认用户手动修改范围及实施方案前，不修改业务代码。

## 项目定位

- Gorge 是自定义面向对象语言及其 Rust 编译器、字节码虚拟机和业务框架移植。
- 运行时采用解释型 VM 和类型分离栈，目标是与 C# 参考实现保持语义兼容。
- C# 编译器参考：`references/gorge-compiler/`。
- C# 运行时参考：`references/gorge-core-csharp/`。
- C# 框架参考：`references/gorge-framework/`。
- Gorge 谱面/字节码样例：`references/gorge_file/`。

## Workspace 与目录

根 `Cargo.toml` 使用 resolver 2，当前包含 6 个成员：

| 目录 | crate / 产物 | 当前职责 |
|---|---|---|
| `GorgeCore/` | `gorge_core` | IR、字节码、VM、运行时对象、注入器、委托、native 互操作、内建集合 |
| `GorgeCompiler/` | `gorge_compiler`、`gorgec`、`gorge` | 词法/语法/符号/多趟编译、代码生成、优化、编译 API、CLI 与 VM CLI |
| `GorgeFramework/GorgeMacros/` | `gorge_macros` | `#[gorge_native_class]` / `#[gorge_native_impl]` 桥接宏 |
| `GorgeFramework/GorgeFramework/` | `gorge_framework` | native 业务类、谱面、信号、自动机、仿真、资产和平台抽象 |
| `GorgeFramework/GorgeRunner/` | `gorge_runner` | 加载 `.gorge`、注册类并执行方法 |
| `DemoImplement/MarcoquadDemo/` | `MarcoquadDemo` | Macroquad 平台适配和真实 Dremu 包加载演示 |

补充目录：

- `reports/`：设计与阶段报告；内容可能落后于源码。
- `test_output/`：端到端 Gorge 用例和脚本。
- `DemoImplement/MarcoquadDemo/gorge_package/`：当前真实演示包。

`GorgeFramework/Cargo.toml` 仍保留 Framework/Macros/Runner 三成员子 workspace；根 workspace 已覆盖全部 crate。

## 当前验证状态

### 全量 Rust 测试

2026-07-22 在当前工作树执行：

```powershell
cargo test --workspace --all-targets
```

结果：**686 passed，0 failed，Rust 编译零 warning**。

当前测试构成：

- GorgeCompiler 单元测试 238；集成测试 4。
- GorgeCore 127。
- GorgeFramework 295。
- GorgeMacros 集成测试 13。
- MarcoquadDemo 9。
- 其余二进制目标当前无测试。

Cargo 仍会输出 `could not canonicalize path: C:\Users\daxingyi` 环境提示；这不是 Rust 编译 warning。

### 真实 Demo

- 三个 zip 包共 **126/126 Gorge 源文件解析成功**。
- Gorge 编译阶段成功生成 **227 个类**，已知编译诊断清零。
- `EnvironmentGlobal` 初始化顺序已修：`RuntimeManager::new()` 负责初始化；编译类与重新加载后的资产会在资源提取前同步。
- 当前 workspace 测试会编译 MarcoquadDemo，构建通过。
- 完整可见窗口的 7/7 交互启动仍未复验。隐藏窗口验证因同步编译期间窗口不处理消息，只捕获到 3/7 后按超时终止；未遗留进程。

因此当前准确结论是：**语言编译链路已打通真实包，Demo 最终运行链路仍需可见窗口复验，不能视为整体完成。**

## 稳定架构

### 编译器

- 前端：Logos 词法器 + 手写 Pratt/递归下降解析器。
- 编译流程：
  1. Pass 1 收集 namespace、class、interface、enum 骨架。
  2. Pass 2 解析 using、父类、接口和枚举值。
  3. Pass 3 声明字段、方法、构造、注解和注入器信息，并冻结编号。
  4. Pass 4 按 CompileTask 生成 IR，再执行优化。
- 符号表使用 Arena + newtype ID + 嵌套 Scope；支持 namespace、using、限定名与 using 别名。
- 已支持类/接口/枚举、继承与接口映射、重载、构造/super、泛型类参数基础、委托/Lambda、注解/metadata、注入器、数组、控制流及多目标 break/continue。
- 表达式支持 cast、条件表达式、算术/比较/相等/逻辑、成员链、隐式 this 调用、数组访问和注入器字段访问。
- 语义层已有重复声明、修饰符白名单、继承/实现合法性、冻结、参数数量与类型、switch 类型等诊断。
- `compile_sources` 是供 Demo 使用的多文件编译 API；诊断支持每个源文件独立 `source_id` 和命名源码渲染。
- 支持 `CancellationToken`、加权进度和线程式异步编译入口。
- 优化器已接入基本块、活跃变量、DCE 和全局 CSE；优化质量仍未完全对齐 C# 的局部 DAG 实现。

### 字节码与 VM

- 序列化格式魔数为 `GORG`，当前写出格式常量为 **V6**。
- V6 包含类/继承/接口、方法与构造、字段初始化器、注入器、委托、类/方法/构造注解等元数据，并保留旧版本反序列化路径。
- VM 使用 Int/Float/Bool/String/Object 类型分离栈和分组参数池。
- 已实现实例、静态、接口、构造、super、注入器构造和委托调用，返回值覆盖五类值类型。
- native 类可经 `NativeContext` 访问参数、返回值、对象字段、注入器、集合、其他 native 对象与 VM 委托。
- 运行时支持 native 继承、接口方法映射、字段初始化器、注入器默认值、对象克隆、集合 payload、注解查询和按方法 ID 调用。
- VM 分派失败现在返回明确错误，不再静默当作成功。

### native 桥接约定

- native stub 与 Rust 注册类的方法顺序、签名和字段布局必须一致。
- 字段 offset 按 Int/Float/Bool/String/Object 五种值类型分别编号，不是单一连续 offset。
- native 静态方法和实例方法使用各自编号空间；跨层分派时必须做正确映射。
- `#[gorge_method]` 的接收者参数名必须精确写成 `this`；写成 `_this` 会被宏误计为 Object 参数。
- 注入器字段初始化顺序与 C# 一致：先应用注入器字段，再由显式构造参数覆盖。
- Framework 当前测试要求至少注册 68 个 native 类；实际注册入口在 `GorgeFramework/GorgeFramework/src/lib.rs::native_classes()`。

### GorgeFramework

- 已有数学、向量、颜色、随机、曲线与组合曲线、变换器、Node/Element/Note、Signal、InputGraph、History/TimeStack、SignalTsiga、Asset、Sprite、Audio/Video 等 native 类。
- 已有 `PlatformBase` 与 sprite/audio 等平台 trait，Headless 后端可记录调用；MarcoquadDemo 提供实际渲染适配。
- chart 模块支持文件夹/zip 包、BOM 处理、资源/源码拆分、Period/Staff 数据和 JSON 转换。
- RuntimeManager、SimulationMachine、环境 Manager、计分、信号边沿、定时生成/销毁和注解扫描已具备基础实现。
- ScoringV1、AutomatonManager 基础信号操作、函数曲线、图形节点与部分音视频资产路径已有测试覆盖。

## Gorge 语义要点

- 类声明：`class Child : Parent :: InterfaceA, InterfaceB`；`:` 是父类，`::` 是接口列表。
- 注解使用 `@Name(...)`；metadata 块与命名参数会进入编译元数据。
- 注入器字段使用 `^field` / `obj.^field`；注入器对象和数组常量可递归序列化。
- 枚举在 VM 中按 Int 存储，可自动转换为 Int。
- Int 可自动提升为 Float；字符串加法支持 Int/Float/Bool 转字符串。
- 逻辑与/或按 C# 参考语义不做短路。
- 数组运行时映射到 IntArray/FloatArray/BoolArray/StringArray/ObjectArray，并公开 `length` 字段语义。
- Object ID `0` 表示 null；对象栈地址 0 在实例方法中用于 `this`，两者不能混淆。
- 字节码和 native 方法 ID 属于稳定协议，修改结构时必须同时更新编译器、序列化、加载器、VM、Runner 和测试。

## 2026-07-19 至 2026-07-22 的有效成果

- 修复跨 namespace/using 的父类、接口和 native 类型解析，CodeGenerator 不再假定类都位于 global scope。
- 补齐非法继承、非法实现和不可注入接口/枚举的诊断。
- VM 八类分派/索引失败由静默成功改为显式错误。
- 修复 cast 后缀结合性及方法接收者重复求值。
- 补齐 enum 类型解析与 `Enum.Value` 成员访问。
- 补齐隐式 this 方法调用、字段接收者链式类型推导和字段初始化器注入器上下文。
- 支持限定名、注入器复合类型、数组 `length`、泛型类声明参数和对象数组元素类型传播。
- 修复 null 临时槽位与多文件诊断定位；真实包的编译诊断已清零。
- 字节码注解结构升级到 V6；谱表提取、Form 扫描和 RuntimeManager 编译类上下文已经接线。
- 修复 `EnvironmentGlobal` 初始化与资源同步顺序，使真实 Demo 越过原 5/7 panic 点的代码路径具备正确前置条件。

## 当前真实遗留

### P0：编译与运行时正确性

1. **无初始化器局部变量类型错误**
   - `GorgeCompiler/src/visitors/codegen.rs` 对 `Type name;` 一律分配 `ValueType::Int`。
   - Float/Bool/String/Object 无初始化声明可能使用错误栈；需按 `var_type` 推导并补五类回归测试。

2. **对象闭包捕获未完成**
   - `GorgeCore/src/objective/delegate.rs` 构造捕获值时，Object 捕获仍固定写入 `0`。
   - 捕获对象的 Lambda/委托语义不完整，需要真实读取外层对象槽并做端到端测试。

3. **S7 自动机主链仍是部分实现**
   - `UpdatePendingDetectionCondition::do_action` 为空。
   - PreciseAutomatonSimulator 的 async target 仍返回固定 MAX/MIN；瞬时推进为空；前后向推进调用 SignalTsiga 后丢弃动作列表。
   - 待决检测条件、状态转移动作传播和竞争检测尚未形成完整闭环。

### P1：Framework 数据与生命周期

4. **Form/Staff 反射仍有骨架**
   - `@Form` 静态方法尚未经 VM 执行，元素类型列表固定为空。
   - Element 继承校验未接入。
   - `IStaff::periods/try_get_period` 的 trait-object 路径因存储类型不一致仍返回空/None；具体类型方法可用。

5. **谱表注入器与即时音频未完整实例化**
   - Element/Audio period 仍从常量近似推导，未走完整注入器实例化。
   - `load_instant_audio` 当前只清空表，未执行 `@InstantAudio` 方法并加载返回对象。

6. **运行时 Manager 生命周期钩子多为 no-op**
   - chart/audio/graphics/automaton/simulation/scene 的 load/start/stop/destruct 内部仍有空实现。
   - 外层状态机和调用顺序已存在，但不能据此认为每个子系统生命周期已完成。

7. **Element 销毁链不完整**
   - Terminate 回调未调用。
   - 缺少 node→element 反查，DestroyElement 不会完整移除图形节点。

### P2：平台、兼容层与协议债务

8. **Macroquad 音视频适配不完整**
   - 音频时长、播放状态和 seek 是占位；从路径/字节数据创建音频尚未实现。
   - 视频目前退化为纹理/占位路径。

9. **资产 native 桥未完全接 Runtime AssetManager**
   - AudioAsset/VideoAsset 的部分 LoadAsset/GetAsset 路径仍固定失败或返回 null。

10. **旧 NativeArray 对象接口有潜在 panic**
    - 五类数组的 `GorgeObject::gorge_class()` 仍为 `unimplemented!()`。
    - 当前主路径使用 VM native payload，因此测试未触发；任何改走旧 trait 路径的代码都必须先补齐。

11. **字节码版本元数据需统一**
    - 序列化文件头写 V6，但 `GorgeCompiler::compile_sources` 当前构造 `CompiledModule { version: 5, ... }`。
    - 现有序列化测试通过，因为写出路径使用格式常量；仍应统一内存模块版本，避免调用方读取到不一致信息。

12. **最终 Demo 验收**
    - EnvironmentGlobal 修复后需在可见窗口完成一次 7/7 启动、资产加载、仿真启动和基本交互验证。

## 建议后续顺序

1. 修复无初始化变量的值类型分配，并补回归测试。
2. 完成对象闭包捕获，验证捕获对象 Lambda 端到端。
3. 打通 `@Form`/`@InstantAudio` 静态调用和注入器实例化。
4. 完成 PreciseAutomatonSimulator、待决检测条件和动作传播闭环。
5. 补齐 Manager 生命周期、Element 销毁反查与资产桥。
6. 完成 Macroquad 音频能力并执行真实可见窗口 7/7 验收。

## 维护注意事项

- 当前工作树有大量未提交修改，覆盖 Compiler/Core/Framework/Macros/Demo 和 workspace 配置；后续修改必须保留这些成果。
- 未跟踪的 `err*.txt`、`run_*` 日志、`DemoImplement/target/` 和 `GorgeCompiler/tests/tmp_repro_eq.rs` 属于清理候选；未经用户确认不要删除。
- `reports/csharp-parity-todo.md`（2026-07-10）和 `reports/framework-completion-plan.md`（2026-07-18）包含大量已完成或已变化的判断，不应直接当作当前 TODO。
- native ID 对齐相关历史记录见 `reports/native-id-alignment.md`，实际编号仍以源码和测试为准。
- 子智能体禁止任何 git 写操作；只允许只读 status/diff。该规则同时记录在 `AGENTS.md`。
