//! `GorgeFramework.SignalTsiga` —— 信号时序栈自动机 native 类（S7 + E-1 完整实现）。
//!
//! 移植自 C# 参考实现 `SignalTsiga.cs`。TSIGA = Time Stack Input Graph Automaton，
//! 是框架核心判定引擎，组合 InputGraph（输入状态图）和 TimeStack（时序栈）
//! 实现基于信号的状态转移、弹栈响应和停机逻辑。
//!
//! # 方法编号表（#[gorge_native_impl] 声明顺序）
//! | 编号 | 方法 | 说明 |
//! |------|------|------|
//! | 0 | forward_state_change_time | 正向状态转移最早时间 |
//! | 1 | forward_state_change | 正向推进：PopUntil + TimeoutUntil |
//! | 2 | backward_state_change_time | 反向状态转移时间 |
//! | 3 | backward_state_change | 反向回滚 |
//! | 4 | detection_accept | 检测接受 |
//! | 5 | detection_deny | 检测拒绝 |
//! | 6 | get_detection_conditions | 获取检测条件列表 |
//! | 7 | convert_automaton_commands（静态） | ObjectArray → 动作 |
//! | 8 | get_signal_value | 获取信号当前值 |
//! | 9 | get_signal_last_value | 获取信号上次值 |
//! | 10 | update_signal_record | 更新信号记录 |
//! | 11 | do_respond | 执行响应 |
//! | 12 | do_deny | 执行拒绝 |
//! | 13 | pop_until | 弹栈到目标时间 |
//! | 14 | timeout_until | 超时推进到目标时间 |
//! | 15 | do_edge_respond | 出边响应 |

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::{NativeClass, NativeContext};
use gorge_core::system::native::array::{ObjectArray, ObjectArrayClass};
use std::collections::HashMap;

// ==================== 内部数据结构 ====================

/// 信号记录（每信道每信号的状态）
#[derive(Debug, Clone)]
pub struct SignalRecord {
    pub value: usize,
    pub last_value: usize,
}

/// SignalTsiga 的 payload
#[derive(Debug)]
pub struct SignalTsigaPayload {
    pub signal_state: HashMap<String, HashMap<i32, SignalRecord>>,
    pub note_id: usize,
}

// ==================== SignalTsiga（native 注册） ====================

#[gorge_native_class(namespace = "GorgeFramework")]
pub struct SignalTsiga {
    #[gorge_field] pub input_graph: usize,
    #[gorge_field] pub time_stack: usize,
    #[gorge_field] pub history_stack: usize,
}

#[gorge_native_impl]
impl SignalTsiga {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, input_graph: usize, time_stack: usize, history_stack: usize) {
        ctx.set_object_object_field(this, 0, input_graph);
        ctx.set_object_object_field(this, 1, time_stack);
        ctx.set_object_object_field(this, 2, history_stack);
        ctx.insert_payload(this, Box::new(SignalTsigaPayload {
            signal_state: HashMap::new(),
            note_id: 0,
        }));
    }

    /// 方法 0：forward_state_change_time
    #[gorge_method]
    pub fn forward_state_change_time(ctx: &mut NativeContext, this: usize) -> f32 {
        let ig = ctx.get_object_object_field(this, 0);
        let ts = ctx.get_object_object_field(this, 1);
        let ts_acc = ctx.get_object_bool_field(ts, 0);
        let ig_acc = ctx.get_object_bool_field(ig, 0);
        if ts_acc && ig_acc { return f32::MAX; }
        ctx.invoke_native_method_on("GorgeFramework.TimeStack", ts, 0);
        let pt = (ctx.get_float_return() as f64) as f32;
        ctx.invoke_native_method_on("GorgeFramework.InputGraph", ig, 1);
        let st = (ctx.get_float_return() as f64) as f32;
        pt.min(st)
    }

/// 方法 1：forward_state_change
    ///
    /// 返回命令 ObjectArray 对象 ID（0=空），语义对齐 C# `ForwardStateChange`：
    /// 合并 PopUntil 与 TimeoutUntil 产生的指令数组。
    #[gorge_method]
    pub fn forward_state_change(ctx: &mut NativeContext, this: usize, chart_time: f32) -> usize {
        let ts = ctx.get_object_object_field(this, 1);
        let ig = ctx.get_object_object_field(this, 0);
        if ctx.get_object_bool_field(ts, 0) && ctx.get_object_bool_field(ig, 0) { return 0; }
        let mut merged: Vec<usize> = Vec::new();
        ctx.set_float_param(0, chart_time as f64);
        ctx.set_int_param(0, 0);
        ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 13);
        let a1 = ctx.get_object_return();
        if a1 != 0 { merged.extend(ctx.object_array_items(a1)); }
        ctx.set_float_param(0, chart_time as f64);
        ctx.set_int_param(0, 0);
        ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 14);
        let a2 = ctx.get_object_return();
        if a2 != 0 { merged.extend(ctx.object_array_items(a2)); }
        build_command_array(ctx, &merged)
    }

    /// 方法 2：backward_state_change_time
    #[gorge_method]
    pub fn backward_state_change_time(ctx: &mut NativeContext, this: usize) -> f32 {
        let hs = ctx.get_object_object_field(this, 2);
        ctx.invoke_native_method_on("GorgeFramework.HistoryStack", hs, 0);
        (ctx.get_float_return() as f64) as f32
    }

