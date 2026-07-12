//! `#[gorge_native_class]` 属性宏实现。
//!
//! 解析被标注的结构体，提取命名空间、字段与注入器字段声明，
//! 生成去除 Gorge 专用属性后的结构体定义，以及承载类元数据与
//! 字段索引常量的固有 `impl` 块。

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr};

use crate::common::ValueKind;

/// 解析后的字段信息
struct FieldSpec {
    /// 字段名
    name: syn::Ident,
    /// 字段的 Rust 类型
    rust_ty: syn::Type,
    /// 值类型
    kind: ValueKind,
    /// 是否为对象字段（`#[gorge_field]`）
    is_field: bool,
    /// 是否为注入器字段（`#[inject]`）
    is_inject: bool,
    /// 注入器默认值表达式（`#[inject(default = <expr>)]`）
    default_value: Option<TokenStream2>,
}

/// 展开 `#[gorge_native_class]`
pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let namespace = match parse_namespace(attr) {
        Ok(ns) => ns,
        Err(e) => return e.to_compile_error().into(),
    };

    let input = parse_macro_input!(item as DeriveInput);
    let struct_ident = input.ident.clone();
    let full_name = format!("{}.{}", namespace, struct_ident);

    // 解析字段
    let fields = match parse_fields(&input) {
        Ok(f) => f,
        Err(e) => return e.to_compile_error().into(),
    };

    // 生成去除 Gorge 属性的干净结构体
    let clean_struct = match strip_gorge_attrs(input) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };

    // 统计各值类型字段数量（对象字段）
    let field_counts = count_by_kind(fields.iter().filter(|f| f.is_field));
    // 统计各值类型注入器字段数量
    let inject_counts = count_by_kind(fields.iter().filter(|f| f.is_inject));

    let tc_field = build_type_count(&field_counts);
    let tc_inject = build_type_count(&inject_counts);

    // 生成对象字段的索引常量（按值类型分组编号）
    let field_index_consts = build_field_index_consts(&struct_ident, &fields, true);
    // 生成注入器字段的索引常量
    let inject_index_consts = build_field_index_consts(&struct_ident, &fields, false);

    // 生成注入器默认值查询方法
    let injector_defaults = build_injector_defaults(&fields);

    let expanded = quote! {
        #[derive(Debug)]
        #clean_struct

        impl #struct_ident {
            /// 类全名（含命名空间）
            pub const GORGE_FULL_NAME: &'static str = #full_name;

            /// 类全名访问器
            pub fn gorge_full_name() -> &'static str {
                Self::GORGE_FULL_NAME
            }

            /// 对象字段各值类型数量
            pub fn gorge_field_type_count() -> ::gorge_core::types::TypeCount {
                #tc_field
            }

            /// 注入器字段各值类型数量
            pub fn gorge_injector_field_type_count() -> ::gorge_core::types::TypeCount {
                #tc_inject
            }

            #(#field_index_consts)*
            #(#inject_index_consts)*
            #injector_defaults
        }
    };

    expanded.into()
}

/// 解析属性参数 `namespace = "..."`
fn parse_namespace(attr: TokenStream) -> syn::Result<String> {
    let attr2: TokenStream2 = attr.into();
    if attr2.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "gorge_native_class 需要 namespace 参数，例如 #[gorge_native_class(namespace = \"GorgeFramework\")]",
        ));
    }
    // 解析形如 `namespace = "GorgeFramework"`
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("namespace") {
            let value: LitStr = meta.value()?.parse()?;
            NAMESPACE_TMP.with(|c| *c.borrow_mut() = Some(value.value()));
            Ok(())
        } else {
            Err(meta.error("未知参数，仅支持 namespace"))
        }
    });
    syn::parse::Parser::parse2(parser, attr2)?;
    NAMESPACE_TMP
        .with(|c| c.borrow_mut().take())
        .ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "缺少 namespace 参数",
            )
        })
}

