# Rust 移植版与 C# 参考实现的不一致清单（TODO）

> 生成日期：2026-07-10。基于对 `references/gorge-compiler/`（C# 编译器）、
> `references/gorge-core-csharp/`（C# 运行时）的完整分析，对照 Rust 侧
> `GorgeCompiler` + `GorgeCore` + `GorgeFramework` 的实现现状。
>
> 状态图例：❌ 未实现 / ⚠️ 部分实现（占位或简化） / ✅ 已实现（本清单不列）

---

## 一、语言语义 —— 严重缺口（影响正确性）

### T1. ⚠️ break / continue 多层/按类型离块 —— **仅生成 Nop**
- **现状**：`codegen.rs:1208` `Statement::Break | Statement::Continue` 直接 `emit(nop())`，完全不跳转。
- **C# 语义**：`break N`（跳 N 层）、`break for/while/switch/if/else/do`（按类型跳）、可组合 `break for for`；靠 `LeaveBlockBackPatchTask` 队列回填，每个代码块有 continue/break 双入口。
- **影响**：所有含 break/continue 的循环/switch 行为错误。**高优先级**。
- **涉及**：`codegen.rs`（循环/switch 生成块入口 + break/continue 回填）、AST 已有 target 信息。
- **✅ 已完成（Phase D，2026-07-10）**：parser 解析多层目标（ByLayer/ByKeyword）；codegen 引入 `BlockKind`+`PendingLeave` 队列+`backpatch_block` 回填（对齐 C# `LeaveBlockBackPatchTask`）；if/while/for/do-while/switch 块上下文 + break/continue 落点（break→块尾、continue→续点）；**if/else 也算层**（对齐 C#，plain `break` 会被最内 if 捕获，跳循环需 `break while` 或计入层数）；越层/无匹配块严格报编译错误。端到端 4 场景（break while=10、continue while=8、break 3 跨层=76、switch内 break while=100）+ 5 parser 单测全通过。

### T2. ⚠️ 强制类型转换表达式 `(T)expr` —— **codegen 无处理**
- **现状**：codegen 无 `Expression::Cast` 分支；IR 有 IntToFloat/FloatToInt/IntCastToString 等操作码但未在表达式生成中使用。
- **C# 语义**：`(int)floatVal`、`(float)intVal`、对象向下转型 `ObjectCastToObject`。
- **影响**：强制转换无效或编译遗漏。
- **涉及**：`codegen.rs` 表达式生成、AST Cast 节点。
- **✅ 已完成（Phase E3，2026-07-10）**：parser 新增 `try_parse_cast`（`(Type)expr` 消歧：内建类型必为 cast，标识符需后接可开启表达式的 token）；codegen `generate_cast` 据 E1 源类型+E2 可转换性选操作码；新增 `ObjectCastToObject` 操作码（IR/bytecode 67/VM）。端到端 `(int)3.7=3`、`(float)5/2.0=2.5`。

### T7. ⚠️ 类型转换规则不完整
- **✅ 已完成（Phase E2，2026-07-10）**：runtime `can_auto_cast_to` 补齐 Enum→Int、数组协变、Delegate 协变/逆变、null→String、接口→Object；新增编译期 `can_auto_cast`/`can_cast`（TypeInfo 层，查符号表继承链）。两侧各 2 单测。

### T8. ❌ 方法重载解析（按参数类型）
- **✅ 已完成（Phase E4，2026-07-10）**：codegen `resolve_instance_method`（实例）/静态调用/`resolve_constructor`（构造）三级匹配（完全相等>全可自动转换>不匹配），歧义报错；配套修复 `find_matching_method_body`/`find_matching_constructor_decl`（按签名归属方法体/构造体，之前同名重载都取第一个）；VM `copy_params_to_locals` 按类型分组把参数复制到 callee 局部。端到端：方法重载 add(int)/add(int,int)、构造重载 Calc()/Calc(int) 全部正确（5/105）。

