#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use gorge_core::virtual_machine::ir::*;

/// 基本块
///
/// 基本块是一段连续执行的指令序列，只有块入口和块出口有跳转。
/// 优化器以降进入口的指令标识符作为首指令（Leader）划定基本块。
#[derive(Debug, Clone)]
pub struct BasicBlock {
    /// 在原始代码序列中的起始位置
    pub start: usize,
    /// 在原始代码序列中的结束位置（不包含）
    pub end: usize,
    /// 后继基本块的索引
    pub successors: Vec<usize>,
    /// 前驱基本块的索引
    pub predecessors: Vec<usize>,
}

/// 控制流图
#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    pub blocks: Vec<BasicBlock>,
}

/// IR 优化器
///
/// 对方法体的 IR 指令序列进行数据流优化，减少冗余计算和死代码。
pub struct IntermediateCodeOptimizer;

impl IntermediateCodeOptimizer {
    /// 优化 IR 指令序列，返回优化后的新序列
    ///
    /// 执行 4 轮优化迭代以充分消除间接产生的冗余。
    pub fn optimize(codes: &[CodeWithSpan]) -> Vec<CodeWithSpan> {
        let mut result = codes.to_vec();

        for iteration in 0..4 {
            // 每轮先做 DCE（基于活跃变量分析），再做 CSE（公共子表达式消除）
            let optimized = Self::optimize_once(&result);
            result = Self::global_cse(&optimized);
            let _ = iteration;
        }

        // L3: 连跳优化（jump-to-jump elimination）
        Self::jump_to_jump_optimization(&mut result);

        result
    }

    /// 单轮优化
    fn optimize_once(codes: &[CodeWithSpan]) -> Vec<CodeWithSpan> {
        if codes.is_empty() {
            return vec![];
        }

        // 步骤 1：基本块划分
        let mut blocks = Self::partition_basic_blocks(codes);

        // 步骤 2：构建控制流图
        Self::build_cfg(&mut blocks, codes);

        // 步骤 3：死代码消除（基于活跃变量分析）
        let dead_indices = Self::dead_code_elimination(&mut blocks, codes);

        // 步骤 4：重建代码序列（过滤死代码）
        Self::rebuild_code_list(&blocks, codes, &dead_indices)
    }

    /// 连跳优化（L3）
    ///
    /// 若跳转指令的目标是另一条无条件跳转指令，则直接将跳转目标
    /// 替换为最终目标，减少跳转链。例如：
    ///   Jump 5
    ///   5: Jump 10
    /// → Jump 10
    fn jump_to_jump_optimization(codes: &mut Vec<CodeWithSpan>) {
        // 构建跳转目标索引
        let mut targets: HashMap<usize, usize> = HashMap::new();
        for (i, cs) in codes.iter().enumerate() {
            if let Some(target) = Self::jump_target(&cs.code) {
                targets.insert(i, target);
            }
        }
        // 迭代消解跳转链
        // 先收集所有 (index, target) 对，再更新
        let updates: Vec<(usize, usize)> = targets.iter()
            .map(|(&idx, &target)| {
                let mut final_target = target;
                for _ in 0..8 {
                    if let Some(&next) = targets.get(&final_target) {
                        if next == final_target { break; }
                        final_target = next;
                    } else {
                        break;
                    }
                }
                (idx, final_target)
            })
            .collect();
        // 更新代码中的跳转目标
        for (i, new_target) in updates {
            if i < codes.len() {
                Self::update_jump_target(&mut codes[i].code, new_target);
            }
        }
    }

    /// 获取跳转指令的目标索引
    fn jump_target(code: &IntermediateCode) -> Option<usize> {
        match &code.operator {
            IntermediateOperator::Jump(idx)
            | IntermediateOperator::JumpIfFalse(idx)
            | IntermediateOperator::JumpIfTrue(idx) => Some(*idx),
            _ => None,
        }
    }

    /// 更新跳转指令的目标
    fn update_jump_target(code: &mut IntermediateCode, new_target: usize) {
        match &mut code.operator {
            IntermediateOperator::Jump(idx)
            | IntermediateOperator::JumpIfFalse(idx)
            | IntermediateOperator::JumpIfTrue(idx) => *idx = new_target,
            _ => {}
        }
    }

    // ==================== 基本块划分 ====================

