// MIT/Apache2 License

use crate::{field::*, state::State, syn_util::*, Failures, MalformedStruct};
use itertools::Itertools;
use proc_macro2::Span;
use std::{collections::HashMap, convert::TryInto, iter};
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

  Ok(vec![
    xstruct_defn(name, &subelems),
    match subelems.is_empty() {
      false => xstruct_abs_impl(name, &subelems),
      true => xstruct_abs_impl_unit(name),
    },
  ])
}

#[inline]
fn xstruct_defn(name: &str, subelems: &[Field]) -> syn::Item {
  let fields: syn::Fields = match subelems.is_empty() {
    false => syn::FieldsNamed {
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
    true => syn::Fields::Unit,
  };

  /* Equivalent generated code:

     #[derive(Default, Debug, Clone)]
     pub struct {name} {
         {fields}
     }
  */
  syn::Item::Struct(syn::ItemStruct {
    attrs: vec![derives(&["Default", "Debug", "Clone"])],
    vis: pub_vis(),
    struct_token: Default::default(),
    ident: syn::Ident::new(&name, Span::call_site()),
    generics: Default::default(),
    fields,
    semi_token: match subelems.is_empty() {
      true => Some(Default::default()),
      false => None,
    },
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
      syn::ImplItem::Method(xstruct_from_bytes(name, fields)),
    ],
  })
}

#[inline]
fn xstruct_abs_impl_unit(name: &str) -> syn::Item {
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
          ident: syn::Ident::new("size", Span::call_site()),
          generics: Default::default(),
          paren_token: Default::default(),
          inputs: iter::empty::<syn::FnArg>().collect(),
          variadic: None,
          output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty("usize"))),
        },
        block: syn::Block {
          brace_token: Default::default(),
          stmts: vec![syn::Stmt::Expr(int_litexpr("0"))],
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
          stmts: vec![],
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
            Box::new(single_generic("Option", "Self")),
          ),
        },
        block: syn::Block {
          brace_token: Default::default(),
          stmts: vec![syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            func: Box::new(str_to_exprpath("Some")),
            paren_token: Default::default(),
            args: iter::once(str_to_exprpath("Self")).collect(),
          }))],
        },
      }),
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
          .map(|s| s.size())
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
  let mut sizing_init = sizing_init();

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
      method_call_for_as_bytes(Box::new(syn::Expr::Paren(syn::ExprParen {
        attrs: vec![],
        paren_token: Default::default(),
        expr: Box::new(syn::Expr::Cast(syn::ExprCast {
          attrs: vec![],
          expr: Box::new(syn::Expr::MethodCall(syn::ExprMethodCall {
            attrs: vec![],
            receiver: self_field(subelems[*target_index].name_or_panic().clone()),
            dot_token: Default::default(),
            method: syn::Ident::new("len", Span::call_site()),
            turbofish: None,
            paren_token: Default::default(),
            args: Punctuated::new(),
          })),
          as_token: Default::default(),
          ty: Box::new(str_to_ty("Card16")),
        })),
      })))
    }
  });

  let increment_statements = subelems
    .iter()
    .map(|subelem| increment_subelem_size(subelem));

  iter::once(sizing_init)
    .chain(as_bytes_statements.interleave(increment_statements))
    .map(|expr| syn::Stmt::Semi(expr, Default::default()))
    .collect()
}

#[inline]
fn increment_subelem_size(field: &Field) -> syn::Expr {
  syn::Expr::AssignOp(syn::ExprAssignOp {
    attrs: vec![],
    left: Box::new(str_to_exprpath("index")),
    op: syn::BinOp::AddEq(Default::default()),
    right: field.size(),
  })
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

#[inline]
fn xstruct_from_bytes(name: &str, fields: &[Field]) -> syn::ImplItemMethod {
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
      ident: syn::Ident::new("from_bytes", Span::call_site()),
      generics: Default::default(),
      paren_token: Default::default(),
      inputs: iter::once(bytes_fnarg_immut()).collect(),
      variadic: None,
      output: syn::ReturnType::Type(
        Default::default(),
        Box::new(single_generic("Option", "Self")),
      ),
    },
    block: syn::Block {
      brace_token: Default::default(),
      stmts: xstruct_from_bytes_stmts(name, fields),
    },
  }
}

