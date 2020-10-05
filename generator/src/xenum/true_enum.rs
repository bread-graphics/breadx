// MIT/Apache2 License

use crate::{syn_util::*, Failures};
use proc_macro2::Span;
use std::iter;
use treexml::Element;

#[inline]
pub fn xtrueenum(
  name: &str,
  subelems: Vec<Element>,
) -> Result<Box<dyn FnOnce(&str) -> Vec<syn::Item>>, Failures> {
  let variants = subelems
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

          Variant { name, value }
        })
      } else {
        None
      }
    })
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
          output: syn::ReturnType::Default,
        },
        block: syn::Block {
          brace_token: Default::default(),
          stmts: vec![xenum_transmute_then_asb(name, underlying)],
        },
      }),
    ],
  })
}

#[inline]
fn xenum_transmute_then_asb(name: &str, underlying: &str) -> syn::Stmt {
  /* Expected generated output:

     (self as i32).as_bytes(bytes);
  */
  syn::Stmt::Semi(
    syn::Expr::MethodCall(syn::ExprMethodCall {
      attrs: vec![],
      receiver: Box::new(syn::Expr::Paren(syn::ExprParen {
        attrs: vec![],
        paren_token: Default::default(),
        expr: Box::new(syn::Expr::Cast(syn::ExprCast {
          attrs: vec![],
          expr: Box::new(str_to_exprpath("self")),
          as_token: Default::default(),
          ty: Box::new(str_to_ty(underlying)),
        })),
      })),
      dot_token: Default::default(),
      method: syn::Ident::new("as_bytes", Span::call_site()),
      turbofish: None,
      paren_token: Default::default(),
      args: iter::once(str_to_exprpath("bytes")).collect(),
    }),
    Default::default(),
  )
}

#[derive(Clone)]
struct Variant {
  name: String,
  value: i32,
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
