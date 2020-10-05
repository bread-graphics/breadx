// MIT/Apache2 License

use crate::{syn_util::*, Failures};
use heck::SnakeCase;
use proc_macro2::Span;
use std::iter;
use treexml::Element;

#[inline]
pub fn xbitflags(
  name: &str,
  subelems: Vec<Element>,
) -> Result<Box<dyn FnOnce(&str) -> Vec<syn::Item>>, Failures> {
  let flags = subelems
    .into_iter()
    .filter_map(|elem| {
      if elem.name.as_str() == "doc" {
        return None;
      }

      let child = &elem.children[0];
      match child.name.as_str() {
        "value" => match child.text.as_deref() {
          None => None,
          Some(text) => {
            if let Some('0') = text.chars().nth(0) {
              None
            } else {
              Some(Err(Failures::MalformedBitflags))
            }
          }
        },
        "bit" => Some(Ok(Flag {
          name: {
            let name = elem.attributes.get("name").unwrap().to_owned();
            let name = super::name_safety(name);
            name.to_snake_case()
          },
          bit: child.text.as_deref().unwrap().parse().unwrap(),
        })),
        _ => panic!("Unexpected variant: {} on {}", child.name, name),
      }
    })
    .collect::<Result<Vec<Flag>, Failures>>()?;

  let name = name.to_owned();
  Ok(Box::new(move |ty| {
    let flags = flags;
    vec![
      xbitflag_decl(&name, &flags),
      xbitflag_asb(&name, &flags, ty),
    ]
  }))
}

#[inline]
fn xbitflag_decl(name: &str, flags: &[Flag]) -> syn::Item {
  /* Expected generated code:

     pub struct {name} {
         {flag1}: bool,
         // and so on and so forth
     }
  */
  syn::Item::Struct(syn::ItemStruct {
    attrs: vec![derives(&["Debug", "Copy", "Clone", "Default"])],
    vis: pub_vis(),
    struct_token: Default::default(),
    ident: syn::Ident::new(name, Span::call_site()),
    generics: Default::default(),
    fields: syn::FieldsNamed {
      brace_token: Default::default(),
      named: flags.iter().map(Flag::as_field).collect(),
    }
    .into(),
    semi_token: None,
  })
}

#[inline]
fn xbitflag_asb(name: &str, flags: &[Flag], underlying: &str) -> syn::Item {
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
        attrs: vec![],
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
          stmts: asb_stmts(name, flags, underlying),
        },
      }),
    ],
  })
}

#[inline]
fn asb_stmts(name: &str, flags: &[Flag], underlying: &str) -> Vec<syn::Stmt> {
  /* Expected generated code:

    let mut asb: {underlying} = 0;
    asb |= if self.item1 { 1 << bit } else { 0 };
    // so on and so forth
    asb.as_bytes()
  */
  let init_i32 = syn::Stmt::Semi(
    syn::Expr::Let(syn::ExprLet {
      attrs: vec![],
      let_token: Default::default(),
      pat: syn::Pat::Type(syn::PatType {
        attrs: vec![],
        pat: Box::new(syn::Pat::Ident(syn::PatIdent {
          attrs: vec![],
          by_ref: None,
          mutability: Some(Default::default()),
          ident: syn::Ident::new("asb", Span::call_site()),
          subpat: None,
        })),
        colon_token: Default::default(),
        ty: Box::new(str_to_ty(underlying)),
      }),
      eq_token: Default::default(),
      expr: Box::new(int_litexpr("0")),
    }),
    Default::default(),
  );

  let asb_i32 = syn::Stmt::Semi(
    syn::Expr::MethodCall(syn::ExprMethodCall {
      attrs: vec![],
      receiver: Box::new(str_to_exprpath("asb")),
      dot_token: Default::default(),
      method: syn::Ident::new("as_bytes", Span::call_site()),
      turbofish: None,
      paren_token: Default::default(),
      args: iter::once(str_to_exprpath("bytes")).collect(),
    }),
    Default::default(),
  );

  iter::once(init_i32)
    .chain(flags.iter().map(Flag::as_incrementer_for_asb))
    .chain(iter::once(asb_i32))
    .collect()
}

struct Flag {
  name: String,
  bit: usize,
}

impl Flag {
  #[inline]
  fn as_field(&self) -> syn::Field {
    syn::Field {
      attrs: vec![],
      vis: pub_vis(),
      ident: Some(syn::Ident::new(&self.name, Span::call_site())),
      colon_token: None,
      ty: str_to_ty("bool"),
    }
  }

  #[inline]
  fn as_incrementer_for_asb(&self) -> syn::Stmt {
    let bit = format!("{}", self.bit);
    let bit = Box::new(int_litexpr(&bit));

    syn::Stmt::Semi(
      syn::Expr::AssignOp(syn::ExprAssignOp {
        attrs: vec![],
        left: Box::new(str_to_exprpath("asb")),
        op: syn::BinOp::BitOrEq(Default::default()),
        right: Box::new(syn::Expr::If(syn::ExprIf {
          attrs: vec![],
          if_token: Default::default(),
          cond: self_field(syn::Ident::new(&self.name, Span::call_site())),
          then_branch: syn::Block {
            brace_token: Default::default(),
            stmts: vec![syn::Stmt::Expr(syn::Expr::Binary(syn::ExprBinary {
              attrs: vec![],
              left: Box::new(int_litexpr("1")),
              op: syn::BinOp::Shl(Default::default()),
              right: bit,
            }))],
          },
          else_branch: Some((
            Default::default(),
            Box::new(syn::Expr::Block(syn::ExprBlock {
              attrs: vec![],
              label: None,
              block: syn::Block {
                brace_token: Default::default(),
                stmts: vec![syn::Stmt::Expr(int_litexpr("0"))],
              },
            })),
          )),
        })),
      }),
      Default::default(),
    )
  }
}
