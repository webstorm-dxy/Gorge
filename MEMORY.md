# Gorge 项目记忆

> 2026-08-08 判定线实际可见闭环（截图像素验证）。上一轮验收后用户反馈"画面里没看到判定线"，root 定位两个渲染层遗留：1) 线宽 0.1px 不可见——CurveSprite 默认 width=0.1（Gorge 单位），render_curve 直接按像素 draw_line(0.1)，屏幕上看不见；修复为线宽按坐标尺度缩放（×64，保底 1px，实际 6.4px）。2) 后续构建一直未生效——main.rs 验收截图用了 `image.export_png(...)` 的 Result 匹配，但该版本 macroquad 返回 ()，导致 cargo build 失败、连续多轮运行的都是旧 exe（15:52 构建），用户看到的正是旧构建。修复 export_png 调用后，最新 exe（17:40:38）真实运行：draw_line 诊断确认非零线段实际提交（屏幕 (0.01,358.4)→(3.23,358.4)，宽度 6.4）；5 秒自动截图 test_output/screenshot.png（1280x720）经 System.Drawing 像素分析：11526 个白色像素、覆盖全部 1280 列、y 343-362（屏幕中部）——判定线横贯可见。全量 800 测试通过、零 warning。遗留：截图代码保留为验收辅助（一次性）；部分 drawStartX=0 的轨道曲线退化为单点不可见，但 Main1 等非零轨道横线已显示；test_output/ 诊断日志待确认清理；全部修复未提交。

> 2026-08-08 判定线显示修复闭环（真实窗口验收通过）。在画面空白修复基础上，用户复验仍空，root 沿「判定线 CurveSprite → Transform → points → 渲染」整链排查并修复 15 个深层缺陷：1) **ICurveSprite 接口只传点数不传坐标**（C# `SetLine(ObjectArray)` 被降级为 `set_line(usize)`）→ 改 `set_points(&[(f32,f32)])`，CurveSprite 构造/update_node 遍历 points 数组上传 Vector2 坐标。2) **transformer 用 native 分派调用编译类**（DremuMainLaneLineTransformer 等）静默失败 → `element_simulate` 区分 native/编译类，编译类经新增 `invoke_instance_method_by_global_id_batch`（ParamMode::Batch + 参数池 reset）按方法名分派。3) **InvokeInstance 未切换 current_class**（传 None）→ 改 Some(class_name)。4) **codegen Object 局部槽 0 是 this、参数从 1 起**，而 Batch/ByType/ByCount 从 0 复制 → 三处参数布置 Object +1。5) **Node 族 native 布局与 Node.g 不一致**（position/rotation/size 在 Gorge 声明是 Vector3 对象字段，Rust 实现是 x/y/z float）→ Node/Sprite/NineSliceSprite/CurveSprite 统一改为对象字段（usize），读值经 `read_vec3_field`；`calc_global_position/rotation/size` 改 pub(crate) 供 update_node 按 C# 语义结算 reference 链。6) **注入器常量对象/数组字段丢字段名**（`InjectorConstField::InjectObject/Array` 无名字）→ 枚举加字段名（序列化 tag5/6 扩展 + 全消费端），`const_object_to_json` 按名字输出（不再位置对齐），源码跳过对象字段（如 animation）不再错位。7) **接口类型包装具体类**（`FunctionCurve^ : {ConstantFunctionCurve : {...}}` 类名带 `.^`）→ `expr_as_dotted_name` 忽略 `^`；`const_object_to_json`/`materialize_injector_constant` 对「单子对象字段名==内层类名且外层不可解析」递归用内层。8) **数组字段单对象未包装**（laneLines 单曲线）→ `InjectorFieldDef/Info` 加 `is_array`（编译器 `type_ref_is_array` 递归识别 `^[]^`），materialize 单对象包装为单元素数组；默认值仅 InjectObject 才包装（Array 常量不双重包装）。9) **`new laneLines[i]()` 动态构造缺失**（parser 误判为数组构造）→ 新增 `Expression::DynamicNew`（parser 识别 `[size](`、codegen 求值目标+SetInjector+InvokeInjectorConstructor+恢复外层注入器）。10) **注入器数组复制**：`new (^laneLines)[N]` 创建空数组 → InvokeArrayConstructor 在 current_injector 为 ObjectArray 时复制元素。11) **native 构造 ID 错位**：Vector2.g 两个构造（无参/带参），Rust 缺无参 → 补 `new_empty`，带参 new 变方法 1。12) **injector_declaration_for_constant 只查 class_table** → 补 native_class_table（ConstantFunctionCurve 等物化成功）。13) **坐标映射**：render_curve 增加谱面→屏幕变换（中心 640,360，x 每单位 64px、y 每单位 4px），否则曲线画在原点/屏幕外。14) **字段读写越界防护**：Set*Field 补 check_field_bounds、NativeContext 5 个 getter 越界回退（避免 InputGraph 等旧对象裸 panic）。验收：全 workspace **800** 测试通过、零 warning；headless e2e 通过（真实谱面 CurveSetPoints 含非零坐标）；真实 Macroquad Demo 稳定运行 40s+：score_elements=499、alive 峰值 94、nodes=46、**curves=22-23（判定线曲线存活并渲染）**。遗留：音符 NineSliceSprite 仍为 0（谱面时序/坐标映射未覆盖，独立后续项）；`test_output/` 大量诊断日志待确认清理；全部修复未提交。

