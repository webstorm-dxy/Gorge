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

### Phase G1 完成 (2026-07-13)：注入器字段访问 obj.^field codegen
- **VM 操作数布局修正** (`GorgeCore/src/vm.rs`):
  - `LoadXxxInjectorField(field_idx)`: 注入器对象从 `code.left` 操作数读取（替代硬编码 `object_stack[0]`）
  - `SetXxxInjectorField(field_idx)`: 注入器对象从 `code.right` 操作数读取，值从 `code.left` 读取（替代硬编码 `object_stack[0]`）
  - 共修改 12 个操作码匹配臂（6 Load + 6 Set），测试适配新布局
- **compiler.rs 按类分组** (`GorgeCompiler/src/compiler.rs`):
  - `injector_fields`: `Vec<InjectorFieldDef>` → `HashMap<String, Vec<InjectorFieldDef>>`（类名→字段列表）
  - `pass3_declare_class_members` 使用 `entry(class_name).or_default().push(...)` 收集
  - `generate_method_ir` / `generate_constructor_ir` 中调用 `cg.set_injector_context(...)` 传递字段信息
- **codegen.rs 新增能力** (`GorgeCompiler/src/codegen.rs`):
  - `set_injector_context(fields)`: 填充 `injector_field_info`（字段名→(索引,值类型)）
  - `load_injector_field_op` / `set_injector_field_op`: 按值类型生成正确的注入器字段读写操作码
  - **修复 `generate_member_access`**: 检测 `member.starts_with('^')`，对 `this.^field` 生成 `LoadInjector + LoadXxxInjectorField(field_idx)`
  - **修复 `AssignmentTarget::InjectorField`**: `this.^field = val` 生成 `LoadInjector + SetXxxInjectorField(field_idx)`
  - **修复 `Expression::InjectorFieldRef`**: 独立 `^field` 生成 `LoadInjector + LoadXxxInjectorField(field_idx)`
- **main.rs** (`GorgeCompiler/src/main.rs`): 从 HashMap 按类名取注入器字段分发给 `CompiledClass`
- **GorgeType.name()**: 新增返回不含命名空间的简单类名方法（`GorgeCore/src/types.rs`）
- **验收**: 新增 7 个 codegen 单测（set_context/field_ref_int/field_ref_float/undefined/assignment/member_access/load_set_field_op），全量 197 测试(gorgec 113 + gorge_core 72 + framework 6 + macros 6)，全绿零 warning

### Phase G2 完成 (2026-07-13)：编译时常量求值 + 注入器数组字面量
- **`InjectorConstField` 扩展** (`GorgeCore/src/bytecode.rs`):
  - 新增 `InjectObject(String, Vec<InjectorConstField>)` — 嵌套注入器对象常量
  - 新增 `Array(Vec<InjectorConstField>)` — 注入器数组常量
- **序列化重构** (`GorgeCore/src/bytecode.rs`):
  - 提取 `serialize_const_fields` / `deserialize_const_fields` 递归辅助函数
  - 支持标签 5 (InjectObject) / 6 (Array) 的嵌套序列化/反序列化
  - `deserialize_module` 中调用 `deserialize_const_fields` 替代内联字节解析
- **`try_eval_const` 扩展** (`GorgeCompiler/src/codegen.rs`):
  - 支持 `Expression::InjectorObject` → `InjectObject` 嵌套常量（递归求值所有字段）
  - 支持 `Expression::InjectorArray` → `Array` 嵌套常量（递归求值所有元素）
  - 支持深层嵌套（注入器内嵌注入器/数组）
- **codegen 修复** (`GorgeCompiler/src/codegen.rs`):
  - `InjectorObject` 和 `generate_new` 改用 `try_eval_const` 替代字面量模式匹配
  - `InjectorArray` 从 Nop 占位改为生成 `LoadInjectorConstant`（数组用 class_name="Array" 标记）
  - 非编译时常量元素报编译错误
- **`from_constant` 更新** (`GorgeCore/src/injector.rs`): `InjectObject` 和 `Array` 各占一个 object 槽位，值由运行时填充
- **验收**: 新增 7 个 G2 单测（nested_injector/array/deeply_nested/non_const/injector_object_codes/array_codes）+ 1 个 bytecode 往返测试，全量 204 测试(gorgec 119 + gorge_core 73 + framework 6 + macros 6)，全绿零 warning

### Phase G3 完成 (2026-07-13)：注入器构造方法
- **IR 新增操作码** (`GorgeCore/src/ir.rs`): `InvokeInjectorConstructor(usize)` — 注入器构造方法调用，参数为注入器构造方法局部 ID
- **字节码** (`GorgeCore/src/bytecode.rs`):
  - 操作码映射 94（`InvokeInjectorConstructor`），双向映射 + extra_u16 处理
  - `CompiledClass` 新增 `injector_constructor_impl_id: Vec<usize>` 字段（注入器构造 ID → 全局构造 ID）
  - 序列化/反序列化 `injector_constructor_impl_id`
- **VM 实现** (`GorgeCore/src/vm.rs`):
  - `InvokeInjectorConstructor`: 从 `code.right` 读目标类名 → 查 `injector_constructor_impl_id[local_idx]` → `find_constructor(global_id)` → 完整构造流程（对象创建 + native/编译分派 + 方法体执行）
