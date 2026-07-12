# Gorge 项目记忆

## 项目概览
- **项目名称**: Gorge — 自定义面向对象编程语言及其自举编译器
- **目标语言**: Rust
- **运行时架构**: 解释型虚拟机（类型分离栈），保持与 C# 参考实现大体兼容
- **参考实现**: `references/gorge-compiler/`（C# 编译器）和 `references/gorge-core-csharp/`（C# 运行时）

## 项目结构
```
Gorge/
├── Cargo.toml                    # workspace
├── AGENTS.md / MEMORY.md
├── references/
│   ├── gorge-compiler/src/       # C# 参考实现：Gorge.g4, GorgeExpression.g4, GorgeLexerRules.g4, GorgeStatement.g4 + 全部 C# 源码
│   └── gorge-core-csharp/src/    # C# 运行时参考实现：VirtualMachine, GorgeObject, GorgeClass 等
├── GorgeCore/                    # lib crate (rlib) — 运行时核心 + 共享模块
│   └── src/   diagnostics.rs, ir.rs, vm.rs, bytecode.rs (17 tests)
├── GorgeCompiler/                # bin crate (gorgec + gorge) — 编译器前端
│   └── src/   ast.rs, lexer.rs, parser.rs, symbol.rs, compiler.rs, codegen.rs, optimizer.rs
│              main.rs → gorgec, vm_main.rs → gorge  (80 tests)
```

## C# 参考实现关键信息

### 词法规则 (GorgeLexerRules.g4)
- 42 个关键字: do, new, for, while, default, case, switch, else, if, return, static, extends, native, class, interface, inject, null, int, float, bool, string, enum, this, super, void, injector, invokes, break, continue, delegate, object, auto, using, namespace
- 操作符: =, ?, ||, &&, ==, !=, <=, >=, +, -, *, /, %, !
- 界符: ( ) [ ] { } < >
- 分隔符: , : :: ; .
- **标识符字符: @ (At), ^ (Caret), -> (LambdaArrow)**
- 字面量: IntLiteral(0|非零开头), FloatLiteral(IntLiteral.Digit+), BoolLiteral(true|false), StringLiteral("..." 支持转义)
- 隐藏通道: 空白/块注释/行注释/换行

**⚠ Rust lexer 缺失**: `@`, `^`, `->`, `::`, `do`, `inject`, `invokes`, `auto`, `super`, `default`, `extends`, `null`(已有) token

### 表达式语法 (GorgeExpression.g4) — 14 级优先级
1. 赋值 = (右结合)
2. 条件 ?: (右结合)
3. 逻辑或 || (左结合)
4. 逻辑与 && (左结合)
5. 相等 == != (左结合)
6. 比较 < > <= >= (左结合)
7. 加减 + - (左结合)
8. 乘除模 * / % (左结合)
9. 一元右结合: 取反- 逻辑非! 强制转换(T) (右结合)
10. 一元左结合: 成员访问. 注入器访问.^ 方法调用() 注入器字面量:{} 数组访问[] Lambda:(){} 泛型<> (左结合)
11. 主表达式: 字面量 ( ) new Delegate<>

赋值目标: this | identifier | target.field | target.^field | target[index]

### 语句语法 (GorgeStatement.g4)
- 6 种代码块: NormalBlock, IfBlock(含Else), SwitchBlock, WhileBlock, DoWhileBlock, ForBlock
- 语句: 本地变量声明 `type name [= expr];` | 表达式语句 | return [expr] | break [target*] | continue [target*]
- break/continue 支持多层目标: 数字(层数) 或 关键字(for/while/switch/else/if/do)

### 顶层语法 (Gorge.g4)
- using: `using [alias =] expr;`
- namespace: `namespace name(.name)*;`
- 类: `annotation* [native] class Name : superClass :: Interface, Interface { members }`
- 接口: `[native] interface Name { methods }`
- 枚举: `[native] enum Name { values }`
- **注解**: `metadata? @Name genericType? (key=value, ...)`
- metadata: `[ expr name [= expr], ... ]`
- 字段: `annotation* type name [= expr];`
- 方法: `annotation* [static] type name(params) { body } | ;`
- 构造: `annotation* [injector] name(params) [: super(args)] { body } | ;`
- 参数: `type name`

### 编译流程 (Four-Pass)
1. **Pass 1** (TypeIdentifierVisitor): 收集命名空间/类/接口/枚举标识符 → 符号表骨架
2. **Pass 2** (TypeExtensionVisitor): 解析 using 引用、超类/接口继承关系、枚举值声明
3. **Pass 3** (TypeDeclarationVisitor): 声明类成员(字段/方法/构造)、深度优先遍历继承树、冻结继承→产出 CompileTask 列表
4. **Pass 4** (CompileTask 执行): BlockListVisitor → BlockVisitor → StatementVisitor → ExpressionVisitor → AppendCodes() 生成 IR → 优化器优化