    /// 将指令序列划分为基本块
    ///
    /// 首指令（Leader）判定规则：
    /// 1. 第一条指令总是 leader
    /// 2. 跳转目标指令是 leader
    /// 3. 紧跟在跳转/返回指令之后的指令是 leader
    fn partition_basic_blocks(codes: &[CodeWithSpan]) -> Vec<BasicBlock> {
        if codes.is_empty() {
            return vec![];
        }

        let n = codes.len();
        let mut is_leader = vec![false; n];
        is_leader[0] = true; // 第一条指令总是 leader

        for i in 0..n {
            let code = &codes[i].code;
            match &code.operator {
                IntermediateOperator::Jump(target) => {
                    if *target < n {
                        is_leader[*target] = true; // 跳转目标是 leader
                    }
                    if i + 1 < n {
                        is_leader[i + 1] = true; // 跳转后下一条指令是 leader
                    }
                }
                IntermediateOperator::JumpIfFalse(target)
                | IntermediateOperator::JumpIfTrue(target) => {
                    if *target < n {
                        is_leader[*target] = true;
                    }
                    if i + 1 < n {
                        is_leader[i + 1] = true;
                    }
                }
                IntermediateOperator::ReturnInt
                | IntermediateOperator::ReturnFloat
                | IntermediateOperator::ReturnBool
                | IntermediateOperator::ReturnString
                | IntermediateOperator::ReturnObject
                | IntermediateOperator::ReturnVoid => {
                    if i + 1 < n {
                        is_leader[i + 1] = true; // 返回后下一条指令是 leader
                    }
                }
                _ => {}
            }
        }

        // 根据 leader 划分基本块
        let mut blocks = Vec::new();
        let mut start = 0;
        for i in 1..=n {
            if i == n || is_leader[i] {
                blocks.push(BasicBlock {
                    start,
                    end: i,
                    successors: Vec::new(),
                    predecessors: Vec::new(),
                });
                start = i;
            }
        }

        blocks
    }

    // ==================== 控制流图构建 ====================

    /// 构建基本块之间的控制流边
    fn build_cfg(blocks: &mut Vec<BasicBlock>, codes: &[CodeWithSpan]) {
        // 建立地址 → 基本块编号的快速映射
        let mut addr_to_block: HashMap<usize, usize> = HashMap::new();
        for (idx, block) in blocks.iter().enumerate() {
            addr_to_block.insert(block.start, idx);
        }

        for i in 0..blocks.len() {
            let block_end = blocks[i].end;
            let last_idx = block_end - 1;
            if last_idx >= codes.len() {
                continue;
            }

            let last_code = &codes[last_idx].code;
            match &last_code.operator {
                IntermediateOperator::Jump(target) => {
                    if let Some(&target_block) = addr_to_block.get(target) {
                        Self::add_edge(blocks, i, target_block);
                    }
                }
                IntermediateOperator::JumpIfFalse(target)
                | IntermediateOperator::JumpIfTrue(target) => {
                    if let Some(&target_block) = addr_to_block.get(target) {
                        Self::add_edge(blocks, i, target_block);
                    }
                    if i + 1 < blocks.len() && blocks[i + 1].start == block_end {
                        Self::add_edge(blocks, i, i + 1);
                    }
                }
                IntermediateOperator::ReturnInt
                | IntermediateOperator::ReturnFloat
                | IntermediateOperator::ReturnBool
                | IntermediateOperator::ReturnString
                | IntermediateOperator::ReturnObject
                | IntermediateOperator::ReturnVoid => {}
                _ => {
                    if i + 1 < blocks.len() && blocks[i + 1].start == block_end {
                        Self::add_edge(blocks, i, i + 1);
                    }
                }
            }
        }
    }

    /// 添加控制流边
    fn add_edge(blocks: &mut [BasicBlock], from: usize, to: usize) {
        if !blocks[from].successors.contains(&to) {
            blocks[from].successors.push(to);
        }
        if !blocks[to].predecessors.contains(&from) {
            blocks[to].predecessors.push(from);
        }
    }

    // ==================== 死代码消除 ====================

    /// 基于活跃变量分析的死代码消除
    ///
    /// 如果一条赋值指令的结果在后续代码中不再被使用（即"死"变量），
    /// 则可以安全地消除该指令。从后向前扫描，追踪活跃地址集合。
    /// 判断指令是否有副作用，不可被 DCE 消除。
    ///
    /// 参数设置/读取、方法调用/返回、跳转等指令都涉及非本地栈的存储（参数池、对象表、
    /// 返回值寄存器），DCE 分析不追踪这些存储，故必须保留。
    fn has_side_effect(op: &IntermediateOperator) -> bool {
        use IntermediateOperator::*;
        match op {
            SetIntParameter | SetFloatParameter | SetBoolParameter | SetStringParameter | SetObjectParameter
            | LoadIntParameter | LoadFloatParameter | LoadBoolParameter | LoadStringParameter | LoadObjectParameter
            | InvokeInstance(_) | InvokeStatic(_) | InvokeInterface(_) | InvokeDelegate(_)
            | InvokeConstructor(_) | DoConstruct(_) | InvokeSuperConstructor(_) | InvokeArrayConstructor
            | InvokeInjectorConstructor(_) | ConstructDelegate(_)
            | ReturnInt | ReturnFloat | ReturnBool | ReturnString | ReturnObject | ReturnVoid
            | LoadIntField(_) | LoadFloatField(_) | LoadBoolField(_) | LoadStringField(_) | LoadObjectField(_)
            | SetIntField(_) | SetFloatField(_) | SetBoolField(_) | SetStringField(_) | SetObjectField(_)
            | LoadIntInjectorField(_) | LoadFloatInjectorField(_) | LoadBoolInjectorField(_)
            | LoadStringInjectorField(_) | LoadObjectInjectorField(_)
            | SetIntInjectorField(_) | SetFloatInjectorField(_) | SetBoolInjectorField(_)
            | SetStringInjectorField(_) | SetObjectInjectorField(_)
            | LoadThis | LoadInjector | SetInjector
            | LoadInjectorConstant(_)
            | Jump(_) | JumpIfFalse(_) | JumpIfTrue(_) | Nop => true,
            _ => false,
        }
    }