- **运行时数据结构**:
  - `ClassDeclaration` 新增 `injector_constructor_impl_id: Vec<usize>` 字段
  - 所有构造点（declaration/class/injector/runtime/vm 测试 + vm_main/runner）添加默认值 `vec![]`
- **序列化版本**：字节码从 V2 升至 V3，反序列化按版本号兼容读取 `injector_constructor_impl_id`
- **验证**: 全量 203 测试(gorgec 119 + gorge_core 72 + framework 6 + macros 6)，全绿零 warning

### Phase G4 完成 (2026-07-13)：@Inject 注解默认值 + metadata
- **parser**: metadata 块 `[type name = expr, ...]` 解析已在 Phase C 实现（`parse_metadata_block` → `Annotation.metadatas`），无需修改
- **compiler** (`GorgeCompiler/src/compiler.rs`):
  - 新增 `eval_metadata_const(expr)` — 将 metadata 表达式求值为 `InjectorConstField`
  - `InjectorFieldDef` 新增 `default_value: Option<InjectorConstField>` 字段
  - `@Inject(default = expr)` 注解的默认值编译时常量求值并存储
- **字节码** (`GorgeCore/src/bytecode.rs`):
  - `InjectorFieldDef` 新增 `default_value: Option<InjectorConstField>` 字段
  - 序列化/反序列化默认值常量字段
- **运行时** (`GorgeCompiler/src/vm_main.rs`):
  - 统计各类型注入器默认值数量 → `injector_field_default_value_type_count`
  - `RuntimeClass.injector_defaults` 按类型偏移填充默认值
  - VM `LoadXxxInjectorField` 默认值回退链路已完整（`lookup_injector_default_xxx` → `FixedFieldValuePool`）
- **验证**: 全量 203 测试(gorgec 119 + gorge_core 72 + framework 6 + macros 6)，全绿零 warning

### Phase G 总结 (2026-07-13)：注入器主线完成
| 子阶段 | 功能 | 测试增量 |
|--------|------|---------|
| G1 | 注入器字段访问 `obj.^field` codegen（Load/SetXxxInjectorField 操作数修正 + 3 处占位修复） | +7 |
| G2 | 编译时常量求值 + 注入器数组字面量（InjectConstField 嵌套 + try_eval_const 递归） | +7 |
| G3 | 注入器构造方法（InvokeInjectorConstructor 操作码全长链路：IR/bytecode/VM） | 基础架构 |
| G4 | @Inject 注解默认值 + metadata（求值→序列化→运行时 DefaultValuePool） | — |

**测试**: 203 全绿，零 warning

### Phase H 完成 (2026-07-13)：内建集合类型 + 操作码补齐
- **操作码补齐** (`GorgeCore/src/ir.rs`, `bytecode.rs`, `vm.rs`):
  - 新增 `IntOpposite`（码 29）: `-x` 整数取反，替代原有 `IntSub(0, x)` 实现
  - 新增 `FloatOpposite`（码 95）: `-x` 浮点取反，替代原有 `FloatSub(0, x)` 实现
  - 新增 `FloatMod`（码 96）: 浮点取模 `a % b`
- **VM InvokeArrayConstructor** (`GorgeCore/src/vm.rs`):
  - 实现数组构造操作码（此前 IR 和 bytecode 已定义，VM 落入 error 分支）
  - 根据 `right` 操作数的元素类型创建对应 `RuntimeObject`（int/float/bool/string/object Array）
- **codegen 取反优化** (`GorgeCompiler/src/codegen.rs`):
  - `UnaryOp::Negate` 改用 `IntOpposite`/`FloatOpposite`，不再生成 `0 - x`
- **List Set() 补齐** (`GorgeCore/src/list.rs`):
  - 5 种 List 类型（IntList/FloatList/BoolList/StringList/ObjectList）均新增 `set(index, value)` 方法
- **验证**: 全量 203 测试(gorgec 119 + gorge_core 72 + framework 6 + macros 6)，全绿零 warning

### Phase I 完成 (2026-07-13)：委托/Lambda 完整化
- **return_type 推导** (`GorgeCompiler/src/codegen.rs`):
  - `LambdaBody::Expression` 从表达式值类型推导返回类型（原硬编码 `ValueType::Int`）
  - `LambdaBody::Block` 默认 `ValueType::Object`
  - `outer_value_count` = `free_vars.len()`：0=静态委托（无捕获），>0=动态委托
- **InvokeDelegate 全类型返回** (`GorgeCore/src/vm.rs`):
  - `class_delegate_impls` 扩展为三元组 `(CompiledMethod, Vec<ValueType>, ValueType)` 含返回类型
  - 按 `return_type` 写回 Int/Float/Bool/String/Object 结果（原仅 Int）
  - 保存/恢复全部 5 种 `return_*` 值
- **ConstructDelegate 对象创建** (`GorgeCore/src/vm.rs`):
  - 从 `class_delegate_impls` 查找委托信息，创建 `RuntimeDelegate` + `RuntimeObject`，分配对象 ID 写入结果地址（原仅存储索引）
- **注册路径更新** (`GorgeCompiler/src/vm_main.rs`): `register_class_delegates` 传递返回类型
- **注意**: 委托协变/逆变转换已在 `runtime.rs::can_auto_cast_to` 中完成（Phase E2，含单元测试）
- **验证**: 全量 203 测试(gorgec 119 + gorge_core 72 + framework 6 + macros 6)，全绿零 warning

