// MIT/Apache2 License

use super::Failures;
use crate::syn_util::*;
use heck::CamelCase;
use proc_macro2::Span;
use std::iter;
use syn::punctuated::Punctuated;
use treexml::Element;

#[inline]
pub fn xidunion(name: &str, subelems: Vec<Element>) -> Result<Vec<syn::Item>, Failures> {
  let subtypes = subelems
    .into_iter()
    .filter_map(|elem| {
      if &elem.name.to_lowercase() == "type" {
        Some(elem.text.unwrap().to_camel_case())
      } else {
        None
      }
    })
    .collect::<Vec<String>>();

  if subtypes.is_empty() {
    Err(Failures::MalformedXidunion)
  } else {
    Ok(vec![
      enum_decl(name, &subtypes),
      xidtype_impl(name, &subtypes),
      default_impl(name, &subtypes),
    ])
  }
}

#[inline]
fn enum_decl(name: &str, subtypes: &[String]) -> syn::Item {
  /* Equivalent generated code:

     #[derive(Debug, Copy, Clone, Hash)]
     pub enum {name} {
         {item1}({item1}),
         {item2}({item2}),
         ...
     }
  */
  let name = match name.chars().position(|x| x == ':') {
    None => name,
    Some(posn) => &name[posn + 1..],
  };

  syn::Item::Enum(syn::ItemEnum {
    attrs: vec![derives(&["Debug", "Copy", "Clone", "Hash"])],
    vis: syn::Visibility::Public(syn::VisPublic {
      pub_token: Default::default(),
    }),
    enum_token: Default::default(),
    ident: syn::Ident::new(&name, Span::call_site()),
    generics: Default::default(),
    brace_token: Default::default(),
    variants: subtypes
      .iter()
      .map(|t| {
        let t = match t.chars().position(|x| x == ':') {
          None => t,
          Some(posn) => &t[posn + 1..],
        };

        syn::Variant {
          attrs: vec![],
          ident: syn::Ident::new(&t.to_camel_case(), Span::call_site()),
          discriminant: None,
          fields: syn::Fields::Unnamed(syn::FieldsUnnamed {
            paren_token: syn::token::Paren {
              span: Span::call_site(),
            },
            unnamed: iter::once(syn::Field {
              attrs: vec![],
              vis: syn::Visibility::Inherited,
              ident: None,
              colon_token: None,
              ty: str_to_ty(&t.to_camel_case()),
            })
            .collect(),
          }),
        }
      })
      .collect(),
  })
}