### 中间代码操作码分类 (IntermediateOperator)
- 本地变量赋值: LocalInt/Float/Bool/String/ObjectAssign
- 算术: Int/FloatAddition/Subtraction/Multiplication/Division/Remainder, Int/FloatOpposite, StringAddition
- 比较: Int/FloatLess/Greater/LessEqual/GreaterEqual
- 相等: Int/Float/Bool/String/ObjectEquality/Inequality
- 逻辑: LogicalAnd/LogicalOr/LogicalNot
- 类型转换: IntCastToFloat, FloatCastToInt, Int/Float/BoolCastToString, ObjectCastToObject
- 字段读写: Load/Set Int/Float/Bool/String/ObjectField (需要字段索引)
- 注入器字段: Load/Set Int/Float/Bool/String/ObjectInjectorField
- 方法调用: InvokeMethod, InvokeStaticMethod, InvokeInterfaceMethod, InvokeConstructor, InvokeInjectorConstructor, InvokeDelegate, ConstructDelegate, InvokeArrayConstructor
- 参数: Set/Load Int/Float/Bool/String/ObjectParameter, GetReturn*
- 控制流: Jump, JumpIfFalse, JumpIfTrue, Nop
- 其他: LoadThis, LoadInjector, SetInjector, DoConstruct, Return*

### 符号表作用域层次
```
GlobalScope → NamespaceScope (N层) → ClassScope/InterfaceScope/EnumScope
  ClassScope → FieldScope, MethodGroupScope→MethodScope, ConstructorGroupScope→ConstructorScope, InjectorScope, AnnotationScope→MetadataScope, GenericsSymbol[]
  InterfaceScope → MethodGroupScope→MethodScope
  EnumScope → EnumValueScope
```

### 类型系统
- 基本类型: Int, Float, Bool, String, Object, Enum, Interface, Delegate, Null
- 自动转换: Int→Float, Enum→Int, Class→父类/接口, Null→任意Object, Object→祖先
- 强制转换: CanCastTo = 自动转换 ∪ 反向自动转换

### 优化器算法
1. 基本块划分 (leader识别: 首指令/跳转目标/跳转后继/返回后继)
2. 活跃变量分析 (反向数据流: Out=∪In, In=(Out-Def)∪Use, 迭代至不动点)
3. 死代码消除 (删除出口不活跃的根节点和非活跃多重定值)
4. 可用表达式分析 (前向数据流: In=∩Out, Out=(In-Kill)∪Gen, 迭代至不动点)
5. 公共子表达式消除 (复制传播 + 合并定值节点 + 回填临时变量)
6. 代码重建 (拼接基本块 + 回填跳转目标)
- 整个流程迭代 4 次

### DAG 副作用分析 (DoKill)
- SetField → 杀死 LoadField
- SetInjectorField → 杀死 LoadInjectorField
- Invoke/Construct → 杀死所有 Load/GetReturn (保守)
- Return → 杀死 GetReturn

## 当前 Rust 实现进度

### Phase 1-9 全部完成
| Phase | 模块 | Tests | 状态 |
|-------|------|-------|------|
| 1 | diagnostics + ast + lexer | 16 | ✅ |
| 2 | parser (Pratt + 递归下降) | 28 | ✅ |
| 3 | symbol (Arena + Scope + TypeInfo) | 12 | ✅ |
| 4 | compiler (Pass1 + Pass2) | 14 | ✅ |
| 5 | compiler (Pass3 + CompileTask) | 6 | ✅ |
| 6 | GorgeCore/ir.rs + codegen.rs (Pass4) | 10 | ✅ |
| 7 | GorgeCore/vm.rs (类型分离栈 VM) | 6 | ✅ |
| 8 | optimizer.rs (基本块 + DCE + 重建) | 7 | ✅ |
| 9 | GorgeCore/runtime.rs + vm.rs 参数/字段操作码 | 2 | ✅ |

### 字节码 + CLI + 运行时
| 模块 | 说明 |
|------|------|
| GorgeCore/bytecode.rs | 序列化/反序列化 (4 tests) |
| GorgeCore/param_pool.rs | 调用参数池，参数/返回值传递 (3 tests) |
| GorgeCore/runtime.rs | 运行时注册中心，类层次/类型转换判定 (2 tests) |
| gorgec (main.rs) | `.g` → 编译 → `.gorge` |
| gorge (vm_main.rs) | `.gorge` → VM 执行 |
| 端到端测试 | test.g 编译并执行通过 |

