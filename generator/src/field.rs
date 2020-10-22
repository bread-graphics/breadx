// MIT/Apache2 License

use crate::{is_vec_of_char, name_safety, syn_util::*};
use heck::CamelCase;
use proc_macro2::Span;
use std::{
  convert::TryFrom,
  fmt::{self, Write},
  iter,
};
use syn::punctuated::Punctuated;
use tinyvec::TinyVec;
use treexml::Element;

#[derive(Clone)]
pub enum Field {
  Actual {
    is_string: bool,
    ty: syn::Type,
    name: syn::Ident,
    vec_len_index: Option<ListLen>,
  },
  PaddingHint {
    bytes: usize,
  },
  LenSlot {
    target_index: usize,
    ty: syn::Type,
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

  #[inline]
  pub fn size(&self) -> Box<syn::Expr> {
    let field = self;
    match field {
      Field::Actual {
        ty, vec_len_index, ..
      } => Box::new({
        let ptr_size = {
          let base = syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            func: Box::new({
              let base = syn::Expr::Path(syn::ExprPath {
                attrs: vec![],
                qself: Some(syn::QSelf {
                  lt_token: Default::default(),
                  ty: Box::new(match vec_len_index {
                    None => ty.clone(),
                    Some(ll) => match ll.is_fixed_size() {
                      false => Self::inner_vec_ty(ty),
                      true => ty.clone(), // AsByteSequence should be implemented for arrays
                    },
                  }),
                  position: 0,
                  as_token: Default::default(),
                  gt_token: Default::default(),
                }),
                path: syn::Path {
                  leading_colon: Some(Default::default()),
                  segments: iter::once(str_to_pathseg("size")).collect(),
                },
              });

              base
            }),
            paren_token: Default::default(),
            args: Punctuated::new(),
          });

          match vec_len_index.as_ref().map(|ll| ll.is_fixed_size()) {
            None | Some(true) => base,
            Some(false) => syn::Expr::Binary(syn::ExprBinary {
              attrs: vec![],
              left: Box::new(base),
              op: syn::BinOp::Mul(Default::default()),
              right: Box::new(int_litexpr("32")),
            }),
          }
        };

        ptr_size
      }),
      Field::PaddingHint { bytes } => Box::new(int_litexpr(&format!("{}", bytes))),
      Field::LenSlot { .. } => Box::new(int_litexpr("4")),
    }
  }

  #[inline]
  pub fn inner_vec_ty(ty: &syn::Type) -> syn::Type {
    match ty {
      syn::Type::Path(syn::TypePath { path, .. }) => {
        if Some("String".to_owned()) == path.get_ident().map(|i| format!("{}", i)) {
          return str_to_ty("u8");
        }

        match path {
          syn::Path { ref segments, .. } => match segments[0] {
            syn::PathSegment {
              arguments:
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                  ref args,
                  ..
                }),
              ..
            } => match args[0] {
              syn::GenericArgument::Type(ref ty) => ty.clone(),
              _ => unreachable!(),
            },
            _ => unreachable!(),
          },
        }
      }
      _ => unreachable!(),
    }
  }

  #[inline]
  pub fn as_bytes_stmt(&self, subelems: &[Field]) -> syn::Expr {
    syn::Expr::AssignOp(syn::ExprAssignOp {
      attrs: vec![],
      left: Box::new(str_to_exprpath("index")),
      op: syn::BinOp::AddEq(Default::default()),
      right: Box::new(match self {
        Field::Actual {
          name,
          vec_len_index,
          ty,
          is_string,
        } => match vec_len_index.as_ref().map(|ll| ll.clone().is_fixed_size()) {
          None | Some(true) => method_call_for_as_bytes(self_field(name.clone())),
          Some(false) => {
            // call vector_as_bytes or string_as_bytes
            syn::Expr::Call(syn::ExprCall {
              attrs: vec![],
              func: Box::new(match is_string {
                true => str_to_exprpath("string_as_bytes"),
                false => str_to_exprpath("vector_as_bytes"),
              }),
              paren_token: Default::default(),
              args: vec![
                syn::Expr::Reference(syn::ExprReference {
                  expr: self_field(name.clone()),
                  mutability: None,
                  and_token: Default::default(),
                  raw: Default::default(),
                  attrs: vec![],
                }),
                as_bytes_slice(),
              ]
              .into_iter()
              .collect(),
            })
          }
        },
        Field::PaddingHint { bytes } => int_litexpr(&format!("{}", bytes)),
        Field::LenSlot { target_index, ty } => {
          method_call_for_as_bytes(Box::new(syn::Expr::Paren(syn::ExprParen {
            attrs: vec![],
            paren_token: Default::default(),
            expr: Box::new(syn::Expr::Cast(syn::ExprCast {
              attrs: vec![],
              expr: Box::new(match &subelems[*target_index] {
                Field::Actual {
                  vec_len_index,
                  name,
                  ..
                } => match vec_len_index.clone().unwrap().into_single_item() {
                  Ok(_) => syn::Expr::MethodCall(syn::ExprMethodCall {
                    attrs: vec![],
                    receiver: self_field(name.clone()),
                    dot_token: Default::default(),
                    method: syn::Ident::new("len", Span::call_site()),
                    turbofish: None,
                    paren_token: Default::default(),
                    args: Punctuated::new(),
                  }),
                  Err(ll) => ll.expr(),
                },
                _ => unreachable!(),
              }),
              as_token: Default::default(),
              ty: Box::new(ty.clone()),
            })),
          })))
        }
      }),
    })
  }

  #[inline]
  pub fn new(mut element: Element, state: &mut crate::state::State) -> Result<Vec<Field>, ()> {
    #[inline]
    fn resolve_ty(elem: &Element, state: &mut crate::state::State) -> Result<String, ()> {
      let en = match elem.attributes.get("enum") {
        Some(en) => Some(en),
        None => match elem.attributes.get("mask") {
          Some(ma) => Some(ma),
          None => None,
        },
      };

      Ok(match en {
        Some(en) => {
          let ty = elem.attributes.get("type").unwrap().to_camel_case();
          state.resolve_enum(&ty, en);
          ty
        }
        None => elem.attributes.get("type").ok_or(())?.to_camel_case(),
      })
    }

    match element.name.to_lowercase().as_str() {
      "field" => Ok(vec![Field::Actual {
        ty: str_to_ty(&resolve_ty(&element, state)?),
        name: syn::Ident::new(
          &name_safety(element.attributes.get("name").ok_or(())?.to_owned()),
          Span::call_site(),
        ),
        vec_len_index: None,
        is_string: false,
      }]),
      "pad" => Ok(vec![Field::PaddingHint {
        bytes: element
          .attributes
          .get("bytes")
          .ok_or(())?
          .parse()
          .map_err(|_| ())?,
      }]),
      "list" => Ok(vec![{
        if element.children.is_empty() {
          return Err(());
        }

        let ll = ListLen::parse(element.children.remove(0));
        let mut ty = match ll.fixed_size() {
          None => syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
              leading_colon: None,
              segments: iter::once(syn::PathSegment {
                ident: syn::Ident::new("Vec", Span::call_site()),
                arguments: syn::PathArguments::AngleBracketed(
                  syn::AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: iter::once(syn::GenericArgument::Type(str_to_ty(&resolve_ty(
                      &element, state,
                    )?)))
                    .collect(),
                    gt_token: Default::default(),
                  },
                ),
              })
              .collect(),
            },
          }),
          Some(fixed_size) => syn::Type::Array(syn::TypeArray {
            bracket_token: Default::default(),
            elem: Box::new(str_to_ty(&resolve_ty(&element, state)?)),
            semi_token: Default::default(),
            len: int_litexpr(&format!("{}", fixed_size)),
          }),
        };

        let mut is_string = false;
        if is_vec_of_char(&ty) {
          ty = str_to_ty("String");
          is_string = true;
        }

        Field::Actual {
          ty,
          name: syn::Ident::new(element.attributes.get("name").ok_or(())?, Span::call_site()),
          vec_len_index: Some(ll),
          is_string,
        }
      }]),
      "valueparam" => Ok({
        let name = syn::Ident::new(
          element.attributes.get("value-mask-name").ok_or(())?,
          Span::call_site(),
        );

        vec![
          Field::Actual {
            ty: str_to_ty(
              &element
                .attributes
                .get("value-mask-type")
                .ok_or(())?
                .to_camel_case(),
            ),
            name: name.clone(),
            vec_len_index: None,
            is_string: false,
          },
          Field::Actual {
            vec_len_index: Some(ListLen::bits_of(name)),
            ty: syn::Type::Path(syn::TypePath {
              qself: None,
              path: syn::Path {
                leading_colon: None,
                segments: iter::once(syn::PathSegment {
                  ident: syn::Ident::new("Vec", Span::call_site()),
                  arguments: syn::PathArguments::AngleBracketed(
                    syn::AngleBracketedGenericArguments {
                      colon2_token: None,
                      lt_token: Default::default(),
                      args: iter::once(syn::GenericArgument::Type(str_to_ty("u32"))).collect(),
                      gt_token: Default::default(),
                    },
                  ),
                })
                .collect(),
              },
            }),
            name: syn::Ident::new(
              element.attributes.get("value-list-name").unwrap(),
              Span::call_site(),
            ),
            is_string: false,
          },
        ]
      }),
      _ => Err(()),
    }
  }
}