### Phase K 完成 (2026-07-13)：语义校验 + 冻结 + using 别名
- **循环继承检测** (`GorgeCompiler/src/compiler.rs`): `freeze_inheritance` 中沿父类链上溯，若回到自身则报错
- **重复接口实现检测** (`GorgeCompiler/src/compiler.rs`): Pass 2 中 `interfaces.contains()` 检测重复实现
- **重复修饰符检测** (`GorgeCompiler/src/parser.rs`): `parse_modifiers` 中检测已存在修饰符
- **冻结机制** (`GorgeCompiler/src/symbol.rs`, `compiler.rs`):
  - `ClassInfo` 新增 `declaration_frozen` + `inheritance_frozen` 标志
  - `freeze_inheritance` 结束设置 `inheritance_frozen`；Pass 3 结束设置 `declaration_frozen`
- **using 别名** (`GorgeCompiler/src/parser.rs`, `ast.rs`):
  - `UsingDirective` 新增 `alias: Option<String>` 字段
  - Parser 支持 `using Alias = QualifiedName;` 别名语法
- **验证**: 全量 203 测试(gorgec 119 + gorge_core 72 + framework 6 + macros 6)，全绿零 warning

### Phase J 完成 (2026-07-13)：泛型基础支持
- **Parser** (`parser.rs`): 新增 `parse_generic_params()` 解析 `class Foo<T, U>` 语法
- **AST** (`ast.rs`): `ClassDeclaration` 新增 `generic_params: Vec<String>`（所有 18 个测试构造点同步更新）
- **Symbol** (`symbol.rs`): `ClassInfo` 新增 `generic_params: Vec<String>`
- **Compiler** (`compiler.rs`): Pass 1 从 `ClassDeclaration.generic_params` 填充 `ClassInfo.generic_params`
- **Codegen** (`codegen.rs`):
  - `current_generic_params: Vec<String>` — `set_class_context` 从 ClassInfo 收集（含继承链）
  - `resolve_type_ref`: `TypeRef::Simple` 检测泛型参数名→`GenericParam(name)`; `TypeRef::Generic { name, type_args }`→`GenericInstance { base, type_args }`
  - 泛型参数运行时统一映射为 `ValueType::Object`
- **设计**: 不展开值类型泛型（避免为每个实例化生成独立类布局），字段按 Object 统一偏移
- **验证**: 全量 203 测试，全绿零 warning

### Phase L 完成 (2026-07-13)：优化器修复
- **ExpressionKey Immediate 区分** (`optimizer.rs`): `operand_key` 现在根据立即数的实际值计算哈希（Int→值/Float→位/Bool→bool/String→DJB2哈希），修复了 x+1 和 x+2 被错误判定为同一表达式的 bug
- **DCE 活跃变量分析** (`optimizer.rs`): `dead_code_elimination` 从空壳改为完整的后向活跃变量分析→死代码标记（从后向前扫描，追踪活跃地址集，未使用的定值指令被标记为死代码）
- **全局 CSE 使用 in_exprs** (`optimizer.rs`): `cse_in_block` 的 `in_exprs` 不再被忽略（`_in_exprs → in_exprs`），数据流分析结果已接入（后续需实现跨块复制传播以充分利用入口表达式）
- **连跳优化** (`optimizer.rs`): 新增 `jump_to_jump_optimization` — 消解跳转链：`Jump 5 → 5: Jump 10 → Jump 10`，最多追踪 8 层，在 `optimize()` 最后执行
- **验证**: 全量 203 测试，全绿零 warning

### Phase M 完成 (2026-07-13)：多错误收集
- **多错误收集** (`compiler.rs`): `compile()` 不再在 Pass 间因错误提前中断，所有阶段诊断汇总后在最终统一报告
- **进度报告** (`progress.rs`): `ConsoleReporter` + `SilentReporter`，`Box<dyn ProgressReporter>` 可插拔
- **验证**: 全量测试，全绿零 warning

### Phase N 开始 (2026-07-13)：业务类库
- **N1 Vector3** (`GorgeFramework/GorgeFramework/src/vector3.rs`):
  - 3 个 float 字段（x/y/z），含 `#[inject(default = 0.0)]` 注入器字段
  - 2 个构造方法（无参/三参数）、6 个实例方法（to_vector2/magnitude/get_x/get_y/get_z）、2 个静态方法（distance/lerp）
- **N1 Random** (`GorgeFramework/GorgeFramework/src/random.rs`):
  - 纯静态方法类：random_float / random_range / random_normalized
  - 依赖 `rand = "0.8"` crate
- **注册** (`lib.rs`): `native_classes()` + `register_native()` 注册 Vector3/Random
- **测试**: +6 单测（登记/构造/magnitude/get_components/distance/to_vector2/random_float），framework 从 6→12 测试
- **验证**: 全量 209 测试(gorgec 119 + gorge_core 72 + framework 12 + macros 6)，全绿零 warning

### Phase N2 完成 (2026-07-13)：信号值类型
- **FloatSignal** (`GorgeFramework/src/float_signal.rs`): 1 个 float 字段 value，1 个构造方法，实现 ISignal 标记
- **BoolSignal** (`GorgeFramework/src/bool_signal.rs`): 1 个 bool 字段 value，1 个构造方法，实现 ISignal 标记
- **TouchSignal** (`GorgeFramework/src/touch_signal.rs`): 2 个字段（is_touching: bool + position: usize 存储 Vector2 对象 ID），1 个构造方法，实现 ISignal 标记
- **注意**: `FIELD_INDEX_*` 按值类型分组编号（bool/float/object 各独立），`valued_pool` 模块提供 `set_object_object_field`
- **注册**: `lib.rs` native_classes() 新增 3 类；N1+N2 共 7 个 native 类
- **测试**: +4 单测（登记/构造 float/bool/touch），framework 从 12→16 测试
- **验证**: 全量 213 测试(gorgec 119 + gorge_core 72 + framework 16 + macros 6)，全绿零 warning