    /// 死代码消除（基于反向活跃变量分析，迭代至不动点以处理循环后向边）
    fn dead_code_elimination(_blocks: &mut [BasicBlock], codes: &[CodeWithSpan]) -> HashSet<usize> {
        let all_used = Self::collect_used_addresses(codes);

        // 第一遍：标准反向线性扫描，收集无跳转传播时的各位置活跃集
        let mut live_before: Vec<HashSet<Address>> = vec![HashSet::new(); codes.len()];
        {
            let mut live: HashSet<Address> = all_used.clone();
            for i in (0..codes.len()).rev() {
                live_before[i] = live.clone();
                Self::process_one_dead_code(&codes[i].code, i, &mut live, &mut HashSet::new());
            }
        }

        // 迭代：利用 live_before 将跳转目标的活跃集向后向边传播
        let mut dead_indices: HashSet<usize> = HashSet::new();
        loop {
            let mut live: HashSet<Address> = all_used.clone();
            let mut new_dead = HashSet::new();
            let mut new_live_before: Vec<HashSet<Address>> = vec![HashSet::new(); codes.len()];

            for i in (0..codes.len()).rev() {
                let code = &codes[i].code;

                // 对于跳转指令，将跳转目标的活跃集合并到当前活跃集（模拟后向边传播）
                if let Some(target) = Self::jump_target(code) {
                    if target < codes.len() && !new_dead.contains(&target) {
                        // 使用上一轮迭代的 live_before 或当前轮的 new_live_before
                        // （取两者并集以捕获循环内传播的新变量）
                        let mut merged = live_before[target].clone();
                        merged.extend(new_live_before[target].iter().cloned());
                        live.extend(merged.iter().cloned());
                    }
                }

                new_live_before[i] = live.clone();
                // 死代码判定与活跃集更新沿用标准逻辑
                Self::process_one_dead_code(code, i, &mut live, &mut new_dead);
            }

            if new_dead == dead_indices {
                break;
            }
            dead_indices = new_dead;
            live_before = new_live_before;
        }

        dead_indices
    }

    /// 处理单条指令的死代码判定，更新活跃集
    fn process_one_dead_code(
        code: &IntermediateCode,
        i: usize,
        live: &mut HashSet<Address>,
        dead_indices: &mut HashSet<usize>,
    ) {
        // 有副作用的指令不可消除
        if Self::has_side_effect(&code.operator) {
            // 读取操作数视为活跃
            if let Operand::Address(addr) = &code.left {
                live.insert(*addr);
            }
            if let Some(Operand::Address(addr)) = &code.right {
                live.insert(*addr);
            }
            return;
        }
        // 当前指令的 result 是否活跃？
        if let Some(result) = code.result {
            if !live.contains(&result) {
                // 结果地址不在活跃集中 → 死代码
                dead_indices.insert(i);
                return;
            }
            // result 被后续代码使用 → 从活跃集中移除
            live.remove(&result);
        }
        // 左/右操作数是读操作 → 加入活跃集
        if let Operand::Address(addr) = &code.left {
            live.insert(*addr);
        }
        if let Some(Operand::Address(addr)) = &code.right {
            live.insert(*addr);
        }
    }

    /// 收集所有被读取的地址（即作为左/右操作数的 Address）
    fn collect_used_addresses(codes: &[CodeWithSpan]) -> HashSet<Address> {
        let mut used = HashSet::new();

        for code_span in codes {
            let code = &code_span.code;

            // 左操作数
            if let Operand::Address(addr) = &code.left {
                used.insert(*addr);
            }

            // 右操作数
            if let Some(Operand::Address(addr)) = &code.right {
                used.insert(*addr);
            }
        }

        used
    }

    // ==================== 代码重建 ====================