### 总计: 194 个测试, 零 warning

### Phase F 完成 (2026-07-10)：接口方法映射 + native 类被继承（T5/T13/B-4）
- **F1 接口方法映射**:
  - `symbol.rs`: ClassInfo 新增 `interface_method_impl_id: Map<接口名, Vec<类方法全局ID>>`
  - `compiler.rs`: freeze 阶段 `build_interface_impl_map`/`find_impl_method_global_id` 按名字+签名匹配类方法建映射
  - `bytecode.rs`: CompiledClass 新增 `interface_method_impl_id` 并序列化/反序列化
  - `codegen.rs`: `resolve_interface_method`（接口方法本地ID+接口名+返回类型）；变量为接口类型（var_types 记录 TypeInfo::Interface）时发 `InvokeInterface(本地ID)`+right 存接口名
  - `vm.rs`: `InvokeInterface` 读对象类名→查 `interface_method_impl_id[接口名][本地ID]`→类方法全局ID→实例分派流程
  - `gorge_runner`: RuntimeClass 填 `interface_method_impl_id`
  - **注意**：接口继承语法用 `::`（`class Rect :: IShape`），`:` 是父类
  - 端到端 `IShape s = new Rect(3,4); s.area()`=12
- **B-4 + F2 native 被编译类继承**:
  - B-4：编译器从手写 native 存根（`native class Vector2 { 字段; 方法签名; }`）推导字段计数/方法数/构造，参与 freeze 继承编号冻结
  - `vm.rs` `dispatch_native_construct`: **仅 target=None（新建）时归一化 class_name 为 native 类名；target=Some（super 调用到 native 父类）保留子类 class_name**（关键 bug 修复）
  - `vm.rs` `find_native_ancestor` + `class_super_name` 映射：编译子类调用继承自 native 父类的方法时，find_method 失败则沿父类名链找 native 祖先，方法全局ID 直接作 native 方法索引分派
  - `gorge_runner`: `register_class_super` 注册父类名（含 native 父类，因 class_table 不含 native）
  - 端到端 `class Labeled : Vector2`：继承 native 方法 get_x=3、子类字段 getLabel=7、super(x,y) 到 native Vector2 构造
- **示例** (`GorgeFramework/examples/`): interface.g、native_inherit.g
- **测试统计**: 主 workspace 182(gorgec 110 + gorge_core 72)、框架 12(gorge_framework 6 + gorge_macros 6)，全绿零 warning
- **后续**: 见 `reports/csharp-parity-plan.md`，Phase G（注入器主线，框架核心价值）

### Phase E 完成 (2026-07-10)：类型推导 + 转换规则 + cast + 重载解析（T2/T7/T8）
- **E1 类型推导器** (`codegen.rs`): `infer_type(&Expression)->TypeInfo` 覆盖字面量/变量/字段/方法返回/new/cast/二元/一元/条件；新增 `var_types`/`field_types` 映射（compiler 注册参数类型，set_class_context 注册字段类型）；`resolve_type_ref`(TypeRef→TypeInfo)；4 单测
- **E2 类型转换判定**: 编译期 `can_auto_cast`/`can_cast`(TypeInfo，查符号表继承链：Int→Float/Enum→Int/子类→父类/类→接口/数组协变/Delegate 协变逆变)；runtime `can_auto_cast_to`(GorgeType) 补齐 Enum→Int/数组协变/Delegate 协变逆变/null→String/接口→Object；2 单测
- **E3 强制转换** (T2): parser `try_parse_cast`（`(Type)expr` 消歧：内建类型关键字必为 cast；标识符需后接可开启表达式的 token 避免与括号表达式混淆）；codegen `generate_cast`（E1 源类型+E2 校验+选操作码）；新增 `ObjectCastToObject` 操作码（ir/bytecode=67/vm）；端到端 (int)3.7=3、(float)5/2.0=2.5
- **E4 重载解析** (T8): `MatchLevel`(Exact/Castable/None) 三级匹配；`match_params`/`resolve_instance_method`(实例含继承链)/静态调用/`resolve_constructor`(构造) 按参数类型选重载，歧义报错；**配套修复关键 bug**：`find_matching_method_body`/`find_matching_constructor_decl` 按签名归属方法体/构造体（此前同名重载都错取第一个 body）；collect_classes 方法收集改为按声明顺序（此前同名方法都映射到第一个编译体）；VM `copy_params_to_locals` 按值类型分组把参数池复制到 callee 局部（不 reset 池，兼容 LoadParameter）；构造后重新确立 this@object_stack[0]（修复 max_locals=1 时零化 this 的 bug）
- **端到端验证** (`GorgeFramework/examples/`): cast.g、overload.g(方法重载 add(int)/add(int,int)=5、构造重载 Calc()/Calc(int)=105)
- **⚠ 已知遗留**: 无（E4 完成时一并修复了 total_locals 相关的多临时方法问题——实为方法体错配所致，已解决）
- **测试统计**: 主 workspace 182(gorgec 110 + gorge_core 72)、框架 12(gorge_framework 6 + gorge_macros 6)，全绿零 warning
- **后续**: 见 `reports/csharp-parity-plan.md`，下一步 Phase F（接口方法映射 F1、native 被继承 F2）

