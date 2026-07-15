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

    // 生成注入器字段初始化方法（native 构造时应用注入器覆写，对齐 C# FieldInitialize）
    let field_initialize = build_field_initialize(&struct_ident, &fields);

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
            #field_initialize
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

/// 生成注入器字段初始化方法 `gorge_field_initialize(ctx, this)`。
///
/// 对齐 C# 自动生成的 `FieldInitialize(injector)`：对每个既是对象字段
/// (`#[gorge_field]`) 又是注入器字段 (`#[inject]`) 的同名字段，从当前注入器
/// 读取该字段值（`ctx.injector_xxx(注入器索引)`）——若注入器未显式设置该字段
/// （返回 `None`），则回退到注入器默认值 `gorge_injector_default_<name>()`（若声明了
/// `#[inject(default=..)]`）或该 Rust 类型的 `Default`。最终写入对象字段。
///
/// native 构造方法体在设置显式参数前应先调用本方法，使 `:{...}` 注入器覆写生效。
fn build_field_initialize(struct_ident: &syn::Ident, fields: &[FieldSpec]) -> TokenStream2 {
    // 对象字段与注入器字段各自按值类型分组编号，需分别计数以取得正确索引
    let mut field_counters = KindCounts::default();
    let mut inject_counters = KindCounts::default();
    let mut stmts = Vec::new();

    for f in fields {
        // 对象字段索引（仅当是对象字段时推进对应计数器）
        let field_idx = if f.is_field {
            Some(match f.kind {
                ValueKind::Int => bump(&mut field_counters.int),
                ValueKind::Float => bump(&mut field_counters.float),
                ValueKind::Bool => bump(&mut field_counters.bool),
                ValueKind::String => bump(&mut field_counters.string),
                ValueKind::Object => bump(&mut field_counters.object),
            })
        } else {
            None
        };
        // 注入器字段索引
        let inject_idx = if f.is_inject {
            Some(match f.kind {
                ValueKind::Int => bump(&mut inject_counters.int),
                ValueKind::Float => bump(&mut inject_counters.float),
                ValueKind::Bool => bump(&mut inject_counters.bool),
                ValueKind::String => bump(&mut inject_counters.string),
                ValueKind::Object => bump(&mut inject_counters.object),
            })
        } else {
            None
        };

        // 仅处理既是对象字段又是注入器字段的字段
        let (fi, ii) = match (field_idx, inject_idx) {
            (Some(fi), Some(ii)) => (fi, ii),
            _ => continue,
        };

        // 默认值表达式：有 #[inject(default=..)] 用生成的默认值方法，否则用 Default::default()
        let default_method = format_ident!("gorge_injector_default_{}", f.name);
        let default_expr: TokenStream2 = if f.default_value.is_some() {
            quote! { <#struct_ident>::#default_method() }
        } else {
            quote! { ::core::default::Default::default() }
        };

        // 按值类型选择注入器读取器与对象字段写入器；注入器值可能需类型转换以匹配字段 Rust 类型
        let stmt = match f.kind {
            ValueKind::Float => quote! {
                {
                    let __v = ctx.injector_float(#ii).map(|x| x as f32).unwrap_or_else(|| #default_expr);
                    ctx.set_object_float_field(this, #fi, __v as f64);
                }
            },
            ValueKind::Int => quote! {
                {
                    let __v = ctx.injector_int(#ii).map(|x| x as i32).unwrap_or_else(|| #default_expr);
                    ctx.set_object_int_field(this, #fi, __v as i64);
                }
            },
            ValueKind::Bool => quote! {
                {
                    let __v = ctx.injector_bool(#ii).unwrap_or_else(|| #default_expr);
                    ctx.set_object_bool_field(this, #fi, __v);
                }
            },
            ValueKind::String => quote! {
                {
                    let __v = ctx.injector_string(#ii).unwrap_or_else(|| #default_expr);
                    ctx.set_object_string_field(this, #fi, __v);
                }
            },
            ValueKind::Object => quote! {
                {
                    if let ::core::option::Option::Some(__v) = ctx.injector_object(#ii) {
                        ctx.set_object_object_field(this, #fi, __v);
                    }
                }
            },
        };
        stmts.push(stmt);
    }

    if stmts.is_empty() {
        // 无注入器对象字段：仍生成空方法，使构造入口可无条件调用
        return quote! {
            /// 应用注入器字段覆写（本类无可覆写字段，空实现）。
            #[allow(dead_code, unused_variables)]
            pub fn gorge_field_initialize(ctx: &mut ::gorge_core::native::NativeContext, this: usize) {}
        };
    }

    quote! {
        /// 应用注入器字段覆写到对象字段（native 构造时调用，对齐 C# FieldInitialize）。
        #[allow(dead_code)]
        pub fn gorge_field_initialize(ctx: &mut ::gorge_core::native::NativeContext, this: usize) {
            #(#stmts)*
        }
    }
}