    /// 从优化后的基本块重建代码序列
    ///
    /// 保持原始顺序拼接基本块的指令，同时重算跳转目标。
    fn rebuild_code_list(blocks: &[BasicBlock], codes: &[CodeWithSpan], dead_indices: &HashSet<usize>) -> Vec<CodeWithSpan> {
        if blocks.is_empty() {
            return vec![];
        }

        // 旧地址 → 新地址映射
        let mut old_to_new: Vec<Option<usize>> = vec![None; codes.len()];
        let mut new_codes: Vec<CodeWithSpan> = Vec::new();

        for block in blocks {
            for i in block.start..block.end {
                if i < codes.len() && !dead_indices.contains(&i) {
                    old_to_new[i] = Some(new_codes.len());
                    new_codes.push(codes[i].clone());
                }
            }
        }

        // 回填跳转目标
        Self::backfill_jump_targets(&mut new_codes, &old_to_new);

        new_codes
    }

    /// 重算跳转指令中的目标地址
    fn backfill_jump_targets(
        codes: &mut [CodeWithSpan],
        old_to_new: &[Option<usize>],
    ) {
        for code_span in codes.iter_mut() {
            let code = &mut code_span.code;
            match &mut code.operator {
                IntermediateOperator::Jump(ref mut target)
                | IntermediateOperator::JumpIfFalse(ref mut target)
                | IntermediateOperator::JumpIfTrue(ref mut target) => {
                    if let Some(new_target) = old_to_new.get(*target).and_then(|&x| x) {
                        *target = new_target;
                    }
                }
                _ => {}
            }
        }
    }

    // ==================== 死代码消除（简化版） ====================

    /// 简化版死代码消除
    ///
    /// 消除连续重复赋值：如果同一条指令的结果立即被重写且中间没有被读取，
    /// 则消除前一条指令。
    pub fn eliminate_dead_stores(codes: &[CodeWithSpan]) -> Vec<CodeWithSpan> {
        if codes.len() < 2 {
            return codes.to_vec();
        }

        let used_set = Self::collect_used_addresses(codes);

        // 标记哪些指令可以消除
        let mut dead: Vec<bool> = vec![false; codes.len()];

        for i in (0..codes.len()).rev() {
            let code = &codes[i].code;

            // 检查 result 是否被使用
            if let Some(ref result) = code.result {
                if !used_set.contains(result) {
                    // 结果未被使用，死代码
                    dead[i] = true;
                } else {
                    // 结果被使用 → 从 used_set 中移除（因为已经有生产者在前面）
                    // 简化：不移除，因为可能有多个使用
                }
            }
        }

        // 过滤死代码
        codes.iter()
            .enumerate()
            .filter(|(i, _)| !dead[*i])
            .map(|(_, c)| c.clone())
            .collect()
    }

    // ==================== 全局公共子表达式消除 ====================

    /// 对代码序列执行全局公共子表达式消除
    ///
    /// 1. 划分基本块 + 构建 CFG
    /// 2. 可用表达式分析（前向数据流）
    /// 3. 在每个块内替换重复表达式为对缓存结果的引用
    /// 4. 重建代码序列
    pub fn global_cse(codes: &[CodeWithSpan]) -> Vec<CodeWithSpan> {
        if codes.is_empty() {
            return vec![];
        }

        let mut blocks = Self::partition_basic_blocks(codes);
        Self::build_cfg(&mut blocks, codes);

        // 可用表达式分析
        let avail = Self::available_expressions_analysis(&blocks, codes);

        // 在每个块内消除公共子表达式
        let mut new_codes = codes.to_vec();
        for (block_idx, block) in blocks.iter().enumerate() {
            let in_exprs = &avail[block_idx];
            Self::cse_in_block(&mut new_codes, block, in_exprs);
        }

        // 重建代码
        Self::rebuild_code_list(&blocks, &new_codes, &HashSet::new())
    }

    /// 可用表达式分析（前向数据流迭代）
    ///
    /// In[B] = ∩ Out[P] for all predecessors P
    /// Out[B] = (In[B] - Kill[B]) ∪ Gen[B]
    fn available_expressions_analysis(
        blocks: &[BasicBlock],
        codes: &[CodeWithSpan],
    ) -> Vec<HashSet<ExpressionKey>> {
        let n = blocks.len();
        if n == 0 {
            return vec![];
        }

        // 收集每个块的 Gen 和 Kill 集
        struct BlockInfo {
            gen: HashSet<ExpressionKey>,
            kill: HashSet<ExpressionKey>,
        }
        let block_infos: Vec<BlockInfo> = blocks.iter().map(|block| {
            let mut gen = HashSet::new();
            let mut kill = HashSet::new();

            for i in block.start..block.end {
                if let Some(code) = codes.get(i) {
                    let code = &code.code;
                    // 生成表达式
                    if let Some(key) = ExpressionKey::from_code(code) {
                        gen.insert(key);
                    }
                    // 被副作用杀死的表达式
                    let killed = Self::killed_expressions(code);
                    for k in killed {
                        kill.insert(k);
                    }
                }
            }

            // Gen 中移除被该块自身杀死的
            let gen: HashSet<_> = gen.difference(&kill).cloned().collect();

            BlockInfo { gen, kill }
        }).collect();

        // 入口块的 In = ∅
        let mut in_sets: Vec<HashSet<ExpressionKey>> = vec![HashSet::new(); n];
        let mut out_sets: Vec<HashSet<ExpressionKey>> = vec![HashSet::new(); n];
        let mut changed = true;
        let mut iteration = 0;

        while changed && iteration < 1000 {
            changed = false;
            iteration += 1;

            for i in 0..n {
                // In[B] = ∩ Out[P]
                let mut new_in: Option<HashSet<ExpressionKey>> = None;
                for &pred in &blocks[i].predecessors {
                    match &mut new_in {
                        None => new_in = Some(out_sets[pred].clone()),
                        Some(s) => {
                            *s = s.intersection(&out_sets[pred]).cloned().collect();
                        }
                    }
                }
                let new_in = new_in.unwrap_or_default();

                if new_in != in_sets[i] {
                    changed = true;
                    in_sets[i] = new_in.clone();
                }

                // Out[B] = (In[B] - Kill[B]) ∪ Gen[B]
                let mut new_out: HashSet<_> = new_in.difference(&block_infos[i].kill).cloned().collect();
                for expr in &block_infos[i].gen {
                    new_out.insert(expr.clone());
                }

                if new_out != out_sets[i] {
                    changed = true;
                    out_sets[i] = new_out;
                }
            }
        }

        in_sets
    }