### Phase D 完成 (2026-07-10)：break/continue 多层/按类型离块（T1）
- **parser** (`parser.rs`): `parse_break_targets` 解析 break/continue 后目标序列——整数→`ByLayer(n)`、for/while/switch/do/if/else 关键字→`ByKeyword`；空默认 `[ByLayer(1)]`
- **codegen** (`codegen.rs`):
  - 新增 `BlockKind`(For/While/DoWhile/Switch/If/Else) + `PendingLeave`(占位Jump+目标队列VecDeque+is_break) + `BlockCtx`
  - `emit_leave`: break/continue 发 `Jump(0)` 占位并登记 PendingLeave
  - `backpatch_block`: 每个控制流块结束时对未完成离块任务尝试回填——队首 ByLayer(n) 减到0出队/ByKeyword 匹配块类型出队，队清空则回填（对齐 C# `LeaveBlockBackPatchTask.TryBackPatch`）
  - if/while/for/do-while/switch 生成时 push BlockCtx、结束时 backpatch_block；break→块尾、continue→续点(while:条件复检/for:update段/do:条件/switch:块尾)
  - `report_unresolved_leaves`: 方法体生成后仍有未回填任务→报编译错误（越层/无匹配块）
- **⚠ Gorge 语义（对齐 C#）**: **if/else 也算一层**，plain `break` 会被最内层 if 捕获；从 if 内跳出循环需 `break while` 或把 if 计入层数（如 `break 2`）
- **IR 无新增操作码**（复用 Jump/Nop），字节码格式不变
- **验收**: 5 parser 单测 + 端到端 `examples/break_continue.g` 4 场景(break while=10/continue while=8/break 3 跨层=76/switch内 break while=100) + 越层报错用例；主 workspace 176(gorgec 106 + gorge_core 70)、框架 12，全绿零 warning
- **后续路线**: 见 `reports/csharp-parity-plan.md`，下一步 Phase E（类型推导基础设施 E1→E2→E3→E4）

### GorgeFramework 移植计划（进行中）
- **决策**: 目标=核心骨架+示范类(Math/Vector2); 桥接=proc-macro(GorgeMacros); 先补齐 GorgeCore; 平台层仅抽象 trait(本轮不做)
- **crate 命名**: `gorge_framework`(native 类库，待建) + `gorge_macros`(proc-macro，已建)
- **⚠ workspace 拆分 (2026-07-10)**: GorgeFramework 相关拆到独立 workspace
  - 主 workspace `Gorge/`(根 Cargo.toml): 只含 `GorgeCompiler` + `GorgeCore`(编译器与运行时)
  - 新 workspace `Gorge/GorgeFramework/`(独立 Cargo.toml): 含 `GorgeMacros` + `GorgeFramework`(native 类库) + `GorgeRunner`(端到端运行入口)
  - 依赖: 新 workspace 通过相对路径 `../../GorgeCore` 依赖主仓库 GorgeCore
  - 两个 workspace 各有独立 Cargo.lock 与 target/；`.gitignore` 已加 `/GorgeFramework/target/`
  - 构建: 主 workspace 在 `Gorge/` 下 `cargo build/test`；framework 在 `Gorge/GorgeFramework/` 下 `cargo build/test`