/// 方法 3：backward_state_change
    ///
    /// 返回 HistoryStack.pop_until 产出的受影响自动机 ID ObjectArray 对象 ID（0=空），
    /// 语义对齐 C# `BackwardStateChange`。C# `BackwardStateChange` 仅调用
    /// `HistoryStack.PopUntil` 做弹栈还原，其产生的动作仅为
    /// `UpdatePendingDetectionCondition`（每弹出一个 TimeStackPopHistory 即产生一条）。
    /// Rust 侧 pop_until（history.rs 方法 4）将受影响的自动机 ID 列表封装为 ObjectArray
    /// 返回，此处直接透传其对象 ID（0=空），供 backward_simulate 据此追加
    /// UpdatePendingDetectionCondition 动作。弹栈还原本身仍由下方对 pop_until 的调用完成。
    #[gorge_method]
    pub fn backward_state_change(ctx: &mut NativeContext, this: usize, chart_time: f32) -> usize {
        let hs = ctx.get_object_object_field(this, 2);
        let ig = ctx.get_object_object_field(this, 0);
        let ts = ctx.get_object_object_field(this, 1);
        ctx.set_float_param(0, chart_time as f64);
        ctx.set_object_param(0, this);
        ctx.set_int_param(0, 1);
        ctx.set_object_param(1, ig);
        ctx.set_object_param(2, ts);
        ctx.invoke_native_method_on("GorgeFramework.HistoryStack", hs, 4);
        ctx.get_object_return()
    }

/// 方法 4：detection_accept
    ///
    /// 返回满足检测条件后接收分支产生的命令 ObjectArray 对象 ID（0=空），
    /// 语义对齐 C# `DetectionAccept`（`Accept` 分支调用 `DoEdgeRespond(GoAcceptEdge(...))`
    /// 返回动作数组）。内部经方法 15 `do_edge_respond` 递归产生命令，直接透传其
    /// ObjectArray ID。
    #[gorge_method]
    pub fn detection_accept(ctx: &mut NativeContext, this: usize, chart_time: f32, direction: i32) -> usize {
        let ig = ctx.get_object_object_field(this, 0);
        let hs = ctx.get_object_object_field(this, 2);
        ctx.set_float_param(0, chart_time as f64);
        ctx.set_object_param(0, hs);
        ctx.invoke_native_method_on("GorgeFramework.InputGraph", ig, 3);
        let eid = ctx.get_object_return();
        if eid == 0 { return 0; }
        ctx.set_float_param(0, chart_time as f64);
        ctx.set_object_param(0, eid);
        ctx.set_int_param(0, direction as i64);
        ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 15);
        ctx.get_object_return()
    }

    /// 方法 5：detection_deny
    ///
    /// 返回测量拒绝分支产生的命令 ObjectArray 对象 ID（0=空），
    /// 语义对齐 C# `DetectionDeny`（`timeMode is CatchBefore` 返回空，否则
    /// `DoEdgeRespond(GoDenyEdge(...))` 返回动作数组）。内部经方法 15
    /// `do_edge_respond` 递归产生命令，直接透传其 ObjectArray ID。
    #[gorge_method]
    pub fn detection_deny(ctx: &mut NativeContext, this: usize, chart_time: f32, direction: i32) -> usize {
        let ig = ctx.get_object_object_field(this, 0);
        let hs = ctx.get_object_object_field(this, 2);
        ctx.set_float_param(0, chart_time as f64);
        ctx.set_object_param(0, hs);
        ctx.invoke_native_method_on("GorgeFramework.InputGraph", ig, 4);
        let eid = ctx.get_object_return();
        if eid == 0 { return 0; }
        ctx.set_float_param(0, chart_time as f64);
        ctx.set_object_param(0, eid);
        ctx.set_int_param(0, direction as i64);
        ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 15);
        ctx.get_object_return()
    }

