use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Token};

fn get_inner_type(input: &DeriveInput) -> &syn::Type {
    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Unnamed(ref fields) => {
                if fields.unnamed.len() != 1 {
                    panic!("Derive macros only support structs with exactly one unnamed field");
                }
                &fields.unnamed[0].ty
            }
            _ => panic!("Derive macros only support tuple structs"),
        },
        _ => panic!("Derive macros only support structs"),
    }
}

fn get_arithmetic_promote(attrs: &[syn::Attribute]) -> Option<(syn::Path, syn::Type)> {
    for attr in attrs {
        if attr.path().is_ident("cupid_arithmetic_promote") {
            return attr
                .parse_args_with(|input: syn::parse::ParseStream| {
                    let target_type: syn::Path = input.parse()?;
                    let _comma: Token![,] = input.parse()?;
                    let cast_type: syn::Type = input.parse()?;
                    Ok((target_type, cast_type))
                })
                .ok();
        }
    }
    None
}

#[proc_macro_derive(InnerType, attributes(cupid_type_id))]
pub fn derive_inner_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let inner_type = get_inner_type(&input);

    // Extract the type ID from attributes
    let mut type_id: Option<syn::Expr> = None;
    for attr in &input.attrs {
        if attr.path().is_ident("cupid_type_id") {
            type_id = Some(
                attr.parse_args()
                    .expect("Expected a value for cupid_type_id"),
            );
        }
    }
    let type_id = type_id.expect("cupid_type_id attribute is required for InnerType");

    let expanded = quote! {
        impl From<#inner_type> for #name {
            fn from(value: #inner_type) -> Self {
                #name(value)
            }
        }

        impl From<#inner_type> for crate::lang::type_system::Atom<#name> {
            fn from(value: #inner_type) -> Self {
                crate::lang::type_system::Atom::new(#name(value))
            }
        }

        impl crate::lang::type_system::InnerTypeTrait for #name {
            fn get_type() -> i16 {
                #type_id
            }
        }

        impl From<#inner_type> for crate::lang::type_system::List<#name> {
            fn from(v: #inner_type) -> Self {
                crate::lang::type_system::List::new(vec![#name::from(v)])
            }
        }

        impl From<Vec<#inner_type>> for crate::lang::type_system::List<#name> {
            fn from(values: Vec<#inner_type>) -> Self {
                crate::lang::type_system::List::new(values.into_iter().map(#name::from).collect())
            }
        }

        impl From<#name> for crate::lang::type_system::Atom<#name> {
            fn from(value: #name) -> Self {
                crate::lang::type_system::Atom::new(value)
            }
        }

        impl From<#name> for crate::lang::type_system::List<#name> {
            fn from(value: #name) -> Self {
                crate::lang::type_system::List::new(vec![value])
            }
        }

        impl From<Vec<#name>> for crate::lang::type_system::List<#name> {
            fn from(values: Vec<#name>) -> Self {
                crate::lang::type_system::List::new(values)
            }
        }
    };

    TokenStream::from(expanded)
}

fn impl_add(
    name: &syn::Ident,
    promote: Option<&(syn::Path, syn::Type)>,
) -> proc_macro2::TokenStream {
    if let Some((target_type, cast_type)) = promote {
        quote! {
            impl crate::lang::invokable::operator::OperatorAdd<&Self, #target_type> for #name {
                fn add(lhs: &Self, rhs: &Self) -> Result<#target_type, crate::Error> {
                    Ok(#target_type((lhs.0 as #cast_type) + (rhs.0 as #cast_type)))
                }
            }
        }
    } else {
        quote! {
            impl crate::lang::invokable::operator::OperatorAdd<&Self, Self> for #name {
                fn add(lhs: &Self, rhs: &Self) -> Result<Self, crate::Error> {
                    Ok(#name(lhs.0 + rhs.0))
                }
            }
        }
    }
}

#[proc_macro_derive(InnerTypeAdd, attributes(cupid_arithmetic_promote))]
pub fn derive_inner_type_add(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    // We don't strictly need inner type here as we access .0, but good to validate structure
    let _ = get_inner_type(&input);
    let promote = get_arithmetic_promote(&input.attrs);
    TokenStream::from(impl_add(name, promote.as_ref()))
}

#[proc_macro_derive(InnerTypeEquals)]
pub fn derive_inner_type_equals(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let _ = get_inner_type(&input);

    let expanded = quote! {
        impl crate::lang::invokable::operator::OperatorEquals<&Self> for #name {
            fn equals(lhs: &Self, rhs: &Self) -> bool {
                lhs.0 == rhs.0
            }
        }
    };
    TokenStream::from(expanded)
}

fn impl_sub(
    name: &syn::Ident,
    promote: Option<&(syn::Path, syn::Type)>,
) -> proc_macro2::TokenStream {
    if let Some((target_type, cast_type)) = promote {
        quote! {
            impl crate::lang::invokable::operator::OperatorSubtract<&Self, #target_type> for #name {
                fn sub(lhs: &Self, rhs: &Self) -> Result<#target_type, crate::Error> {
                    Ok(#target_type((lhs.0 as #cast_type) - (rhs.0 as #cast_type)))
                }
            }
        }
    } else {
        quote! {
            impl crate::lang::invokable::operator::OperatorSubtract<&Self, Self> for #name {
                fn sub(lhs: &Self, rhs: &Self) -> Result<Self, crate::Error> {
                    Ok(#name(lhs.0 - rhs.0))
                }
            }
        }
    }
}

