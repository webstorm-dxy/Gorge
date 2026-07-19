//! `GorgeFramework` — 资源基类（C# `Asset`）。
//!
//! 移植自 C# 参考实现 `System/Native/Asset`。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 资源基类（C# `Asset`）
///
/// 字段 `name` 为资产名称，方法 `LoadAsset` 为虚方法（基类返回 false 表示未实现）。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Asset {
    /// 资产名称
    #[gorge_field]
    pub name: String,
}

#[gorge_native_impl]
impl Asset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, Asset::FIELD_INDEX_name, String::new());
    }

    /// 0 号方法：加载资产
    ///
    /// 基类默认返回 false（对齐 C# abstract 语义：未实现时抛异常）。
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        false
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
    fn test_asset_construct_and_load() {
        let a = Asset { name: String::new() };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(Asset { name: String::new() }));
        let id = { let mut ctx = NativeContext::new(&mut vm); a.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        // LoadAsset 基类返回 false
        { let mut ctx = NativeContext::new(&mut vm); a.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return());
    }
}