> 2026-08-07 画面空白修复最终闭环（root 接管验收，补 4 个深层遗留）：在 subagent 修复（Lambda 隐藏方法/精确回填/注入器默认值/字段初始化器等）基础上，验收暴露的 4 个新根因已全部修复：1) **仿真起始时间**——Demo `extract_simulation_resources(0.0,...)` 与 `create_simulation_runtime(0.0,...)` 硬编码 0，C# 参考为 -1s；generateTime=0 的轨道因 `forward_simulate` 严格 `time > chart_from` 永不生成，已两处对齐 -1.0。2) **浮点除零常量折叠**——真实谱面用 `(-1.0/0.0)` 表示负无穷（C# `InjectorHardcodeGenerator.FloatToString` 标准写法），`eval_binary_const`（compiler.rs）与 `try_eval_const`（codegen.rs）原先对零分母返回 None，含无穷字段的元素整体不可折叠 → 数组走逐元素写回 → `fill_element_period_from_method` 无法从 Array 常量恢复（此前 score_elements 仅 33，实际谱面 499 元素）；已改为浮点除零按 IEEE 754 折叠 ±Inf/NaN（int 除零仍不可折叠），`try_eval_const` 补 Binary/Unary 常量折叠。3) **JSON 非有限数**——serde_json 不支持 ±Inf/NaN，`const_object_to_json`/`const_element_to_json` 改为输出字符串 `"Infinity"/"-Infinity"/"NaN"`（新辅助 `json_float`），`materialize_injector` 经 `json_value_as_f64` 还原。4) **FindAliveLane 全名/短名**——Demo 以短类名注册，音符按源码全名 `"Dremu.DremuMainLane"` 查找失败致构造提前 return null 无节点；改为全名/末段短名双向匹配。**验收**：全 workspace **799** 测试通过、零 warning（cargo check 确认）；headless e2e 通过（生成时间非零、alive 峰值 134、nodes=46、平台精灵创建日志存在）；真实 Macroquad Demo 稳定运行：`score_elements=499`、forward=498/backward=498、generate_range=(-3.0,44.92)、alive=101、nodes=46、curves=23（画面有轨道/音符曲线）。新增回归：编译器浮点除零折叠（含注入器对象整体可折叠）、codegen 常量折叠、JSON 非有限字符串编码、`json_value_as_f64` 还原；更新 2 个旧测试（用变量/方法调用代替常量算术作为"运行时会变表达式"）。已知特性（非缺陷）：负时间音符早于轨道生成时 FindAliveLane 查不到轨道、NineSliceSprite 不渲染，C# 参考同样按严格大于判定。`test_output/` 下 `demo-*.log`/`e2e-*.log`/`final-*.log` 为验收日志，待用户确认后清理；全部修复仍未提交（未执行 git 写操作）。

> 2026-08-07 画面空白修复闭环（分 subagent + 直接接管集成，全部验收通过）：**根因链为注解 Lambda 隐藏方法返回委托对象 → 生成表全 0 → 元素永不生成**，修复沿整条链落地：1) 编译器 `generate_hidden_method_ir` 对注解 Lambda 特判为“编译函数体并返回实际值”（参数经 `LoadXxxParameter` 从参数池装载、按操作数类型定 Return 指令），`finalize_annotation_ids` 改用唯一占位 `Delegate(0x8000_0000|task_idx)` 精确回填（同类 Forward/Backward 两个 time 不再串到同一隐藏方法），隐藏方法全局 ID 加 1 避免 Delegate(0) 歧义；2) 框架 `resolve_annotation_time`/`resolve_annotation_time_from_method` 不再静默 0：缺 time 参数、Delegate 成功但 return_float 为空均输出 `[Gorge]` 警告；`resolve_annotation_time_from_method` 无 time 参数时改为调用被注解方法自身（`@ForwardTimedDestroy float ForwardDestroyTime()` 语义）并把 method_id 传入；3) 修复过程中顺带闭环的深坑：`generate_conditional` 结果临时变量误用条件类型（bool）导致对象分支被 BoolAssign 错写；Pass3 类声明按继承深度排序（否则 DremuGuideLane 先于 DremuLane 处理，字段偏移漏中间层→轨道字段串位）；`InvokeConstructor` 编译路径恢复调用者 this 槽（嵌套构造后 LoadThis 读到内层对象）；字段初始化器沿继承链执行（父类 @Inject 字段如 lane.name 不再丢失）；注入器默认值全链路物化（`InjectorFieldInfo.default_value` 携带常量、chart JSON 数组字段物化为 ObjectArray 包装、对象型默认值如 `LinearFunctionCurve : {...}` 由编译器 `injector_literal_to_const` 转换、LoadXxxInjectorField 直接读存储值）；EnvironmentGlobal 存活元素快照注册/注销（FindAliveLane 不再恒空）；native 类 `NativeClass::declaration()` 补 `is_native=true` 使注入器构造回退生效；VM 全部字段读取/写入越界改为带类名+索引+当前执行类的明确错误（不再裸 panic），方法错误带方法名。**验收**：真实 Dremu 包 headless 端到端测试通过（生成时间非零 `generate_range=(-2.47,44.92)`、峰值 alive>0、nodes>0、平台调用日志含 CreateCurveSprite 等渲染对象）；全 workspace `cargo test --workspace --all-targets` 约 796 项全绿、`cargo check` 零 warning。新增回归测试：编译器（Lambda 隐藏方法 IR/VM 执行/同名注解占位精确回填/Pass3 继承深度顺序/字段偏移），core（ObjectArray 构造 ABI/空数组物化/对象数组元素/失败路径 current_injector 恢复/嵌套构造 this 槽恢复/数组类名映射），框架（Lambda time Delegate 非零解析/无 time 注解方法调用/默认值物化）。工作区仍有大量未提交改动（含本会话全部修复），未执行任何 git 写操作。

> 2026-08-07 画面空白根因闭环（纯文本分析，未改业务代码）：现象为 `score_elements=33`、`forward=32/backward=32` 但**生成表所有时间均为 0.0**（`generate_range=Some((0.0,0.0))`、`next_generate=None`），导致 `TimedElementGenerator.forward_simulate` 的 `time > chart_from` 恒不成立，元素永不生成 → `alive=0/nodes=0/sprites=0` → 画面空白。根因链：1) `@ForwardTimedGenerate` 的 `time` 是 Lambda 表达式（`float:(DremuTap^ noteInjector) -> {...}`），`annotation_expr_to_value` 无法常量折叠 → 登记隐藏方法 + `Delegate(0)` 占位；2) `generate_hidden_method_ir` 把**整个 Lambda** 编译进隐藏方法，codegen 的 Lambda 分支生成的是 `ConstructDelegate`（构造委托对象 + `ReturnObject`），而非执行函数体返回 float；`infer_expression_value_type` 对 Lambda 也落入 `_ => Object`；3) `ChartManager.resolve_annotation_time` 调用隐藏方法成功（无错误日志），但读 `return_float` 为 None → `unwrap_or(0.0)` → 时间静默变 0。叠加 bug：`finalize_annotation_ids` 回填时把所有 `Delegate(0)` 参数统一替换成**同一个**隐藏方法 ID（不按 (类,注解,参数) 精确对应），修复返回类型后会导致 Forward/Backward 时间互相错位。修复方案（已输出，待用户确认）：隐藏方法遇到 Lambda 时改编译其 body（参数注册 + 按 body 返回指令定返回类型）、Delegate 占位按注解参数路径精确回填、`resolve_annotation_time` 找不到 time 或返回类型不符时打日志不再静默 0，并补编译器/框架/VM 回归测试，最终以 `sprites/nodes > 0` 验收。

> 2026-08-07 用户要求分多个 subagent 实施画面空白修复。已重新读取 MEMORY 与 C# references，并将任务拆为对象数组 ABI、动态注入器构造、回归测试与 Demo 验收三路；所有 subagent 均被明确禁止 Git 写操作。实施前工作区核对：`GorgeCore/src/objective/class.rs`、`object.rs`、`system/native/array.rs` 虽显示 `M`，但 `git diff` 无内容差异，仅有 LF/CRLF 提示；当前等待用户确认这些文件是否包含需保留的手动修改，以及确认采用“统一对象数组映射 + 按 current_injector 动态具体类分派 + 完整回归测试”的推荐方案。