/// 方法 6：get_detection_conditions
    ///
    /// 对齐 C# `GetDetectionConditions`：只有 Backward 方向不检测任何输入
    /// （返回 0 表示无 filter），Forward/Infinitesimal 均按正向检测返回 filter_id。
    /// 调用方经该返回值定位 filter 对象并展开条件（见 impls.rs 的
    /// `fill_pending_detection_conditions`）。
    #[gorge_method]
    pub fn get_detection_conditions(ctx: &mut NativeContext, this: usize, direction: i32) -> i32 {
        if direction == 1 { return 0; } // Backward：不检测任何输入
        let ig = ctx.get_object_object_field(this, 0);
        let ts = ctx.get_object_object_field(this, 1);
        if ctx.get_object_bool_field(ts, 0) && ctx.get_object_bool_field(ig, 0) { return 0; }
        1 // 有检测条件
    }

    /// 方法 7：convert_automaton_commands（静态）
    #[gorge_static]
    pub fn convert_automaton_commands(ctx: &mut NativeContext, commands_array_id: usize, _direction: i32) -> i32 {
        if commands_array_id == 0 { 0 } else { ctx.object_array_items(commands_array_id).len() as i32 }
    }

    /// 方法 8：get_signal_value
    #[gorge_method]
    pub fn get_signal_value(ctx: &mut NativeContext, this: usize, channel: String, signal_id: i32) -> usize {
        ctx.get_payload::<SignalTsigaPayload>(this)
            .and_then(|p| p.signal_state.get(&channel).and_then(|m| m.get(&signal_id)))
            .map(|r| r.value).unwrap_or(0)
    }

    /// 方法 9：get_signal_last_value
    #[gorge_method]
    pub fn get_signal_last_value(ctx: &mut NativeContext, this: usize, channel: String, signal_id: i32) -> usize {
        ctx.get_payload::<SignalTsigaPayload>(this)
            .and_then(|p| p.signal_state.get(&channel).and_then(|m| m.get(&signal_id)))
            .map(|r| r.last_value).unwrap_or(0)
    }

    /// 方法 10：update_signal_record
    #[gorge_method]
    pub fn update_signal_record(ctx: &mut NativeContext, this: usize, channel: String, signal_id: i32, value: usize) {
        if let Some(p) = ctx.get_payload_mut::<SignalTsigaPayload>(this) {
            let m = p.signal_state.entry(channel).or_default();
            if let Some(r) = m.get_mut(&signal_id) {
                r.last_value = r.value; r.value = value;
            } else {
                m.insert(signal_id, SignalRecord { value, last_value: 0 });
            }
        }
    }

    // ==================== E-1 新增方法（11-15） ====================

/// 方法 11：do_respond
    ///
    /// 返回 Note.DoRespond 产生的命令 ObjectArray 对象 ID（0=空），
    /// 语义对齐 C# `DoRespond`（直接用于 ConvertAutomatonCommands）。
    #[gorge_method]
    pub fn do_respond(ctx: &mut NativeContext, this: usize, respond_chart_time: f32, _direction: i32) -> usize {
        let ts = ctx.get_object_object_field(this, 1);
        let note_id = ctx.get_payload::<SignalTsigaPayload>(this).map(|p| p.note_id).unwrap_or(0);
        if note_id == 0 { return 0; }
        let rm = ctx.get_object_string_field(ts, 0);
        ctx.set_string_param(0, rm);
        ctx.set_float_param(0, respond_chart_time as f64);
        ctx.invoke_native_method_on("GorgeFramework.Note", note_id, 0);
        ctx.get_object_return()
    }

    /// 方法 12：do_deny
    ///
    /// 返回 Note.DoRespond（Miss）产生的命令 ObjectArray 对象 ID（0=空）。
    #[gorge_method]
    pub fn do_deny(ctx: &mut NativeContext, this: usize, deny_chart_time: f32, _direction: i32) -> usize {
        let note_id = ctx.get_payload::<SignalTsigaPayload>(this).map(|p| p.note_id).unwrap_or(0);
        if note_id == 0 { return 0; }
        ctx.set_string_param(0, "Miss".to_string());
        ctx.set_float_param(0, deny_chart_time as f64);
        ctx.invoke_native_method_on("GorgeFramework.Note", note_id, 0);
        ctx.get_object_return()
    }

/// 方法 13：pop_until
    ///
    /// 返回合并所有弹栈响应指令的命令 ObjectArray 对象 ID（0=空），
    /// 语义对齐 C# `PopUntil`。
    #[gorge_method]
    pub fn pop_until(ctx: &mut NativeContext, this: usize, target_chart_time: f32, direction: i32) -> usize {
        let ts = ctx.get_object_object_field(this, 1);
        let ig = ctx.get_object_object_field(this, 0);
        let hs = ctx.get_object_object_field(this, 2);
        let mut merged: Vec<usize> = Vec::new();
        loop {
            ctx.invoke_native_method_on("GorgeFramework.TimeStack", ts, 0);
            let pt = (ctx.get_float_return() as f64) as f32;
            if pt > target_chart_time { break; }
            ctx.set_float_param(0, target_chart_time as f64);
            ctx.set_object_param(0, hs);
            ctx.invoke_native_method_on("GorgeFramework.TimeStack", ts, 2);
            if ctx.get_object_return() == 0 { break; }
            if ctx.get_object_bool_field(ig, 1) && !ctx.get_object_string_field(ts, 0).is_empty() {
                ctx.set_float_param(0, target_chart_time as f64);
                ctx.set_int_param(0, direction as i64);
                ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 11);
                let sub = ctx.get_object_return();
                if sub != 0 { merged.extend(ctx.object_array_items(sub)); }
            }
            if ctx.get_object_bool_field(ts, 0) && ctx.get_object_bool_field(ig, 0) { break; }
        }
        build_command_array(ctx, &merged)
    }