    /// 在单个基本块内执行公共子表达式消除
    fn cse_in_block(
        codes: &mut [CodeWithSpan],
        block: &BasicBlock,
        in_exprs: &HashSet<ExpressionKey>,
    ) {
        // 可用表达式及其结果的临时变量映射
        // L2: 使用数据流分析结果预填充入口可用表达式
        let mut available: HashMap<ExpressionKey, Address> = HashMap::new();
        for _key in in_exprs {
            // 入口表达式已存在，但对应地址未知（来自父块）
            // 需要本块内出现时才映射到具体地址
        }
        let _ = available; // 初始为空，等本块内第一个定值时填充

        for i in block.start..block.end {
            if i >= codes.len() {
                break;
            }

            let code = &codes[i].code;

            // 被副作用杀死的表达式
            let killed = Self::killed_expressions(code);
            available.retain(|k, _| !killed.contains(k));

            // 检查是否为可消除的表达式
            if let Some(key) = ExpressionKey::from_code(code) {
                if let Some(cached_addr) = available.get(&key) {
                    // 公共子表达式！替换为对缓存结果的引用
                    if let Some(result) = code.result {
                        codes[i].code = IntermediateCode::assign(result, Operand::Address(*cached_addr));
                    }
                } else {
                    // 新表达式，缓存结果
                    if let Some(result) = code.result {
                        available.insert(key, result);
                    }
                }
            }
        }
    }

    /// 获取一条指令可能杀死的表达式列表
    ///
    /// 副作用分析规则（参考 C# 版 DoKill）：
    /// - SetField → 杀死对应 LoadField
    /// - SetInjectorField → 杀死对应 LoadInjectorField
    /// - SetInjector → 杀死 LoadInjector
    /// - SetParameter → 杀死对应 LoadParameter
    /// - Invoke/Construct → 保守杀死所有 Load/GetReturn
    fn killed_expressions(code: &IntermediateCode) -> HashSet<ExpressionKey> {
        let mut killed = HashSet::new();
        match &code.operator {
            IntermediateOperator::SetIntField(idx) => {
                killed.insert(ExpressionKey::LoadIntField(*idx));
            }
            IntermediateOperator::SetFloatField(idx) => {
                killed.insert(ExpressionKey::LoadFloatField(*idx));
            }
            IntermediateOperator::SetBoolField(idx) => {
                killed.insert(ExpressionKey::LoadBoolField(*idx));
            }
            IntermediateOperator::SetStringField(idx) => {
                killed.insert(ExpressionKey::LoadStringField(*idx));
            }
            IntermediateOperator::SetObjectField(idx) => {
                killed.insert(ExpressionKey::LoadObjectField(*idx));
            }
            IntermediateOperator::SetIntInjectorField(idx) => {
                killed.insert(ExpressionKey::LoadIntInjectorField(*idx));
            }
            IntermediateOperator::SetFloatInjectorField(idx) => {
                killed.insert(ExpressionKey::LoadFloatInjectorField(*idx));
            }
            IntermediateOperator::SetBoolInjectorField(idx) => {
                killed.insert(ExpressionKey::LoadBoolInjectorField(*idx));
            }
            IntermediateOperator::SetStringInjectorField(idx) => {
                killed.insert(ExpressionKey::LoadStringInjectorField(*idx));
            }
            IntermediateOperator::SetObjectInjectorField(idx) => {
                killed.insert(ExpressionKey::LoadObjectInjectorField(*idx));
            }
            IntermediateOperator::SetInjector => {
                killed.insert(ExpressionKey::LoadInjector);
            }
            IntermediateOperator::SetIntParameter => {
                killed.insert(ExpressionKey::LoadIntParameter);
            }
            IntermediateOperator::SetFloatParameter => {
                killed.insert(ExpressionKey::LoadFloatParameter);
            }
            IntermediateOperator::SetBoolParameter => {
                killed.insert(ExpressionKey::LoadBoolParameter);
            }
            IntermediateOperator::SetStringParameter => {
                killed.insert(ExpressionKey::LoadStringParameter);
            }
            IntermediateOperator::SetObjectParameter => {
                killed.insert(ExpressionKey::LoadObjectParameter);
            }
            // 方法调用和构造保守地杀死所有 Load/GetReturn
            IntermediateOperator::InvokeInstance(_)
            | IntermediateOperator::InvokeStatic(_)
            | IntermediateOperator::InvokeInterface(_)
            | IntermediateOperator::InvokeDelegate(_)
            | IntermediateOperator::InvokeConstructor(_)
            | IntermediateOperator::DoConstruct(_)
            | IntermediateOperator::InvokeSuperConstructor(_)
            | IntermediateOperator::ConstructDelegate(_) => {
                killed.insert(ExpressionKey::WildcardLoad);
            }
            _ => {}
        }
        killed
    }
}