> 2026-08-07 画面空白只读诊断：启动当前 `target/debug/MarcoquadDemo.exe` 复验，谱面与资源加载正常（`score_elements=33`、`textures=10`、音频正常播放），但运行期始终为 `alive=0 nodes=0`、`sprites=0 nine_slices=0 curves=0`。根因发生在渲染前的元素构造链：1) `GorgeCore/src/virtual_machine/vm.rs` 的 `InvokeArrayConstructor`/`materialize_injector_array` 把对象数组按元素名包装为 `NodeArray`/`FunctionCurveArray`，而 native ABI 实际统一为 `ObjectArray`，导致 `DremuMainLane` 构造中的数组写入无法把方法全局 ID 1 分派到 `ObjectArray.set`；2) `InvokeInjectorConstructor` 按 IR 静态目标类 `FunctionCurve` 查 `injector_constructor_impl_id`，没有从 `current_injector` 的 `injection_class_declaration` 恢复真实具体类（如 `LinearFunctionCurve`/`PiecewiseFunctionCurve`），导致 `DremuHold` 报 `FunctionCurve` 缺少注入器构造映射。修复方向：抽取统一的数组 native 类解析（基本类型分别映射，所有对象类型统一 `ObjectArray`，构造与常量物化共用）；注入器构造改为优先按当前注入器的动态具体类解析并分派 native/compiled 构造；为对象数组 get/set/length 和多态注入器构造补回归测试，最后以 `sprites/nodes > 0`、截图可见轨道/音符、全 workspace 零 warning 为验收。截图工具尝试失败（Computer Use 返回 `EPERM`，诊断进程无可捕获窗口句柄），故用运行时渲染计数验证。未修改业务代码；诊断进程与临时日志已清理。

