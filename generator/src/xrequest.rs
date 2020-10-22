// MIT/Apache2 License

use crate::{field::*, syn_util::*, xstruct, Failures};
use heck::CamelCase;
use proc_macro2::Span;
use std::iter;
use syn::punctuated::Punctuated;
use treexml::Element;

#[inline]
pub fn xrequest(
  name: &str,
  opcode: usize,
  mut children: Vec<Element>,
  state: &mut crate::state::State,
) -> Result<Vec<syn::Item>, Failures> {
  // remove the "reply" element, it's important
  let mut reply = children
    .iter()
    .rposition(|x| x.name.as_str() == "reply")
    .map(|p| children.remove(p));

  // generate the two rust structs
  let request_name = format!("{}Request", name);
  let reply_name = format!("{}Reply", name);
  let request_items = xstruct::xstruct(&request_name, children, state)?;
  let reply_items = if let Some(ref mut reply) = reply {
    let mut fields: Vec<Field> = xreq_fields()
      .into_iter()
      .chain(
        reply
          .children
          .drain(..)
          .filter_map(|e| Field::new(e, state).ok())
          .flatten(),
      )
      .collect();
    normalize_fields(&mut fields);

    xstruct::xstruct_with_fields(&reply_name, fields)
  } else {
    vec![]
  };

  Ok(
    request_items
      .into_iter()
      .chain(reply_items)
      .chain(iter::once(xrequest_impl(
        &request_name,
        opcode,
        reply.map(move |_| reply_name),
      )))
      .collect(),
  )
}

/// Fields that exist by default.
#[inline]
fn xreq_fields() -> Vec<Field> {
  vec![
    Field::Actual {
      name: syn::Ident::new("sequence", Span::call_site()),
      ty: str_to_ty("Card16"),
      vec_len_index: None,
      is_string: false,
    },
    Field::Actual {
      name: syn::Ident::new("length", Span::call_site()),
      ty: str_to_ty("Card32"),
      vec_len_index: None,
      is_string: false,
    },
  ]
}

#[inline]
fn xrequest_impl(name: &str, opcode: usize, reply_name: Option<String>) -> syn::Item {
  syn::Item::Impl(syn::ItemImpl {
    generics: Default::default(),
    attrs: vec![],
    defaultness: None,
    unsafety: None,
    impl_token: Default::default(),
    trait_: Some((None, str_to_path("Request"), Default::default())),
    self_ty: Box::new(str_to_ty(name)),
    brace_token: Default::default(),
    items: vec![
      syn::ImplItem::Type(syn::ImplItemType {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        defaultness: None,
        type_token: Default::default(),
        ident: syn::Ident::new("Reply", Span::call_site()),
        generics: Default::default(),
        eq_token: Default::default(),
        ty: match reply_name {
          Some(reply_name) => str_to_ty(&reply_name),
          None => syn::Type::Tuple(syn::TypeTuple {
            paren_token: Default::default(),
            elems: Punctuated::new(),
          }),
        },
        semi_token: Default::default(),
      }),
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
          ident: syn::Ident::new("opcode", Span::call_site()),
          generics: Default::default(),
          paren_token: Default::default(),
          inputs: iter::once(self_fnarg()).collect(),
          variadic: None,
          output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty("Byte"))),
        },
        block: syn::Block {
          brace_token: Default::default(),
          stmts: vec![syn::Stmt::Expr(int_litexpr(&format!("{}", opcode)))],
        },
      }),
    ],
  })
}