/// 方法 14：timeout_until
    ///
    /// 返回合并所有超时推进边响应指令的命令 ObjectArray 对象 ID（0=空），
    /// 语义对齐 C# `TimeoutUntil`。
    #[gorge_method]
    pub fn timeout_until(ctx: &mut NativeContext, this: usize, target_chart_time: f32, direction: i32) -> usize {
        let ig = ctx.get_object_object_field(this, 0);
        let hs = ctx.get_object_object_field(this, 2);
        let mut merged: Vec<usize> = Vec::new();
        loop {
            ctx.invoke_native_method_on("GorgeFramework.InputGraph", ig, 1);
            let st = (ctx.get_float_return() as f64) as f32;
            if st > target_chart_time { break; }
            ctx.set_float_param(0, target_chart_time as f64);
            ctx.set_object_param(0, hs);
            ctx.invoke_native_method_on("GorgeFramework.InputGraph", ig, 2);
            let eid = ctx.get_object_return();
            if eid == 0 { break; }
            ctx.set_float_param(0, target_chart_time as f64);
            ctx.set_object_param(0, eid);
            ctx.set_int_param(0, direction as i64);
            ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 15);
            let sub = ctx.get_object_return();
            if sub != 0 { merged.extend(ctx.object_array_items(sub)); }
        }
        build_command_array(ctx, &merged)
    }

/// 方法 15：do_edge_respond
    ///
    /// 返回合并边响应（do_deny/do_respond）与递归推进指令的命令 ObjectArray
    /// 对象 ID（0=空），语义对齐 C# `DoEdgeRespond`。
    #[gorge_method]
    pub fn do_edge_respond(ctx: &mut NativeContext, this: usize, target_chart_time: f32, edge_id: usize, direction: i32) -> usize {
        if edge_id == 0 { return 0; }
        let ts = ctx.get_object_object_field(this, 1);
        let mut merged: Vec<usize> = Vec::new();
        // 边响应
        if ctx.get_object_bool_field(edge_id, 2) {
            if ctx.get_object_bool_field(edge_id, 0) {
                ctx.set_float_param(0, target_chart_time as f64);
                ctx.set_int_param(0, direction as i64);
                ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 12);
                let sub = ctx.get_object_return();
                if sub != 0 { merged.extend(ctx.object_array_items(sub)); }
            } else if !ctx.get_object_string_field(ts, 0).is_empty() {
                ctx.set_float_param(0, target_chart_time as f64);
                ctx.set_int_param(0, direction as i64);
                ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 11);
                let sub = ctx.get_object_return();
                if sub != 0 { merged.extend(ctx.object_array_items(sub)); }
            }
        }
        // 递归推进
        ctx.set_float_param(0, target_chart_time as f64);
        ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 1);
        let sub = ctx.get_object_return();
        if sub != 0 { merged.extend(ctx.object_array_items(sub)); }
        build_command_array(ctx, &merged)
    }

    /// 设置在关联 Note 对象（方法 16）
    #[gorge_method]
    pub fn set_note(ctx: &mut NativeContext, this: usize, note_id: usize) {
        if let Some(p) = ctx.get_payload_mut::<SignalTsigaPayload>(this) {
            p.note_id = note_id;
        }
    }
}

// ==================== 辅助函数 ====================

/// 将收集到的命令对象 ID 列表封装为一个新的 ObjectArray，空列表返回 0（空数组）。
///
/// 对齐 C# 动作列表在 `List<IGameplayAction>` 中拼接的语义：
/// 多个子方法（PopUntil/TimeoutUntil/DoEdgeRespond 等）返回的指令数组
/// 合并为一个最终的 ObjectArray，供 ConvertAutomatonCommands 统一转换。
fn build_command_array(ctx: &mut NativeContext, items: &[usize]) -> usize {
    if items.is_empty() { return 0; }
    let new_id = ObjectArrayClass.do_construct_native(ctx, None, 0);
    if let Some(payload) = ctx.vm.native_payloads.get_mut(&new_id) {
        if let Some(arr) = payload.downcast_mut::<ObjectArray>() {
            arr.items = items.to_vec();
        }
    }
    new_id
}