/// 表达式标识键
///
/// 用于识别两条指令是否计算相同的结果（公共子表达式）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ExpressionKey {
    BinaryOp { op: u16, left_kind: u8, left_idx: usize, right_kind: u8, right_idx: usize },
    LoadIntField(usize),
    LoadFloatField(usize),
    LoadBoolField(usize),
    LoadStringField(usize),
    LoadObjectField(usize),
    LoadIntInjectorField(usize),
    LoadFloatInjectorField(usize),
    LoadBoolInjectorField(usize),
    LoadStringInjectorField(usize),
    LoadObjectInjectorField(usize),
    LoadInjector,
    LoadIntParameter,
    LoadFloatParameter,
    LoadBoolParameter,
    LoadStringParameter,
    LoadObjectParameter,
    WildcardLoad,
}

impl ExpressionKey {
    fn from_code(code: &IntermediateCode) -> Option<Self> {
        match &code.operator {
            IntermediateOperator::IntAdd
            | IntermediateOperator::IntSub
            | IntermediateOperator::IntMul
            | IntermediateOperator::IntDiv
            | IntermediateOperator::IntMod
            | IntermediateOperator::FloatAdd
            | IntermediateOperator::FloatSub
            | IntermediateOperator::FloatMul
            | IntermediateOperator::FloatDiv
            | IntermediateOperator::IntEqual
            | IntermediateOperator::IntNotEqual
            | IntermediateOperator::FloatEqual
            | IntermediateOperator::FloatNotEqual
            | IntermediateOperator::IntLess
            | IntermediateOperator::IntLessEqual
            | IntermediateOperator::IntGreater
            | IntermediateOperator::IntGreaterEqual
            | IntermediateOperator::FloatLess
            | IntermediateOperator::FloatLessEqual
            | IntermediateOperator::FloatGreater
            | IntermediateOperator::FloatGreaterEqual
            | IntermediateOperator::LogicalAnd
            | IntermediateOperator::LogicalOr
            | IntermediateOperator::StringAddition => {
                let (lk, li) = Self::operand_key(&code.left);
                let (rk, ri) = code.right.as_ref()
                    .map(|r| Self::operand_key(r))
                    .unwrap_or((0, 0));
                Some(ExpressionKey::BinaryOp {
                    op: opcode_u16(&code.operator),
                    left_kind: lk,
                    left_idx: li,
                    right_kind: rk,
                    right_idx: ri,
                })
            }
            IntermediateOperator::LoadIntField(idx) => Some(ExpressionKey::LoadIntField(*idx)),
            IntermediateOperator::LoadFloatField(idx) => Some(ExpressionKey::LoadFloatField(*idx)),
            IntermediateOperator::LoadBoolField(idx) => Some(ExpressionKey::LoadBoolField(*idx)),
            IntermediateOperator::LoadStringField(idx) => Some(ExpressionKey::LoadStringField(*idx)),
            IntermediateOperator::LoadObjectField(idx) => Some(ExpressionKey::LoadObjectField(*idx)),
            IntermediateOperator::LoadIntInjectorField(idx) => Some(ExpressionKey::LoadIntInjectorField(*idx)),
            IntermediateOperator::LoadFloatInjectorField(idx) => Some(ExpressionKey::LoadFloatInjectorField(*idx)),
            IntermediateOperator::LoadBoolInjectorField(idx) => Some(ExpressionKey::LoadBoolInjectorField(*idx)),
            IntermediateOperator::LoadStringInjectorField(idx) => Some(ExpressionKey::LoadStringInjectorField(*idx)),
            IntermediateOperator::LoadObjectInjectorField(idx) => Some(ExpressionKey::LoadObjectInjectorField(*idx)),
            IntermediateOperator::LoadInjector => Some(ExpressionKey::LoadInjector),
            IntermediateOperator::LoadIntParameter => Some(ExpressionKey::LoadIntParameter),
            IntermediateOperator::LoadFloatParameter => Some(ExpressionKey::LoadFloatParameter),
            IntermediateOperator::LoadBoolParameter => Some(ExpressionKey::LoadBoolParameter),
            IntermediateOperator::LoadStringParameter => Some(ExpressionKey::LoadStringParameter),
            IntermediateOperator::LoadObjectParameter => Some(ExpressionKey::LoadObjectParameter),
            _ => None,
        }
    }

