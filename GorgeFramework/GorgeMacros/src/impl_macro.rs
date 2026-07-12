//! `#[gorge_native_impl]` 属性宏实现。
//!
//! 解析被标注的 `impl` 块，按方法上的 `#[gorge_static]` / `#[gorge_method]` /
//! `#[gorge_ctor]` 分类，生成 `NativeClass` trait 实现：按方法编号分派到用户
//! 编写的业务方法，并完成参数拆箱与返回值装箱。未标注的方法原样保留。

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, FnArg, ImplItem, ItemImpl, Pat, ReturnType, Type};

use crate::common::{read_param_expr, write_return_stmt, ValueKind};

/// 方法分类
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Static,
    Method,
    Ctor,
}

/// 一个被标注的 Gorge 方法
struct GorgeMethod {
    /// 方法名
    ident: syn::Ident,
    /// 分类
    kind: Kind,
    /// 在混合方法表中的编号（Static/Method 共享），或构造方法编号（Ctor 独立）
    id: usize,
    /// 值参数（不含 ctx/this）：每项为 (值类型, Rust 类型)
    value_params: Vec<(ValueKind, Type)>,
    /// 返回值类型（None 表示无返回）
    ret: Option<(ValueKind, Type)>,
}

/// 展开 `#[gorge_native_impl]`
pub fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let self_ty = (*input.self_ty).clone();

    let mut methods = Vec::new();
    // 混合方法编号计数器（Static + Method 共享）
    let mut method_counter = 0usize;
    // 构造方法编号计数器
    let mut ctor_counter = 0usize;

    for item in &input.items {
        if let ImplItem::Fn(f) = item {
            let kind = match classify(&f.attrs) {
                Ok(Some(k)) => k,
                Ok(None) => continue, // 未标注，保留为普通方法
                Err(e) => return e.to_compile_error().into(),
            };
            let parsed = match parse_method(f, kind, &mut method_counter, &mut ctor_counter) {
                Ok(m) => m,
                Err(e) => return e.to_compile_error().into(),
            };
            methods.push(parsed);
        }
    }

    // 生成去除 Gorge 属性的原 impl（保留业务方法体）
    let clean_impl = strip_impl(input.clone());

    // 生成三个分派方法体
    let static_arms = build_arms(&self_ty, &methods, Kind::Static);
    let method_arms = build_arms(&self_ty, &methods, Kind::Method);
    let ctor_arms = build_ctor_arms(&self_ty, &methods);

    let expanded = quote! {
        #clean_impl

        impl ::gorge_core::native::NativeClass for #self_ty {
            fn full_name(&self) -> &str {
                <#self_ty>::GORGE_FULL_NAME
            }

            fn field_type_count(&self) -> &::gorge_core::types::TypeCount {
                // 返回一次性初始化的字段计数（借用要求返回引用，用 OnceLock 缓存）
                static COUNT: ::std::sync::OnceLock<::gorge_core::types::TypeCount> =
                    ::std::sync::OnceLock::new();
                COUNT.get_or_init(|| <#self_ty>::gorge_field_type_count())
            }

            fn invoke_native_static(
                &self,
                ctx: &mut ::gorge_core::native::NativeContext,
                method_id: usize,
            ) {
                match method_id {
                    #(#static_arms)*
                    _ => {}
                }
            }

            fn invoke_native_method(
                &self,
                ctx: &mut ::gorge_core::native::NativeContext,
                this: usize,
                method_id: usize,
            ) {
                match method_id {
                    #(#method_arms)*
                    _ => {}
                }
            }

            fn do_construct_native(
                &self,
                ctx: &mut ::gorge_core::native::NativeContext,
                target: ::std::option::Option<usize>,
                ctor_id: usize,
            ) -> usize {
                // 若无目标对象框架，创建一个新对象
                let this = match target {
                    ::std::option::Option::Some(id) => id,
                    ::std::option::Option::None => {
                        let obj = ::gorge_core::object::RuntimeObject::new_simple(
                            <#self_ty>::GORGE_FULL_NAME.to_string(),
                            &<#self_ty>::gorge_field_type_count(),
                        );
                        ctx.register_object(obj)
                    }
                };
                match ctor_id {
                    #(#ctor_arms)*
                    _ => {}
                }
                // 构造结果对象 ID 同时写入返回位，便于 VM 统一读取
                ctx.set_object_return(this);
                this
            }
        }
    };

    expanded.into()
}

/// 根据方法属性判定分类
fn classify(attrs: &[syn::Attribute]) -> syn::Result<Option<Kind>> {
    let mut found = None;
    for attr in attrs {
        let k = if attr.path().is_ident("gorge_static") {
            Some(Kind::Static)
        } else if attr.path().is_ident("gorge_method") {
            Some(Kind::Method)
        } else if attr.path().is_ident("gorge_ctor") {
            Some(Kind::Ctor)
        } else {
            None
        };
        if let Some(k) = k {
            if found.is_some() {
                return Err(syn::Error::new_spanned(
                    attr,
                    "一个方法只能标注一个 Gorge 属性",
                ));
            }
            found = Some(k);
        }
    }
    Ok(found)
}

