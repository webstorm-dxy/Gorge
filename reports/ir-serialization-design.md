# Gorge 中间代码序列化设计技术报告

## 1. 概述

Gorge 是一个自定义面向对象编程语言及其自举编译器。编译流程分为词法分析 → 语法分析 → 四趟语义分析（Pass 1-4）→ IR 生成 → IR 优化 → **字节码序列化** → 虚拟机执行。

中间代码序列化模块（`GorgeCore/src/bytecode.rs`，856 行）负责将优化后的三地址码（TAC）IR 转换为轻量级二进制格式（`.gorge` 文件），供运行时虚拟机加载执行。本报告分析其设计方法。

---

## 2. 中间表示（IR）设计

### 2.1 三地址码模型

IR 采用经典的三地址码（Three-Address Code）表示，每条指令格式为：

```
result = left operator right
```

其中 operator 为操作码（~80 种），left 和 right 为操作数（Operand），result 为目标地址（Option\<Address\>）。

```rust
// ir.rs:260
pub struct IntermediateCode {
    pub result: Option<Address>,      // 目标地址
    pub operator: IntermediateOperator, // 操作码
    pub left: Operand,                // 左操作数
    pub right: Option<Operand>,       // 右操作数（可选）
}
```

### 2.2 操作数类型

操作数分为两种：
- **Address**（栈变量引用）：含值类型（ValueType）和索引（usize），指向类型分离栈中对应类型栈的槽位
- **Immediate**（编译时常量）：Int(i64)、Float(f64)、Bool(bool)、String(String)

### 2.3 值类型体系

虚拟机为五种值类型维护独立栈：Int、Float、Bool、String、Object。每种操作码都有按类型细分的变体（如 IntAssign/FloatAssign/BoolAssign 等），共 ~80 种操作码，涵盖赋值、算术、比较、逻辑、类型转换、字段读写、参数传递、方法调用、控制流、返回等。

---

## 3. 序列化层次结构

序列化遵循三层嵌套结构：

```
CompiledModule (.gorge 文件)
├── 文件头: Magic + Version + 类数量
├── CompiledClass × N
│   ├── 类名（UTF-8 字符串）
│   ├── is_native 标志
│   ├── 字段计数（int/float/bool/string/object 各 4 字节 u32）
│   ├── 父类名（可选 UTF-8 字符串）
│   ├── 接口名列表
│   └── CompiledMethod × N
│       ├── 方法名（UTF-8 字符串）
│       ├── 局部变量数（u32）
│       ├── 指令数量（u32）
│       └── IntermediateCode × N
│           ├── 操作码编号（u16）
│           ├── 额外数据（u16，字段索引/跳转目标/方法ID）
│           ├── result（可选 Address）
│           ├── left（Operand）
│           └── right（可选 Operand）
```

---

## 4. 二进制格式设计

### 4.1 文件头

```
偏移  大小  字段
0     4     Magic "GORG"（ASCII）
4     2     Version（u16，小端序）
6     2     类数量（u16，v1 时为方法数量）
```

魔数 "GORG" 用于快速识别文件类型，版本号支持格式演进。

### 4.2 操作码编码

每个操作码映射到一个 u16 编号（0-255），编号按功能分组：

| 范围 | 类别 | 示例 |
|------|------|------|
| 0-4 | 本地赋值 | IntAssign=0, FloatAssign=1 |
| 10-19 | 字段读写 | LoadIntField=10, SetIntField=15 |
| 20-28 | 算术运算 | IntAdd=20, FloatMul=27 |
| 30-47 | 比较/相等 | IntLess=30, StringEqual=41 |
| 50-52 | 逻辑运算 | LogicalAnd=50 |
| 60-66 | 基本类型转换 | IntToFloat=60, BoolToString=66 |
| 70-72 | 控制流跳转 | Jump=70 |
| 80-84 | 方法调用 | InvokeInstance=80, InvokeConstructor=84 |
| 90 | 对象创建 | DoConstruct=90 |
| 100-105 | 返回 | ReturnVoid=105 |
| 200-222 | 扩展操作 | StringAddition=200, LoadThis=201, SetInjector=221 |
| 255 | 空操作 | Nop=255 |

