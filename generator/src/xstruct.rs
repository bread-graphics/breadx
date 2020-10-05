// MIT/Apache2 License

use crate::{field::*, state::State, syn_util::*, Failures, MalformedStruct};
use itertools::Itertools;
use proc_macro2::Span;
use std::{convert::TryInto, iter};
use syn::punctuated::Punctuated;
use treexml::Element;

#[inline]
pub fn xstruct(
  name: &str,
  subelems: Vec<Element>,
  state: &mut State,
) -> Result<Vec<syn::Item>, Failures> {
  let mut subelems = subelems
    .into_iter()
    .filter_map(|item| Field::new(item, state).ok())
    .collect::<Vec<Field>>();
  normalize_fields(&mut subelems);
  if subelems.is_empty() {
    Err(MalformedStruct::NoFields.into())
  } else {
    Ok(vec![
      xstruct_defn(name, &subelems),
      xstruct_abs_impl(name, &subelems),
    ])
  }
}

#[inline]
fn xstruct_defn(name: &str, subelems: &[Field]) -> syn::Item {
  /* Equivalent generated code:

     #[derive(Default, Debug)]
     pub struct {name} {
         {fields}
     }
  */
  syn::Item::Struct(syn::ItemStruct {
    attrs: vec![derives(&["Default", "Debug"])],
    vis: pub_vis(),
    struct_token: Default::default(),
    ident: syn::Ident::new(&name, Span::call_site()),
    generics: Default::default(),
    fields: syn::FieldsNamed {
      brace_token: Default::default(),
      named: subelems
        .iter()
        .filter_map(|f| {
          if let Field::Actual { name, ty, .. } = f {
            Some(syn::Field {
              attrs: vec![],
              vis: pub_vis(),
              ident: Some(name.clone()),
              colon_token: Some(Default::default()),
              ty: ty.clone(),
            })
          } else {
            None
          }
        })
        .collect(),
    }
    .into(),
    semi_token: None,
  })
}

#[inline]
fn xstruct_abs_impl(name: &str, fields: &[Field]) -> syn::Item {
  syn::Item::Impl(syn::ItemImpl {
    unsafety: None,
    attrs: vec![],
    defaultness: None,
    impl_token: Default::default(),
    generics: Default::default(),
    trait_: Some((None, str_to_path("AsByteSequence"), Default::default())),
    self_ty: Box::new(str_to_ty(&name)),
    brace_token: Default::default(),
    items: vec![
      syn::ImplItem::Method(xstruct_size_fn(name, fields)),
      syn::ImplItem::Method(xstruct_as_bytes(name, fields)),
    ],
  })
}

#[inline]
fn xstruct_size_fn(name: &str, subelems: &[Field]) -> syn::ImplItemMethod {
  /* Equivalent generated code:

     #[inline]
     fn size(&self) -> usize {
         /* the sum of the size of the fields, the padding hints, and the lengths
            (which are represented as 16 bit unsigned integers, by the way */
     }
  */
  syn::ImplItemMethod {
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
      inputs: iter::empty::<syn::FnArg>().collect(),
      variadic: None,
      output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty("usize"))),
    },
    block: syn::Block {
      brace_token: Default::default(),
      stmts: vec![syn::Stmt::Expr({
        // construct the unary operator
        let mut sizes = subelems
          .iter()
          .map(|s| size_of_field(s))
          .collect::<Vec<Box<syn::Expr>>>();

        // combine them into a single size
        let mut current_size = sizes.pop().unwrap();
        while let Some(s) = sizes.pop() {
          current_size = Box::new(syn::Expr::Binary(syn::ExprBinary {
            attrs: vec![],
            left: current_size,
            op: syn::BinOp::Add(Default::default()),
            right: s,
          }));
        }

        *current_size
      })],
    },
  }
}