> 2026-08-06 P3 延伸：**Demo 音频播放链路全通（真实出声）**。原 P3 完成后 Demo 仍无声，排查出 4 个链路线缺陷并修复：1) **编译器丢 metadata**——`collect_annotations_from_decl` 只处理注解括号参数 `arguments`，`[ PeriodConfig^ config = ... ]` 元数据块（C# `Annotation.Metadatas`，parser 已解析进 `ast_ann.metadatas`）被丢弃 → `@Song`/`@Chart` 的 config（timeOffset）全丢；修复：新增 `annotation_expr_to_value` 辅助（常量折叠/隐藏方法 Delegate 同参数路径），metadatas 并入 parameters（同名参数优先）。2) **隐藏方法插入索引错位**（lib.rs S3b）——`while len <= local { push }` 后追加导致 gid=1 的方法落在索引 2；且**存量 bug**：`finalize_annotation_ids` 里 `class_info_map` 查类名失败时 `unwrap_or(0)` 使隐藏方法 global_id=0（真实谱面 config 一直 Delegate(0) 解析失败，从未被测试覆盖——现有测试断言的全是常量参数）；修复按索引放置 + 测试补注入器字面量 metadata→Delegate 非零断言。3) **register_audio_periods 无调用点**——AudioStaff 乐段提取成功但从未注册到 AudioManager（period_players=0，SongSimulator 无播放器可控制）；修复：`RuntimeManager::load_score` 补 `seed_audio_periods`（遍历 stave AudioStaff → audio_injector JSON 的 `name` 查 loaded_assets 得句柄 → AudioPeriodData{自增 period_id, audio_id, time_offset} → register_audio_periods）。4) **create_period_player 缺 SetAudio**（对齐 C# `DoAddPeriod` 的 `SetAudio(period.Audio)`）——播放器 audio_id 恒 0 → len=0/play 失效；修复：从 cached_periods 查 audio_id 调 set_audio。修复后 Demo 实测：`@Song` timeOffset=0.373 正确、period_players=1、Song.wav（130.7s）在 chart_to>0.373 后真实播放（music[5] paused:false、pos 随仿真推进）。新增测试：编译器 3（metadata 标量/注入器字面量 Delegate/同名参数优先）、runtime_manager 2（seed_audio_periods 注册+资产缺失降级）、environment 1（cached_periods 数据链）。全 workspace **775** 测试通过、零 warning。遗留：DremuTest.zip 的 Song.wav 为 46MB 真实音乐（之前误判资源缺失，实际存在）；时间线 0.373s 处起播与谱面元素时间对齐待 V-1 可见窗口复验。
>
> 2026-08-06 P3 完成（平台适配·Macroquad Demo 音视频，经用户确认采用 sasa 方案）：**引入 sasa 作为唯一音频后端**（git 依赖 `https://github.com/Mivik/sasa` + rev `4470cd7` 锁定；未发布 crates.io，首次构建需 GitHub 网络）。**关键决策**：不启用 macroquad `audio` feature（其 quad-snd 与 sasa 的 cpal 会各开一个 WASAPI 设备），macroquad 保持默认 dummy 音频。**P3-1**：`create_audio_from_data` = `AudioClip::new`（symphonia 同步解码，支持 WAV/MP3/FLAC/OGG，无需异步上下文）→ `AudioManager::create_music`；`create_audio(path)` 读磁盘字节复用同一路径（读不到返回 0）。`audio_length`=clip.length()（真实时长）、`is_playing`=!music.paused()（真实状态）、`set_time`=music.seek_to()（真实 seek），与 C# SongSimulator 语义完全对齐；音效走 `create_sfx`（每次从 0 播放可叠加）。**坑**：sasa `AudioManager` 含 `Box<dyn Backend>` 无 Send 标记，放不进 `Arc<Mutex<InnerState>>`（PlatformBase: Send+Sync 编译错）→ 改用 **thread_local 单例** `AUDIO_MANAGER: Mutex<Option<AudioManager>>`（仅主线程访问），惰性创建、无音频设备优雅降级（返回明确 Err / 播放 no-op）。**P3-2**：macroquad 无视频解码，新增 `is_video_data` 魔数识别（MP4 ftyp/WebM EBML/AVI RIFF/FLV/WMV GUID）→ 命中返回明确 Err，图片类继续纹理加载。新增诊断 `audio_resource_counts()`/`audio_playback_diagnostics()`；Demo 实测音频真实加载 clips=5 music=5 sfx=3（此前 create_audio_from_data 返回 Err 全为 0）。Demo 测试 9→13（新增 WAV 解码时长 8kHz 单声道 + 44.1kHz 立体声、视频魔数 2 项；`make_wav_bytes` 测试辅助的坑：block_align 必须 u16，声明 u32 会多写 2 字节致 bits 错位）。全 workspace **762** 测试通过、零 warning。
>
> 2026-08-06 编译产物非确定性修复（随机 panic 根因闭环）：修复注入器字段索引后 Demo 仍"首次 OK、再次 panic"（`get_injector_float_default_value` len=0 index=0）——根因是**编译产物逐进程漂移**：Demo 每次启动重新编译，编译器多处依赖 HashMap 迭代顺序（种子每进程随机）。**collect_classes 双重递归 bug**：namespace 的 scope 同时经 `symbols` 的 Namespace 条目与 `scope.children` 两条路径递归，导致**每个类被收集 2 次**（227 = 真实 116 + 重复 110，"227 个类"是虚假数字）；重复类使 loader 注册时 `vm.injector_constants` 重复 extend → 全局常量池索引错位，且 classes 顺序（HashMap 迭代）随机 → 每次运行错位不同 → 随机 panic。修复（lib.rs）：1) `collect_classes` 加 `visited: HashSet<ScopeId>` 去重（同一 scope 只收集一次）；2) `scope.symbols` 按**名字排序**遍历，消除类收集顺序随机。顺带确定性化：`simulation_score.rs` 提取 `@Chart`/`@Song` 乐段按方法全局 ID 排序（原 HashMap 迭代 → periods 顺序随机）；`modify_injector` 修改器收集按全局 ID 排序。**`RuntimeInjector::from_constant` 重构**（injector.rs）：字段索引由"常量内字段顺序分组"改为**按类声明分组索引**（新增 `field_index_map`：Int/Float/Bool/String/Object 分组独立编号、继承父类在前，与 codegen/materialize 布局一致）——常量缺字段不再错位；数组大小按类声明统计；哑声明回退常量顺序（数据不丢）。`materialize_injector_constant` 第二遍同步按相对顺序写 object 槽。新增回归测试 5 个（codegen 3：混合类型分组/继承链分组合并/对象跨链解析；core 2：from_constant 声明索引+哑声明回退）。全 workspace **769** 测试通过、零 warning；编译产物指纹三次一致；Demo 连跑 10 次全 OK、score_elements 恒 33。遗留：无（本问题闭环）。
>
> 2026-08-06 注入器字段索引错位修复（对齐 C# ClassMemberCounter）：词法提速后 Demo 首跑到 `load_score`，`@PeriodModifier` 静态方法体 `LoadFloatInjectorField(1)` 越界 panic（len=0/1）。根因：**codegen 注入器字段索引用"类内跨类型全局声明序号"**（`set_injector_context` 按 `enumerate` 全局编号；`resolve_object_injector_field` 跨继承链 `offset += fields.len()` 跨类型累计），而运行时/物化端（`RuntimeInjector` 五组数组、`materialize_injector` 分组索引）按 **Int/Float/Bool/String/Object 类型分组编号**（C# `ClassMemberCounter.cs:84/168`：`InjectorFieldIndex` 从 super 的 TypeCount 起算 + `Count(BasicType)` 分组计数）。DremuNote 的 `^hitTime` 前有 `laneName`(String) 挤占 → 编成 1 而非 0；DremuLane 的 `^generateTime` 前有 `name`(String) → 编成 1 而非 0。修复（codegen.rs 两处）：1) `set_injector_context` 改为**沿继承链合并字段**（根类在前，从 `injector_fields` 表查）+ 按类型分组编号（未设类上下文时回退传入字段，兼容测试）；2) `resolve_object_injector_field` 改为五组 offset 按类型分组累计（父类字段在前）。`this.^field` 也因继承合并首次支持父类注入器字段。新增 3 个回归测试（混合类型分组编号、继承链分组合并、对象类型跨继承链解析）。Demo 实测：完整走到"加载完成"，score_elements=33 无 panic。全 workspace **767** 测试通过、零 warning。遗留：from_constant 常量字段索引（已于同日后续修复，见上）。
>
> 2026-08-06 词法行列计算 O(n²) 修复（对齐 C# ANTLR 方案）：定位"3/7 编译源码"耗时 230s 根因——`tokenize`（lexer.rs）对**每个 token** 调用 `offset_to_line_column` 从文件头全量重扫源文件，总复杂度 O(token数×文件长)。修复：改为**增量游标**（对齐 C# ANTLR 运行时 `_line`/`_charPositionInLine` 增量维护）：logos `spanned()` 的 range 单调递增，逐段推进 `(last_offset, line, column)`；`last_offset..start` 段计入被跳过空白/注释的换行、`start..end` 段计入多行 token 内部换行，语义与旧实现完全一致（按字符计列、`\n` 重置列）。删除旧朴素函数，新增 2 个回归测试（增量 vs 朴素全量等价性对照、多字节+跨行块注释行列断言）。实测真实包 126 文件：词法 231.8s → **26.7ms**（~8700x），logos 本体 13ms 不变；全 workspace **764** 测试通过、零 warning。遗留：无（性能问题闭环）。
>
> 2026-08-05 H3A 完成：修复 `new Type^[N]{...}` 数组构造**静默丢弃内联元素**导致 `@Chart` 返回空数组（`score_elements=0`）。三处修复：1) **codegen `new_array` 分支**（codegen.rs 940/新增 `generate_array_constructor_with_elements`）——带内联元素时全部可折叠 → 生成 `class_name="Array"` 常量 + `LoadInjectorConstant`（对齐 `Expression::InjectorArray` 路径，框架 `fill_element_period_from_method` 逐元素恢复）；含运行时会变表达式 → 退化为逐元素 `SetArrayElement` 写回不丢失。2) **`try_eval_const` 补 `Expression::Null`**（→ `InjectorConstField::Object(id=0)`，VM null 约定）——真实谱面元素 `progressCurve : null` 此前使元素整体折叠失败、落入逐元素写回导致无 `LoadInjectorConstant`。3) **parser 注入器字面量 class_name 提取**（parser.rs `expr_as_dotted_name`）——限定名 `Dremu.MainLane : {...}` 此前赋空（只认 `Identifier`）；及 **compiler `collect_annotations_from_decl` 去 `!parameters.is_empty()` 守卫**（compiler.rs 1271）——无参注解 `@Chart`/`@Song` 此前被整体丢弃，`method_annotations` 为空，框架找不到谱表方法。实测 Demo `score_elements` 0→**33**（forward=32/backward=32）。新增回归测试 6 个（codegen 3：常量数组/注入器对象元素/逐元素写回；compiler 3：AST 端到端 Array 常量、源码文本端到端含 null、无参注解捕获）。全量 workspace 测试 **759** 通过、零 warning。遗留：`@Chart` 元素经 `const_object_to_json` 恢复，`null` 字段转 JSON 目前为 `0`（非 `null`，可后续优化）；`score_elements` 已非零但 Demo 画面渲染仍待可见窗口复验。