需要额外数据的操作码（如字段索引 `LoadIntField(usize)`、跳转目标 `Jump(usize)`），其参数值写入紧接着的 u16 字段。

### 4.3 操作数编码

操作数采用标签化编码（tagged encoding），用一个字节的 kind 区分类型：

| kind | 含义 | 编码 |
|------|------|------|
| 0 | Address | kind + ValueType(u8) + index(u32) = 6 字节 |
| 1 | Int 立即数 | kind + i64(LE) = 9 字节 |
| 2 | Float 立即数 | kind + f64(LE) = 9 字节 |
| 3 | Bool 立即数 | kind + 0/1 = 2 字节 |
| 4 | String 立即数 | kind + len(u16) + UTF-8 数据 |

对于可选操作数（result、right），前导一个标志字节（1=存在，0=不存在），避免为常见缺省情况浪费空间。

### 4.4 字符串编码

类名、方法名、接口名等字符串使用"长度前缀 + UTF-8 字节"编码：
- 长度前缀：u16（小端序），允许最长 65535 字节
- 字符串体：原始 UTF-8 字节

---

## 5. 版本兼容性设计

### 5.1 v1 格式（旧版兼容）

```
MAGIC | VERSION=1 | 方法数量(u16) | CompiledMethod × N
```

- 仅包含扁平方法列表，无类结构信息
- 反序列化时自动包装为单一 `Module` 伪类
- 保留向后兼容：`serialize()` 和 `deserialize()` 函数

### 5.2 v2 格式（当前版本）

```
MAGIC | VERSION=2 | 类数量(u16) | CompiledClass × N
```

- 扩展支持类元数据：类名、is_native、字段计数、父类名、接口列表
- 支持多类文件（一个 `.gorge` 可包含完整模块）
- 反序列化时根据 version 分支处理

版本判断逻辑（`deserialize_module` 函数）：
```rust
if version == 1 {
    // 按 v1 格式解析方法列表，包装为 Module 伪类
} else {
    // 按 v2 格式解析类结构
}
```

---

## 6. 序列化/反序列化流程

### 6.1 编译端（序列化）

```
main.rs
  ↓ 词法分析 → 语法分析 → 编译（Pass 1-4）→ 优化
  ↓ 收集 CompiledClass（从符号表递归遍历类/命名空间）
  ↓ CompiledModule { version: 2, classes }
  ↓ serialize_module()
  ↓ Vec<u8> → 写入 .gorge 文件
```

`serialize_module` 按序写入：
1. 文件头（MAGIC + version + class_count）
2. 对每个类：串行写入类元数据 + 方法列表
3. 对每个方法：串行写入名称 + 局部变量计数 + 指令序列
4. 对每条指令：写入操作码(u16) + 额外数据(u16) + result + left + right

### 6.2 运行时端（反序列化）

```
vm_main.rs
  ↓ 读取 .gorge 文件 → Vec<u8>
  ↓ deserialize_module() → CompiledModule
  ↓ 构建 GorgeRuntime + RuntimeClass
  ↓ VirtualMachine 执行 IR
```

`deserialize_module` 使用游标式解析：
- 维护 `pos` 游标，逐字段前移
- 每个字段读取前检查越界，返回描述性错误信息
- 字符串使用 `String::from_utf8_lossy` 容错处理

反序列化时损失信息：
- **Span**（源码位置）还原为 `Span::dummy()`（全零），因为字节码不保存调试信息
- **操作码编号到代码的映射**：`u16_to_opcode(code, extra)` 通过一个大 match 语句反向映射

---

## 7. 与 C# 参考实现的对比

