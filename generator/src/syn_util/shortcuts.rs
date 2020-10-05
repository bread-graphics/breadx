// MIT/Apache2 License

use proc_macro2::Span;
use std::iter;

#[inline]
pub fn str_to_pathseg(s: &str) -> syn::PathSegment {
  syn::PathSegment {
    ident: syn::Ident::new(s, Span::call_site()),
    arguments: syn::PathArguments::None,
  }
}

#[inline]
pub fn str_to_path(s: &str) -> syn::Path {
  syn::Path {
    leading_colon: None,
    segments: iter::once(str_to_pathseg(s)).collect(),
  }
}

#[inline]
pub fn str_to_ty(s: &str) -> syn::Type {
  syn::Type::Path(syn::TypePath {
    qself: None,
    path: str_to_path(s),
  })
}

#[inline]
pub fn str_to_exprpath(s: &str) -> syn::Expr {
  syn::Expr::Path(syn::ExprPath {
    attrs: vec![],
    qself: None,
    path: str_to_path(s),
  })
}
