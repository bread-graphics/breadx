// MIT/Apache2 License

use crate::{name_safety, syn_util::*};
use heck::{CamelCase, ShoutySnakeCase};
use proc_macro2::Span;
use std::iter;
use treexml::Element;

#[inline]
pub fn const_items(name: &str, elements: Vec<Element>, is_xidtype: bool) -> Vec<syn::Item> {
  let mut last_value: Option<usize> = None;

  elements
    .into_iter()
    .filter_map(|mut e| {
      #[inline]
      fn get_literal(element: &Element, last_value: &mut Option<usize>) -> Option<usize> {
        let child = match element.children.get(0) {
          Some(child) => child,
          None => {
            return Some(match last_value {
              Some(last_value) => {
                *last_value += 1;
                *last_value
              }
              None => {
                *last_value = Some(0);
                0
              }
            })
          }
        };

        let inner: usize = child.text.clone()?.parse().ok()?;
        *last_value = Some(inner);

        Some(match child.name.as_str() {
          "value" => inner,
          "bits" => 2usize.pow(inner as _),
          _ => return None,
        })
      }

      let literal = int_litexpr(&format!("{}", { get_literal(&e, &mut last_value)? }));

      let id = format!(
        "{}_{}",
        name.to_shouty_snake_case(),
        name_safety(e.attributes.remove("name").unwrap()).to_shouty_snake_case(),
      );

      Some(syn::Item::Const(syn::ItemConst {
        attrs: vec![],
        vis: pub_vis(),
        const_token: Default::default(),
        ident: syn::Ident::new(&id, Span::call_site()),
        colon_token: Default::default(),
        ty: Box::new(str_to_ty(&name.to_camel_case())),
        eq_token: Default::default(),
        expr: Box::new(if is_xidtype {
          syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            func: Box::new(syn::Expr::Path(syn::ExprPath {
              attrs: vec![],
              qself: None,
              path: syn::Path {
                leading_colon: None,
                segments: iter::once(str_to_pathseg(name))
                  .chain(iter::once(str_to_pathseg("const_from_xid")))
                  .collect(),
              },
            })),
            paren_token: Default::default(),
            args: iter::once(literal).collect(),
          })
        } else {
          literal
        }),
        semi_token: Default::default(),
      }))
    })
    .collect()
}