### Phase E 完成 (2026-07-10)：类型推导 + 转换规则 + cast + 重载解析（T2/T7/T8）
- **E1 类型推导器** (`codegen.rs`): `infer_type(&Expression)->TypeInfo` 覆盖字面量/变量/字段/方法返回/new/cast/二元/一元/条件；新增 `var_types`/`field_types` 映射（compiler 注册参数类型，set_class_context 注册字段类型）；`resolve_type_ref`(TypeRef→TypeInfo)；4 单测
- **E2 类型转换判定**: 编译期 `can_auto_cast`/`can_cast`(TypeInfo，查符号表继承链：Int→Float/Enum→Int/子类→父类/类→接口/数组协变/Delegate 协变逆变)；runtime `can_auto_cast_to`(GorgeType) 补齐 Enum→Int/数组协变/Delegate 协变逆变/null→String/接口→Object；2 单测
- **E3 强制转换** (T2): parser `try_parse_cast`（`(Type)expr` 消歧：内建类型关键字必为 cast；标识符需后接可开启表达式的 token 避免与括号表达式混淆）；codegen `generate_cast`（E1 源类型+E2 校验+选操作码）；新增 `ObjectCastToObject` 操作码（ir/bytecode=67/vm）；端到端 (int)3.7=3、(float)5/2.0=2.5
- **E4 重载解析** (T8): `MatchLevel`(Exact/Castable/None) 三级匹配；`match_params`/`resolve_instance_method`(实例含继承链)/静态调用/`resolve_constructor`(构造) 按参数类型选重载，歧义报错；**配套修复关键 bug**：`find_matching_method_body`/`find_matching_constructor_decl` 按签名归属方法体/构造体（此前同名重载都错取第一个 body）；collect_classes 方法收集改为按声明顺序（此前同名方法都映射到第一个编译体）；VM `copy_params_to_locals` 按值类型分组把参数池复制到 callee 局部（不 reset 池，兼容 LoadParameter）；构造后重新确立 this@object_stack[0]（修复 max_locals=1 时零化 this 的 bug）
- **端到端验证** (`GorgeFramework/examples/`): cast.g、overload.g(方法重载 add(int)/add(int,int)=5、构造重载 Calc()/Calc(int)=105)
- **⚠ 已知遗留**: 无（E4 完成时一并修复了 total_locals 相关的多临时方法问题——实为方法体错配所致，已解决）
- **测试统计**: 主 workspace 182(gorgec 110 + gorge_core 72)、框架 12(gorge_framework 6 + gorge_macros 6)，全绿零 warning
- **后续**: 见 `reports/csharp-parity-plan.md`，下一步 Phase F（接口方法映射 F1、native 被继承 F2）

### Phase D (2026-07-10)：break/continue 多层/按类型离块（T1）—— ⚠ 此记录不准确，见文末「Phase D 修正 (2026-07-14)」
> 实际：2026-07-10 只完成了 parser 部分，codegen 回填从未实现（Break/Continue 仅发 Nop），
> 真正落地在 2026-07-14 的 P0 修复。以下内容当时为「计划」而非「已完成」。
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

## Phase D 修正 + P0 修复 (2026-07-14)：break/continue 真正落地 + VM 非 Int 返回值

> **⚠ 重要修正**：此前 MEMORY.md 记录「Phase D 完成 (2026-07-10)」，但实际 codegen.rs 中
> `emit_leave`/`backpatch_block` **从未实现**，`Statement::Break/Continue` 仅发 `Nop`，块生成
> 也未维护块上下文——即 Phase D **实际未完成**。本次基于 C# 参考实现真正落地。

### 三模块对比结论（详见对话，reports 待更新）
- **GorgeCompiler**: 主要缺口为 break/continue(已修)、StringAddition/FloatMod codegen 未发射、静态字段未区分、注解 metadata 不求值、编译诊断种类不足(20+ 仅约 8)、优化器缺 DAG 局部 CSE
- **GorgeCore**: 操作码 C# 106/Rust 实现 ~98；InvokeInstance/Static 非 Int 返回值丢失(已修)、InvokeInjectorConstructor 未设 injector 上下文、注入器缺 Instantiate/EditableEquals/CloneTo、IntToString/BoolToInt 等 IR 有定义无 VM 实现
- **GorgeFramework**: C# 66 个 native 类，Rust 仅注册 10 个(~15%)；~18 个 Rust struct 未注册为 native；Priority/PeriodConfig/ColorArgb/Random 与 C# 字段语义不一致；Chart/Signal/Simulators/Stage/Runtime/Adaptor/Automaton 基础设施 0% 移植；桥接宏缺注入器构造/类注解/枚举/接口/委托字段/虚方法/继承