> 2026-08-05 P2-8 完成：SongSimulator 播放控制（对齐 C#）。`SongSimulator::forward_simulate`（impls.rs 187）实体化——遍历 `runtime.audio.period_audio_sources`（period 对象 ID → 平台播放器句柄），对每段乐段：`start = period.Config.timeOffset + StaticConfig.RespondDelay`、`end = start + player.AudioLength()`，chart_to 落在 [start,end) 内且未播放 → `SetTime(chart_to-start)+Play`，否则若播放 → `Stop`，返回空动作列表。**新增** `AudioManager::period_player(period_id)`（句柄→播放器引用）、`period_time_offset(period_id)`（读 `cached_periods` 缓存数值，对齐 C# `period.Config.timeOffset`）两个公开访问器；`RESPOND_DELAY` 常量（对齐 `StaticConfig.RespondDelay=0.0`）；`#[cfg(test)] inject_audio_source_for_test` 直注可控播放器 mock。新增 6 个回归测试（窗口内未播放→play+setTime / 已播放不重复 / 窗口外已播放→stop / 窗口边界 / 多乐段独立 / 空表）。全量 workspace 测试 **753** 通过、零 warning。遗留：HeadlessAudio 的 `audio_length`/`is_playing` 仍硬编码（0.0/false），SongSimulator 用可控制 mock 测试绕过；Macroquad 真实 `audio_length`/seek 仍是占位（P2-8 遗留）。

> 2026-08-05 P2-9 完成：HistoryStack 受影响动作列表 + 反向 UpdatePendingDetectionCondition 传播（对齐 C#）。`HistoryStack.pop_until`（history.rs 方法 4）返回类型由 `usize` 计数改为受影响自动机 ID 的 **ObjectArray 对象 ID**（0=空）：去掉 `| 0x8000_0000` 标志位，改为对每个弹出的 `TimeStackPop` 压入**纯 automaton_id**，用 `ObjectArrayClass.do_construct_native` 构造数组返回（空列表返回 0），逆序 revert 逻辑不变。`SignalTsiga` native 方法 3（backward_state_change）改为**透传** pop_until 返回的 ObjectArray ID（不再固定返回 0）。`PreciseAutomatonSimulator::backward_simulate`（impls.rs 917）不再把方法 3 返回值当命令数组用 `convert_actions_from_commands`，改为读 `object_array_items` 判断非空即追加一个 `UpdatePendingDetectionCondition::new(tsiga_id, Backward)`（对齐 C# 弹栈产生更新待决动作驱动收敛）。波及：history.rs `test_history_stack_pop_until` 断言由 `get_int_return()==1` 改为 `get_object_return()==0`（3 个 push 均非 TimeStackPop）；新增 2 个回归测试（`test_history_stack_pop_until_affected_time_stack_pop` 验证含 TimeStackPop 时返回 `[automaton_id]`、`test_backward_simulate_propagates_update_pending_detection_condition` 验证反向弹栈发出动作）。全量 workspace 测试 **747** 通过、零 warning。遗留：反向弹栈的 UpdatePendingDetectionCondition 已能发出，但 `UpdatePendingDetectionCondition::do_action` 在 Backward 方向经方法 6 重算会得到空 filter（方法 6 仍只支持 Forward，direction!=0 返回 0），反向收敛条件刷新后续再补。

> 2026-08-05 P2-1 完成：待决检测条件重算对齐 C#。`fill_pending_detection_conditions`（impls.rs 669）加 `direction: SimulateDirection` 参数，替换原硬编码 `SimulateDirection::Forward`，方向经 `match` 映射为方法 6 方向码（Forward=0/Backward=1/Infinitesimal=2）传入 `get_detection_conditions`，并存入 `SignalDetectionCondition.direction`；两处创生调用点（GenerateElement 469、do_derive_element 801）同步传 `SimulateDirection::Forward`。`UpdatePendingDetectionCondition::do_action`（impls.rs 827）实体化——调 `fill_pending_detection_conditions(self.automaton_id, self.direction, runtime, vm)` 重新计算并整体覆盖写回 `pending_detection_conditions[automaton_id]`（对齐 C# `DoAction` 整体覆盖语义）。新增回归测试 `test_update_pending_detection_condition_recomputes_and_overwrites`（构造最小 SignalTsiga，验证首填 + 二次整体覆盖）。framework 已过 **346**，全 workspace 全绿、零 warning。遗留：方法 6 `get_detection_conditions` 仍只支持 Forward（`direction != 0` 返回 0），Backward/Infinitesimal 方向重算会得到空 filter（P-改方法 6 范围项）。

> 2026-08-05 P2-6 完成：后向推进动作传播（对齐 C#）。`SignalTsiga` native 方法 3（`backward_state_change`）返回类型由 `i32` 改为 `usize` 命令 ObjectArray ID（0=空），与 P2-5 一致。经核对 C# `BackwardStateChange`（SignalTsiga.cs 104-107）仅调 `HistoryStack.PopUntil`（HistoryStack.cs 45-58）做弹栈还原，产生的动作仅为 `UpdatePendingDetectionCondition` 直接动作（每弹出一个 TimeStackPopHistory 一条），**非命令动作**（DeriveElement/DestroyElement/AppendSignal），无法编码进命令 ObjectArray；且 Rust `history.rs` 的 `pop_until` 仅返回计数占位（P2-9 范围，未改其契约），无法还原命令数组，故方法 3 返回 0（对齐 C#，注释已说明）。`PreciseAutomatonSimulator::backward_simulate`（impls.rs 913）实体化——遍历自动机调方法 3，复用 `convert_actions_from_commands` 转动作，命令数组非空时追加 `UpdatePendingDetectionCondition::new(tsiga_id, Backward)`（当前恒为空故不追加），多余 TODO 注释改为中文对齐说明。全量 workspace 测试 **742** 通过、零 warning。遗留：反向弹栈的 UpdatePendingDetectionCondition 传播受 pop_until 计数占位限制暂无法发出（P2-9 解除后补）。

> 2026-08-05 P2-5 完成：前向推进动作传播（对齐 C#）。`SignalTsiga` native 方法 1/11/12/13/14/15 返回类型由 `i32` 计数改为 `usize` 命令 ObjectArray ID（0=空）：`do_respond`/`do_deny` 直接返回 Note.DoRespond 的数组 ID；`pop_until`/`timeout_until`/`do_edge_respond`/`forward_state_change` 将多份命令数组 items 合并为一份新 ObjectArray（新增辅助 `build_command_array`，空列表返回 0）。方法 4/5（detection_accept/deny）内部改读 `get_object_return()` 并返回数组长度计数。`PreciseAutomatonSimulator::forward_simulate`（impls.rs）改为收集每个自动机方法 1 返回的命令数组，用现有 `convert_actions_from_commands` 转动作并 extend；数组非空时追加 `UpdatePendingDetectionCondition::new(tsiga_id, Forward)`（对应 C# DoEdgeRespond 的刷新待决意图）。波及测试断言由 `get_int_return()` 改为 `get_object_return()`（signal_tsiga 6 处）；新增 2 个回归测试（`build_command_array` 合并语义、`do_respond` 传播命令数组）。全量 workspace 测试 **743** 通过、零 warning。

