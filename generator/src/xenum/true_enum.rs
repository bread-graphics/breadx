// MIT/Apache2 License

use crate::{syn_util::*, Failures};
use heck::CamelCase;
use proc_macro2::Span;
use std::{
  collections::HashSet,
  hash::{Hash, Hasher},
  iter,
};
use syn::punctuated::Punctuated;
use treexml::Element;

#[inline]
pub fn xtrueenum(
  name: &str,
  subelems: Vec<Element>,
) -> Result<Box<dyn FnOnce(&str) -> Vec<syn::Item>>, Failures> {
  let mut variants = subelems
    .into_iter()
    .filter_map(|e| {
      if "item" == e.name.to_lowercase().as_str() {
        Some({
          let name = e.attributes.get("name").unwrap().to_owned();
          let name = super::name_safety(name);

          let child = &e.children[0];

          let value: i32 = match child.name.as_str() {
            "value" => child.text.as_deref().unwrap().parse().unwrap(),
            _ => return None,
          };

          Variant {
            name: name.to_camel_case(),
            value,
          }
        })
      } else {
        None
      }
    })
    .collect::<HashSet<Variant>>()
    .into_iter()
    .collect::<Vec<Variant>>();

  if variants.is_empty() {
    log::warn!("Empty enum: {}", name);
    Ok(Box::new(|_| vec![]))
  } else {
    let name = name.to_owned();
    Ok(Box::new(move |ty| {
      let variants = variants;
      vec![
        xenum_decl(&name, &variants, ty),
        xenum_asb(&name, &variants, ty),
        xenum_default(&name, &variants),
      ]
    }))
  }
}

#[inline]
fn xenum_decl(name: &str, variants: &[Variant], underlying: &str) -> syn::Item {
  syn::Item::Enum(syn::ItemEnum {
    attrs: vec![
      derives(&["Debug", "Copy", "Clone", "PartialEq", "Eq"]),
      syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        path: str_to_path("repr"),
        tokens: format!("({})", underlying).parse().unwrap(),
      },
    ],
    vis: pub_vis(),
    enum_token: Default::default(),
    ident: syn::Ident::new(name, Span::call_site()),
    generics: Default::default(),
    brace_token: Default::default(),
    variants: variants.iter().map(|v| v.as_syn()).collect(),
  })
}

#[inline]
fn xenum_asb(name: &str, variants: &[Variant], underlying: &str) -> syn::Item {
  syn::Item::Impl(syn::ItemImpl {
    attrs: vec![],
    defaultness: None,
    unsafety: None,
    impl_token: Default::default(),
    generics: Default::default(),
    trait_: Some((None, str_to_path("AsByteSequence"), Default::default())),
    self_ty: Box::new(str_to_ty(name)),
    brace_token: Default::default(),
    items: vec![
      super::size_of_tmethod(underlying),
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
          ident: syn::Ident::new("as_bytes", Span::call_site()),
          generics: Default::default(),
          paren_token: Default::default(),
          inputs: iter::once(self_fnarg())
            .chain(iter::once(bytes_fnarg()))
            .collect(),
          variadic: None,
          output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty("usize"))),
        },
        block: syn::Block {
          brace_token: Default::default(),
          stmts: vec![xenum_transmute_then_asb(name, underlying)],
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
          ident: syn::Ident::new("from_bytes", Span::call_site()),
          generics: Default::default(),
          paren_token: Default::default(),
          inputs: iter::once(bytes_fnarg_immut()).collect(),
          variadic: None,
          output: syn::ReturnType::Type(
            Default::default(),
            Box::new(single_generic_type(
              "Option",
              syn::Type::Tuple(syn::TypeTuple {
                paren_token: Default::default(),
                elems: vec![str_to_ty("Self"), str_to_ty("usize")]
                  .into_iter()
                  .collect(),
              }),
            )),
          ),
        },
        block: syn::Block {
          brace_token: Default::default(),
          stmts: xenum_fsb_stmts(name, variants, underlying),
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
          ident: syn::Ident::new("includes_optimization", Span::call_site()),
          generics: Default::default(),
          paren_token: Default::default(),
          inputs: Punctuated::new(),
          variadic: None,
          output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty("bool"))),
        },
        block: syn::Block {
          brace_token: Default::default(),
          stmts: vec![syn::Stmt::Expr(syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Bool(syn::LitBool {
              value: false,
              span: Span::call_site(),
            }),
          }))],
        },
      }),
    ],
  })
}