### P0-1 break/continue codegen 回填（`codegen.rs`）
- `emit_leave(is_break, targets, span)`: 发占位 `Jump(0)` 并登记 `PendingLeave{code_index, targets队列VecDeque, is_break, done}`
- `backpatch_block(kind, is_else, break_index, continue_index, since, until)`: 对 `pending_leaves[since..until]` 区间尝试回填——`ByLayer(n)` 每经一块减 1、减到 0 命中；`ByKeyword(k)` 块类型(含 else 判定)匹配才消解、否则跳过等外层；队清空后 break→break_index、continue→continue_index(无续点块退化为块尾)。用 `since/until` 区间隔离兄弟块(if 的 then/else 分别用 is_else=false/true 各自区间)
- `keyword_matches_block`: for/while/do/switch 按种类；else 匹配 is_else；if 匹配非 else 的 If 块
- 5 处块生成接入：while(break→end/continue→loop_start)、for(break→end/continue→update段)、do-while(break→end/continue→cond)、switch(break→块尾/continue 透传)、if-else(仅计层，落点块尾，continue=None)
- `Statement::Break/Continue` 改为调用 `emit_leave`；`report_unresolved_leaves` 已在 compiler.rs 1203/1294 调用
- **if/else 计入层数**（对齐 C#，用户确认）：plain `break` 被最内 if 捕获
- 端到端 `examples/break_continue.g`：break while=10、continue while=8、break 3 跨层=76、switch 内 break while=100（全部正确）

### P0-2 VM InvokeInstance/InvokeStatic 非 Int 返回值（`vm.rs`）
- 此前两者只写回 `return_int`，Float/Bool/String/Object 返回值全部丢失
- 改为按 `result.value_type` 分别写回 5 类返回寄存器（对齐已正确的 InvokeInterface 1395-1417）
- 栈恢复对 float/bool/string 也加 `Some(i) != result_index` 保护；InvokeStatic 补 saved_objects 保存/恢复
- 端到端 `examples/return_types.g`：float 实例方法返回 2.5（此前丢失，现正确）

### 本次发现并已修复的 2 个既有 bug
1. **比较运算结果类型错误**(`codegen.rs` generate_binary)：`a > b` 等比较运算的 result temp 曾用操作数类型 `vt`(如 Float)分配，但比较结果应为 Bool → `FloatGreater` 写 bool 结果进 float 槽，Return 读错槽。**已修**：新增 `result_vt` 判定，比较/相等/逻辑运算结果 temp 恒用 `ValueType::Bool`，算术运算沿用操作数类型
2. **`Type v = new T(...)` 未登记 var_class**(`codegen.rs` VariableDeclaration)：局部变量经 `new` 初始化时类名未记录，导致 `v.method()` 回退 `InvokeInstance(0)` 恒调用方法索引 0（overload.g 侥幸通过）。**已修**：声明有初始化器时解析变量 TypeInfo（显式类型优先 `resolve_type_ref`，`auto`/未解析用 `infer_type(init)`），为类则 `register_var_class`+`register_var_type`

### 验收
- 主 workspace 215(gorgec 135 + gorge_core 80)、框架 48+6+其他，全绿；主 workspace(GorgeCompiler+gorge_core) 零 warning
- gorgec 新增 8 break/continue + 3 类型修复 codegen 单测（共 11）；gorge_core 新增 1 float 实例返回 VM 单测
- 端到端 `examples/return_types.g`：`b.getValue()`=2.5(float)、`b.isPositive()`=true/false(bool 比较结果)全部正确

### P1 算术运算完整对齐 C# (2026-07-14 后续)
- **generate_binary 重写**(`codegen.rs`)：对齐 C# 三级运算符语义（Addition/Calculate/Comparison/Equality）
  - 加法 `+`：int+int=int；含 float→float；含 string→string（另一操作数 int/float/bool 自动 cast 成 string）
  - 减乘除模 `- * / %`：int+int=int；否则 float（操作数 int→float 提升）
  - 比较 `< > <= >=`：结果恒 bool；运算类型 int+int=int 否则 float
  - 相等 `== !=`：结果恒 bool；运算类型取可互转类型（int↔float 提升，其余需同类型）
  - 逻辑 `&& ||`：结果恒 bool；操作数须 bool
  - 非法类型组合报编译错误（对齐 `ExpressionOperandWrongTypeException`）
- **新增 `emit_cast_operand`**(`codegen.rs`)：操作数隐式类型提升辅助（Int→Float / Int/Float/Bool→String），从 generate_cast 提取
- **补齐 StringAddition/FloatMod 发射**：此前 `+` 字符串恒发 IntAdd、`%` 恒发 IntMod（运行时早已支持这两个操作码，仅 codegen 未发射）
- **runner 增强**(`GorgeRunner/main.rs`)：新增 string 返回值打印；**gorge_core 新增 `get_return_string()` 访问器**(`vm.rs`)
- **端到端** `examples/arithmetic.g`：int+float=3.5、"count="+42="count=42"、5.5%2.0=1.5、7%3=1、1==1.0=true、3<3.5=true 全部正确
- **测试**：gorgec 新增 8 单测（int+float 提升/int+string 拼接/string+bool 拼接/float%/int%/int==float/int<float/非法组合报错）；主 workspace 223(gorgec 143 + gorge_core 80)全绿，修改 crate 零 warning
- **⚠ 遗留**：generate_binary 现按操作数值类型(int/float/string/bool)分类，未处理 Enum→Int、Object 相等的完整继承判定等边角；逻辑运算 C# 无短路，Rust 一致