| 维度 | C# 参考实现 | Rust 实现 |
|------|------------|-----------|
| **序列化** | **不进行序列化** — IR 对象直接保留在内存中，VM 直接执行 `IntermediateCode[]` 数组 | **自定义二进制序列化** — 将 IR 编译为 `.gorge` 字节码文件 |
| **部署模型** | 编译产物为内存对象，运行时一次性全部加载 | 编译产物为独立文件，运行时从磁盘反序列化 |
| **IR 完整度** | 类型系统完整，含泛型、接口方法实现映射、多态分派 | IR 简化，部分操作码归并为 Nop |
| **操作码数量** | ~100+ 种，含 Injector 构造、数组构造、运行时类型转换等 | ~80 种，Injector 字段读写/构造/委托构造等部分为占位 |
| **版本系统** | 无 | v1/v2 两套格式，v1 向后兼容 |
| **类结构** | 完整含方法重写映射、接口实现映射 | v2 格式含基本元数据（父类/接口/字段计数），缺失方法重写信息 |

C# 参考实现**没有字节码序列化层**——它是一个进程内编译器运行时，编译产物直接在内存中执行。Rust 实现独立设计了二进制序列化格式，使其可独立部署和执行 `.gorge` 文件，这是一个重要的架构差异。

---

## 8. 设计权衡分析

### 8.1 优点

1. **轻量紧凑**：二进制格式相比文本序列化（如 JSON/XML）体积小，解析速度快
2. **类型化操作码**：按值类型区分操作码，虚拟机无需运行时类型检查，直接按类型栈执行
3. **版本机制**：魔数 + 版本号支持格式演进，v1/v2 并存保证向后兼容
4. **越界安全检查**：反序列化中每步都有边界检查，防止畸形文件导致崩溃
5. **模块化结构**：`CompiledModule → CompiledClass → CompiledMethod → IntermediateCode` 层次清晰

### 8.2 不足与改进方向

1. **无调试信息**：Span 信息在序列化时丢失，`CompiledMethod` 序列化时未保留 `local_count` 的各类型细分（仅保留总数），反序列化后无法进行精确的错误定位
2. **游标式解析脆弱**：手动维护 pos 偏移，容易出错且不利于扩展。可考虑用 `Read` trait 或 serde 等成熟方案
3. **操作码映射冗余**：`opcode_to_u16` 和 `u16_to_opcode` 是两个超大 match 语句（各 ~100 条分支），新增操作码需两边同步修改。可用宏或 codegen 生成
4. **数值范围限制**：字段索引、跳转目标等用 u16 存储，最多支持 65535 个字段/指令目标。大规模程序可能不够
5. **字符串编码开销**：每个字符串都重复写入长度前缀，实际格式为每个方法重复写入类名前缀，无符号表去重
6. **操作数编码冗余**：每条指令独立编码左操作数和结果地址，三地址码中 result 与 left 经常是同一地址（如 `t1 = t1 + t2`），可以做差值编码

### 8.3 C# 参考设计中值得借鉴的点

C# 参考实现的优化器构建了 **DAG（有向无环图）**进行全局公共子表达式消除（CSE），使用 `Expression` 不可变结构做值编号（value numbering），并通过 `DoKill()` 进行副作用分析（如 SetField 杀死 LoadField 的定值）。当前 Rust 优化器较简化，缺少 CSE 和副作用建模，这些都可以在字节码层面进一步优化。

---

## 9. 总结

Gorge 的中间代码序列化设计采用**自定义二进制格式**，以三地址码 IR 为核心，通过多层嵌套结构（模块→类→方法→指令）组织数据。格式设计兼顾了紧凑性、可扩展性和向后兼容性：魔数 + 版本号保证格式识别和演进，标签化操作数编码支持不同类型的立即数，游标式反序列化配合全面的越界检查确保健壮性。

核心设计决策——**将 IR 从内存对象持久化为独立二进制文件**——使得 Gorge 编译器与运行时解耦，支持"编译一次，多次执行"的工作流程，这是相比 C# 参考实现的关键架构创新。