> 2026-08-05 P2-2 完成：ISimulator trait 三个异步目标方法签名统一加 `vm: &mut VirtualMachine`（`simulators.rs` 40/43/46 行），全部 7 处实现（TimedElementGenerator/Destroyer、Song/GraphicsNode/PreciseAutomatonSimulator、ElementSimulatorAdapter）同步加参数；`PreciseAutomatonSimulator::forward_async_simulation_target`（impls.rs 850 行）实体化——遍历 `runtime.automaton.automatons` 经 `NativeContext` 调 SignalTsiga 方法 0（forward_state_change_time）取最小时间，空表返回 f32::MAX，对齐 C# `ForwardAsyncSimulationTarget`。`simulation_machine.rs` 的 `get_or_calc_task`/`calculate_simulation_task` 及三处 async target 调用、drive 内调用点均透传 vm。framework `cargo build`/`cargo test` 全过 341 项、零 warning。backward/infinitesimal 目标仍为 TODO 占位（f32::MIN/MAX）。

> 2026-08-05 P2-3 完成：`PreciseAutomatonSimulator::backward_async_simulation_target`（impls.rs 865 行）实体化——遍历 `runtime.automaton.automatons` 经 `NativeContext` 调 SignalTsiga 方法 2（backward_state_change_time）取最大时间，空表返回 f32::MIN，对齐 C# `BackwardAsyncSimulationTarget`；`vm` 参数真正使用（去下划线）。framework `cargo build`/`cargo test` 全过 341 项、零 warning。infinitesimal 目标仍为 TODO 占位（f32::MAX）。

> 2026-08-05 P2-4 完成：`PreciseAutomatonSimulator::infinitesimal_async_simulation_target`（impls.rs 880 行）确认并收尾——返回 `f32::MAX`，签名含 P2-2 加入的 `&mut VirtualMachine`；过时 TODO 注释改为中文说明对齐 C# `InfinitesimalAsyncSimulationTarget` 固定返回 float.MaxValue、不基于 pending_detection_conditions 计算目标（竞争检测在 instant_simulate）。无逻辑改动。framework `cargo build`/`cargo test` 全过 341 项、零 warning。

> 最后核对：2026-07-22。
> 本文件只记录当前有效状态、稳定约定和真实遗留项；阶段流水与已解决故障不再保留。
> 代码与测试是事实来源。`reports/` 中部分计划和缺口清单已过期，只能作为历史资料。