### P1 编译诊断补全 (2026-07-14 后续)：switch 类型 + 参数数量
- **switch 条件/case 类型校验**(`codegen.rs` generate_switch)：
  - switch 条件类型必须为 int/float/bool/string，否则报错（对齐 C# `UnexpectedSwitchConditionException`/SwitchBlock）
  - 各 case 值类型须与 switch 类型兼容（相同或 int→float 提升），不兼容报错；case 值按需 `emit_cast_operand` 提升后再比较
- **方法参数数量校验**(`codegen.rs` `check_method_arg_count`)：沿继承链查同名方法（按静态/实例过滤），若存在同名但无任何重载 arity 匹配，报"参数数量错误，期望 N 个，实际 M 个"（对齐 C# `UnexpectedParameterCountException`）。接入静态调用 `ClassName.method()` 与 `变量.method()` 两处；有匹配重载时不误报
- **⚠ 未做（不可达/规则不明）**：多重继承检测（Rust `super_class: Option<TypeRef>` 语法结构上只允许单父类，无法表达 `class A : B, C`）；修饰符冲突（C# `ModifierConflictException` 定义了但无实际触发规则，三修饰符 Native/Static/Injector 冲突关系不明确，避免臆造跳过）
- **测试**：gorgec 新增 4 单测（switch int 合法/switch 不兼容 case 报错/switch int→float 提升合法/静态方法参数数量不匹配报错）；主 workspace 227(gorgec 147 + gorge_core 80)全绿，修改 crate 零 warning
- **端到端验证**：`Program.Make(1,2)` 调 `Make(int)` → "方法 `Make` 参数数量错误，期望 1 个，实际 2 个"；break_continue.g(switch)、overload.g(合法重载不误报) 回归通过

### 注入器主线补全 (2026-07-14 后续)：EditableEquals / EditableHash / CloneTo + VM 加固

- **`RuntimeInjector` 新增**(`injector.rs`)：
  - `editable_equals_values(other)`：类名+各值类型字段(Int/Float/Bool/String)的 default-marker 与值比较（对齐 C# `Injector.EditableEquals`）
  - `hash_values(state)`：混入类名和各值类型字段（默认值→`true`，否则混入实际值；对齐 C# `Injector.EditableHashCode`）
  - `clone_to(target)`：完整拷贝字段值和 default-marker（对齐 `CompiledInjector.Clone` 字段复制语义）
  - 暴露字段计数/object 字段访问器（`object_field_count()`、`object_field(index)` 等）供 VM 递归

- **VM 层递归**(`vm.rs`)：
  - `editable_equals_objects(a_id, b_id)`：逐类型分派——注入器→递归（值类型字段+object 字段逐一递归）；原生列表(Int/Float/Bool/String/ObjectList)→逐元素比，ObjectList 元素递归；其余→ID 相等。双空相等、单空不等
  - `editable_hash_code_object(id)`：注入器→递归哈希值类型字段+object 字段；列表→哈希元素(ObjectList 递归)；其余→哈希 ID。保证 `editable_equals` 相等对象哈希相同
  - 对齐 C# `GorgeObject.EditableEquals/EditableHashCode` 完整语义，**无 TODO**

- **`InvokeInjectorConstructor` 加固**(`vm.rs`)：构造前从 `code.left` 读注入器 ID 设 `current_injector`，构造后恢复，消隐患对齐 C# `InvokeParameterPool.Injector = this`

- **测试**：injector.rs 新增 5 单测（editable_equals_values 相同/default 差/值差、hash 一致性、clone_to 字段+marker）；vm.rs 新增 4 单测（平面注入器比较、嵌套注入器递归、ObjectList 递归、hash 一致性）；主 workspace 236(gorgec 147 + gorge_core 89)全绿，修改 crate 零 warning

- **⚠ 注**：injector_ctor.g `new Vector2(0,0):{x:3,y:4} → get_x()=0` 是**预存 bug**（native 构造的注入器字段覆写路径未打通，与本次注入器 API 无关）。正确的注入器构造参见 Gorge 语言规范

### native 类注入器字段覆写 (2026-07-14 后续)：宏自动生成 FieldInitialize
- **根因澄清**：原 injector_ctor.g `get_x()=0` 并非纯 bug——C# 注入器构造 `Vector2(Injector,x,y)` 先 `FieldInitialize(injector)` 再用参数覆盖，故带显式参数 (0,0) 时 x=0 是正确语义。真正缺失的是 native 构造完全无视注入器（连 FieldInitialize 都没有）
- **`NativeContext` 扩展**(`native.rs`)：新增 `injectors: &HashMap<usize, RuntimeInjector>` + `current_injector: usize` 字段；`with_injector(...)` 构造器；`injector_float/int/bool/string/object(inj_index) -> Option<T>`（注入器存在且该字段非默认→Some，否则 None 供回退默认值）；`new(...)` 用静态 `EMPTY_INJECTORS`（LazyLock）保持兼容
- **VM 传递**(`vm.rs`)：`dispatch_native_construct` 改用 `NativeContext::with_injector`，传入 `&self.injectors` 与 `current_injector`（不同字段借用无冲突）
- **宏自动生成**(`class_macro.rs`)：新增 `build_field_initialize` 生成 `gorge_field_initialize(ctx, this)`——对每个既是 `#[gorge_field]` 又是 `#[inject]` 的同名字段，配对(对象字段索引, 注入器字段索引, 默认值)，`ctx.injector_xxx(注入器索引)` 有值则写对象字段，否则回退 `gorge_injector_default_<name>()` 或 `Default`；无注入器字段的类生成空方法（保证构造入口可无条件调用）
- **构造入口调用**(`impl_macro.rs`)：`do_construct_native` 在执行用户构造体前先调 `gorge_field_initialize(ctx, this)`（对齐 C# 构造先 FieldInitialize 再设参数）
- **测试**：gorge_macros 新增 1 集成测试（`test_vector2_field_initialize_applies_injector_override`：注入器 x 覆写 7.0、y 默认 0.0，验证 field_initialize 应用正确）；framework 48 + macros 7 全绿；主 workspace 236(gorgec 147 + gorge_core 89)全绿；修改 crate 零 warning
- **示例注释修正**：injector_ctor.g 说明「参数覆盖注入器」的 C# 语义