### T3. ⚠️ 注入器对象/数组字面量 codegen —— **占位 Nop / DoConstruct(0)**
- **现状**：`InjectorObject` 生成 `DoConstruct(0)` 但字段值未真正写入；`InjectorArray` 直接 `nop()`（codegen.rs:435-440）。
- **C# 语义**：`类型:{field:val,...}` 编译时构造注入器常量并作为立即数嵌入；数组注入器 `类型:{v1,v2}`。
- **影响**：谱面数据（Gorge 核心用途）无法真正构造。**高优先级**（框架核心场景）。

### T4. ⚠️ 注入器字段访问 `obj.^field` codegen —— **占位**
- **现状**：赋值目标 `AssignmentTarget::InjectorField` 与读取 `MemberAccess ^` 解析已支持，但 codegen 为占位（codegen.rs:686、724 附近"暂为占位"）。
- **C# 语义**：`Load/SetXxxInjectorField` 操作码（VM 侧已实现），需 codegen 正确发射。

---

## 二、类型系统与语义分析 —— 缺口

### T5. ❌ 接口方法实现映射（InterfaceMethodImplementationId）
- **现状**：`ClassDeclaration.interface_method_impl_id` 恒为空 HashMap；VM `InvokeInterface(_) => {}` 空实现（vm.rs:1205）。
- **C# 语义**：类冻结时建立「接口全名 → (接口方法ID → 类实现方法ID)」映射；`InvokeInterfaceMethod` 经此映射分派。
- **影响**：接口方法调用完全不工作。
- **✅ 已完成（Phase F1，2026-07-10）**：freeze 阶段 `build_interface_impl_map` 按名字+签名匹配建映射；字节码序列化 `interface_method_impl_id`；codegen 对接口类型变量调用发 `InvokeInterface(接口方法本地ID)`+right 存接口名；VM 查映射→类方法全局ID→实例分派；runner 注册映射。端到端 `IShape s=new Rect(3,4); s.area()`=12。

### T6. ❌ 泛型（generics）语义
- **现状**：parser 有 `TypeRef::Generic` 节点，但无泛型参数声明收集、类型参数替换、泛型实例化、参数匹配。
- **C# 语义**：类级 `<T1,T2>`、`GenericsType`、`CreateGenericsInstanceType`、方法重载的泛型匹配。
- **影响**：泛型类/方法无法使用。

### T7. ⚠️ 类型转换规则不完整
- **现状**：`runtime.rs::can_auto_cast_to` 有 Int→Float、null→Object、子类→父类、类→接口；缺 Enum→Int、数组协变、注入器协变、Delegate 协变/逆变、null→String。
- **C# 语义**：11 种自动转换规则 + 双向强制转换。
- **影响**：部分合法转换被拒或非法转换被放行。

### T8. ❌ 方法重载解析（按参数类型）
- **现状**：codegen 按方法名匹配（`resolve_instance_method` 只比名字），不比参数类型；构造方法只支持 0 号。
- **C# 语义**：`MatchArguments` 三级（完全相等 > 可转换 > 不匹配），含泛型匹配；构造方法按签名选择。
- **影响**：同名重载方法/多构造方法选择错误。

### T9. ❌ 语义校验/编译错误诊断种类
- **现状**：Rust 侧诊断较少。
- **C# 语义**：20+ 种编译异常（多重继承、循环继承、无效接口实现、重复实现、参数数量/类型不符、switch 条件类型、冻结后声明、引用未定义类型等）。
- **影响**：非法程序不报错或报错信息缺失。

---

## 三、运行时（VM）—— 操作码与机制缺口