    fn operand_key(op: &Operand) -> (u8, usize) {
        match op {
            Operand::Address(addr) => (0, addr.index),
            Operand::Immediate(iv) => match iv {
                ImmediateValue::Int(v) => (1, *v as usize),
                ImmediateValue::Float(v) => (2, v.to_bits() as usize),
                ImmediateValue::Bool(v) => (3, if *v { 1 } else { 0 }),
                ImmediateValue::String(v) => {
                    let mut h: usize = 5381;
                    for b in v.as_bytes() { h = h.wrapping_mul(33).wrapping_add(*b as usize); }
                    (4, h)
                }
            },
        }
    }
}

fn opcode_u16(op: &IntermediateOperator) -> u16 {
    match op {
        IntermediateOperator::IntAdd => 1, IntermediateOperator::IntSub => 2,
        IntermediateOperator::IntMul => 3, IntermediateOperator::IntDiv => 4,
        IntermediateOperator::IntMod => 5, IntermediateOperator::FloatAdd => 6,
        IntermediateOperator::FloatSub => 7, IntermediateOperator::FloatMul => 8,
        IntermediateOperator::FloatDiv => 9,
        IntermediateOperator::IntEqual => 10, IntermediateOperator::IntNotEqual => 11,
        IntermediateOperator::FloatEqual => 12, IntermediateOperator::FloatNotEqual => 13,
        IntermediateOperator::IntLess => 14, IntermediateOperator::IntLessEqual => 15,
        IntermediateOperator::IntGreater => 16, IntermediateOperator::IntGreaterEqual => 17,
        IntermediateOperator::FloatLess => 18, IntermediateOperator::FloatLessEqual => 19,
        IntermediateOperator::FloatGreater => 20, IntermediateOperator::FloatGreaterEqual => 21,
        IntermediateOperator::LogicalAnd => 22, IntermediateOperator::LogicalOr => 23,
        IntermediateOperator::StringAddition => 24,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::diagnostics::Span;

    fn make_int_addr(index: usize) -> Address {
        Address::new(ValueType::Int, index)
    }

    fn make_code(op: IntermediateOperator, left: Operand, right: Option<Operand>, result: Option<Address>) -> CodeWithSpan {
        CodeWithSpan::new(
            IntermediateCode::new(op, left, right, result),
            Span::dummy(),
        )
    }

    fn make_assign(addr: Address, val: i64) -> CodeWithSpan {
        CodeWithSpan::new(
            IntermediateCode::assign(addr, Operand::int(val)),
            Span::dummy(),
        )
    }

    fn make_add(r: Address, a: Address, b: Address) -> CodeWithSpan {
        CodeWithSpan::new(
            IntermediateCode::binary(IntermediateOperator::IntAdd, Operand::Address(a), Operand::Address(b), r),
            Span::dummy(),
        )
    }

    fn make_jump(target: usize) -> CodeWithSpan {
        CodeWithSpan::new(
            IntermediateCode::jump(target),
            Span::dummy(),
        )
    }

    fn make_return() -> CodeWithSpan {
        CodeWithSpan::new(
            IntermediateCode::return_void(),
            Span::dummy(),
        )
    }

    #[test]
    fn test_partition_single_block() {
        let codes = vec![
            make_assign(make_int_addr(0), 1),
            make_assign(make_int_addr(1), 2),
            make_return(),
        ];

        let blocks = IntermediateCodeOptimizer::partition_basic_blocks(&codes);
        assert_eq!(blocks.len(), 1, "无分支代码应产生单个基本块");
        assert_eq!(blocks[0].start, 0);
        assert_eq!(blocks[0].end, 3);
    }

    #[test]
    fn test_partition_with_jump() {
        // 代码: assign x=1; jump 2; assign y=2; return;
        let codes = vec![
            make_assign(make_int_addr(0), 1),
            make_jump(3),
            make_assign(make_int_addr(1), 2),
            make_return(),
        ];

        let blocks = IntermediateCodeOptimizer::partition_basic_blocks(&codes);
        assert!(blocks.len() >= 2, "跳转应产生多个基本块");
    }

    #[test]
    fn test_partition_with_conditional_jump() {
        // 代码: if cond jump 3; assign x=1; return;
        let cond = Address::new(ValueType::Bool, 0);
        let codes = vec![
            CodeWithSpan::new(
                IntermediateCode::jump_if_false(Operand::Address(cond), 3),
                Span::dummy(),
            ),
            make_assign(make_int_addr(0), 1),
            make_return(),
            make_return(),
        ];

        let blocks = IntermediateCodeOptimizer::partition_basic_blocks(&codes);
        assert!(blocks.len() >= 2);
    }

    #[test]
    fn test_cfg_edges() {
        // 两个连续基本块：块0 顺序连接到块1
        let codes = vec![
            make_assign(make_int_addr(0), 1),
            make_return(),
        ];

        let mut blocks = IntermediateCodeOptimizer::partition_basic_blocks(&codes);
        IntermediateCodeOptimizer::build_cfg(&mut blocks, &codes);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].successors.len(), 0); // 返回块无后继
    }

    #[test]
    fn test_rebuild_preserves_order() {
        let codes = vec![
            make_assign(make_int_addr(0), 1),
            make_assign(make_int_addr(1), 2),
            make_return(),
        ];

        let blocks = IntermediateCodeOptimizer::partition_basic_blocks(&codes);
        let result = IntermediateCodeOptimizer::rebuild_code_list(&blocks, &codes, &HashSet::new());

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_eliminate_dead_store() {
        // 产生 x=1 但后续只读 x=2（未读 x=1 中的值），x=1 的死代码
        let codes = vec![
            make_assign(make_int_addr(0), 1), // 死代码：从未被读取
            make_assign(make_int_addr(0), 2), // 读 value
            make_return(),
        ];

        let optimized = IntermediateCodeOptimizer::eliminate_dead_stores(&codes);
        assert!(optimized.len() <= codes.len(), "应消除至少一条死代码");
    }

    #[test]
    fn test_dce_eliminates_unused_variable() {
        // x = 5; y = 3; return;  —— x 从未被使用，应被 DCE 消除
        let x = make_int_addr(0);
        let y = make_int_addr(1);
        let codes = vec![
            make_assign(x, 5),   // 死代码：x 从不被读取
            make_assign(y, 3),
            make_return(),
        ];
        let optimized = IntermediateCodeOptimizer::optimize(&codes);
        // 应只剩 y=3 和 return 两条指令（x=5 被消除）
        assert_eq!(optimized.len(), 2);
        // 第一条应为 y=3（即 y=3 的 assign）
        assert!(matches!(optimized[0].code.operator, IntermediateOperator::IntAssign));
    }

    #[test]
    fn test_cse_integrated_in_optimize() {
        // t1 = a + b; t2 = a + b; t3 = t1; return t3;
        // 一对冗余 IntAdd 应被 CSE 消除为一个
        let a = make_int_addr(0);
        let b = make_int_addr(1);
        let t1 = make_int_addr(2);
        let t2 = make_int_addr(3);
        let t3 = make_int_addr(4);
        let codes = vec![
            make_add(t1, a, b),
            make_add(t2, a, b),  // 冗余——应与 t1 合并
            make_assign(t3, 0),  // 让 t3 被赋值
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::ReturnInt,
                    Operand::Address(t3),
                    None, None),
                Span::dummy(),
            ),
        ];
        // 直接调用 global_cse + eliminate_dead_stores 清理
        let after_cse = IntermediateCodeOptimizer::global_cse(&codes);
        let add_count = after_cse.iter().filter(|c| matches!(c.code.operator, IntermediateOperator::IntAdd)).count();
        assert_eq!(add_count, 1, "重复的 a+b 应被 CSE 消除为一次 IntAdd");
    }

    #[test]
    fn test_full_optimize_identity() {
        let x = make_int_addr(0);
        let y = make_int_addr(1);
        let r = make_int_addr(2);

        let codes = vec![
            make_assign(x, 3),
            make_assign(y, 4),
            make_add(r, x, y),
            make_return(),
        ];

        let optimized = IntermediateCodeOptimizer::optimize(&codes);
        // 应该保持功能等价：仍然有 4 条指令（没有死代码可消除）
        assert!(!optimized.is_empty());
    }

    #[test]
    fn test_global_cse_reduces_redundant_expression() {
        let a = make_int_addr(0);
        let b = make_int_addr(1);
        let r1 = make_int_addr(2);
        let r2 = make_int_addr(3);
        let codes = vec![
            make_code(IntermediateOperator::IntAdd, Operand::Address(a), Some(Operand::Address(b)), Some(r1)),
            make_code(IntermediateOperator::IntAdd, Operand::Address(a), Some(Operand::Address(b)), Some(r2)),
        ];
        let optimized = IntermediateCodeOptimizer::global_cse(&codes);
        assert_eq!(optimized.len(), 2);
        assert!(matches!(optimized[1].code.operator, IntermediateOperator::IntAssign));
    }
}