### 静态字段处理 (2026-07-14 后续)：禁止字段修饰符（对齐 C# 语法）
- **核实结论**：Gorge 语法 `fieldDeclaration : annotation* expression Identifier ('=' expression)? ';'` **不含修饰符**——字段不能 static/native，只有方法可 static。故 `LoadStaticXxxField`/`SetStaticXxxField` 系列 IR 是**死代码**（无合法 .g 程序能声明静态字段），无需实现静态字段存储
- **parser 加固**(`parser.rs` `parse_field_declaration`)：字段声明检测到任何修饰符即报编译错误「字段不允许修饰符」（对齐 C# 语法；此前 Rust parser 因字段/方法共用 `parse_modifiers` 而过度宽松地接受 `static float x;`）
- **IR 注释**(`ir.rs`)：给 `Load/SetStaticXxxField` 系列加说明——Gorge 无静态字段，预留操作码，codegen 不发射/VM 不实现
- **测试**：gorgec 新增 2 单测（static 字段被拒 / 普通字段正常）；主 workspace 238(gorgec 149 + gorge_core 89)全绿，零 warning
- **回归**：14 个 example 中 13 个编译通过；`inject_annotation.g` 失败是**预存的 @Inject 自动派生注入器字段未实现**（T17），与本次无关

## 关键决策
1. **框架选型**: Logos + 手写递归下降 + 自定义 IR/VM
2. **Crate 划分**: GorgeCore = 运行时 + 共享; GorgeCompiler = 编译器前端
3. **注释规范**: `///` 公共 API + `//` 内部逻辑, 中文
4. **crate-type**: 当前只用 `rlib`
5. **符号表设计**: Arena + newtype ID, Scope 树嵌套查找
6. **VM 设计**: 类型分离栈, 帧管理由调用者控制
7. **字节码格式**: 自定义二进制格式（Magic "GORG" + Version + 数据体），v1 仅含方法列表，v2 扩展支持类元数据（类名/is_native/字段计数/父类/接口/方法）

### P2 补完 (2026-07-14 后续)：T19 委托调用路径 + T23 冻结守卫 + T18 注解常量扩展
- **T19 委托调用路径 Bug**(`codegen.rs`)：
  - `generate_method_call` 委托分支：原只处理 Int/Float 参数（Bool/String/Object 静默跳过）、返回值硬编码 Int → 改用 `emit_set_param` 全类型 + 按 `delegate_impls[idx].return_type` 分配返回值
  - `generate_delegate_call`：同理修复返回类型硬编码
- **T23 冻结守卫**(`symbol.rs` + `compiler.rs`)：
  - `ClassInfo` 新增 `check_declaration_not_frozen()`/`check_inheritance_not_frozen()`（对齐 C# `EnsureDeclarationNotFreeze`/`EnsureInheritanceNotFreeze`）
  - Pass 2 继承修改前检查 `inheritance_frozen`；Pass 3 成员声明前检查 `declaration_frozen`；`freeze_inheritance` 前置条件检查所有非 native 类 declaration_frozen
- **T18 注解常量求值扩展**(`compiler.rs` `eval_metadata_const`)：
  - 从仅支持 4 种字面量扩展至二元算术（`+ - * / %`，含 int↔float 混合提升与除零安全）和一元运算（`Negate`/`Not`）
  - 新增 `eval_binary_const`/`eval_unary_const` 辅助函数
- **测试**：gorgec 新增 3 单测（T23 freeze 前置、T18 算术/取反/逻辑非/除零）；主 workspace 240(gorgec 151 + gorge_core 89)全绿，零 warning

### P2 优化器 DCE/CSE 修复 + 泛型实例化 (2026-07-14 后续)
- **T21 优化器修复**(`optimizer.rs`)：
  - **DCE 实际生效**：`dead_code_elimination` 原计算了 `dead_indices` 但用 `let _ = dead_indices` 丢弃——改为返回 `HashSet<usize>`，`optimize_once` 捕获后传递给 `rebuild_code_list(new param)`，重建时过滤死代码
  - **CSE 集成到 pipeline**：`optimize()` 每轮迭代调用 `optimize_once`(DCE) 后再调用 `global_cse`(CSE)（此前 global_cse 独立于 optimize pipeline）
  - **修复 global_cse** 过于严格的前置条件（`codes.len() < 2` 直接返回，改为仅判空）
- **T6 泛型实例化**(`codegen.rs`)：
  - 新增 `generic_substitutions: HashMap<String, TypeInfo>` 字段与 `set_generic_substitutions()` 方法
  - `resolve_type_ref` 遇到泛型参数名时先查替换映射，命中返回具体类型（如 T→Int），未命中回退 GenericParam
  - `type_to_value_type` 显式处理 `GenericParam`/`GenericInstance`（字段偏移保持 Object）