#[proc_macro_derive(InnerTypeSub, attributes(cupid_arithmetic_promote))]
pub fn derive_inner_type_sub(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let _ = get_inner_type(&input);
    let promote = get_arithmetic_promote(&input.attrs);
    TokenStream::from(impl_sub(name, promote.as_ref()))
}

fn impl_mul(
    name: &syn::Ident,
    promote: Option<&(syn::Path, syn::Type)>,
) -> proc_macro2::TokenStream {
    if let Some((target_type, cast_type)) = promote {
        quote! {
            impl crate::lang::invokable::operator::OperatorMultiply<&Self, #target_type> for #name {
                fn mul(lhs: &Self, rhs: &Self) -> Result<#target_type, crate::Error> {
                    Ok(#target_type((lhs.0 as #cast_type) * (rhs.0 as #cast_type)))
                }
            }
        }
    } else {
        quote! {
            impl crate::lang::invokable::operator::OperatorMultiply<&Self, Self> for #name {
                fn mul(lhs: &Self, rhs: &Self) -> Result<Self, crate::Error> {
                    Ok(#name(lhs.0 * rhs.0))
                }
            }
        }
    }
}

#[proc_macro_derive(InnerTypeMul, attributes(cupid_arithmetic_promote))]
pub fn derive_inner_type_mul(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let _ = get_inner_type(&input);
    let promote = get_arithmetic_promote(&input.attrs);
    TokenStream::from(impl_mul(name, promote.as_ref()))
}

fn impl_div(
    name: &syn::Ident,
    promote: Option<&(syn::Path, syn::Type)>,
) -> proc_macro2::TokenStream {
    if let Some((target_type, cast_type)) = promote {
        quote! {
            impl crate::lang::invokable::operator::OperatorDivide<&Self, #target_type> for #name {
                fn div(lhs: &Self, rhs: &Self) -> Result<#target_type, crate::Error> {
                    Ok(#target_type((lhs.0 as #cast_type) / (rhs.0 as #cast_type)))
                }
            }
        }
    } else {
        quote! {
            impl crate::lang::invokable::operator::OperatorDivide<&Self, Self> for #name {
                fn div(lhs: &Self, rhs: &Self) -> Result<Self, crate::Error> {
                    Ok(#name(lhs.0 / rhs.0))
                }
            }
        }
    }
}

#[proc_macro_derive(InnerTypeDiv, attributes(cupid_arithmetic_promote))]
pub fn derive_inner_type_div(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let _ = get_inner_type(&input);
    let promote = get_arithmetic_promote(&input.attrs);
    TokenStream::from(impl_div(name, promote.as_ref()))
}

fn impl_cast(name: &syn::Ident, inner_type: &syn::Type) -> proc_macro2::TokenStream {
    let type_str = quote!(#inner_type).to_string();

    // Define known types and their wrappers
    let types = vec![
        ("i32", "TypeInt", "i32"),
        ("f64", "TypeFloat", "f64"),
        ("u8", "TypeByte", "u8"),
        ("bool", "TypeBool", "bool"),
    ];

    let mut impls = proc_macro2::TokenStream::new();

    for (_, wrapper, primitive) in types {
        let wrapper_ident = syn::Ident::new(wrapper, proc_macro2::Span::call_site());
        let primitive_ident = syn::Ident::new(primitive, proc_macro2::Span::call_site());

        // Don't implement From<Self> for Self (already exists)
        if name.to_string() == wrapper {
            continue;
        }

        let cast_expr = if type_str == "bool" && primitive == "bool" {
            quote! { val.0 }
        } else if type_str == "bool" {
            quote! { if val.0 { 1 as #primitive_ident } else { 0 as #primitive_ident } }
        } else if primitive == "bool" {
            quote! { val.0 != 0 as #inner_type }
        } else if type_str == "f64" && (primitive == "i32" || primitive == "u8") {
            quote! { val.0.round() as #primitive_ident }
        } else {
            quote! { val.0 as #primitive_ident }
        };

        impls.extend(quote! {
            impl From<#name> for crate::lang::type_system::#wrapper_ident {
                fn from(val: #name) -> Self {
                    crate::lang::type_system::#wrapper_ident(#cast_expr)
                }
            }
        });
    }
    impls
}

#[proc_macro_derive(InnerTypeArithmetic, attributes(cupid_arithmetic_promote))]
pub fn derive_inner_type_arithmetic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let inner_type = get_inner_type(&input);
    let promote = get_arithmetic_promote(&input.attrs);

    let add = impl_add(name, promote.as_ref());
    let sub = impl_sub(name, promote.as_ref());
    let mul = impl_mul(name, promote.as_ref());
    let div = impl_div(name, promote.as_ref());
    let cast = impl_cast(name, inner_type);

    let expanded = quote! {#add
        #sub
        #mul
        #div
        #cast
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(InnerTypeCast)]
pub fn derive_inner_type_cast(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let inner_type = get_inner_type(&input);
    TokenStream::from(impl_cast(name, inner_type))
}
