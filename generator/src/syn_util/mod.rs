// MIT/Apache2 License

mod attrs;
pub use attrs::*;
mod shortcuts;
pub use shortcuts::*;

use proc_macro2::Span;
use std::{boxed::Box, iter};
use syn::punctuated::Punctuated;

#[inline]
pub fn pub_vis() -> syn::Visibility {
  syn::Visibility::Public(syn::VisPublic {
    pub_token: Default::default(),
  })
}

#[inline]
pub fn self_fnarg() -> syn::FnArg {
  syn::FnArg::Receiver(syn::Receiver {
    attrs: vec![],
    reference: Some((Default::default(), None)),
    mutability: None,
    self_token: Default::default(),
  })
}

#[inline]
pub fn bytes_fnarg() -> syn::FnArg {
  syn::FnArg::Typed(syn::PatType {
    attrs: vec![],
    pat: Box::new(syn::Pat::Ident(syn::PatIdent {
      ident: syn::Ident::new("bytes", Span::call_site()),
      attrs: vec![],
      by_ref: None,
      mutability: None,
      subpat: None,
    })),
    colon_token: Default::default(),
    ty: Box::new(syn::Type::Reference(syn::TypeReference {
      and_token: Default::default(),
      lifetime: None,
      mutability: Some(Default::default()),
      elem: Box::new(syn::Type::Slice(syn::TypeSlice {
        bracket_token: Default::default(),
        elem: Box::new(str_to_ty("u8")),
      })),
    })),
  })
}

#[inline]
pub fn self_field(field: syn::Ident) -> Box<syn::Expr> {
  let e = syn::Expr::Field(syn::ExprField {
    attrs: vec![],
    base: Box::new(str_to_exprpath("self")),
    dot_token: Default::default(),
    member: syn::Member::Named(field),
  });
  Box::new(e)
}

#[inline]
pub fn int_litexpr(t: &str) -> syn::Expr {
  syn::Expr::Lit(syn::ExprLit {
    attrs: vec![],
    lit: syn::Lit::Int(syn::LitInt::new(t, Span::call_site())),
  })
}

#[inline]
pub fn size_of_ty(t: &str) -> syn::Expr {
  syn::Expr::Call(syn::ExprCall {
    attrs: vec![],
    func: Box::new(syn::Expr::Path(syn::ExprPath {
      attrs: vec![],
      qself: Some(syn::QSelf {
        lt_token: Default::default(),
        ty: Box::new(str_to_ty(t)),
        gt_token: Default::default(),
        as_token: None,
        position: 0,
      }),
      path: syn::Path {
        leading_colon: Some(Default::default()),
        segments: iter::once(str_to_pathseg("size")).collect(),
      },
    })),
    paren_token: Default::default(),
    args: Punctuated::new(),
  })
}
