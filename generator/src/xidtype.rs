// MIT/Apache2 License

use crate::syn_util::*;
use proc_macro2::Span;
use std::iter;
use syn::punctuated::Punctuated;

#[inline]
pub fn xidtype(name: &str) -> Vec<syn::Item> {
  vec![
    xidtype_struct_decl(name),
    xidtype_trait_impl(name),
    xidtype_default(name),
    xidtype_const_ctor(name),
  ]
}

#[inline]
fn xidtype_struct_decl(name: &str) -> syn::Item {
  /* Equivalent generated code:

     #[derive(Debug, Copy, Clone, Hash)]
     #[repr(transparent)]
     pub struct {name} {
         inner: XID
     }
  */
  syn::Item::Struct(syn::ItemStruct {
    attrs: vec![
      derives(&["Debug", "Copy", "Clone", "Hash"]),
      repr_transparent(),
    ],
    vis: pub_vis(),
    struct_token: Default::default(),
    ident: syn::Ident::new(&name, Span::call_site()),
    generics: Default::default(),
    fields: syn::Fields::Named(syn::FieldsNamed {
      brace_token: Default::default(),
      named: iter::once(syn::Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        ident: Some(syn::Ident::new("inner", Span::call_site())),
        colon_token: Some(Default::default()),
        ty: str_to_ty("XID"),
      })
      .collect(),
    }),
    semi_token: None,
  })
}

#[inline]
fn xidtype_trait_impl(name: &str) -> syn::Item {
  /* Equivalent generated code:

     impl XidType for {name} {
         #[inline]
         fn xid(&self) -> XID {
             self.inner
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
          ident: syn::Ident::new("xid", Span::call_site()),
          generics: Default::default(),
          paren_token: Default::default(),
          inputs: iter::once(self_fnarg()).collect(),
          variadic: None,
          output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty("XID"))),
        },
        block: syn::Block {
          brace_token: Default::default(),
          stmts: vec![syn::Stmt::Expr(syn::Expr::Field(syn::ExprField {
            attrs: vec![],
            base: Box::new(str_to_exprpath("self")),
            dot_token: Default::default(),
            member: syn::Member::Named(syn::Ident::new("inner", Span::call_site())),
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
            pat: Box::new(syn::Pat::Path(syn::PatPath {
              attrs: vec![],
              qself: None,
              path: str_to_path("xid"),
            })),
            colon_token: Default::default(),
            ty: Box::new(str_to_ty("XID")),
          }))
          .collect(),
          variadic: None,
          output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty(&name))),
        },
        block: syn::Block {
          brace_token: Default::default(),
          stmts: vec![syn::Stmt::Expr(syn::Expr::Struct(syn::ExprStruct {
            attrs: vec![],
            path: str_to_path(&name),
            brace_token: Default::default(),
            fields: iter::once(syn::FieldValue {
              attrs: vec![],
              member: syn::Member::Named(syn::Ident::new("inner", Span::call_site())),
              colon_token: Some(Default::default()),
              expr: syn::Expr::Path(syn::ExprPath {
                attrs: vec![],
                qself: None,
                path: str_to_path("xid"),
              }),
            })
            .collect(),
            dot2_token: None,
            rest: None,
          }))],
        },
      }),
    ],
  })
}

#[inline]
fn xidtype_default(name: &str) -> syn::Item {
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
        output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty(name))),
      },
      block: syn::Block {
        brace_token: Default::default(),
        stmts: vec![syn::Stmt::Expr(syn::Expr::Struct(syn::ExprStruct {
          attrs: vec![],
          path: str_to_path(name),
          brace_token: Default::default(),
          fields: iter::once(syn::FieldValue {
            attrs: vec![],
            member: syn::Member::Named(syn::Ident::new("inner", Span::call_site())),
            colon_token: Some(Default::default()),
            expr: int_litexpr("0"),
          })
          .collect(),
          dot2_token: None,
          rest: None,
        }))],
      },
    })],
  })
}

#[inline]
fn xidtype_const_ctor(name: &str) -> syn::Item {
  syn::Item::Impl(syn::ItemImpl {
    attrs: vec![],
    defaultness: None,
    unsafety: None,
    impl_token: Default::default(),
    generics: Default::default(),
    trait_: None,
    self_ty: Box::new(str_to_ty(name)),
    brace_token: Default::default(),
    items: vec![syn::ImplItem::Method(syn::ImplItemMethod {
      attrs: vec![inliner()],
      vis: syn::Visibility::Restricted(syn::VisRestricted {
        pub_token: Default::default(),
        paren_token: Default::default(),
        in_token: None,
        path: Box::new(str_to_path("crate")),
      }),
      defaultness: Default::default(),
      sig: syn::Signature {
        constness: Some(Default::default()),
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: syn::Ident::new("const_from_xid", Span::call_site()),
        generics: Default::default(),
        paren_token: Default::default(),
        inputs: iter::once(syn::FnArg::Typed(syn::PatType {
          attrs: vec![],
          pat: Box::new(syn::Pat::Path(syn::PatPath {
            attrs: vec![],
            qself: None,
            path: str_to_path("xid"),
          })),
          colon_token: Default::default(),
          ty: Box::new(str_to_ty("XID")),
        }))
        .collect(),
        variadic: None,
        output: syn::ReturnType::Type(Default::default(), Box::new(str_to_ty(name))),
      },
      block: syn::Block {
        brace_token: Default::default(),
        stmts: vec![syn::Stmt::Expr(syn::Expr::Struct(syn::ExprStruct {
          attrs: vec![],
          path: str_to_path(name),
          brace_token: Default::default(),
          fields: iter::once(syn::FieldValue {
            attrs: vec![],
            member: syn::Member::Named(syn::Ident::new("inner", Span::call_site())),
            colon_token: Some(Default::default()),
            expr: int_litexpr("0"),
          })
          .collect(),
          dot2_token: None,
          rest: None,
        }))],
      },
    })],
  })
}