#[inline]
fn xenum_fsb_stmts(name: &str, variants: &[Variant], underlying: &str) -> Vec<syn::Stmt> {
  // let (ul, len) = <underlying>::from_bytes(bytes)?;
  let ul_init = syn::Stmt::Semi(
    syn::Expr::Let(syn::ExprLet {
      attrs: vec![],
      let_token: Default::default(),
      pat: syn::Pat::Tuple(syn::PatTuple {
        attrs: vec![],
        paren_token: Default::default(),
        elems: vec![
          syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: syn::Ident::new("ul", Span::call_site()),
            subpat: None,
          }),
          syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: syn::Ident::new("len", Span::call_site()),
            subpat: None,
          }),
        ]
        .into_iter()
        .collect(),
      }),
      eq_token: Default::default(),
      expr: Box::new(syn::Expr::Try(syn::ExprTry {
        attrs: vec![],
        question_token: Default::default(),
        expr: Box::new(syn::Expr::Call(syn::ExprCall {
          attrs: vec![],
          paren_token: Default::default(),
          args: iter::once(str_to_exprpath("bytes")).collect(),
          func: Box::new(syn::Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: Some(syn::QSelf {
              lt_token: Default::default(),
              ty: Box::new(str_to_ty(underlying)),
              position: 0,
              as_token: None,
              gt_token: Default::default(),
            }),
            path: syn::Path {
              leading_colon: Some(Default::default()),
              segments: iter::once(str_to_pathseg("from_bytes")).collect(),
            },
          })),
        })),
      })),
    }),
    Default::default(),
  );

  /* Some((match ul {
       val1 => Self::Type1,
       ...
       _ => return None,
     }, len))
  */
  let ul_match = syn::Expr::Match(syn::ExprMatch {
    attrs: vec![],
    match_token: Default::default(),
    expr: Box::new(str_to_exprpath("ul")),
    brace_token: Default::default(),
    arms: variants
      .iter()
      .map(|v| syn::Arm {
        attrs: vec![],
        pat: syn::Pat::Lit(syn::PatLit {
          attrs: vec![],
          expr: Box::new(int_litexpr(&format!("{}", v.value))),
        }),
        guard: None,
        fat_arrow_token: Default::default(),
        body: Box::new(syn::Expr::Path(syn::ExprPath {
          attrs: vec![],
          qself: None,
          path: syn::Path {
            leading_colon: None,
            segments: iter::once(str_to_pathseg("Self"))
              .chain(iter::once(str_to_pathseg(&v.name)))
              .collect(),
          },
        })),
        comma: Some(Default::default()),
      })
      .chain(iter::once(syn::Arm {
        attrs: vec![],
        pat: syn::Pat::Wild(syn::PatWild {
          attrs: vec![],
          underscore_token: Default::default(),
        }),
        guard: None,
        fat_arrow_token: Default::default(),
        body: Box::new(syn::Expr::Return(syn::ExprReturn {
          attrs: vec![],
          return_token: Default::default(),
          expr: Some(Box::new(str_to_exprpath("None"))),
        })),
        comma: Some(Default::default()),
      }))
      .collect(),
  });
  let ul_match = syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
    attrs: vec![],
    paren_token: Default::default(),
    func: Box::new(str_to_exprpath("Some")),
    args: iter::once(syn::Expr::Tuple(syn::ExprTuple {
      attrs: vec![],
      paren_token: Default::default(),
      elems: vec![ul_match, str_to_exprpath("len")].into_iter().collect(),
    }))
    .collect(),
  }));

  vec![ul_init, ul_match]
}

#[inline]
fn xenum_default(name: &str, variants: &[Variant]) -> syn::Item {
  /* Expected generated code:

    impl Default for {name} {
        #[inline]
        fn default() -> Self {
          // either the variant that equals zero or the first variant
        }
    }
  */
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
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: syn::Ident::new("default", Span::call_site()),
        generics: Default::default(),
        paren_token: Default::default(),
        inputs: Punctuated::new(),
        variadic: None,
        output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty("Self"))),
      },
      block: syn::Block {
        brace_token: Default::default(),
        stmts: vec![syn::Stmt::Expr(syn::Expr::Path(syn::ExprPath {
          attrs: vec![],
          qself: None,
          path: syn::Path {
            leading_colon: None,
            segments: iter::once(str_to_pathseg("Self"))
              .chain(iter::once(str_to_pathseg(
                &variants
                  .iter()
                  .find(|v| v.value == 0)
                  .unwrap_or_else(|| &variants[0])
                  .name,
              )))
              .collect(),
          },
        }))],
      },
    })],
  })
}

#[inline]
fn xenum_transmute_then_asb(name: &str, underlying: &str) -> syn::Stmt {
  /* Expected generated output:

     (self as i32).as_bytes(bytes);
  */
  syn::Stmt::Expr(syn::Expr::MethodCall(syn::ExprMethodCall {
    attrs: vec![],
    receiver: Box::new(syn::Expr::Paren(syn::ExprParen {
      attrs: vec![],
      paren_token: Default::default(),
      expr: Box::new(syn::Expr::Cast(syn::ExprCast {
        attrs: vec![],
        expr: Box::new(syn::Expr::Unary(syn::ExprUnary {
          attrs: vec![],
          op: syn::UnOp::Deref(Default::default()),
          expr: Box::new(str_to_exprpath("self")),
        })),
        as_token: Default::default(),
        ty: Box::new(str_to_ty(underlying)),
      })),
    })),
    dot_token: Default::default(),
    method: syn::Ident::new("as_bytes", Span::call_site()),
    turbofish: None,
    paren_token: Default::default(),
    args: iter::once(str_to_exprpath("bytes")).collect(),
  }))
}

#[derive(Clone)]
struct Variant {
  name: String,
  value: i32,
}

impl PartialEq for Variant {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.value.eq(&other.value)
  }
}

impl Eq for Variant {}

impl Hash for Variant {
  #[inline]
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    Hash::hash(&self.value, hasher)
  }
}

impl Variant {
  #[inline]
  fn as_syn(&self) -> syn::Variant {
    syn::Variant {
      attrs: vec![],
      ident: syn::Ident::new(&self.name, Span::call_site()),
      fields: syn::Fields::Unit,
      discriminant: Some((
        Default::default(),
        syn::Expr::Lit(syn::ExprLit {
          attrs: vec![],
          lit: syn::Lit::Int(syn::LitInt::new(
            &format!("{}", self.value),
            Span::call_site(),
          )),
        }),
      )),
    }
  }
}