#[inline]
fn size_of_field(field: &Field) -> Box<syn::Expr> {
  match field {
    Field::Actual {
      ty, vec_len_index, ..
    } => Box::new(syn::Expr::Call(syn::ExprCall {
      attrs: vec![],
      func: Box::new(syn::Expr::Path(syn::ExprPath {
        attrs: vec![],
        qself: None,
        path: syn::Path {
          leading_colon: Some(Default::default()),
          segments: vec![
            str_to_pathseg("std"),
            str_to_pathseg("mem"),
            syn::PathSegment {
              ident: syn::Ident::new("size_of", Span::call_site()),
              arguments: syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                colon2_token: Some(Default::default()),
                lt_token: Default::default(),
                args: iter::once(syn::GenericArgument::Type(match vec_len_index {
                  None => ty.clone(),
                  Some(_) => match ty {
                    syn::Type::Path(syn::TypePath {
                      path: syn::Path { ref segments, .. },
                      ..
                    }) => match segments[0] {
                      syn::PathSegment {
                        arguments:
                          syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                            ref args,
                            ..
                          }),
                        ..
                      } => match args[0] {
                        syn::GenericArgument::Type(ref ty) => syn::Type::Ptr(syn::TypePtr {
                          star_token: Default::default(),
                          const_token: None,
                          mutability: Some(Default::default()),
                          elem: Box::new(ty.clone()),
                        }),
                        _ => unreachable!(),
                      },
                      _ => unreachable!(),
                    },
                    _ => unreachable!(),
                  },
                }))
                .collect(),
                gt_token: Default::default(),
              }),
            },
          ]
          .into_iter()
          .collect(),
        },
      })),
      paren_token: Default::default(),
      args: iter::empty::<syn::Expr>().collect(),
    })),
    Field::PaddingHint { bytes } => Box::new(syn::Expr::Lit(syn::ExprLit {
      attrs: vec![],
      lit: syn::Lit::Int(syn::LitInt::new(&format!("{}", bytes), Span::call_site())),
    })),
    Field::LenSlot { .. } => Box::new(syn::Expr::Lit(syn::ExprLit {
      attrs: vec![],
      lit: syn::Lit::Int(syn::LitInt::new("2", Span::call_site())),
    })),
  }
}

#[inline]
fn xstruct_as_bytes(name: &str, subelems: &[Field]) -> syn::ImplItemMethod {
  /* Expected generated output:

     #[inline]
     fn as_bytes(&self, bytes: &mut [u8]) {
         let mut index = 0;
         self.field1.as_bytes(&mut bytes[index..]);
         index += {size of field1};
         // and so on and so forth
     }
  */
  syn::ImplItemMethod {
    attrs: vec![inliner()],
    vis: syn::Visibility::Inherited,
    defaultness: None,
    sig: syn::Signature {
      constness: None,
      asyncness: None,
      unsafety: None,
      abi: None,
      fn_token: Default::default(),
      ident: syn::Ident::new("as_bytes", Span::call_site()),
      generics: Default::default(),
      paren_token: Default::default(),
      variadic: None,
      output: syn::ReturnType::Default,
      inputs: iter::once(self_fnarg())
        .chain(iter::once(bytes_fnarg()))
        .collect(),
    },
    block: syn::Block {
      brace_token: Default::default(),
      stmts: as_bytes_stmts(name, subelems),
    },
  }
}

#[inline]
fn as_bytes_stmts(name: &str, subelems: &[Field]) -> Vec<syn::Stmt> {
  let mut sizing_init = syn::Expr::Let(syn::ExprLet {
    attrs: vec![],
    let_token: Default::default(),
    pat: syn::Pat::Ident(syn::PatIdent {
      attrs: vec![],
      by_ref: None,
      mutability: Some(Default::default()),
      ident: syn::Ident::new("index", Span::call_site()),
      subpat: None,
    }),
    eq_token: Default::default(),
    expr: Box::new(syn::Expr::Lit(syn::ExprLit {
      attrs: vec![],
      lit: syn::Lit::Int(syn::LitInt::new("0", Span::call_site())),
    })),
  });

  let as_bytes_statements = subelems.iter().map(|subelem| match subelem {
    Field::Actual {
      name,
      vec_len_index,
      ..
    } => match vec_len_index {
      None => method_call_for_as_bytes(self_field(name.clone())),
      Some(_) => method_call_for_as_bytes(Box::new(syn::Expr::MethodCall(syn::ExprMethodCall {
        attrs: vec![],
        receiver: self_field(name.clone()),
        dot_token: Default::default(),
        method: syn::Ident::new("as_ptr", Span::call_site()),
        turbofish: None,
        paren_token: Default::default(),
        args: Punctuated::new(),
      }))),
    },
    Field::PaddingHint { .. } => syn::Expr::Verbatim("".parse().unwrap()),
    Field::LenSlot { target_index } => {
      method_call_for_as_bytes(Box::new(syn::Expr::MethodCall(syn::ExprMethodCall {
        attrs: vec![],
        receiver: self_field(subelems[*target_index].name_or_panic().clone()),
        dot_token: Default::default(),
        method: syn::Ident::new("len", Span::call_site()),
        turbofish: None,
        paren_token: Default::default(),
        args: Punctuated::new(),
      })))
    }
  });

  let increment_statements = subelems.iter().map(|subelem| {
    syn::Expr::AssignOp(syn::ExprAssignOp {
      attrs: vec![],
      left: Box::new(str_to_exprpath("index")),
      op: syn::BinOp::AddEq(Default::default()),
      right: size_of_field(subelem),
    })
  });

  iter::once(sizing_init)
    .chain(as_bytes_statements.interleave(increment_statements))
    .map(|expr| syn::Stmt::Semi(expr, Default::default()))
    .collect()
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
    args: iter::once(syn::Expr::Reference(syn::ExprReference {
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
    }))
    .collect(),
  })
}