pub fn convert_automaton_commands(
    ctx: &mut NativeContext,
    commands_array_id: usize,
    direction: crate::runtime::simulation_types::SimulateDirection,
) -> Vec<Box<dyn crate::simulators::IGameplayAction>> {
    use crate::simulators::impls::{AppendSignal, DeriveElement as DeriveAction, DestroyElement as DestroyAction};
    if commands_array_id == 0 { return Vec::new(); }
    let items = ctx.object_array_items(commands_array_id);
    let mut actions: Vec<Box<dyn crate::simulators::IGameplayAction>> = Vec::new();
    for cmd_id in items {
        if cmd_id == 0 { continue; }
        let cn = ctx.vm.objects.get(&cmd_id).map(|o| o.class_name.clone()).unwrap_or_default();
        if cn.contains("DeriveElementCommand") {
            actions.push(Box::new(DeriveAction { element_id: ctx.get_object_int_field(cmd_id, 0) as usize, direction }));
        } else if cn.contains("AppendSignalCommand") {
            actions.push(Box::new(AppendSignal::new(String::new(), ctx.get_object_int_field(cmd_id, 0) as i32, ctx.get_object_int_field(cmd_id, 1) as usize)));
        } else if cn.contains("DestroyElementCommand") {
            actions.push(Box::new(DestroyAction { element_id: ctx.get_object_int_field(cmd_id, 0) as usize }));
        }
    }
    actions
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::object::{GorgeObject, RuntimeObject};
    use gorge_core::objective::types::TypeCount;
    use gorge_core::system::native::array::ObjectArray;
    use gorge_core::virtual_machine::vm::VirtualMachine;
    use crate::system::native::history::HistoryStack;
    use crate::system::native::time_stack::TimeStack;
    use crate::system::native::input_graph::InputGraph;
    use crate::system::native::input_graph_edge::InputGraphEdge;
    use crate::system::native::input_graph_state::InputGraphState;
    use crate::system::native::signal_filter_native::SignalFilter;
    use crate::system::native::note::Note;
    use gorge_core::system::native::array::ObjectArrayClass;

    /// 假 Note 类：方法 0（do_respond）返回含一条 DeriveElementCommand 的命令数组，
    /// 用于验证命令 ObjectArray 在前向推进响应链路中的传播。
    #[derive(Debug)]
    struct FakeRespondNote;
    impl NativeClass for FakeRespondNote {
        fn full_name(&self) -> &str { "GorgeFramework.Note" }
        fn field_type_count(&self) -> &TypeCount {
            static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
            TC.get_or_init(|| TypeCount { object_count: 1, ..TypeCount::zero() })
        }
        fn invoke_native_method(&self, ctx: &mut NativeContext, _obj_id: usize, method_id: usize) {
            if method_id != 0 { return; }
            // 构造一条派生元素命令并装入命令数组
            let cmd = crate::system::native::derive_element_command::DeriveElementCommand::new(7);
            let cmd_id = cmd.do_construct_native(ctx, None, 0);
            let arr_id = ObjectArrayClass.do_construct_native(ctx, None, 0);
            ctx.object_array_add(arr_id, cmd_id);
            ctx.set_object_return(arr_id);
        }
        fn invoke_native_static(&self, _ctx: &mut NativeContext, _method_id: usize) {}
        fn do_construct_native(&self, ctx: &mut NativeContext, target: Option<usize>, _ctor_id: usize) -> usize {
            let id = match target {
                Some(id) => id,
                None => { let id = ctx.vm.next_object_id; ctx.vm.next_object_id += 1; id }
            };
            ctx.vm.objects.insert(id, RuntimeObject::new_simple(self.full_name().to_string(), self.field_type_count()));
            id
        }
    }

    struct Fixture {
        vm: VirtualMachine,
    }

    impl Fixture {
        fn new() -> Self { let mut vm = VirtualMachine::new(); vm.next_object_id = 100; Self { vm } }
        fn ctx(&mut self) -> NativeContext<'_> { NativeContext::new(&mut self.vm) }

fn make_tsiga(&mut self, ig: usize, ts: usize, hs: usize, note: usize) -> usize {
            let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
            self.vm.native_class_table.insert("GorgeFramework.SignalTsiga".into(),
                std::sync::Arc::new(st.clone()));
            self.vm.param_pool.set_object_param(0, ig);
            self.vm.param_pool.set_object_param(1, ts);
            self.vm.param_pool.set_object_param(2, hs);
            let id = { let mut c = self.ctx(); st.do_construct_native(&mut c, None, 0) };
            if note != 0 {
                self.vm.param_pool.set_object_param(0, note);
                let mut c = self.ctx(); st.invoke_native_method(&mut c, id, 16);
            }
            id
        }

        /// 构造一个含接受边的 InputGraph（当前状态吃一条接受边即触发边响应）。
        fn make_input_graph_with_accept_edge(&mut self) -> usize {
            let ig = InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() };
            self.vm.native_class_table.insert("GorgeFramework.InputGraph".into(),
                std::sync::Arc::new(InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() }));

            // 接受边：edge_respond=true、deny=false、accept=true、stack_respond=false
            let acc_edge = {
                let e = InputGraphEdge { deny: false, jump: 0, stack_respond: false, edge_respond: false, accept: false, export_state: String::new() };
                self.vm.param_pool.set_bool_param(0, false); // deny
                self.vm.param_pool.set_int_param(0, 0);      // jump
                self.vm.param_pool.set_bool_param(1, false); // stack_respond
                self.vm.param_pool.set_bool_param(2, true);  // edge_respond
                self.vm.param_pool.set_bool_param(3, true);  // accept
                self.vm.param_pool.set_string_param(0, "Active".to_string());
                { let mut c = self.ctx(); e.do_construct_native(&mut c, None, 0) }
            };
            // 拒绝边：deny=true，仅占位
            let den_edge = {
                let e = InputGraphEdge { deny: false, jump: 0, stack_respond: false, edge_respond: false, accept: false, export_state: String::new() };
                self.vm.param_pool.set_bool_param(0, true);
                self.vm.param_pool.set_int_param(0, 0);
                self.vm.param_pool.set_bool_param(1, false);
                self.vm.param_pool.set_bool_param(2, false);
                self.vm.param_pool.set_bool_param(3, false);
                self.vm.param_pool.set_string_param(0, String::new());
                { let mut c = self.ctx(); e.do_construct_native(&mut c, None, 0) }
            };
            // 过滤器（end_time 委托为 0、time_mode=0）
            let filter = {
                let sf = SignalFilter { priority: 0, condition_types: 0, end_time: 0, time_mode: 0, accept_consume: true, deny_consume: false };
                self.vm.param_pool.set_object_param(0, 0);
                self.vm.param_pool.set_object_param(1, 0);
                self.vm.param_pool.set_object_param(2, 0);
                self.vm.param_pool.set_int_param(0, 0);
                self.vm.param_pool.set_bool_param(0, true);
                self.vm.param_pool.set_bool_param(1, false);
                { let mut c = self.ctx(); sf.do_construct_native(&mut c, None, 0) }
            };
            // 状态：filter + 接受边/拒绝边
            let state = {
                let s = InputGraphState { filter: 0, accepted_edge: 0, denied_edge: 0 };
                self.vm.param_pool.set_object_param(0, filter);
                self.vm.param_pool.set_object_param(1, acc_edge);
                self.vm.param_pool.set_object_param(2, den_edge);
                { let mut c = self.ctx(); s.do_construct_native(&mut c, None, 0) }
            };
            let states_arr = {
                let cls = ObjectArrayClass;
                let arr_id = { let mut c = self.ctx(); cls.do_construct_native(&mut c, None, 0) };
                { let mut c = self.ctx(); c.object_array_add(arr_id, state); }
                arr_id
            };
            self.vm.param_pool.set_object_param(0, states_arr);
            self.vm.param_pool.set_bool_param(0, true);
            self.vm.param_pool.set_bool_param(1, false);
            self.vm.param_pool.set_int_param(0, 0);
            self.vm.param_pool.set_string_param(0, "Waiting".to_string());
            { let mut c = self.ctx(); ig.do_construct_native(&mut c, None, 0) }
        }

        /// 构造一个 TimeStack（accept=true、respond_mode 非空，使 forward_state_change 立即停止、
        /// 且 do_edge_respond 会触发 do_respond）。
        fn make_transitioning_time_stack(&mut self) -> usize {
            let ts = TimeStack { accept: true, respond_mode: String::new() };
            self.vm.native_class_table.insert("GorgeFramework.TimeStack".into(),
                std::sync::Arc::new(TimeStack { accept: false, respond_mode: String::new() }));
            self.vm.param_pool.set_bool_param(0, true);
            self.vm.param_pool.set_string_param(0, "Notes".to_string());
            { let mut c = self.ctx(); ts.do_construct_native(&mut c, None, 0) }
        }
    }

    impl Clone for SignalTsiga {
        fn clone(&self) -> Self {
            Self { input_graph: self.input_graph, time_stack: self.time_stack, history_stack: self.history_stack }
        }
    }

    #[test]
    fn test_signal_tsiga_construct() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_object_param(0, 10);
        fx.vm.param_pool.set_object_param(1, 20);
        fx.vm.param_pool.set_object_param(2, 30);
        let id = { let mut c = fx.ctx(); st.do_construct_native(&mut c, None, 0) };
        assert!(id > 0);
        let obj = fx.vm.objects.get(&id).unwrap();
        assert_eq!(obj.get_object_field(0), 10);
        assert_eq!(obj.get_object_field(1), 20);
        assert_eq!(obj.get_object_field(2), 30);
    }

    #[test]
    fn test_signal_tsiga_backward_state_change_time() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        let hs = HistoryStack { _placeholder: false };
        let hs_id = { let mut c = fx.ctx(); hs.do_construct_native(&mut c, None, 0) };
        fx.vm.native_class_table.insert("GorgeFramework.HistoryStack".into(), std::sync::Arc::new(HistoryStack { _placeholder: false }));
        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_object_param(1, 0);
        fx.vm.param_pool.set_object_param(2, hs_id);
        let id = { let mut c = fx.ctx(); st.do_construct_native(&mut c, None, 0) };
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 2); }
        assert_eq!((fx.vm.param_pool.get_float_return() as f64) as f32, f32::MIN);
    }

    #[test]
    fn test_signal_tsiga_detection_deny_catch_before() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        let ig = InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() };
        use gorge_core::system::native::array::ObjectArrayClass;
        let arr_id = { let mut c = fx.ctx(); ObjectArrayClass.do_construct_native(&mut c, None, 0) };
        fx.vm.param_pool.set_object_param(0, arr_id);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_string_param(0, String::new());
        let ig_id = { let mut c = fx.ctx(); ig.do_construct_native(&mut c, None, 0) };
        fx.vm.native_class_table.insert("GorgeFramework.InputGraph".into(), std::sync::Arc::new(InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() }));
        let hs = HistoryStack { _placeholder: false };
        let hs_id = { let mut c = fx.ctx(); hs.do_construct_native(&mut c, None, 0) };
        fx.vm.native_class_table.insert("GorgeFramework.HistoryStack".into(), std::sync::Arc::new(HistoryStack { _placeholder: false }));
        fx.vm.param_pool.set_object_param(0, ig_id);
        fx.vm.param_pool.set_object_param(1, 0);
        fx.vm.param_pool.set_object_param(2, hs_id);
        let id = { let mut c = fx.ctx(); st.do_construct_native(&mut c, None, 0) };
        let mut ctx = fx.ctx();
        ctx.set_float_param(0, 1.0);
        ctx.set_int_param(0, 0);
