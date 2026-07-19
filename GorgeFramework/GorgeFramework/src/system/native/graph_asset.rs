//! `GorgeFramework` — 图形资产基类（C# `GraphAsset`，继承自 `Asset`）。
//!
//! 移植自 C# 参考实现 `System/Native/GraphAsset`。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 图形资产基类（C# `GraphAsset`，继承自 `Asset`）
///
/// 定义虚方法 `GetAsset`，子类覆盖返回具体的 Graph 对象。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct GraphAsset {
    /// 资产名称（继承自 Asset）
    #[gorge_field]
    pub name: String,
}

#[gorge_native_impl]
impl GraphAsset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, GraphAsset::FIELD_INDEX_name, String::new());
    }

    /// 0 号方法：加载资产（继承自 Asset）
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        false
    }

    /// 1 号方法：获取图形资产
    ///
    /// 基类默认抛异常（对齐 C# abstract 语义），返回 0 表示无。
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn get_asset(ctx: &mut NativeContext, this: usize) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    fn make_vm() -> VirtualMachine {
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        vm
    }

    fn register(vm: &mut VirtualMachine, cls: std::sync::Arc<dyn NativeClass>) {
        let name = cls.full_name().to_string();
        vm.register_native_class(&name, cls);
    }

    #[test]
    fn test_graph_asset_get_asset_base() {
        let ga = GraphAsset { name: String::new() };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(GraphAsset { name: String::new() }));
        let id = { let mut ctx = NativeContext::new(&mut vm); ga.do_construct_native(&mut ctx, None, 0) };
        // 基类 GetAsset 返回 0
        { let mut ctx = NativeContext::new(&mut vm); ga.invoke_native_method(&mut ctx, id, 1); }
        assert_eq!(vm.param_pool.get_object_return(), 0);
    }
}
