//! `GorgeFramework` — 时序系统（native 类注册）。
//!
//! 移植自 C# 参考实现。TimeItem 注册为 native 数据类；
//! TimeStack（含 Vec<TimeItem>）保留为内部 Rust 类型。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::native::NativeContext;

// ==================== TimeItem（native 注册） ====================

/// 时间项（时间栈元素）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct TimeItem {
    /// 时间值
    #[gorge_field]
    pub time: f32,
    /// 是否已响应
    #[gorge_field]
    pub accept: bool,
    /// 响应模式
    #[gorge_field]
    pub respond_mode: String,
}

impl TimeItem {
    pub fn new(time: f32, respond_mode: &str) -> Self {
        Self { time, accept: false, respond_mode: respond_mode.into() }
    }
}

#[gorge_native_impl]
impl TimeItem {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, time: f32, accept: bool, respond_mode: String) {
        ctx.set_object_float_field(this, TimeItem::FIELD_INDEX_time, time as f64);
        ctx.set_object_bool_field(this, TimeItem::FIELD_INDEX_accept, accept);
        ctx.set_object_string_field(this, TimeItem::FIELD_INDEX_respond_mode, respond_mode);
    }
}

// ==================== TimeStack（内部 Rust 类型，含 Vec<TimeItem> 不可作 Gorge 字段） ====================

/// 时序栈
#[derive(Debug)]
pub struct TimeStack {
    stack: Vec<TimeItem>,
    pub accept: bool,
    pub respond_mode: String,
}

impl TimeStack {
    pub fn new() -> Self {
        Self { stack: Vec::new(), accept: true, respond_mode: String::new() }
    }
    pub fn len(&self) -> usize { self.stack.len() }
    pub fn is_empty(&self) -> bool { self.stack.is_empty() }
    pub fn peek(&self) -> Option<&TimeItem> { self.stack.last() }
    pub fn push(&mut self, _time: f32, item: TimeItem) { self.stack.push(item); self.accept = true; }
    pub fn try_pop(&mut self, target: f32) -> Option<TimeItem> {
        match self.stack.last() { Some(top) if top.time < target => self.stack.pop(), _ => None }
    }
    pub fn pop(&mut self) -> Option<TimeItem> { self.stack.pop() }
    pub fn init_push(&mut self, item: TimeItem) { self.stack.clear(); self.stack.push(item); }
    pub fn timeout_items(&self, current_time: f32) -> Vec<usize> {
        self.stack.iter().enumerate().filter(|(_, item)| item.time <= current_time && !item.accept).map(|(i, _)| i).collect()
    }
}
impl Default for TimeStack { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_stack_push_pop() {
        let mut ts = TimeStack::new();
        ts.push(1.0, TimeItem::new(1.0, ""));
        ts.push(2.0, TimeItem::new(2.0, ""));
        assert_eq!(ts.len(), 2);
        assert_eq!(ts.peek().unwrap().time, 2.0);
        let popped = ts.pop().unwrap();
        assert_eq!(popped.time, 2.0);
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn test_time_stack_try_pop() {
        let mut ts = TimeStack::new();
        ts.push(1.0, TimeItem::new(1.0, ""));
        ts.push(3.0, TimeItem::new(3.0, ""));
        assert!(ts.try_pop(2.0).is_none());
        let popped = ts.try_pop(5.0).unwrap();
        assert_eq!(popped.time, 3.0);
    }
}
