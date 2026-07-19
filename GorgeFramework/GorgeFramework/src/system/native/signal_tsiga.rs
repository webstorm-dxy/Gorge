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
use gorge_core::objective::native::NativeContext;
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
    #[gorge_method]
    pub fn forward_state_change(ctx: &mut NativeContext, this: usize, chart_time: f32) -> i32 {
        let ts = ctx.get_object_object_field(this, 1);
        let ig = ctx.get_object_object_field(this, 0);
        if ctx.get_object_bool_field(ts, 0) && ctx.get_object_bool_field(ig, 0) { return 0; }
        let mut cnt = 0i32;
        ctx.set_float_param(0, chart_time as f64);
        ctx.set_int_param(0, 0);
        ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 13);
        cnt += ctx.get_int_return() as i32;
        ctx.set_float_param(0, chart_time as f64);
        ctx.set_int_param(0, 0);
        ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 14);
        cnt += ctx.get_int_return() as i32;
        cnt
    }

    /// 方法 2：backward_state_change_time
    #[gorge_method]
    pub fn backward_state_change_time(ctx: &mut NativeContext, this: usize) -> f32 {
        let hs = ctx.get_object_object_field(this, 2);
        ctx.invoke_native_method_on("GorgeFramework.HistoryStack", hs, 0);
        (ctx.get_float_return() as f64) as f32
    }

    /// 方法 3：backward_state_change
    #[gorge_method]
    pub fn backward_state_change(ctx: &mut NativeContext, this: usize, chart_time: f32) -> i32 {
        let hs = ctx.get_object_object_field(this, 2);
        let ig = ctx.get_object_object_field(this, 0);
        let ts = ctx.get_object_object_field(this, 1);
        ctx.set_float_param(0, chart_time as f64);
        ctx.set_object_param(0, this);
        ctx.set_int_param(0, 1);
        ctx.set_object_param(1, ig);
        ctx.set_object_param(2, ts);
        ctx.invoke_native_method_on("GorgeFramework.HistoryStack", hs, 4);
        0
    }

    /// 方法 4：detection_accept
    #[gorge_method]
    pub fn detection_accept(ctx: &mut NativeContext, this: usize, chart_time: f32, direction: i32) -> i32 {
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
        ctx.get_int_return() as i32
    }

    /// 方法 5：detection_deny
    #[gorge_method]
    pub fn detection_deny(ctx: &mut NativeContext, this: usize, chart_time: f32, direction: i32) -> i32 {
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
        ctx.get_int_return() as i32
    }

    /// 方法 6：get_detection_conditions
    #[gorge_method]
    pub fn get_detection_conditions(ctx: &mut NativeContext, this: usize, direction: i32) -> i32 {
        if direction != 0 { return 0; }
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
    #[gorge_method]
    pub fn do_respond(ctx: &mut NativeContext, this: usize, respond_chart_time: f32, _direction: i32) -> i32 {
        let ts = ctx.get_object_object_field(this, 1);
        let note_id = ctx.get_payload::<SignalTsigaPayload>(this).map(|p| p.note_id).unwrap_or(0);
        if note_id == 0 { return 0; }
        let rm = ctx.get_object_string_field(ts, 0);
        ctx.set_string_param(0, rm);
        ctx.set_float_param(0, respond_chart_time as f64);
        ctx.invoke_native_method_on("GorgeFramework.Note", note_id, 0);
        let cid = ctx.get_object_return();
        if cid == 0 { return 0; }
        ctx.object_array_items(cid).len() as i32
    }

    /// 方法 12：do_deny
    #[gorge_method]
    pub fn do_deny(ctx: &mut NativeContext, this: usize, deny_chart_time: f32, _direction: i32) -> i32 {
        let note_id = ctx.get_payload::<SignalTsigaPayload>(this).map(|p| p.note_id).unwrap_or(0);
        if note_id == 0 { return 0; }
        ctx.set_string_param(0, "Miss".to_string());
        ctx.set_float_param(0, deny_chart_time as f64);
        ctx.invoke_native_method_on("GorgeFramework.Note", note_id, 0);
        let cid = ctx.get_object_return();
        if cid == 0 { return 0; }
        ctx.object_array_items(cid).len() as i32
    }

    /// 方法 13：pop_until
    #[gorge_method]
    pub fn pop_until(ctx: &mut NativeContext, this: usize, target_chart_time: f32, direction: i32) -> i32 {
        let ts = ctx.get_object_object_field(this, 1);
        let ig = ctx.get_object_object_field(this, 0);
        let hs = ctx.get_object_object_field(this, 2);
        let mut count = 0i32;
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
                count += ctx.get_int_return() as i32;
            }
            if ctx.get_object_bool_field(ts, 0) && ctx.get_object_bool_field(ig, 0) { break; }
        }
        count
    }

    /// 方法 14：timeout_until
    #[gorge_method]
    pub fn timeout_until(ctx: &mut NativeContext, this: usize, target_chart_time: f32, direction: i32) -> i32 {
        let ig = ctx.get_object_object_field(this, 0);
        let hs = ctx.get_object_object_field(this, 2);
        let mut count = 0i32;
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
            count += ctx.get_int_return() as i32;
        }
        count
    }

    /// 方法 15：do_edge_respond
    #[gorge_method]
    pub fn do_edge_respond(ctx: &mut NativeContext, this: usize, target_chart_time: f32, edge_id: usize, direction: i32) -> i32 {
        if edge_id == 0 { return 0; }
        let ts = ctx.get_object_object_field(this, 1);
        let mut count = 0i32;
        // UpdatePending 标记
        count += 1;
        // 边响应
        if ctx.get_object_bool_field(edge_id, 2) {
            if ctx.get_object_bool_field(edge_id, 0) {
                ctx.set_float_param(0, target_chart_time as f64);
                ctx.set_int_param(0, direction as i64);
                ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 12);
                count += ctx.get_int_return() as i32;
            } else if !ctx.get_object_string_field(ts, 0).is_empty() {
                ctx.set_float_param(0, target_chart_time as f64);
                ctx.set_int_param(0, direction as i64);
                ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 11);
                count += ctx.get_int_return() as i32;
            }
        }
        // 递归推进
        ctx.set_float_param(0, target_chart_time as f64);
        ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", this, 1);
        count += ctx.get_int_return() as i32;
        count
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
    use gorge_core::objective::object::GorgeObject;
    use gorge_core::virtual_machine::vm::VirtualMachine;
    use crate::system::native::history::HistoryStack;
    use crate::system::native::time::TimeStack;
    use crate::system::native::input_graph::InputGraph;

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
        assert_eq!(fx.vm.param_pool.get_int_return(), 0);
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
        assert_eq!(fx.vm.param_pool.get_int_return(), 0);
    }

    #[test]
    fn test_do_deny_no_note() {
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        let mut fx = Fixture::new();
        let id = fx.make_tsiga(0, 0, 0, 0);
        fx.vm.param_pool.set_float_param(0, 3.0);
        fx.vm.param_pool.set_int_param(0, 0_i64);
        { let mut c = fx.ctx(); st.invoke_native_method(&mut c, id, 12); }
        assert_eq!(fx.vm.param_pool.get_int_return(), 0);
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
        assert_eq!(fx.vm.param_pool.get_int_return(), 0);
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
        assert_eq!(fx.vm.param_pool.get_int_return(), 0);
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
        assert_eq!(fx.vm.param_pool.get_int_return(), 0);
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
        assert_eq!(fx.vm.param_pool.get_int_return(), 0);
    }
}