#[inline]
fn xidtype_impl(name: &str, subtypes: &[String]) -> syn::Item {
  /* Equivalent generated code

     impl XidType for {name} {
         #[inline]
         fn xid(&self) -> XID {
             match self {
                 Self::{item1}(i) => i.xid(),
                 Self::{item2}(i) => i.xid(),
                 ...
             }
         }
     }
  */
  syn::Item::Impl(syn::ItemImpl {
    attrs: vec![],
    defaultness: None,
    unsafety: None,
    impl_token: Default::default(),
    generics: Default::default(),
    trait_: Some((None, str_to_path("XidType"), Default::default())),
    self_ty: Box::new(syn::Type::Path(syn::TypePath {
      qself: None,
      path: str_to_path(name),
    })),
    brace_token: Default::default(),
    items: vec![
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
          ident: syn::Ident::new("xid", Span::call_site()),
          generics: Default::default(),
          paren_token: Default::default(),
          inputs: iter::once(self_fnarg()).collect(),
          output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty("XID"))),
          variadic: None,
        },
        block: syn::Block {
          brace_token: Default::default(),
          stmts: vec![syn::Stmt::Expr(syn::Expr::Match(syn::ExprMatch {
            attrs: vec![],
            match_token: Default::default(),
            expr: Box::new(str_to_exprpath("self")),
            brace_token: Default::default(),
            arms: subtypes
              .iter()
              .map(|t| syn::Arm {
                attrs: vec![],
                pat: syn::Pat::TupleStruct(syn::PatTupleStruct {
                  attrs: vec![],
                  path: syn::Path {
                    leading_colon: None,
                    segments: iter::once(str_to_pathseg("Self"))
                      .chain(iter::once(str_to_pathseg(&t.to_camel_case())))
                      .collect(),
                  },
                  pat: syn::PatTuple {
                    attrs: vec![],
                    paren_token: Default::default(),
                    elems: iter::once(syn::Pat::Ident(syn::PatIdent {
                      attrs: vec![],
                      by_ref: None,
                      mutability: None,
                      ident: syn::Ident::new("i", Span::call_site()),
                      subpat: None,
                    }))
                    .collect(),
                  },
                }),
                guard: None,
                fat_arrow_token: Default::default(),
                comma: Some(Default::default()),
                body: Box::new(syn::Expr::MethodCall(syn::ExprMethodCall {
                  attrs: vec![],
                  receiver: Box::new(str_to_exprpath("i")),
                  dot_token: Default::default(),
                  method: syn::Ident::new("xid", Span::call_site()),
                  turbofish: None,
                  paren_token: Default::default(),
                  args: Punctuated::new(),
                })),
              })
              .collect(),
          }))],
        },
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
          ident: syn::Ident::new("from_xid", Span::call_site()),
          generics: Default::default(),
          paren_token: Default::default(),
          inputs: iter::once(syn::FnArg::Typed(syn::PatType {
            attrs: vec![],
            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
              attrs: vec![],
              by_ref: None,
              mutability: None,
              ident: syn::Ident::new("xid", Span::call_site()),
              subpat: None,
            })),
            colon_token: Default::default(),
            ty: Box::new(str_to_ty("XID")),
          }))
          .collect(),
          variadic: None,
          output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty("Self"))),
        },
        block: syn::Block {
          brace_token: Default::default(),
          stmts: vec![syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            func: Box::new(syn::Expr::Path(syn::ExprPath {
              attrs: vec![],
              qself: None,
              path: syn::Path {
                leading_colon: None,
                segments: iter::once(str_to_pathseg("Self"))
                  .chain(iter::once(str_to_pathseg(&subtypes[0].to_camel_case())))
                  .collect(),
              },
            })),
            paren_token: Default::default(),
            args: iter::once(syn::Expr::Call(syn::ExprCall {
              attrs: vec![],
              func: Box::new(syn::Expr::Path(syn::ExprPath {
                attrs: vec![],
                qself: None,
                path: syn::Path {
                  leading_colon: None,
                  segments: iter::once(str_to_pathseg(&subtypes[0].to_camel_case()))
                    .chain(iter::once(str_to_pathseg("from_xid")))
                    .collect(),
                },
              })),
              paren_token: Default::default(),
              args: iter::once(str_to_exprpath("xid")).collect(),
            }))
            .collect(),
          }))],
        },
      }),
    ],
  })
}

#[inline]
fn default_impl(name: &str, variants: &[String]) -> syn::Item {
  syn::Item::Impl(syn::ItemImpl {
    attrs: vec![],
    defaultness: None,
    unsafety: None,
    impl_token: Default::default(),
    generics: Default::default(),
    trait_: Some((None, str_to_path("Default"), Default::default())),
    self_ty: Box::new(str_to_ty(name)),
    brace_token: Default::default(),
    items: vec![syn::ImplItem::Method(syn::ImplItemMethod {
      attrs: vec![inliner()],
      vis: syn::Visibility::Inherited,
      defaultness: None,
      sig: syn::Signature {
        paren_token: Default::default(),
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: syn::Ident::new("default", Span::call_site()),
        generics: Default::default(),
        inputs: Punctuated::new(),
        variadic: None,
        output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty(name))),
      },
      block: syn::Block {
        brace_token: Default::default(),
        stmts: vec![syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
          attrs: vec![],
          func: Box::new(syn::Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path: syn::Path {
              leading_colon: None,
              segments: iter::once(str_to_pathseg(name))
                .chain(iter::once(str_to_pathseg(&variants[0])))
                .collect(),
            },
          })),
          paren_token: Default::default(),
          args: iter::once(syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            func: Box::new(syn::Expr::Path(syn::ExprPath {
              attrs: vec![],
              qself: None,
              path: syn::Path {
                leading_colon: None,
                segments: iter::once(str_to_pathseg("Default"))
                  .chain(iter::once(str_to_pathseg("default")))
                  .collect(),
              },
            })),
            paren_token: Default::default(),
            args: Punctuated::new(),
          }))
          .collect(),
        }))],
      },
    })],
  })
}