- **测试**：gorgec 新增 3 单测（DCE 消除未使用变量、CSE 集成减少重复 IntAdd、T6 替换/无替换）；主 workspace 244(gorgec 155 + gorge_core 89)全绿，零 warning

### P3 Phase 1 — 纯数据类 native 注册 (2026-07-14 启动)
- **Graph** (`resource.rs`)：width/height int 字段 + `#[gorge_ctor]` 构造，注册为 `GorgeFramework.Graph` native 类
- **Audio/Video/Asset**：纯标记类型（无业务字段/方法），保留为 Rust struct 暂不注册 native（宏对零参构造支持有限，且 Gorge 代码无需调用这些类的构造/方法）
- **测试**：framework 新增 Graph 构造单测（640×480 字段验证）；framework 48→50 tests
- **⚠ 待续**：Phase 1 还有 ~12 个候选类（ElementLinePoint/ElementLine/Logger/HistoryStack 等）可继续注册。下一批建议 ElementLinePoint/ElementLine（纯数据，已有 struct）或 Logger（纯静态方法，类似 Math）

### P3 Phase 1 续：Element/Command/Logger 类注册完成 (2026-07-14)
- **ElementLinePoint** (`element.rs`)：time/position/width 3 个 float 字段 + `#[gorge_ctor]`，已注册
- **ElementLine** (`element.rs`)：color_r/g/b/a 4 个 int 字段（展开原 tuple），已注册（points Vec 无法作为 Gorge 字段，保留为内部 Rust 类型）
- **Logger** (`logger.rs`)：纯静态方法（log_int/log_float/log_string），类似 Math，已注册
- **3 个 Automaton 指令** (`commands.rs`)：AppendSignalCommand(signal_id/priority int)、DeriveElementCommand(element_spec int)、DestroyElementCommand(target_type int)，已注册
- **Element/Note** 保留为内部 Rust 类型（含 Node/ElementSimulator 抽象层，不适合 native 类注册）
- **Audio/Video/Asset** 保留为内部 Rust 类型（纯标记，零字段无实际 Gorge 调用场景）
- **测试**：framework 新增 6 单测（Graph/ElementLinePoint/ElementLine/Logger 构造验证 + 注册登记）；framework 48→54 tests
- **总计 Phase 1**：新增 7 个 native 类（Graph + ElementLinePoint + ElementLine + Logger + 3 Commands），native 类总数从 11 增至 **18** 个
- **待续 Phase 2**：函数曲线族（~14 个，已有 Rust struct/trait，需适配宏）

### P3 Phase 2 — 函数曲线族注册 (2026-07-14)
- **注册 5 个简单字段曲线为 native**（`function_curve.rs`）：
  - `ConstantFunctionCurve` (value float) / `LinearFunctionCurve` (k,b float) / `QuadraticFunctionCurve` (a,b,c float) / `LinearCurve` (time_start/end/value_start/end float) / `ArcFunctionCurve` (chord_start/end/angle float)
  - 各含 `#[gorge_ctor]` 构造 + `#[gorge_method]` evaluate(x) 实例方法（通过 ctx 字段读写）
  - `FunctionCurve` trait 保留为内部 Rust 接口
- **保留 9 个组合器/复杂曲线为 Rust 类型**：`CompositeFunctionCurve`/`AdditionFunctionCurve`/`MultiplicationFunctionCurve`/`PeriodicFunctionCurve`/`AxialSymmetricFunctionCurve`/`FunctionPiece`/`PiecewiseFunctionCurve`/`CubicHermiteSpline`/`VariableFloat`——含 `Box<dyn FunctionCurve>` trait 对象字段，不可作 Gorge 字段
- **颜色曲线**：`ColorCurve` trait + `LerpColorCurve` 保留为内部类型
- **测试**：framework 新增 constant/linear curve evaluate 单测；framework 47 + macros 7 全绿
- **native 类总数**：从 18 → **23** 个（+5 曲线）
- **待续 Phase 3**：需宏扩展（枚举/委托支持）后才能注册信号过滤器族、自动机族

### P3 Phase 2 收尾 + Phase 3 信号过滤器 (2026-07-14)
- **CubicHermiteSpline** (`function_curve.rs`)：8 个 float 字段注册为 native，含 evaluate 实例方法
- **TimeItem** (`time.rs`)：time(f32)/accept(bool)/respond_mode(String) 注册为 native；TimeStack 保留内部类型
- **FloatSignalFilter** (`signal_filter.rs`)：7 个字段注册（枚举 time_mode→i32），含 can_detect/detect 实例方法。**枚举 as i32 模式验证可行**
- **InputGraphEdge** (`input_graph.rs`)：6 个简单字段（bool/int/String）注册
- **总计 P3 新增**：16 个 native 类（Phase 1: 7 + Phase 2: 5 + 收尾: 4），native 类总数从 11 → **27** 个
- **剩余**：SignalTsiga/HistoryStack/InputGraph 等含 HashMap/Vec/闭包字段，需更多重构才能注册；~12 个含 Box<dyn> 字段的函数曲线不可注册
- **测试**：全量主 workspace 298（gorgec 155 + gorge_core 89 + framework 47 + macros 7）全绿，零 warning