#[inline]
fn method_call_for_as_bytes(field_path: Box<syn::Expr>) -> syn::Expr {
  syn::Expr::MethodCall(syn::ExprMethodCall {
    attrs: vec![],
    receiver: field_path,
    dot_token: Default::default(),
    method: syn::Ident::new("as_bytes", Span::call_site()),
    turbofish: None,
    paren_token: Default::default(),
    args: iter::once(as_bytes_slice()).collect(),
  })
}

#[inline]
fn as_bytes_slice() -> syn::Expr {
  syn::Expr::Reference(syn::ExprReference {
    attrs: vec![],
    and_token: Default::default(),
    raw: Default::default(),
    mutability: Some(Default::default()),
    expr: Box::new(syn::Expr::Index(syn::ExprIndex {
      attrs: vec![],
      expr: Box::new(str_to_exprpath("bytes")),
      bracket_token: Default::default(),
      index: Box::new(syn::Expr::Range(syn::ExprRange {
        attrs: vec![],
        from: Some(Box::new(str_to_exprpath("index"))),
        limits: syn::RangeLimits::HalfOpen(Default::default()),
        to: None,
      })),
    })),
  })
}

impl Default for Field {
  #[inline]
  fn default() -> Self {
    Field::PaddingHint { bytes: 0 }
  }
}