thread_local! {
    /// 临时存放解析出的 namespace（syn::meta::parser 闭包无法直接返回值）
    static NAMESPACE_TMP: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

/// 解析结构体字段
fn parse_fields(input: &DeriveInput) -> syn::Result<Vec<FieldSpec>> {
    let data = match &input.data {
        Data::Struct(s) => s,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "gorge_native_class 只能用于结构体",
            ))
        }
    };
    let named = match &data.fields {
        Fields::Named(n) => n,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "gorge_native_class 只支持具名字段结构体",
            ))
        }
    };

    let mut specs = Vec::new();
    for field in &named.named {
        let name = field.ident.clone().unwrap();
        let mut is_field = false;
        let mut is_inject = false;
        let mut default_value = None;

        for attr in &field.attrs {
            if attr.path().is_ident("gorge_field") {
                is_field = true;
            } else if attr.path().is_ident("inject") {
                is_inject = true;
                // 解析可选 default = <expr>
                if !matches!(attr.meta, syn::Meta::Path(_)) {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("default") {
                            let expr: syn::Expr = meta.value()?.parse()?;
                            default_value = Some(quote! { #expr });
                            Ok(())
                        } else {
                            Err(meta.error("inject 仅支持 default 参数"))
                        }
                    })?;
                }
            }
        }

        if !is_field && !is_inject {
            continue;
        }

        let kind = ValueKind::from_type(&field.ty).ok_or_else(|| {
            syn::Error::new_spanned(
                &field.ty,
                "不支持的字段类型，仅支持 i32/i64/f32/f64/bool/String/usize",
            )
        })?;

        specs.push(FieldSpec {
            name,
            rust_ty: field.ty.clone(),
            kind,
            is_field,
            is_inject,
            default_value,
        });
    }

    Ok(specs)
}

/// 生成去除 Gorge 专用属性的结构体
fn strip_gorge_attrs(mut input: DeriveInput) -> syn::Result<DeriveInput> {
    if let Data::Struct(s) = &mut input.data {
        if let Fields::Named(n) = &mut s.fields {
            for field in &mut n.named {
                field.attrs.retain(|a| {
                    !a.path().is_ident("gorge_field") && !a.path().is_ident("inject")
                });
            }
        }
    }
    Ok(input)
}

/// 各值类型计数
#[derive(Default)]
struct KindCounts {
    int: usize,
    float: usize,
    bool: usize,
    string: usize,
    object: usize,
}

/// 统计一组字段各值类型数量
fn count_by_kind<'a>(fields: impl Iterator<Item = &'a FieldSpec>) -> KindCounts {
    let mut c = KindCounts::default();
    for f in fields {
        match f.kind {
            ValueKind::Int => c.int += 1,
            ValueKind::Float => c.float += 1,
            ValueKind::Bool => c.bool += 1,
            ValueKind::String => c.string += 1,
            ValueKind::Object => c.object += 1,
        }
    }
    c
}

/// 生成 TypeCount 构造表达式
fn build_type_count(c: &KindCounts) -> TokenStream2 {
    let (i, f, b, s, o) = (c.int, c.float, c.bool, c.string, c.object);
    quote! {
        ::gorge_core::types::TypeCount {
            int_count: #i,
            float_count: #f,
            bool_count: #b,
            string_count: #s,
            object_count: #o,
        }
    }
}

/// 生成字段索引常量（按值类型分组编号）
///
/// `for_field` 为 true 时统计对象字段，false 时统计注入器字段。
/// 生成形如 `pub const FIELD_INDEX_x: usize = 0;` 的常量。
fn build_field_index_consts(
    _struct_ident: &syn::Ident,
    fields: &[FieldSpec],
    for_field: bool,
) -> Vec<TokenStream2> {
    let mut counters = KindCounts::default();
    let mut consts = Vec::new();
    for f in fields {
        let included = if for_field { f.is_field } else { f.is_inject };
        if !included {
            continue;
        }
        let idx = match f.kind {
            ValueKind::Int => bump(&mut counters.int),
            ValueKind::Float => bump(&mut counters.float),
            ValueKind::Bool => bump(&mut counters.bool),
            ValueKind::String => bump(&mut counters.string),
            ValueKind::Object => bump(&mut counters.object),
        };
        let const_name = if for_field {
            format_ident!("FIELD_INDEX_{}", f.name)
        } else {
            format_ident!("INJECTOR_INDEX_{}", f.name)
        };
        consts.push(quote! {
            /// 字段在其值类型分组内的索引
            #[allow(dead_code)]
            #[allow(non_upper_case_globals)]
            pub const #const_name: usize = #idx;
        });
    }
    consts
}

/// 取当前值并自增
fn bump(counter: &mut usize) -> usize {
    let v = *counter;
    *counter += 1;
    v
}

/// 生成注入器默认值查询方法
///
/// 为每个带默认值的注入器字段生成一个返回默认值的方法，方法名形如
/// `gorge_injector_default_<name>`，返回类型为字段本身的 Rust 类型。
fn build_injector_defaults(fields: &[FieldSpec]) -> TokenStream2 {
    let mut methods = Vec::new();
    for f in fields {
        if !f.is_inject {
            continue;
        }
        if let Some(dv) = &f.default_value {
            let method_name = format_ident!("gorge_injector_default_{}", f.name);
            let ret_ty = f.rust_ty.clone();
            methods.push(quote! {
                /// 注入器字段默认值
                #[allow(dead_code)]
                pub fn #method_name() -> #ret_ty {
                    #dv
                }
            });
        }
    }
    quote! { #(#methods)* }
}
