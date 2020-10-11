// MIT/Apache2 License

use crate::{
  name_safety,
  state::{State, TypeHint},
  syn_util::*,
  Failures,
};
use heck::CamelCase;
use proc_macro2::Span;
use std::iter;
use treexml::Element;

mod bitflags;
mod const_items;
mod true_enum;

#[inline]
pub fn xenum(
  name: &str,
  subelems: Vec<Element>,
  state: &mut State,
) -> Result<Option<UnresolvedEnum>, Failures> {
  if let Some(xtype) = state.name_hint(name) {
    if let TypeHint::Enum = xtype {
    } else {
      let owned = name.to_owned();
      return Ok(Some(UnresolvedEnum {
        name: "_".to_owned(),
        generator: Some(Box::new(move |_| {
          const_items::const_items(&owned, subelems, matches!(xtype, TypeHint::Xidtype))
        })),
      }));
    }
  }

  // if there's only one element, don't bother
  if subelems.len() == 1 {
    return Ok(None);
  }

  Ok(Some(UnresolvedEnum {
    name: name.to_camel_case(),
    generator: match subelems
      .iter()
      .any(|e| e.children[0].name.as_str() == "bit")
    {
      false => Some(true_enum::xtrueenum(name, subelems)?),
      true => Some(bitflags::xbitflags(name, subelems)?),
    },
  }))
}

#[inline]
pub fn size_of_tmethod(t: &str) -> syn::ImplItem {
  syn::ImplItem::Method(syn::ImplItemMethod {
    attrs: vec![inliner()],
    vis: syn::Visibility::Inherited,
    defaultness: None,
    sig: syn::Signature {
      constness: None,
      asyncness: None,
      unsafety: None,
      abi: None,
      fn_token: Default::default(),
      ident: syn::Ident::new("size", Span::call_site()),
      generics: Default::default(),
      paren_token: Default::default(),
      inputs: iter::once(self_fnarg()).collect(),
      variadic: None,
      output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty("usize"))),
    },
    block: syn::Block {
      brace_token: Default::default(),
      stmts: vec![syn::Stmt::Expr(size_of_ty(t))],
    },
  })
}

pub struct UnresolvedEnum {
  generator: Option<Box<dyn FnOnce(&str) -> Vec<syn::Item>>>,
  name: String,
}

impl UnresolvedEnum {
  #[inline]
  pub fn name(&self) -> &str {
    &self.name
  }

  #[inline]
  pub fn resolve(self, ty: &str) -> Vec<syn::Item> {
    (self.generator.unwrap())(Self::match_ty(ty))
  }

  #[inline]
  fn match_ty(ty: &str) -> &str {
    match ty.to_lowercase().as_str() {
      "byte" => "u8",
      "card8" => "u8",
      "card16" => "u16",
      _ => {
        log::warn!("Unresolved type: {}", ty);
        ty
      }
    }
  }
}

impl Default for UnresolvedEnum {
  #[inline]
  fn default() -> Self {
    Self {
      generator: None,
      name: String::new(),
    }
  }
}
