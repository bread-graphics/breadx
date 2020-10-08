// MIT/Apache2 License

use crate::{field::*, state::State, syn_util::*};
use heck::CamelCase;
use proc_macro2::Span;
use treexml::Element;

#[inline]
pub fn xevent(
  name: String,
  number: usize,
  children: Vec<Element>,
  state: &mut State,
) -> Result<(), crate::Failures> {
  let sname = format!("{}Event", name.to_camel_case());
  let s = super::xstruct::xstruct(&sname, children, state)?;
  state.add_event(name, s, number);
  Ok(())
}

#[inline]
pub fn replace_name(new_name: &str, items: &mut [syn::Item]) {
  items.iter_mut().for_each(|i| match i {
    syn::Item::Struct(syn::ItemStruct { ident, .. }) => {
      *ident = syn::Ident::new(new_name, Span::call_site());
    }
    syn::Item::Impl(syn::ItemImpl { self_ty, .. }) => {
      *self_ty = Box::new(str_to_ty(new_name));
    }
    _ => (),
  });
}
