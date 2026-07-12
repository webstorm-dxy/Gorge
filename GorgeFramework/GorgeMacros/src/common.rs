//! 宏共用辅助：值类型判定与参数/返回值胶水代码生成。

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Type;

/// Gorge 值类型（与 `gorge_core::ir::ValueType` 对应）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    Int,
    Float,
    Bool,
    String,
    /// 对象，用对象 ID（usize）传递
    Object,
}

impl ValueKind {
    /// 从 Rust 类型识别 Gorge 值类型
    ///
    /// 支持的映射：i32/i64 → Int，f32/f64 → Float，bool → Bool，
    /// String → String，usize → Object。无法识别返回 None。
    pub fn from_type(ty: &Type) -> Option<ValueKind> {
        let name = type_last_ident(ty)?;
        match name.as_str() {
            "i32" | "i64" => Some(ValueKind::Int),
            "f32" | "f64" => Some(ValueKind::Float),
            "bool" => Some(ValueKind::Bool),
            "String" => Some(ValueKind::String),
            "usize" => Some(ValueKind::Object),
            _ => None,
        }
    }
}

/// 取类型路径最后一段标识符（如 `f32`、`String`、`std::string::String`→`String`）
pub fn type_last_ident(ty: &Type) -> Option<String> {
    if let Type::Path(tp) = ty {
        tp.path.segments.last().map(|s| s.ident.to_string())
    } else {
        None
    }
}

/// 生成「从参数池读取第 index 个参数」的表达式
///
/// `rust_ty` 为业务方法期望的 Rust 类型，用于把参数池的原始类型
/// （i64/f64）转换为方法签名类型（如 i32/f32）。
pub fn read_param_expr(kind: ValueKind, index: usize, rust_ty: &Type) -> TokenStream2 {
    match kind {
        ValueKind::Int => {
            let getter = quote! { ctx.get_int_param(#index) };
            cast_from_i64(getter, rust_ty)
        }
        ValueKind::Float => {
            let getter = quote! { ctx.get_float_param(#index) };
            cast_from_f64(getter, rust_ty)
        }
        ValueKind::Bool => quote! { ctx.get_bool_param(#index) },
        ValueKind::String => quote! { ctx.get_string_param(#index) },
        ValueKind::Object => quote! { ctx.get_object_param(#index) },
    }
}

/// 生成「把返回值写回参数池」的语句
///
/// `value` 为业务方法返回的表达式，`rust_ty` 为其 Rust 类型，
/// 负责把方法返回类型（如 i32/f32）转换为参数池存储类型（i64/f64）。
pub fn write_return_stmt(kind: ValueKind, value: TokenStream2, rust_ty: &Type) -> TokenStream2 {
    match kind {
        ValueKind::Int => {
            let v = cast_to_i64(value, rust_ty);
            quote! { ctx.set_int_return(#v); }
        }
        ValueKind::Float => {
            let v = cast_to_f64(value, rust_ty);
            quote! { ctx.set_float_return(#v); }
        }
        ValueKind::Bool => quote! { ctx.set_bool_return(#value); },
        ValueKind::String => quote! { ctx.set_string_return(#value); },
        ValueKind::Object => quote! { ctx.set_object_return(#value); },
    }
}

/// 把参数池的 i64 转换为业务方法期望的整数类型
fn cast_from_i64(expr: TokenStream2, rust_ty: &Type) -> TokenStream2 {
    match type_last_ident(rust_ty).as_deref() {
        Some("i64") => expr,
        Some("i32") => quote! { (#expr) as i32 },
        _ => expr,
    }
}

/// 把业务方法返回的整数类型转换为参数池的 i64
fn cast_to_i64(expr: TokenStream2, rust_ty: &Type) -> TokenStream2 {
    match type_last_ident(rust_ty).as_deref() {
        Some("i64") => expr,
        Some("i32") => quote! { (#expr) as i64 },
        _ => expr,
    }
}

/// 把参数池的 f64 转换为业务方法期望的浮点类型
fn cast_from_f64(expr: TokenStream2, rust_ty: &Type) -> TokenStream2 {
    match type_last_ident(rust_ty).as_deref() {
        Some("f64") => expr,
        Some("f32") => quote! { (#expr) as f32 },
        _ => expr,
    }
}

/// 把业务方法返回的浮点类型转换为参数池的 f64
fn cast_to_f64(expr: TokenStream2, rust_ty: &Type) -> TokenStream2 {
    match type_last_ident(rust_ty).as_deref() {
        Some("f64") => expr,
        Some("f32") => quote! { (#expr) as f64 },
        _ => expr,
    }
}
