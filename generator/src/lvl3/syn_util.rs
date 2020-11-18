// MIT/Apache2 License

//! Syn utilities.

use proc_macro2::{Span, TokenStream};
use std::{fmt, iter, str::FromStr};
use syn::Expr;

/// Convert a string to a path segment.
#[inline]
pub fn str_to_pathseg(s: &str) -> syn::PathSegment {
    syn::PathSegment {
        ident: syn::Ident::new(s, Span::call_site()),
        arguments: syn::PathArguments::None,
    }
}

/// Convert a string to a path.
#[inline]
pub fn str_to_path(s: &str) -> syn::Path {
    syn::Path {
        leading_colon: None,
        segments: iter::once(str_to_pathseg(s)).collect(),
    }
}

/// Convert a string to a type.
#[inline]
pub fn str_to_ty(s: &str) -> syn::Type {
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: str_to_path(s),
    })
}

/// Convert a string to an expression path.
#[inline]
pub fn str_to_exprpath(s: &str) -> syn::Expr {
    syn::Expr::Path(syn::ExprPath {
        attrs: vec![],
        qself: None,
        path: str_to_path(s),
    })
}

/// Convert an integer to an integer expression.
#[inline]
pub fn int_litexpr_int<T: fmt::Display>(item: T) -> syn::Expr {
    let f = format!("{}", item);
    syn::Expr::Lit(syn::ExprLit {
        attrs: vec![],
        lit: syn::Lit::Int(syn::LitInt::new(&f, Span::call_site())),
    })
}

/// Inline attribute.
#[inline]
pub fn inliner() -> syn::Attribute {
    syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        path: str_to_path("inline"),
        tokens: TokenStream::new(),
    }
}

/// Public visibility.
#[inline]
pub fn pub_vis() -> syn::Visibility {
    syn::Visibility::Public(syn::VisPublic {
        pub_token: Default::default(),
    })
}

/// Call a method.
#[inline]
pub fn method_call<I: Into<syn::Expr>, T: IntoIterator<Item = I>>(
    name: syn::Path,
    args: T,
) -> syn::Expr {
    syn::Expr::Call(syn::ExprCall {
        attrs: vec![],
        func: Box::new(syn::Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path: name,
        })),
        paren_token: Default::default(),
        args: args.into_iter().map(|m| m.into()).collect(),
    })
}

/// Get a field of an item.
#[inline]
pub fn item_field(name: Expr, fieldname: &str) -> syn::Expr {
    syn::Expr::Field(syn::ExprField {
        attrs: vec![],
        base: Box::new(name),
        dot_token: Default::default(),
        member: syn::Member::Named(syn::Ident::new(fieldname, Span::call_site())),
    })
}

/// Get a field of self.
#[inline]
pub fn self_field(fieldname: &str) -> syn::Expr {
    item_field(str_to_exprpath("self"), fieldname)
}

#[inline]
pub fn derive_attrs(d: &[&'static str]) -> syn::Attribute {
    let mut tokens = String::from("(");
    d.iter().for_each(|de| {
        tokens.push_str(de);
        tokens.push_str(", ");
    });
    tokens.push_str(")");
    syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        path: str_to_path("derive"),
        tokens: TokenStream::from_str(&tokens).unwrap(),
    }
}

#[inline]
pub fn repr_transparent() -> syn::Attribute {
    syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        path: str_to_path("repr"),
        tokens: TokenStream::from_str("(transparent)").unwrap(),
    }
}