/// 解析方法签名，提取值参数与返回值
fn parse_method(
    f: &syn::ImplItemFn,
    kind: Kind,
    method_counter: &mut usize,
    ctor_counter: &mut usize,
) -> syn::Result<GorgeMethod> {
    let ident = f.sig.ident.clone();

    // 分配编号
    let id = match kind {
        Kind::Static | Kind::Method => {
            let v = *method_counter;
            *method_counter += 1;
            v
        }
        Kind::Ctor => {
            let v = *ctor_counter;
            *ctor_counter += 1;
            v
        }
    };

    // 遍历参数：跳过 ctx（类型为引用 &mut NativeContext）与 this（约定参数名），
    // 其余为值参数
    let mut value_params = Vec::new();
    for arg in &f.sig.inputs {
        let pat_ty = match arg {
            FnArg::Receiver(_) => {
                return Err(syn::Error::new_spanned(
                    arg,
                    "Gorge native 方法不应有 self 接收者，请用 ctx/this 参数",
                ))
            }
            FnArg::Typed(pt) => pt,
        };
        // ctx 参数类型为引用（&mut NativeContext），直接跳过
        if matches!(&*pat_ty.ty, Type::Reference(_)) {
            continue;
        }
        let name = match &*pat_ty.pat {
            Pat::Ident(pi) => pi.ident.to_string(),
            _ => String::new(),
        };
        // 跳过 this（实例/构造方法的目标对象，约定参数名）
        if name == "this" {
            continue;
        }
        let kind_v = ValueKind::from_type(&pat_ty.ty).ok_or_else(|| {
            syn::Error::new_spanned(
                &pat_ty.ty,
                "不支持的参数类型，仅支持 i32/i64/f32/f64/bool/String/usize",
            )
        })?;
        value_params.push((kind_v, (*pat_ty.ty).clone()));
    }

    // 返回值
    let ret = match &f.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => {
            let k = ValueKind::from_type(ty).ok_or_else(|| {
                syn::Error::new_spanned(
                    ty,
                    "不支持的返回类型，仅支持 i32/i64/f32/f64/bool/String/usize",
                )
            })?;
            Some((k, (**ty).clone()))
        }
    };

    // 构造方法不应有返回值
    if kind == Kind::Ctor && ret.is_some() {
        return Err(syn::Error::new_spanned(
            &f.sig,
            "构造方法（gorge_ctor）不应有返回值",
        ));
    }

    Ok(GorgeMethod {
        ident,
        kind,
        id,
        value_params,
        ret,
    })
}

/// 为值参数按值类型分组分配参数池索引
///
/// 参数池按值类型分离存储，因此每个参数的读取索引是它在其值类型分组内的序号
/// （与编译器 codegen 的参数布置一致，见 B-2）。返回每个参数的分组内索引。
fn grouped_param_indices(value_params: &[(ValueKind, Type)]) -> Vec<usize> {
    let mut int_i = 0usize;
    let mut float_i = 0usize;
    let mut bool_i = 0usize;
    let mut string_i = 0usize;
    let mut object_i = 0usize;
    value_params
        .iter()
        .map(|(kind, _)| {
            let counter = match kind {
                ValueKind::Int => &mut int_i,
                ValueKind::Float => &mut float_i,
                ValueKind::Bool => &mut bool_i,
                ValueKind::String => &mut string_i,
                ValueKind::Object => &mut object_i,
            };
            let cur = *counter;
            *counter += 1;
            cur
        })
        .collect()
}

/// 生成静态/实例方法的分派臂
fn build_arms(self_ty: &Type, methods: &[GorgeMethod], want: Kind) -> Vec<TokenStream2> {
    let mut arms = Vec::new();
    for m in methods {
        if m.kind != want {
            continue;
        }
        let id = m.id;
        let ident = &m.ident;

        // 读取值参数
        let mut bindings = Vec::new();
        let mut call_args = Vec::new();
        // 静态方法调用参数从 ctx 起；实例方法额外传 this
        if want == Kind::Method {
            call_args.push(quote! { this });
        }
        let pool_indices = grouped_param_indices(&m.value_params);
        for (i, (kind, ty)) in m.value_params.iter().enumerate() {
            let var = quote::format_ident!("__arg{}", i);
            let read = read_param_expr(*kind, pool_indices[i], ty);
            bindings.push(quote! { let #var = #read; });
            call_args.push(quote! { #var });
        }

        // 生成调用与返回值写回
        let call = quote! { <#self_ty>::#ident(ctx, #(#call_args),*) };
        let body = if let Some((rk, rty)) = &m.ret {
            let write = write_return_stmt(*rk, quote! { __ret }, rty);
            quote! {
                #(#bindings)*
                let __ret = #call;
                #write
            }
        } else {
            quote! {
                #(#bindings)*
                let _ = #call;
            }
        };

        arms.push(quote! {
            #id => { #body }
        });
    }
    arms
}

/// 生成构造方法的分派臂
fn build_ctor_arms(self_ty: &Type, methods: &[GorgeMethod]) -> Vec<TokenStream2> {
    let mut arms = Vec::new();
    for m in methods {
        if m.kind != Kind::Ctor {
            continue;
        }
        let id = m.id;
        let ident = &m.ident;

        let mut bindings = Vec::new();
        let mut call_args = vec![quote! { this }];
        let pool_indices = grouped_param_indices(&m.value_params);
        for (i, (kind, ty)) in m.value_params.iter().enumerate() {
            let var = quote::format_ident!("__arg{}", i);
            let read = read_param_expr(*kind, pool_indices[i], ty);
            bindings.push(quote! { let #var = #read; });
            call_args.push(quote! { #var });
        }

        arms.push(quote! {
            #id => {
                #(#bindings)*
                <#self_ty>::#ident(ctx, #(#call_args),*);
            }
        });
    }
    arms
}

/// 去除 impl 块内方法的 Gorge 专用属性，保留业务方法体
fn strip_impl(mut input: ItemImpl) -> ItemImpl {
    for item in &mut input.items {
        if let ImplItem::Fn(f) = item {
            f.attrs.retain(|a| {
                !a.path().is_ident("gorge_static")
                    && !a.path().is_ident("gorge_method")
                    && !a.path().is_ident("gorge_ctor")
            });
        }
    }
    input
}