> 2026-07-24 补充：真实 Dremu 谱面含背景、轨道和音符，Macroquad 画面空白的首要原因是 Score 到 Runtime 创生、Node/Sprite 更新及资产对象链路未接通。用户已选择完整修复方向；在确认用户手动修改范围及实施方案前，不修改业务代码。
>
> 2026-07-26 进展：`RuntimeFormContainer::extract_element_types_from_method` 已从骨架改为真实 VM 调用——`scan_forms_from_compiled`/`RuntimeManager::scan_forms` 增加 `&mut VirtualMachine` 参数，`@Form` 静态方法经 `invoke_method_by_id` 执行后从 `return_object` 的 `StringArray` native 载荷提取元素类型列表；失败回退空列表。全量 workspace 测试通过、零 warning。剩余断点：`@Chart`/`@Song` 静态方法求值、`@PeriodModifier` 应用、`load_instant_audio` 仍未接通。
>
> 2026-07-26 P0-2 完成：`@Chart`/`@Song` 谱表元素提取改为**常量池直读**（真实谱面方法体是纯常量字面量，语义等价 VM 执行）：定位方法字节码 `LoadInjectorConstant` → 类常量池取方法返回常量 → 类型感知 JSON 转换填入 `ElementPeriod.elements`/`AudioPeriod.audio_injector`。配套修复三处：1) 编译器 `InjectorConstField::InjectObject` 首槽位保留类名不再被字段名覆写（codegen.rs 3 处，字节码语义修正、无格式变更）；2) `NativeClass::injector_fields_meta()` 默认方法 + 宏生成字段元数据（class_macro/impl_macro）；3) 嵌套字段名按父类注入器字段声明位置对齐恢复（`InjectorFieldMetaProvider`，合并编译类继承链 + native 元数据）。测试：编译器 239、core 130、framework 305、macros 13 全过零 warning。新发现遗留：VM 执行路径需补 injector_constants 注册 + 嵌套常量物化（TODO P0-7）；曲线族 native 类 `#[inject]` 标记缺失（TODO P0-8）影响嵌套字段命名。
>
> 2026-07-31 P0-7 完成：VM 注入器常量实例化完整化。**编译器**——注入器常量改为按类持有（`CodeGenerator` 以类级池种子续写索引、`take_injector_constants` 只回收新增尾部；compiler.rs 四生成点统一 `class_injector_constants_seed`/`merge_class_injector_constants`），修复多方法类内索引错位。**VM**——`RuntimeInjector::from_constant` 改为纯标量填充（接收类声明参数），`LoadInjectorConstant` 处理器二次遍历递归物化嵌套 `InjectObject`（注入器，类名全名→短名双键解析）与 `Array`（native 载荷 + 编译层包装，length 字段对齐 InvokeArrayConstructor）。**注册**——loader/GorgeRunner 按类注册顺序（继承深度排序）合并 `injector_constants` 进 VM 池。全量 716 测试通过、零 warning。详见记忆 [[p0-7-injector-constants-class-scoped]]。
>
> 2026-08-02 P0-8 完成：曲线/向量族 native 类 `#[inject]` 字段全面对齐谱面存根（`Native.zip` 的 `.g` 声明为权威）与 C# 参考。**结构重构**——`CubicHermiteSpline` 由 8-float 改为 C# 六字段（`startPoint`/`endPoint` 为 Vector2 对象 ID + 4 float，weights 默认 0.33333，null 端点 evaluate 回退 (0,0)/(1,1)）；`PeriodicFunctionCurve` 补 `leftClosed`、`FunctionPiece` 补 `leftClosed`/`rightClosed`、`AxialSymmetricFunctionCurve` 改为 `axis`+`keepLeft`，三者 evaluate 语义按 C# 修正。**纯标记**——Constant/Linear/Quadratic/Arc/LinearCurve/VariableFloat/LerpColorCurve 及全部组合曲线补 `#[inject(name = <camelCase>)]` 与 C# 默认值（Arc.angle 默认 π；baseValue 按 C# 无 default）。**ctor 表**——涉及类 0 号位统一插入 0-arg 构造（对齐存根 `类名();`，宏 `gorge_field_initialize` 先于 ctor 应用注入器字段），原值参 ctor 移 1 号，22 处 `do_construct_native` 调用点同步。全量 **721** 测试通过、零 warning。遗留：Rust-only `RustAxialSymmetricFunctionCurve` 等 trait 族仍旧语义（function_curve.rs，无调用方）；C# 加权 AnimationCurveInterpolant 未移植；宏对对象字段 `#[inject(default)]` 静默忽略（机制限制）。
>
> 2026-08-03 P1-1 确认销项：即时音频加载实现已在 P0-6/P0-7 期间落地（TODO.md 描述过期）。`load_instant_audio` 对齐 C# `LoadInstantAudio`：遍历 `form_container.instant_audio_methods` 经 `invoke_method_by_id` 调用静态方法，返回对象以 `{"__object_id": N}` 延迟物化形式存表；`prepare_score`/`reload_assets` 均已接线。6 项回归测试全过。
>
> 2026-08-03 P1-2 完成：AudioAsset/VideoAsset 资产桥接对齐 C# 全链路（用户确认方案 A）。`Environment::get_asset_by_name` 新增 `audio:`/`video:` 前缀分支（仿 `image:`），包装为 `NativeAudioAsset -> Audio` / `NativeVideoAsset -> Video` VM 对象，平台句柄登记进 `EnvironmentGlobal` 新增的 `audio_handles`/`video_handles` 表（`sync_assets_from` 同步清空）。`load_asset` = GetAssetByName → 资产族判定（对齐 C# `FromGorgeObject` 强转，非族类 false）→ 调资产对象 1 号 `GetAsset` → 结果存 native 载荷（对应 C# `_audio`/`_video`，允许为 0）；`get_asset` 返回载荷缓存。全量 **729** 测试通过、零 warning。
>
> 2026-08-04 P1-3 完成：Manager 生命周期钩子全部实体化（environment.rs 各 `*_start/stop_simulation`、`*_runtime_initialize/destruct`）。Chart Start 逐 `GenerateElement`、Stop 逐 `DestroyElement` 并清五表；Audio Start 从即时音效缓存（资产桥取平台句柄）建音效播放器、按缓存补齐乐段播放器，Stop 先停音乐再 Destruct 清表（缓存保留供 restart）；Graphics 清节点表；Automaton 清三表 + 信号编号 1/0 语义；Simulation 重建优先级堆/注册表/标准模拟器（-1/-1/0/10000 + 尾 100000）并同步 machine 复位；Scene 重建 ScoringV1(1395)。RuntimeManager 接线：`load_score`/`start_simulation` 前 `seed_instant_audio` 播种，start/stop 后同步全局 scoring 与 respond_effects。C# `ChartManager.RemoveCalculatedTime` 无 Rust 对应概念（定时生成表不跟踪已计算时间），不移植。新增 5 个回归测试，全量 **734** 测试通过、零 warning。
>
> 2026-08-04 P1-4 完成：Element 销毁链完整化。`GorgeSimulationRuntime` 新增 `on_terminate: Option<Box<dyn FnMut()>>`（C# `Action OnTerminate`），`Terminate` 动作触发之；`create_simulation_runtime` 加 `on_terminate` 参数（Demo 传 None）。`DestroyElement` 按 C# 权威实现直读元素 `nodes` 字段逐节点销毁（新增 `graphics_node_destroy_method` 分派：Node=6 号、Sprite 族=1 号，alive=false + 平台精灵销毁），不建 node→element 反查表（C# 无此结构）。修正两处存量 bug：自动机注销改按 `note.automaton` 字段移除（原误用元素 ID）；模拟器注销改经 `ChartManager::element_simulator_keys` 新映射精确注销（原 `remove(&element_id)` 拿元素 ID 当注册键永不命中），GenerateElement/DeriveElement 注册时记录键。新增 4 个回归测试，全量 **738** 测试通过、零 warning。
>
> 2026-08-04 P1-5 完成：IStaff trait-object 路径修复（方案 A，对齐 C# 语义，经用户确认）。保持具体存储 `Vec<ElementPeriod>`/`Vec<AudioPeriod>` 不变（对应 C# `List<T> Periods`），trait 签名 `periods()` 改为 `Vec<&dyn IPeriod>`、`try_get_period()` 改为 `Option<&dyn IPeriod>`（对应 C# `IEnumerable<IPeriod>` 接口访问；Rust 无协变，故每次调用构建引用集合）。具体类型方法（返回 `&ElementPeriod`）保留，与 trait 同名方法共存。新增 2 个 trait-object 回归测试，全量 **341** framework 测试通过、零 warning。
>
> 2026-08-04 P1-6 确认销项：`PeriodConfig::from_injector_json` 已在 P0-3 期间完整落地（TODO 描述过期）。对齐 C# `PeriodConfig` 三字段（`timeOffset` 默认 0 / `minLength` 默认 10 / `active` 默认 true）从注入器 JSON 解析、缺失回退默认值，与 native 注册版字段一致；`PeriodData::new`/`update_config` 均经此解析。3 项回归测试全过。

> 2026-08-05 P2 全项完成（自动机与仿真闭环 S7，采用对齐 C# 方案，每项一个 agent 顺序执行）。**trait 重构**——`ISimulator` 三个异步目标方法加 `&mut VirtualMachine`，`simulation_machine.rs` 的 `get_or_calc_task`/`calculate_simulation_task`/`drive` 同步透传。**目标方法**——forward 遍历 automatons 调方法 0 取最小、backward 调方法 2 取最大、infinitesimal 固定 MAX。**动作传播（native 返回命令数组）**——SignalTsiga 方法 1/3/4/5/11/12/13/14/15 由 i32 计数改为返回命令 ObjectArray ID(0=空)，新增 `build_command_array` 合并辅助；`forward_simulate`/`backward_simulate`/`instant_simulate` 复用 `convert_actions_from_commands` 收集传播，命令非空追加 `UpdatePendingDetectionCondition` 驱动收敛。**P2-1**——`fill_pending_detection_conditions` 加方向参数，`UpdatePendingDetectionCondition::do_action` 按方向重算整体覆盖。**P2-9**——`pop_until`（方法 4）返回受影响自动机 ID 的 ObjectArray（去 0x8000_0000 标志位），backward 透传并追加 UpdatePending。**P2-8**——SongSimulator 按 [start,end) 窗口控制 play/stop，新增 `AudioManager::period_player/period_time_offset` 访问器与 `RESPOND_DELAY` 常量。全量 **753** 测试通过、零 warning。遗留：SignalTsiga 方法 6 `get_detection_conditions` 仍只支持 Forward（Backward/Infinitesimal 方向重算得空）；P2-7 Deny 分支 direction 用入参而非 C# 固定 Infinitesimal；Headless/Macroquad 真后端 audio_length/set_time 仍占位（P3 范围）。