#[inline]
fn xstruct_from_bytes_stmts(name: &str, fields: &[Field]) -> Vec<syn::Stmt> {
  let mut sizing_init = syn::Stmt::Semi(sizing_init(), Default::default());
  let mut len_hints: HashMap<usize, String> = HashMap::new();

  let statements = fields.iter().enumerate().flat_map(|(i, f)| {
    match f {
      Field::LenSlot { target_index } => {
        let mut randomized: [u8; 4] = [0; 4];
        getrandom::getrandom(&mut randomized).unwrap();
        let randomized = u32::from_ne_bytes(randomized);
        let randomized = format!("len{}", randomized);

        // load it
        let load_stmt = load_stmt(
          str_to_ty("i32"),
          syn::Ident::new(&randomized, Span::call_site()),
        );
        let inc_stmt = syn::Stmt::Semi(increment_subelem_size(f), Default::default());

        len_hints.insert(*target_index, randomized);
        vec![load_stmt, inc_stmt]
      }
      Field::PaddingHint { .. } => vec![syn::Stmt::Semi(
        increment_subelem_size(f),
        Default::default(),
      )],
      Field::Actual {
        ty,
        name: field_name,
        vec_len_index,
      } => {
        if let Some(ll) = vec_len_index {
          if let Ok(_) = ll.clone().into_single_item() {
            let len = match len_hints.remove(&i) {
              Some(len) => len,
              None => {
                log::error!("Bad list found in {}: {}", name, field_name);
                return vec![];
              }
            };
            // we need to load the pointer, then increment the index,
            // then create the vector via the unsafe function
            // "clone_from_ptr"
            return vec![
              load_stmt(
                syn::Type::Ptr(syn::TypePtr {
                  star_token: Default::default(),
                  const_token: Some(Default::default()),
                  mutability: None,
                  elem: Box::new(str_to_ty("u8")),
                }),
                field_name.clone(),
              ),
              syn::Stmt::Semi(increment_subelem_size(f), Default::default()),
              utilize_clone_from_ptr(ty.clone(), field_name.clone(), &len),
            ];
          }
        }

        vec![
          load_stmt(ty.clone(), field_name.clone()),
          syn::Stmt::Semi(increment_subelem_size(f), Default::default()),
        ]
      }
    }
  });

  let init_struct = syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
    attrs: vec![],
    func: Box::new(str_to_exprpath("Some")),
    paren_token: Default::default(),
    args: iter::once(syn::Expr::Struct(syn::ExprStruct {
      attrs: vec![],
      path: str_to_path("Self"),
      brace_token: Default::default(),
      dot2_token: None,
      rest: None,
      fields: fields
        .iter()
        .filter_map(|f| {
          if let Field::Actual { name, .. } = f {
            Some(syn::FieldValue {
              attrs: vec![],
              member: syn::Member::Named(name.clone()),
              colon_token: None,
              expr: syn::Expr::Verbatim(proc_macro2::TokenStream::new()),
            })
          } else {
            None
          }
        })
        .collect(),
    }))
    .collect(),
  }));

  iter::once(sizing_init)
    .chain(statements)
    .chain(iter::once(init_struct))
    .collect()
}

#[inline]
fn utilize_clone_from_ptr(vec_ty: syn::Type, id: syn::Ident, len: &str) -> syn::Stmt {
  // SAFETY: we know the length of the ptr and the ptr, so this
  //         should work out
  // TODO: increment list len
  syn::Stmt::Semi(
    syn::Expr::Let(syn::ExprLet {
      attrs: vec![],
      pat: syn::Pat::Type(syn::PatType {
        attrs: vec![],
        colon_token: Default::default(),
        pat: Box::new(syn::Pat::Ident(syn::PatIdent {
          attrs: vec![],
          by_ref: None,
          mutability: None,
          ident: id.clone(),
          subpat: None,
        })),
        ty: Box::new(vec_ty),
      }),
      eq_token: Default::default(),
      expr: Box::new(syn::Expr::Unsafe(syn::ExprUnsafe {
        attrs: vec![],
        unsafe_token: Default::default(),
        block: syn::Block {
          brace_token: Default::default(),
          stmts: vec![syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            func: Box::new(str_to_exprpath("clone_from_ptr")),
            paren_token: Default::default(),
            args: iter::once(str_to_exprpath(&format!("{}", id)))
              .chain(iter::once(str_to_exprpath(len)))
              .collect(),
          }))],
        },
      })),
      let_token: Default::default(),
    }),
    Default::default(),
  )
}

#[inline]
fn load_stmt(ty: syn::Type, id: syn::Ident) -> syn::Stmt {
  syn::Stmt::Semi(
    syn::Expr::Let(syn::ExprLet {
      attrs: vec![],
      let_token: Default::default(),
      eq_token: Default::default(),
      pat: syn::Pat::Type(syn::PatType {
        attrs: vec![],
        pat: Box::new(syn::Pat::Ident(syn::PatIdent {
          attrs: vec![],
          by_ref: None,
          mutability: None,
          ident: id,
          subpat: None,
        })),
        colon_token: Default::default(),
        ty: Box::new(ty.clone()),
      }),
      expr: Box::new(syn::Expr::Try(syn::ExprTry {
        question_token: Default::default(),
        attrs: vec![],
        expr: Box::new(syn::Expr::Call(syn::ExprCall {
          attrs: vec![],
          func: Box::new(syn::Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: Some(syn::QSelf {
              lt_token: Default::default(),
              ty: Box::new(ty),
              position: 0,
              as_token: None,
              gt_token: Default::default(),
            }),
            path: syn::Path {
              leading_colon: Some(Default::default()),
              segments: iter::once(str_to_pathseg("from_bytes")).collect(),
            },
          })),
          paren_token: Default::default(),
          args: iter::once(syn::Expr::Reference(syn::ExprReference {
            attrs: vec![],
            and_token: Default::default(),
            raw: Default::default(),
            mutability: None,
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
        })),
      })),
    }),
    Default::default(),
  )
}

#[inline]
fn sizing_init() -> syn::Expr {
  syn::Expr::Let(syn::ExprLet {
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
  })
}