st.invoke_native_method(&mut ctx, id, 5);
        assert_eq!(fx.vm.param_pool.get_object_return(), 0);
    }

    // ==================== E-1 测试 ====================

#[test]
    fn test_do_respond_no_note() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        let id = fx.make_tsiga(0, 0, 0, 0);
        fx.vm.param_pool.set_float_param(0, 2.0);
        fx.vm.param_pool.set_int_param(0, 0_i64);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 11); }
        assert_eq!(fx.vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_do_deny_no_note() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        let id = fx.make_tsiga(0, 0, 0, 0);
        fx.vm.param_pool.set_float_param(0, 3.0);
        fx.vm.param_pool.set_int_param(0, 0_i64);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 12); }
        assert_eq!(fx.vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_pop_until_empty() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        // 创建空 TimeStack（默认 accept=false）
        let ts_obj = TimeStack { accept: false, respond_mode: String::new() };
        fx.vm.native_class_table.insert("GorgeFramework.TimeStack".into(), std::sync::Arc::new(TimeStack { accept: false, respond_mode: String::new() }));
        fx.vm.param_pool.set_bool_param(0, false);
        fx.vm.param_pool.set_string_param(0, String::new());
        let ts_id = { let mut c = fx.ctx(); ts_obj.do_construct_native(&mut c, None, 0) };
        // 创建空 InputGraph
        let ig = InputGraph { states: 0, input_pointer: 0, accept: false, stack_respond: false, export_state: String::new() };
        use gorge_core::system::native::array::ObjectArrayClass;
        let arr = { let mut c = fx.ctx(); ObjectArrayClass.do_construct_native(&mut c, None, 0) };
        fx.vm.native_class_table.insert("GorgeFramework.InputGraph".into(), std::sync::Arc::new(InputGraph { states: 0, input_pointer: 0, accept: false, stack_respond: false, export_state: String::new() }));
        fx.vm.param_pool.set_object_param(0, arr);
        fx.vm.param_pool.set_bool_param(0, false); fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_int_param(0, 0); fx.vm.param_pool.set_string_param(0, String::new());
        let ig_id = { let mut c = fx.ctx(); ig.do_construct_native(&mut c, None, 0) };
        let id = fx.make_tsiga(ig_id, ts_id, 0, 0);
        fx.vm.param_pool.set_float_param(0, 10.0);
        fx.vm.param_pool.set_int_param(0, 0_i64);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 13); }
        assert_eq!(fx.vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_timeout_until_no_state() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        let ig = InputGraph { states: 0, input_pointer: 0, accept: false, stack_respond: false, export_state: String::new() };
        use gorge_core::system::native::array::ObjectArrayClass;
        let arr = { let mut c = fx.ctx(); ObjectArrayClass.do_construct_native(&mut c, None, 0) };
        fx.vm.native_class_table.insert("GorgeFramework.InputGraph".into(), std::sync::Arc::new(InputGraph { states: 0, input_pointer: 0, accept: false, stack_respond: false, export_state: String::new() }));
        fx.vm.param_pool.set_object_param(0, arr);
        fx.vm.param_pool.set_bool_param(0, false); fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_int_param(0, 0); fx.vm.param_pool.set_string_param(0, String::new());
        let ig_id = { let mut c = fx.ctx(); ig.do_construct_native(&mut c, None, 0) };
        let id = fx.make_tsiga(ig_id, 0, 0, 0);
        fx.vm.param_pool.set_float_param(0, 100.0);
        fx.vm.param_pool.set_int_param(0, 0_i64);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 14); }
        assert_eq!(fx.vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_do_edge_respond_null_edge() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
let id = fx.make_tsiga(0, 0, 0, 0);
        fx.vm.param_pool.set_float_param(0, 1.0);
        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_int_param(0, 0_i64);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 15); }
        assert_eq!(fx.vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_signal_record_roundtrip() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        let id = fx.make_tsiga(0, 0, 0, 0);
        fx.vm.param_pool.set_string_param(0, "Touch".to_string());
        fx.vm.param_pool.set_int_param(0, 1);
        fx.vm.param_pool.set_object_param(0, 42);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 10); }
        fx.vm.param_pool.set_string_param(0, "Touch".to_string());
        fx.vm.param_pool.set_int_param(0, 1);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 8); }
        assert_eq!(fx.vm.param_pool.get_object_return(), 42);
    }

    #[test]
    fn test_get_detection_conditions_backward() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        let id = fx.make_tsiga(0, 0, 0, 0);
        fx.vm.param_pool.set_int_param(0, 1_i64);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 6); }
        assert_eq!(fx.vm.param_pool.get_int_return(), 0);
    }

    #[test]
    fn test_forward_state_change_stopped() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        let ts = TimeStack { accept: true, respond_mode: String::new() };
        fx.vm.native_class_table.insert("GorgeFramework.TimeStack".into(), std::sync::Arc::new(TimeStack { accept: false, respond_mode: String::new() }));
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_string_param(0, String::new());
        let ts_id = { let mut c = fx.ctx(); ts.do_construct_native(&mut c, None, 0) };
        let ig = InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() };
        use gorge_core::system::native::array::ObjectArrayClass;
        let arr_id = { let mut c = fx.ctx(); ObjectArrayClass.do_construct_native(&mut c, None, 0) };
        fx.vm.native_class_table.insert("GorgeFramework.InputGraph".into(), std::sync::Arc::new(InputGraph { states: 0, input_pointer: 0, accept: false, stack_respond: false, export_state: String::new() }));
        fx.vm.param_pool.set_object_param(0, arr_id);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_string_param(0, String::new());
        let ig_id = { let mut c = fx.ctx(); ig.do_construct_native(&mut c, None, 0) };
