// MIT/Apache2 License

use super::{index_plus_equal, let_statement, Statement};
use crate::lvl3::{
    syn_util::{int_litexpr_int, str_to_exprpath, str_to_pathseg},
    Type,
};
use proc_macro2::Span;
use std::iter;

#[inline]
pub fn get_pad_align(ty: &Type) -> syn::Expr {
    syn::Expr::Call(syn::ExprCall {
        attrs: vec![],
        func: Box::new(syn::Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path: syn::Path {
                leading_colon: Some(Default::default()),
                segments: vec![
                    str_to_pathseg("core"),
                    str_to_pathseg("mem"),
                    syn::PathSegment {
                        ident: syn::Ident::new("align_of", Span::call_site()),
                        arguments: syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: Some(Default::default()),
                                lt_token: Default::default(),
                                args: iter::once(syn::GenericArgument::Type(ty.to_syn_ty()))
                                    .collect(),
                                gt_token: Default::default(),
                            },
                        ),
                    },
                ]
                .into_iter()
                .collect(),
            },
        })),
        paren_token: Default::default(),
        args: syn::punctuated::Punctuated::new(),
    })
}

/// Create the align_to and block_len fields.
#[derive(Debug, Clone, Copy)]
pub struct CreateAlignToAndBlockLen;

impl Statement for CreateAlignToAndBlockLen {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![
            syn::Stmt::Semi(
                let_statement(
                    "align_to",
                    Type::Basic("usize".into()),
                    int_litexpr_int(0),
                    true,
                ),
                Default::default(),
            ),
            syn::Stmt::Semi(
                let_statement(
                    "block_len",
                    Type::Basic("usize".into()),
                    int_litexpr_int(0),
                    true,
                ),
                Default::default(),
            ),
        ]
    }
}

/// Requires block_len to be set
#[derive(Debug, Clone)]
pub enum SetAlignAndAddPadding {
    AlignType(Type),
    Number(u64),
}

impl Statement for SetAlignAndAddPadding {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Semi(
            index_plus_equal(syn::Expr::Call(syn::ExprCall {
                attrs: vec![],
                func: Box::new(str_to_exprpath("buffer_pad")),
                paren_token: Default::default(),
                args: vec![
                    str_to_exprpath("block_len"),
                    match self {
                        Self::Number(num) => int_litexpr_int(num),
                        Self::AlignType(ty) => get_pad_align(ty),
                    },
                ]
                .into_iter()
                .collect(),
            })),
            Default::default(),
        )]
    }
}