### T10. ❌ IR 缺失的操作码（对照 C# 93 个）
- `IntOpposite` / `FloatOpposite`（取反，Rust 用 0-x 代替？需确认）
- `IntRemainder`（Rust 有 IntMod，命名不同，需确认 FloatRemainder 缺失）
- `FloatMod`（浮点取模）—— Rust 只有 IntMod
- `IntCastToFloat`/`FloatCastToInt`/`ObjectCastToObject` —— IR 有 IntToFloat 等但命名/用法不同，需统一
- `InvokeMethod`（C# 实例方法）对应 Rust `InvokeInstance`，命名不同
- `InvokeInjectorConstructor` —— ❌ 完全缺失
- `InvokeIntArrayConstructor` / Float/Bool/String/Object ArrayConstructor —— ❌ 缺失
- **影响**：注入器构造、数组构造、浮点取模等无法表达。

### T11. ❌ 注入器构造方法（injector constructor）机制
- **现状**：parser 能解析 `injector` 构造方法，但无 `InjectorConstructorImplementationId` 映射、无 `InvokeInjectorConstructor` 操作码与 VM 执行。
- **C# 语义**：注入器构造方法编号 → 实现构造方法编号映射；子类必须实现相同签名。
- **影响**：基于注入器直接构造对象（框架谱面核心）不工作。

### T12. ❌ 逻辑短路求值确认
- **C# 现状**：`LogicalAnd`/`LogicalOr` **无短路**（两操作数都求值）。Rust 需与之对齐或确认行为一致。

### T13. ⚠️ 静态方法/构造的继承分派（跨类）
- **现状**：B-3 已做编译类方法继承/重写分派、super 构造链；但静态方法跨类继承分派、native 类被编译类继承（A5 双向引用在真实构造流程生效）未完成。
- **影响**：编译类继承 native 类的场景不可用。
- **✅ 已完成（Phase F2，2026-07-10）**：编译类继承 native 类端到端打通——super() 到 native 父类构造在 this 上写字段（`dispatch_native_construct(target=Some)`，且不改子类 class_name）；子类自有字段读写正确；继承自 native 父类的方法经 `find_native_ancestor`（沿 `class_super_name` 上溯）分派到 native 实现。端到端 `class Labeled : Vector2`：继承 get_x=3、子类 getLabel=7。

---

## 四、内建类型库（System/Native）—— 缺口

### T14. ⚠️ 内建集合类型运行时不完整
- **现状**：`GorgeCore/src/list.rs`（IntList/FloatList）、`array.rs`（IntArray/FloatArray）存在但简化，未接入 native 桥接与 VM 数组构造操作码。
- **C# 语义**：IntList/FloatList/BoolList/StringList/ObjectList + 5 种 Array，均有 Injector 构造、字面量构造、Clone、EditableEquals。
- **影响**：谱面数组数据不可用。缺 Bool/String/ObjectList、全部 Array 的完整实现。

### T15. ❌ GorgeFramework 业务 native 类库（100+ 类）
- **现状**：仅移植 `Math`、`Vector2` 两个示范类。
- **C# 现状**：`references/gorge-framework/` 有 100+ native 类（Note/Element/FunctionCurve/Sprite/Signal/Chart/Simulators/Stage 等）+ 业务模块。
- **影响**：框架不可实际运行谱面。属长期工作。

---

## 五、注解与元数据 —— 缺口

### T16. ❌ 注解元数据（metadata）系统
- **现状**：parser **跳过** metadata 块（`skip_metadata_block`，parser.rs:829/875）；不收集不使用。
- **C# 语义**：`@Name[type name = expr,...]` 元数据条目，`MetadataScope`/`MetadataEntrySymbol`；`@Inject` 的元数据复制到注入器字段（谱面编辑器用）。
- **影响**：谱面编辑器元数据、@Inject 默认值/校验元数据丢失。

### T17. ⚠️ @Inject 注解自动生成注入器字段
- **现状**：注入器字段靠 `injector { }` 块显式声明收集；@Inject 注解驱动的自动生成未实现。
- **C# 语义**：字段上 `@Inject` 自动派生同名注入器字段 + 默认值 + 元数据。

