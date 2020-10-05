// MIT/Apache2 License

use crate::syn_util::*;
use heck::CamelCase;
use proc_macro2::Span;
use std::{
  convert::TryFrom,
  fmt::{self, Write},
  iter,
};
use tinyvec::TinyVec;
use treexml::Element;

#[derive(Clone)]
pub enum Field {
  Actual {
    ty: syn::Type,
    name: syn::Ident,
    vec_len_index: Option<ListLen>,
  },
  PaddingHint {
    bytes: usize,
  },
  LenSlot {
    target_index: usize,
  },
}

impl Field {
  #[inline]
  pub fn name_or_panic(&self) -> &syn::Ident {
    match self {
      Self::Actual { name, .. } => name,
      _ => panic!(),
    }
  }
}

impl Field {
  #[inline]
  pub fn new(mut element: Element, state: &mut crate::state::State) -> Result<Field, ()> {
    #[inline]
    fn resolve_ty(elem: &Element, state: &mut crate::state::State) -> Result<String, ()> {
      Ok(match elem.attributes.get("enum") {
        Some(en) => {
          let ty = elem.attributes.get("type").unwrap().to_camel_case();
          state.resolve_enum(&ty, en);
          ty
        }
        None => elem.attributes.get("type").ok_or(())?.to_camel_case(),
      })
    }

    match element.name.to_lowercase().as_str() {
      "field" => Ok(Field::Actual {
        ty: str_to_ty(&resolve_ty(&element, state)?),
        name: syn::Ident::new(element.attributes.get("name").ok_or(())?, Span::call_site()),
        vec_len_index: None,
      }),
      "pad" => Ok(Field::PaddingHint {
        bytes: element
          .attributes
          .get("bytes")
          .ok_or(())?
          .parse()
          .map_err(|_| ())?,
      }),
      "list" => Ok(Field::Actual {
        ty: syn::Type::Path(syn::TypePath {
          qself: None,
          path: syn::Path {
            leading_colon: None,
            segments: iter::once(syn::PathSegment {
              ident: syn::Ident::new("Vec", Span::call_site()),
              arguments: syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Default::default(),
                args: iter::once(syn::GenericArgument::Type(str_to_ty(&resolve_ty(
                  &element, state,
                )?)))
                .collect(),
                gt_token: Default::default(),
              }),
            })
            .collect(),
          },
        }),
        name: syn::Ident::new(element.attributes.get("name").ok_or(())?, Span::call_site()),
        vec_len_index: Some(ListLen::parse(element.children.remove(0))),
      }),
      _ => Err(()),
    }
  }
}

impl Default for Field {
  #[inline]
  fn default() -> Self {
    Field::PaddingHint { bytes: 0 }
  }
}

#[inline]
pub fn normalize_fields(fields: &mut [Field]) {
  for i in 0..fields.len() {
    if let Field::Actual {
      vec_len_index: Some(ref id),
      ..
    } = fields[i]
    {
      let id = match id.clone().into_single_item() {
        Ok(id) => id,
        Err(_) => continue,
      };
      fields.iter_mut().any(|f| {
        if let Field::Actual { name, .. } = f {
          if format!("{}", name) == format!("{}", id) {
            *f = Field::LenSlot { target_index: i };
            return true;
          }
        }
        false
      });
    }
  }
}

#[derive(Clone)]
pub enum ListLenOp {
  Add,
  Sub,
  Mult,
  Div,
  FieldReference(syn::Ident),
  Value(i64),
}

impl Default for ListLenOp {
  #[inline]
  fn default() -> Self {
    Self::Add
  }
}

impl fmt::Display for ListLenOp {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Add => f.write_str("+"),
      Self::Sub => f.write_str("-"),
      Self::Mult => f.write_str("*"),
      Self::Div => f.write_str("/"),
      Self::FieldReference(s) => write!(f, "{}", s),
      Self::Value(m) => write!(f, "{}", m),
    }
  }
}

/// postfix notation
#[derive(Clone)]
#[repr(transparent)]
pub struct ListLen {
  opstack: TinyVec<[ListLenOp; 3]>,
}

impl ListLen {
  #[inline]
  pub fn new() -> Self {
    Self {
      opstack: TinyVec::new(),
    }
  }

  /// recursively parse an expression
  pub fn parse(element: Element) -> Self {
    let mut listlen = ListLen::new();
    match element.name.to_lowercase().as_str() {
      "fieldref" => listlen
        .opstack
        .push(ListLenOp::FieldReference(syn::Ident::new(
          &element.text.unwrap(),
          Span::call_site(),
        ))),
      "value" => listlen
        .opstack
        .push(ListLenOp::Value(element.text.unwrap().parse().unwrap())),
      "op" => {
        // get the op and children
        let mut op = element.attributes.get("op").unwrap().to_owned();
        let mut children = element.children;
        let child2 = children.remove(1);
        let child1 = children.remove(0);

        // calculate
        let listlen1 = Self::parse(child1);
        let listlen2 = Self::parse(child2);
        listlen.opstack.extend(listlen1.opstack);
        listlen.opstack.extend(listlen2.opstack);

        listlen.opstack.push(match op.remove(0) {
          '+' => ListLenOp::Add,
          '-' => ListLenOp::Sub,
          '*' => ListLenOp::Mult,
          '/' => ListLenOp::Div,
          _ => unreachable!("Invalid operation"),
        });
      }
      _ => unreachable!("Invalid case: {}", element.name),
    }
    listlen
  }

  #[inline]
  pub fn into_single_item(self) -> Result<syn::Ident, Self> {
    let fr_count = self
      .opstack
      .iter()
      .filter(|f| matches!(f, ListLenOp::FieldReference(_)))
      .count();
    if fr_count == 1 {
      // just get the fieldref and return it
      Ok(
        self
          .opstack
          .into_iter()
          .find_map(|f| {
            if let ListLenOp::FieldReference(t) = f {
              Some(t)
            } else {
              None
            }
          })
          .unwrap(),
      )
    } else {
      Err(self)
    }
  }

  #[inline]
  pub fn expr(&self) -> String {
    let mut mtr = String::with_capacity(24);
    let mut iter = self.opstack.iter().rev();
    while let Some(op) = iter.next() {
      match op {
        ListLenOp::Add | ListLenOp::Sub | ListLenOp::Mult | ListLenOp::Div => {
          let op2 = iter.next().unwrap();
          let op1 = iter.next().unwrap();
          write!(&mut mtr, "({} {} {})", op1, op, op2).unwrap();
        }
        op => write!(&mut mtr, "{}", op).unwrap(),
      }
    }
    mtr
  }
}
