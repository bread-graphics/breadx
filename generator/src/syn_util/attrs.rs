// MIT/Apache2 License

use super::str_to_path;
use std::fmt::Write;

#[inline]
pub fn derives(d: &[&str]) -> syn::Attribute {
  let mut res = "(".to_owned();
  d.iter().enumerate().for_each(|(i, dn)| {
    write!(&mut res, "{}", dn).unwrap();
    if i < d.len() - 1 {
      res.push_str(", ");
    }
  });
  res.push(')');

  syn::Attribute {
    pound_token: Default::default(),
    style: syn::AttrStyle::Outer,
    bracket_token: Default::default(),
    path: str_to_path("derive"),
    tokens: res.parse().unwrap(),
  }
}

#[inline]
pub fn repr_transparent() -> syn::Attribute {
  syn::Attribute {
    pound_token: Default::default(),
    style: syn::AttrStyle::Outer,
    bracket_token: Default::default(),
    path: str_to_path("repr"),
    tokens: "(transparent)".parse().unwrap(),
  }
}

#[inline]
pub fn inliner() -> syn::Attribute {
  syn::Attribute {
    pound_token: Default::default(),
    style: syn::AttrStyle::Outer,
    bracket_token: Default::default(),
    path: str_to_path("inline"),
    tokens: proc_macro2::TokenStream::new(),
  }
}