### T18. ⚠️ 注解参数（编译时常量）求值
- **现状**：注解解析存在但参数值未作为编译时常量求值/存储。
- **C# 语义**：`@Name(key=constExpr)`，值必须编译时常量。

---

## 六、委托 / Lambda —— 缺口

### T19. ⚠️ Lambda 闭包捕获与静态/动态委托区分
- **现状**：codegen 有自由变量分析（`analyze_free_vars`）与委托构造，但完整的静态 vs 动态 Lambda、DelegateField 闭包填充、嵌套 Lambda 未验证完整。
- **C# 语义**：`StaticLambdaExpression`（常量委托）vs `LambdaExpression`（运行时填充闭包）；`DelegateScope` 自动捕获。

### T20. ⚠️ 委托类型协变/逆变转换
- **现状**：类型转换未含 Delegate 协变/逆变。

---

## 七、优化器 —— 缺口（不影响正确性，影响产物质量）

### T21. ⚠️ DAG 局部公共子表达式消除
- **现状**：`optimizer.rs` 有基本块划分、活跃变量分析、全局 CSE（可用表达式数据流）、DCE；缺 C# 的 `BasicBlockDag` 基于 DAG 的局部 CSE + 值编号。
- **C# 语义**：基本块内构建 DAG，消除局部公共子表达式后重生成代码，两种优化交替 4 轮。
- **影响**：优化产物不如 C# 精简（非正确性问题）。

### T22. ❌ 连跳优化（jump-to-jump）
- **C# 现状**：标 TODO；Rust 亦无。低优先级。

---

## 八、编译器基础设施 —— 缺口

### T23. ❌ 冻结机制（Freeze）
- **C# 语义**：声明冻结/继承冻结/实现冻结，冻结后修改抛异常；用于保证多趟一致性与接口实现完整性检查。
- **现状**：Rust 无对应机制（B-3 的 freeze_inheritance 只算编号，不做冻结校验）。

### T24. ⚠️ using 别名
- **现状**：namespace/using 基本支持；`using Alias = expr` 别名未确认。

### T25. ⚠️ native 类导入编译器的规范机制
- **现状**：Phase C/F 用「手写 native 存根声明 + 简单名匹配」方案，编译器已能从存根推导字段/方法/构造用于继承冻结（B-4 完成，支持 F2 继承）；但无从 GorgeFramework 元数据**自动**导出 native 存根的规范路径。
- **风险（仍存）**：存根与宏 impl 的方法顺序/签名必须手工保持一致（对齐规则 M1），易错。可后续做「从宏元数据自动生成 .g 存根」消除。

### T26. ❌ 异步编译 / 进度 / panic 恢复模式
- **C# 语义**：CompileAsync + IProgress + CancellationToken；GorgePanicableVisitor 收集多个错误不中断。
- **现状**：Rust 为同步、遇错即停。低优先级。

---

## 优先级建议

| 优先级 | 项 | 理由 |
|--------|-----|------|
| **P0（正确性核心）** | T1 break/continue、T2 cast、T8 重载解析 | 基础语言语义，影响任意程序 |
| **P0（框架核心）** | T3 注入器字面量、T4 注入器字段访问、T11 注入器构造、T16/T17 元数据+@Inject | 注入器是 Gorge 谱面的核心用途 |
| **P1（面向对象完整）** | T5 接口方法映射、T7 类型转换、T13 native 继承 | OO 特性完整性 |
| **P1（数据）** | T10 数组操作码、T14 集合类型 | 谱面数组数据 |
| **P2（高级特性）** | T6 泛型、T19/T20 委托闭包、T18 注解参数 | 高级但非阻塞 |
| **P2（质量）** | T9 诊断、T21/T22 优化器、T23 冻结、T26 异步 | 健壮性与产物质量 |
| **P3（长期）** | T15 全量 native 类库 | 工作量巨大，按需推进 |
