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

### 总计: 124 个测试, 零 warning

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