- **Step 0 完成**: 产出 `reports/native-id-alignment.md` — 编译器编号对齐规则
  - 关键结论: 静态方法用「静态+实例混合 Vec 下标」编号(非 C# 双编号空间); 字段/参数 index 编译器用「全类型统一递增」而运行时是「类型分离」，混合类型场景会错位
  - 编译器修正 Backlog: B-1(字段 index✅已修) / B-2(参数 index，并入 Phase A) / B-3(继承编号冻结) / B-4(native 类导入编译器✅Phase C 打通跨类分派)
- **Phase A 完成**: GorgeCore native 互操作打通
- **Phase B 完成 (2026-07-10)**: GorgeMacros proc-macro 桥接宏
- **Phase C 完成 (2026-07-10)**: native 类库 + 跨类分派 + 局部变量类型追踪 + gorge_runner 端到端
- **B-2 完成 (2026-07-10)**: 参数 index 按值类型分组（codegen + 宏两侧对齐），混合类型参数方法端到端
- **B-3 完成 (2026-07-10)**: 继承编号冻结 + 方法重写分派 + 字段继承 + super 构造链，编译类继承编译类端到端

### B-2 / B-3 关键信息（编译器继承与参数分组）
- **B-2 参数分组**:
  - `codegen.rs`: `param_index: usize` → `ParamIndexCounters`（int/float/bool/string/object 各独立计数）；新增 `emit_set_param` 统一 6 处参数布置点，按值类型分组分配 index，result 地址值类型统一用 Int 占位（避免与局部变量地址冲突被优化器误删）
  - `GorgeMacros/impl_macro.rs`: `grouped_param_indices` 让宏生成的参数读取也按值类型分组，与 codegen 对齐
  - 验证: `Vector2.lerp(Vector2,Vector2,float)` 混合参数端到端 = 5/10
- **B-3 继承编号冻结**:
  - `symbol.rs`: `ClassInfo` 新增 `method_start_id`/`method_count_total`/`constructor_start_id`/`constructor_count_total`/`method_override_id`/`field_start_type_count`/`field_type_count_total`；新增 `FrozenTypeCount`
  - `compiler.rs`: Pass3 后 `freeze_inheritance` 按继承深度排序，父类先算，子类起始值 = 父类总数；`find_overridden_method` 按名+参数匹配祖先建重写映射
  - `codegen.rs`: `resolve_instance_method` 沿继承链返回全局方法 ID；`set_class_context` 字段用「声明类 field_start + 局部 offset」全局索引；`generate_new` 用目标类 constructor_start_id；新增 `emit_super_constructor_call`（构造体开头发 super 调用）
  - `ir.rs`/`bytecode.rs`: 新增 `InvokeSuperConstructor(usize)` 操作码（码 91），right 携带父类名；CompiledClass 序列化 method_start_id/method_count_total/constructor_start_id/method_override_id/field_start_counts
  - `vm.rs`: `InvokeSuperConstructor` 在当前 this 上执行父类构造体（不新建对象）；InvokeConstructor 用全局 ctor id 经 find_constructor 分派
  - `main.rs`: `CompiledMethodContents` 新增 class_id/is_constructor，`collect_classes` 按 class_id 精确归属方法（修复同名方法跨类错配 bug）
  - `gorge_runner`: 按继承深度构建 RuntimeClass 链；对象字段计数用「父类起始+本类」总数（否则继承字段越界 panic）
  - 验证 (`GorgeFramework/examples/inheritance.g`): `d.sound()`(重写)=42、`d.getLegs()`(继承方法+super初始化字段)=4、`d.getLoyalty()`(子类)=100
- **⚠ 未做**: native 类被编译类继承（依赖 A5 双向引用在构造流程生效）留待后续

### Phase C 关键信息：native 类库 + 端到端打通
- **native 类库** (`GorgeFramework/GorgeFramework/`): `Math`(8 静态方法) + `Vector2`(字段/构造/静态 distance,scale,lerp/实例 magnitude,get_x,get_y) + `register_native`/`native_classes` 入口，6 单测经 NativeContext 验证
- **B-4 跨类分派**（编译器+VM 核心补齐）:
  - `codegen.rs`: `InvokeStatic`/`InvokeConstructor` 的 `right` 操作数携带目标类名（String 立即数）；`generate_new` 从 TypeRef 提取类名
  - `vm.rs`: `read_target_class` 从 right 解析目标类；按目标类分派（native 查 native 表、编译类查方法表并 save/switch/restore current_class）
  - `dispatch_native_construct`: 归一化新对象 class_name 为注册键（native 桥接默认用全名建对象，注册键是简单名）
- **局部变量类型追踪 + native 实例方法解析**（编译器新增能力）:
  - `codegen.rs`: 新增 `var_class: HashMap<String,String>`（变量名→类名），VariableDeclaration 从 var_type 记录、参数从 TypeInfo::Object 记录（compiler.rs generate_method_ir 中 `register_var_class`）
  - `resolve_instance_method`: 按类名+方法名解析实例方法编号与返回类型；`变量.方法()` 生成正确 `InvokeInstance(method_id)`（left=对象引用, right=类名, 返回类型正确）
- **gorge_runner**: 加载 .gorge + 注册 native(简单名+全名为键) + 注册编译类(继承链) + 执行入口；`gorge_runner <file.gorge> [类.方法]`
- **端到端验证** (`GorgeFramework/examples/`): native_math.g → `Math.sqrt(16)`=4；native_vector2.g → `Vector2.distance`=5、`v.magnitude()`=10、`TestLerpX`=5；inheritance.g → 42/4/100
- **⚠ 关键约束（对齐规则 M1 实证）**: `.g` 的 native class 存根声明的方法**顺序与集合必须与宏 impl 完全一致**，否则方法编号错位（曾因存根漏写 scale 导致 magnitude 编号从 2 错成 1）
- **测试统计**: 主 workspace 171(gorgec 101 + gorge_core 70)；框架 workspace 12(gorge_framework 6 + gorge_macros 6)。全绿零 warning

### Phase B: GorgeMacros 桥接宏（proc-macro）关键信息
- **两个属性宏**:
  - `#[gorge_native_class(namespace="...")]` 标注 struct：解析 `#[gorge_field]`/`#[inject(default=..)]`，生成 `GORGE_FULL_NAME`、`gorge_field_type_count()`、`gorge_injector_field_type_count()`、`FIELD_INDEX_<name>`/`INJECTOR_INDEX_<name>` 常量、`gorge_injector_default_<name>()`；**自动 derive(Debug)，禁止手动 derive**
  - `#[gorge_native_impl]` 标注 impl：方法用 `#[gorge_static]`/`#[gorge_method]`/`#[gorge_ctor]` 标注，生成 `impl NativeClass`
- **方法编号**: 静态+实例共享混合编号空间(声明序从0)，构造方法独立编号(声明序从0) — 对齐规则 M1/C1
- **参数约定**: 第一参数 `ctx: &mut NativeContext`(按引用类型识别跳过)；实例/构造方法需 `this: usize`(按参数名跳过)；其余为值参数
- **类型映射**: i32/i64→Int, f32/f64→Float, bool→Bool, String→String, usize→Object(对象ID)。宏自动做 i32↔i64/f32↔f64 转换；但 NativeContext 字段访问器用 f64/i64，业务方法体内需自行 `as` 转换
- **crate 结构**: `GorgeFramework/GorgeMacros/src/` = lib.rs(宏入口) + class_macro.rs + impl_macro.rs + common.rs(类型映射)；`tests/native_bridge.rs`(6测试：Math纯静态 + Vector2字段/构造/实例/静态)
- **field_type_count 返回引用**: 用 `OnceLock<TypeCount>` 缓存（trait 要求返回 `&TypeCount`）

### 最近修改 (2026-07-10)

3. **Phase B: GorgeMacros proc-macro 桥接宏**
   - 新建 `GorgeMacros` crate（proc-macro=true，依赖 syn 2.0/quote/proc-macro2），加入 workspace
   - `lib.rs`: 两个属性宏入口 `gorge_native_class` / `gorge_native_impl`（含完整中文文档）
   - `common.rs`: `ValueKind` 值类型枚举 + Rust 类型→Gorge 值类型映射 + 参数读取/返回值写入胶水代码生成（含 i32↔i64/f32↔f64 转换）
   - `class_macro.rs`: 解析 struct 字段/注入器字段，生成元数据/索引常量/默认值方法，自动 derive(Debug)
   - `impl_macro.rs`: 解析 impl 方法分类分派，生成 `impl NativeClass`（invoke_native_static/method + do_construct_native），参数拆箱、返回值装箱
   - `tests/native_bridge.rs`: Math(纯静态 abs/add_one) + Vector2(字段 x/y、构造 new、静态 distance、实例 get_x) 共 6 测试，经 NativeContext 验证桥接正确
   - 验收: workspace 174 测试全绿（gorgec 99 + gorge_core 69 + gorge_macros 6），零 warning

2. **Phase A: GorgeCore native 互操作打通**
   - `native.rs`(新建): `NativeContext<'a>` 上下文（借用 param_pool/objects/next_object_id，提供参数读取/返回值写入/对象字段访问/对象创建 API）；`NativeClass` trait（对应 C# `Implementation:GorgeClass`，方法 `invoke_native_method`/`invoke_native_static`/`do_construct_native`/`make_empty_object`）
   - `param_pool.rs`: 新增 `injector` 专用位（对应 C# `InvokeParameterPool.Injector`）+ `get_injector`/`set_injector`，reset 含 injector
   - `runtime.rs`: `GorgeRuntime` 新增 `native_classes` 表 + `register_native_class`/`get_native_class`/`is_native_class`（注册时同步注册到 VM）
   - `vm.rs`: 新增 `native_class_table` 字段 + `register_native_class`；新增 native 分派辅助 `dispatch_native_static`/`dispatch_native_method`/`dispatch_native_construct`/`write_native_return_to_result`；`InvokeStatic`/`InvokeConstructor`/`InvokeInstance` 顶部增加 native 分支（当前类/目标对象类为 native 时走桥接）
   - `object.rs`: `RuntimeObject` 新增 `native_object_id`/`outer_compiled_id` 双向引用字段（A5）
   - `vm.rs`: 新增 `link_native_and_compiled`（建立双向引用）+ `resolve_real_object_id`（对应 C# `RealObject`）
   - **A5 注意**: 双向引用机制完整实现并单测验证，但真正触发依赖 B-3(继承编号冻结)+B-4(native 类导入)，本轮端到端不触发
   - 新增测试 5 个: native.rs 2(静态/构造经 Context)、vm.rs 3(IR 调 native 静态方法/native 构造+实例方法/双向引用)

1. **B-1 修复：字段 offset 按值类型分组分配**
   - `compiler.rs`: 新增 `FieldOffsetCounters` 结构体，包含 int/float/bool/string/object 五个独立计数器
   - `pass3_declare_class_members`: 单一 `field_offset` → `FieldOffsetCounters`
   - `pass3_declare_field`: 按字段值类型取对应计数器当前值作为 offset，然后该类型计数器 +1
   - 新增测试 `test_pass3_field_offset_by_value_type`：混合类型字段 offset 验证
   - 现有测试 `test_pass3_field_offset_allocation`（纯 float 字段）行为不变，通过

### 最近修改 (2026-07-07)
1. **LambdaBody 枚举重构**
   - `ast.rs`: 新增 `LambdaBody` 枚举（`Expression(Box<Expression>)` | `Block(Vec<Statement>)`），`Expression::Lambda.body` 从 `Box<Expression>` 改为 `LambdaBody`
   - `parser.rs`: 4 处 Lambda 构造点全部更新，新增 `{` 检测支持 Lambda 语句块体（`x -> { stmts }`）
   - `codegen.rs`: Lambda 代码生成改为 `match body` 处理表达式体和块体；新增 `analyze_free_vars_lambda_body` 和 `collect_free_vars_from_stmt` 辅助函数

### 最近修改 (2026-07-06)
1. **Injector/Delegate 动态组装方案 Steps 5-8**
   - `injector.rs`: `RuntimeInjector` 新增 `from_defs` 方法，从 `InjectorFieldDef` 列表动态构造注入器，按类型区分索引
   - `delegate.rs`: 重写 `RuntimeDelegate`，新增 `from_def` 构造方法（从委托定义和外部值映射动态构造），`GorgeDelegate::invoke` 为占位实现
   - `main.rs`: `collect_classes` 之后将 `compiler.injector_fields` 转换为 `bytecode::InjectorFieldDef` 并附加到每个 `CompiledClass`
   - `vm_main.rs`: 注册类之后，为有注入器字段的类调用 `RuntimeInjector::from_defs` 动态组装注入器
2. **Injector/Delegate 动态组装方案 Steps 1-4**
   - `compiler.rs`: 新增 `InjectorFieldDef` 结构体，`Compiler` 新增 `injector_fields` 字段，Pass 3 中收集注入器字段
   - `codegen.rs`: 新增 `analyze_free_vars` / `collect_free_vars` 自由变量分析函数，重写 Lambda 代码生成为委托构造
   - `bytecode.rs`: 新增 `DelegateImpl` / `InjectorFieldDef` 结构体，`CompiledClass` 新增 `injector_fields` / `delegate_impls` 字段，更新序列化/反序列化逻辑
   - `main.rs`: 所有 `CompiledClass` 构造处补上 `injector_fields` / `delegate_impls` 空 Vec
2. **S2 扩展: DoWhile + InjectorField 赋值目标**
   - `ast.rs`: 新增 `Statement::DoWhile { body, condition, span }` 变体及 `span()` 匹配
   - `ast.rs`: 新增 `AssignmentTarget::InjectorField { object, field, span }` 变体及 `span()` 匹配
   - `parser.rs`: 新增 `parse_do_while_statement` 方法（`do { body } while (condition);`），`parse_statement` 中优先匹配 `KwDo` 再匹配 `KwWhile`
   - `parser.rs`: `parse_infix` 的 `Token::Dot` 分支支持 `^` 注入器字段访问（`obj.^field`），`MemberAccess` 的 member 字段以 `^` 前缀标记
   - `parser.rs`: 赋值目标解析中检测 `MemberAccess.member.starts_with('^')` → `AssignmentTarget::InjectorField`
   - `codegen.rs`: 新增 `InjectorField` 匹配臂（暂为占位实现）
2. **Phase 4: 字节码格式扩展 + CLI 完善**
   - `bytecode.rs`: 新增 `CompiledModule` / `CompiledClass` 结构体，新增 `serialize_module` / `deserialize_module`（支持版本 2 格式，含类元数据），旧版 `serialize`(v1) 和 `deserialize` 保留兼容
   - `main.rs`: 序列化改用 `serialize_module`，新增 `collect_classes` 从符号表构建类元数据
   - `vm_main.rs`: 改用 `deserialize_module` + `GorgeRuntime` 加载并执行类
2. **runtime.rs**: 新建 GorgeRuntime — 类/接口/枚举注册中心，`can_auto_cast_to` / `can_cast_to` 类型转换判定（子类/接口检查）
2. **vm.rs**: 
   - 新增 `param_pool` 字段，SetParameter/LoadParameter/GetReturn 操作码完整实现
   - 新增临时字段存储 `field_*_storage`，LoadField/SetField 模拟实现
   - Invoke*/DoConstruct/ConstructDelegate/LoadInjector/SetInjector 简化为 Nop（带 TODO 注释）
   - Injector 字段读写操作为占位 Nop
3. **param_pool.rs**: 新增 `get_float_return`/`set_float_return`、`get_bool_return`/`set_bool_return`、`get_string_return`/`set_string_return`、`get_object_return`/`set_object_return` 方法；添加 `Clone` derive
4. **lib.rs**: 新增 `pub mod runtime;`
1. **lexer.rs**: 测试 `test_integer_leading_zero_rejected` → `test_integer_leading_zero_split`；`IntLiteral` 移到 `FloatLiteral` 前避免 Logos DFA 冲突；`FloatLiteral` 正则改为 `[1-9][0-9]*\.[0-9]+\|0\.[0-9]+` 消除前缀重叠
2. **parser.rs**: 类继承语法从 `extends` → `:`（`KwExtends` → `Colon`）；测试 `test_parse_class_with_inheritance` 输入改为 `class Dog : Animal :: IPet , IBark { }`
3. **symbol.rs + compiler.rs**: 完整实现 namespace/using 支持
   - Scope 新增 `using_scopes: Vec<ScopeId>` 字段
   - NamespaceInfo 新增 `using_scopes`，ClassInfo/InterfaceInfo/EnumInfo 新增 `namespace_scope_id`
   - `push_scope` 继承父域 using_scopes
   - 新增 `lookup_local_only`（仅本级+父链，不查 using）
   - `lookup` 重写为三级搜索：本级 Symbols → 向上 Parent → 横向 Usings
   - `declare_class/interface/enum` 自动将所属命名空间加入 using_scopes
   - `declare_namespace` 的 NamespaceInfo.using_scopes 从父域继承
   - `find_enum_by_name` 添加到 SymbolTable
   - Pass 2 中 `for _using` 循环替换为真正的 using 解析逻辑
   - `add_using_to_members` 辅助方法将 using 作用域注入到类型成员

## Rust 实现与 C# 参考实现的差异 (TODO)
1. lexer 缺少: `@`(At), `^`(Caret), `->`(LambdaArrow), `::`(DoubleColon), `do`, `inject`, `invokes`, `auto`, `super`, `default`, `extends`
2. parser 注解语法: C# 用 `@Name(...)`, Rust 用 `[Name]` (需对齐)
3. parser Lambda: C# 用 `->`, Rust 用 `=>` (需对齐)
4. parser 继承语法: C# 用 `: super :: iface`, Rust 用 `extends/implements`
5. parser 注入器访问: C# 用 `.^field`，Rust 已支持解析（MemberAccess 带 `^` 前缀），赋值目标已支持 InjectorField
6. 类型系统: C# 引用实现有完整的自动转换和强制转换规则, Rust 版 TypeInfo 较简化
7. 操作码: C# 版有 Load/SetParameter、GetReturn、CastToString、InvokeArrayConstructor 等, Rust 版部分缺失
8. 优化器: Rust 版实现了基本块+简化 DCE, 缺少全局 CSE(公共子表达式消除)和 DAG 副作用分析

## 关键决策
1. **框架选型**: Logos + 手写递归下降 + 自定义 IR/VM
2. **Crate 划分**: GorgeCore = 运行时 + 共享; GorgeCompiler = 编译器前端
3. **注释规范**: `///` 公共 API + `//` 内部逻辑, 中文
4. **crate-type**: 当前只用 `rlib`
5. **符号表设计**: Arena + newtype ID, Scope 树嵌套查找
6. **VM 设计**: 类型分离栈, 帧管理由调用者控制
7. **字节码格式**: 自定义二进制格式（Magic "GORG" + Version + 数据体），v1 仅含方法列表，v2 扩展支持类元数据（类名/is_native/字段计数/父类/接口/方法）