> 2026-08-05 编译器 `new Type^[N]{...}` 数组构造修复 + @PeriodModifier 静态方法调用修复。**数组构造**——`codegen.rs` 新增 `generate_array_constructor_with_elements`（带内联元素折叠为 `Array` 常量 + `LoadInjectorConstant`，运行时会变则逐元素写回），`try_eval_const` 补 null；`parser.rs` 注入器字面量 class_name 改 `expr_as_dotted_name`（支持限定名）；`compiler.rs` `collect_annotations_from_decl` 去 `!parameters.is_empty()` 守卫捕获无参 `@Chart`/`@Song`。真实 Demo `score_elements` **0→33**（用户确认运行生效）。**@PeriodModifier 静态调用**——`modify_injector` 原用 `invoke_method_by_id`（实例路径）调用静态 `@PeriodModifier` 方法导致"未找到方法/目标对象为空"；新增 VM `invoke_static_method_by_global_id`（沿 `class_static_methods` 静态表按 `method_global_id - method_start_id` 分派，对齐 C# `InvokeStaticMethod`），`modify_injector` 改用它。P0-4 测试补注册静态方法表。全量 **760** 测试通过、零 warning。遗留：Demo 在本机个别运行存在**间歇性解析阶段挂起**（疑深层递归，非确定，用户环境可复现成功）；`@PeriodModifier` 修复待用户运行 Demo 复验（本机无法稳定复跑）。

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

2026-08-03 在当前工作树执行：

```powershell
cargo test --workspace --all-targets
```

结果：**760 passed，0 failed，Rust 编译零 warning**。

当前测试构成：

- GorgeCompiler 单元测试 246；集成测试若干。
- GorgeCore 133。
- GorgeFramework 354。
- GorgeMacros 集成测试 13。
- MarcoquadDemo 9。
- 其余二进制目标当前无测试。

Cargo 仍会输出 `could not canonicalize path: C:\Users\daxingyi` 环境提示；这不是 Rust 编译 warning。

### 真实 Demo

- 三个 zip 包共 **126/126 Gorge 源文件解析成功**。
- Gorge 编译阶段成功生成 **116 个类**（2026-08-06 修复 collect_classes 重复收集后为真实类数；此前统计 227 含重复），已知编译诊断清零，编译产物逐次运行完全一致。
- `EnvironmentGlobal` 初始化顺序已修：`RuntimeManager::new()` 负责初始化；编译类与重新加载后的资产会在资源提取前同步。
- 当前 workspace 测试会编译 MarcoquadDemo，构建通过。
- Demo 已可完整跑到"加载完成"（score_elements=33，连跑 10 次一致）；完整可见窗口的 7/7 交互启动与画面渲染仍未复验。隐藏窗口验证因同步编译期间窗口不处理消息，只捕获到 3/7 后按超时终止；未遗留进程。

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

3. ~~**S7 自动机主链仍是部分实现**~~（P2 于 2026-08-05 完成：目标方法/动作传播/竞争检测/待决重算/HistoryStack 受影响列表/SongSimulator 播放控制全部实体化，对齐 C# 方案）。遗留：SignalTsiga 方法 6 `get_detection_conditions` 仍只支持 Forward（Backward/Infinitesimal 方向重算得空 filter）；P2-7 Deny 分支 direction 用入参而非 C# 固定 Infinitesimal。

### P1：Framework 数据与生命周期

4. **Form/Staff 反射仍有骨架**
   - `@Form` 静态方法尚未经 VM 执行，元素类型列表固定为空。
   - Element 继承校验未接入。
   - ~~`IStaff::periods/try_get_period` 的 trait-object 路径因存储类型不一致仍返回空/None；具体类型方法可用~~（P1-5 已于 2026-08-04 完成：trait 签名改为 `Vec<&dyn IPeriod>`/`Option<&dyn IPeriod>`，对齐 C# `IEnumerable<IPeriod>` 语义，具体存储与具体类型方法保留）。

5. **谱表注入器未完整实例化**
   - Element/Audio period 仍从常量近似推导，未走完整注入器实例化。
   - ~~`load_instant_audio` 未接通~~（P1-1 已于 2026-08-03 确认完成：静态方法调用 + 延迟物化存表已落地）。

6. ~~运行时 Manager 生命周期钩子多为 no-op~~（P1-3 已于 2026-08-04 完成：全部钩子实体化并对齐 C#，RuntimeManager 已接线全局同步）。

7. ~~Element 销毁链不完整~~（P1-4 已于 2026-08-04 完成：Terminate 触发 on_terminate；DestroyElement 直读元素 nodes 字段逐节点销毁 + 自动机/模拟器精确注销）。

### P2：平台、兼容层与协议债务

8. **Macroquad 音视频适配不完整**
   - 音频时长、播放状态和 seek 是占位；从路径/字节数据创建音频尚未实现。
   - 视频目前退化为纹理/占位路径。

9. **资产 native 桥已接 Runtime 资产表，平台侧仍待落地**
   - ~~AudioAsset/VideoAsset 的 LoadAsset/GetAsset 路径仍固定失败或返回 null~~（P1-2 已于 2026-08-03 完成：Environment.GetAssetByName 支持 audio:/video: 包装，资产族判定 + 载荷缓存）。
   - 遗留：Macroquad 后端 `create_audio_from_data` 仍返回 Err，`audio:*` 资产在真实 Demo 中进不了资产表（P2-8/P3 范围）。

10. **旧 NativeArray 对象接口有潜在 panic**
    - 五类数组的 `GorgeObject::gorge_class()` 仍为 `unimplemented!()`。
    - 当前主路径使用 VM native payload，因此测试未触发；任何改走旧 trait 路径的代码都必须先补齐。

11. **字节码版本元数据需统一**
    - 序列化文件头写 V6，但 `GorgeCompiler::compile_sources` 当前构造 `CompiledModule { version: 5, ... }`。
    - 现有序列化测试通过，因为写出路径使用格式常量；仍应统一内存模块版本，避免调用方读取到不一致信息。

12. **最终 Demo 验收**
    - EnvironmentGlobal 修复后需在可见窗口完成一次 7/7 启动、资产加载、仿真启动和基本交互验证。

## 建议后续顺序

> 2026-07-26：详细勾选式 TODO 清单已生成至根目录 `TODO.md`（含优先级、修改文件、C# 参考位置），以下顺序以其为准。

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