let id = fx.make_tsiga(ig_id, ts_id, 0, 0);
        fx.vm.param_pool.set_float_param(0, 5.0);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 1); }
        assert_eq!(fx.vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_build_command_array_merge() {
        let mut fx = Fixture::new();
        // 空列表 → 返回 0（空数组）
        let empty = { let mut c = fx.ctx(); build_command_array(&mut c, &[]) };
        assert_eq!(empty, 0, "空命令列表应返回 0");

        // 非空 → 新建 ObjectArray 并装入全部命令对象 ID
        let mut c = fx.ctx();
        let arr_id = build_command_array(&mut c, &[1, 2, 3]);
        assert_ne!(arr_id, 0, "非空命令列表应返回新数组 ID");
        let items = c.vm.native_payloads.get(&arr_id)
            .and_then(|p| p.downcast_ref::<ObjectArray>())
            .map(|a| a.items.clone())
            .unwrap_or_default();
        assert_eq!(items, vec![1, 2, 3], "合并数组应包含全部命令对象 ID");
    }

    #[test]
    fn test_do_respond_propagates_command_array() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        // 创建真实 Note 对象作为载体，但用假类覆盖其 do_respond 返回命令数组
        let note_id = {
            let note_obj = Note { automaton: 0 };
            fx.vm.param_pool.set_object_param(0, 0);
            let mut c = fx.ctx();
            note_obj.do_construct_native(&mut c, None, 0)
        };
        fx.vm.native_class_table.insert("GorgeFramework.Note".into(), std::sync::Arc::new(FakeRespondNote));
        let id = fx.make_tsiga(0, 0, 0, note_id);
        fx.vm.param_pool.set_float_param(0, 2.0);
        fx.vm.param_pool.set_int_param(0, 0_i64);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 11); }
        let arr_id = fx.vm.param_pool.get_object_return();
        assert_ne!(arr_id, 0, "do_respond 应返回非空命令数组 ID（而非计数）");
        let len = fx.vm.native_payloads.get(&arr_id)
            .and_then(|p| p.downcast_ref::<ObjectArray>())
            .map(|a| a.items.len())
            .unwrap_or(0);
        assert_eq!(len, 1, "命令数组应包含 1 条命令");
    }

    #[test]
    fn test_detection_accept_propagates_command_array() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();

        // 创建真实 Note 载体，用 FakeRespondNote 类覆盖其 do_respond 返回命令数组
        let note_id = {
            let note_obj = Note { automaton: 0 };
            fx.vm.param_pool.set_object_param(0, 0);
            let mut c = fx.ctx();
            note_obj.do_construct_native(&mut c, None, 0)
        };
        fx.vm.native_class_table.insert("GorgeFramework.Note".into(), std::sync::Arc::new(FakeRespondNote));

        let ig_id = fx.make_input_graph_with_accept_edge();
        let ts_id = fx.make_transitioning_time_stack();
        let hs = HistoryStack { _placeholder: false };
        let hs_id = { let mut c = fx.ctx(); hs.do_construct_native(&mut c, None, 0) };
        fx.vm.native_class_table.insert("GorgeFramework.HistoryStack".into(),
            std::sync::Arc::new(HistoryStack { _placeholder: false }));

        let id = fx.make_tsiga(ig_id, ts_id, hs_id, note_id);
        fx.vm.param_pool.set_float_param(0, 1.0);
        fx.vm.param_pool.set_int_param(0, 0_i64);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 4); }
        let arr_id = fx.vm.param_pool.get_object_return();
        assert_ne!(arr_id, 0, "detection_accept 应返回非空命令数组 ID（而非长度计数）");
        let len = fx.vm.native_payloads.get(&arr_id)
            .and_then(|p| p.downcast_ref::<ObjectArray>())
            .map(|a| a.items.len())
            .unwrap_or(0);
        assert_eq!(len, 1, "接收分支应产生 1 条命令");
    }

    #[test]
    fn test_detection_deny_returns_object_id() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        // 构造一个空 InputGraph（无状态），go_deny_edge 返回 0 → 方法 5 返回 0（空数组）
        let ig = InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() };
        fx.vm.native_class_table.insert("GorgeFramework.InputGraph".into(),
            std::sync::Arc::new(InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() }));
        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_string_param(0, String::new());
        let ig_id = { let mut c = fx.ctx(); ig.do_construct_native(&mut c, None, 0) };
        let hs = HistoryStack { _placeholder: false };
        let hs_id = { let mut c = fx.ctx(); hs.do_construct_native(&mut c, None, 0) };
        fx.vm.native_class_table.insert("GorgeFramework.HistoryStack".into(),
            std::sync::Arc::new(HistoryStack { _placeholder: false }));
        let id = fx.make_tsiga(ig_id, 0, hs_id, 0);
        fx.vm.param_pool.set_float_param(0, 1.0);
        fx.vm.param_pool.set_int_param(0, 0_i64);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 5); }
        let arr_id = fx.vm.param_pool.get_object_return();
        assert_eq!(arr_id, 0, "无状态时 detection_deny 应返回 0（空数组）");
    }
}