#[inline]
pub fn normalize_fields(fields: &mut Vec<Field>) {
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
        if let Field::Actual { name, ty, .. } = f {
          if format!("{}", name) == format!("{}", id) {
            *f = Field::LenSlot {
              target_index: i,
              ty: ty.clone(),
            };
            return true;
          }
        }
        false
      });
    }
  }

  let mut names: Vec<String> = Vec::with_capacity(fields.len());
  for i in (0..fields.len()).rev() {
    if let Field::Actual { name, .. } = &fields[i] {
      let name = format!("{}", name);
      if names.iter().any(|n| n.as_str() == name.as_str()) {
        fields.remove(i);
      } else {
        names.push(name);
      }
    }
  }
}

#[derive(Debug, Clone)]
pub enum ListLenOp {
  Add,
  Sub,
  Mult,
  Div,
  FieldReference(syn::Ident),
  Value(i64),
  BitsCount(syn::Ident),
}

impl ListLenOp {
  #[inline]
  fn is_value(&self) -> bool {
    match self {
      Self::Value(_) => true,
      _ => false,
    }
  }
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
      Self::BitsCount(s) => write!(f, "BitsOf({})", s),
    }
  }
}

/// postfix notation
#[derive(Debug, Clone)]
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

  #[inline]
  pub fn bits_of(other: syn::Ident) -> Self {
    let mut opstack = TinyVec::new();
    opstack.push(ListLenOp::BitsCount(other));
    Self { opstack }
  }

  /// recursively parse an expression
  #[inline]
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

        listlen.opstack.push(match op.remove(0) {
          '+' => ListLenOp::Add,
          '-' => ListLenOp::Sub,
          '*' => ListLenOp::Mult,
          '/' => ListLenOp::Div,
          _ => unreachable!("Invalid operation"),
        });

        let mut children = element.children;
        let child2 = children.remove(1);
        let child1 = children.remove(0);

        // calculate
        let listlen1 = Self::parse(child1);
        let listlen2 = Self::parse(child2);
        listlen.opstack.extend(listlen1.opstack);
        listlen.opstack.extend(listlen2.opstack);
      }
      _ => unreachable!("Invalid case: {}", element.name),
    }
    listlen
  }

  #[inline]
  pub fn is_fixed_size(&self) -> bool {
    self.fixed_size().is_some()
  }

  #[inline]
  pub fn fixed_size(&self) -> Option<i64> {
    if self.opstack.len() != 1 {
      None
    } else {
      match &self.opstack[0] {
        ListLenOp::Value(i) => Some(*i),
        _ => None,
      }
    }
  }

  #[inline]
  pub fn into_single_item(self) -> Result<syn::Ident, Self> {
    let fr_count = self
      .opstack
      .iter()
      .filter(|f| matches!(f, ListLenOp::FieldReference(_)))
      .count();
    if fr_count == 1 && self.opstack.len() == 1 {
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
  pub fn expr(&self) -> syn::Expr {
    let inner = self.expr_inner(0).0;
    syn::Expr::Cast(syn::ExprCast {
      attrs: vec![],
      expr: Box::new(inner),
      as_token: Default::default(),
      ty: Box::new(str_to_ty("usize")),
    })
  }

  #[inline]
  fn expr_inner(&self, posn: usize) -> (syn::Expr, usize) {
    let op = &self.opstack[posn];
    match op {
      ListLenOp::FieldReference(ref t) => (
        /*syn::Expr::Field(syn::ExprField {
          attrs: vec![],
          base: Box::new(str_to_exprpath("self")),
          dot_token: Default::default(),
          member: syn::Member::Named(t.clone()),
        })*/
        syn::Expr::Cast(syn::ExprCast {
          attrs: vec![],
          as_token: Default::default(),
          expr: Box::new(str_to_exprpath(&format!("{}", t))),
          ty: Box::new(str_to_ty("usize")),
        }),
        1,
      ),
      ListLenOp::Value(v) => (int_litexpr(&format!("{}", v)), 1),
      ListLenOp::Add | ListLenOp::Sub | ListLenOp::Mult | ListLenOp::Div => {
        // binary operation, get the next two exprs
        let mut nextop = posn + 1;
        let (e1, no1) = self.expr_inner(nextop);
        nextop += no1;
        let (e2, no2) = self.expr_inner(nextop);

        (
          syn::Expr::Paren(syn::ExprParen {
            attrs: vec![],
            paren_token: Default::default(),
            expr: Box::new(syn::Expr::Binary(syn::ExprBinary {
              attrs: vec![],
              left: Box::new(e1),
              right: Box::new(e2),
              op: match op {
                ListLenOp::Add => syn::BinOp::Add(Default::default()),
                ListLenOp::Sub => syn::BinOp::Sub(Default::default()),
                ListLenOp::Mult => syn::BinOp::Mul(Default::default()),
                ListLenOp::Div => syn::BinOp::Div(Default::default()),
                _ => unreachable!(),
              },
            })),
          }),
          no1 + no2,
        )
      }
      ListLenOp::BitsCount(b) => (
        syn::Expr::Call(syn::ExprCall {
          attrs: vec![],
          func: Box::new(syn::Expr::Field(syn::ExprField {
            attrs: vec![],
            base: Box::new(str_to_exprpath(&format!("{}", b))),
            dot_token: Default::default(),
            member: syn::Member::Named(syn::Ident::new("count_ones", Span::call_site())),
          })),
          paren_token: Default::default(),
          args: Punctuated::new(),
        }),
        1,
      ),
    }
  }
}

#[test]
fn listlen_test() {
  use tinyvec::tiny_vec;

  let top = ListLenOp::FieldReference(syn::Ident::new("test", Span::call_site()));
  let top2 = top.clone();
  let top3 = top.clone();
  let listlen = ListLen {
    opstack: tiny_vec![[ListLenOp; 3] => top2],
  };
  assert!(listlen.into_single_item().is_ok());

  let (mop, vop) = (ListLenOp::Mult, ListLenOp::Value(4));
  let vop2 = vop.clone();
  let listlen = ListLen {
    opstack: tiny_vec![[ListLenOp; 3] => mop, top, vop],
  };
  assert!(listlen.into_single_item().is_err());

  let listlen = ListLen {
    opstack: tiny_vec![[ListLenOp; 3] => vop2],
  };
  assert!(listlen.clone().into_single_item().is_err());
  assert!(listlen.is_fixed_size());

  let top4 = ListLenOp::FieldReference(syn::Ident::new("test2", Span::call_site()));
  let listlen = ListLen {
    opstack: tiny_vec![[ListLenOp; 3] => top3, top4],
  };
  assert!(listlen.clone().into_single_item().is_err());
  assert!(!listlen.is_fixed_size());
}

#[test]
fn listlen_parse() {
  const DOC: &str = r#"
<?xml version="1.1" encoding="UTF-8"?>
<items> 
  <fieldref>Test1</fieldref>
  <value>123</value>
  <op op="*">
    <fieldref>length</fieldref>
    <value>4</value>
  </op>
  <op op="*">
    <fieldref>keycode_count</fieldref>
    <fieldref>keysyms_per_keycode</fieldref>
  </op>
</items>"#;

  let doc = treexml::Document::parse(DOC.as_bytes()).unwrap();
  let root = doc.root.unwrap();

  let llen1 = ListLen::parse(root.children[0].clone());
  assert!(!llen1.is_fixed_size());
  assert_eq!(llen1.opstack.len(), 1);
  assert!(llen1.into_single_item().is_ok());

  let llen2 = ListLen::parse(root.children[1].clone());
  assert!(llen2.is_fixed_size());
  assert_eq!(llen2.opstack.len(), 1);
  assert!(llen2.into_single_item().is_err());

  let llen3 = ListLen::parse(root.children[2].clone());
  assert!(!llen3.is_fixed_size());
  assert_eq!(llen3.opstack.len(), 3);
  assert!(llen3.into_single_item().is_err());

  let llen4 = ListLen::parse(root.children[3].clone());
  assert!(!llen4.is_fixed_size());
  assert_eq!(llen4.opstack.len(), 3);
  assert!(llen4.into_single_item().is_err());
}
