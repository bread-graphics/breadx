// MIT/Apache2 License

use super::{bytes_slice, index_plus_equal, let_statement, Statement};
use crate::{
    lvl2::MaybeString,
    lvl3::{
        cast_to_usize,
        syn_util::{item_field, str_to_exprpath},
        Type,
    },
};
use proc_macro2::Span;
use std::{borrow::Cow, fmt, iter};

#[derive(Debug, Clone)]
pub struct AsBytesList {
    pub name: Box<str>,
    pub ty: MaybeString,
    pub pad: Option<usize>,
}

#[derive(Clone)]
pub struct FromBytesList {
    pub name: Box<str>,
    pub ty: MaybeString,
    pub len: syn::Expr,
    pub pad: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct FromBytesListRemainder {
    pub name: Box<str>,
    pub ty: MaybeString,
}

impl fmt::Debug for FromBytesList {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FromBytesList")
    }
}

#[derive(Debug, Clone)]
pub struct AppendLengthToIndex {
    pub owner: Box<str>,
    pub ty: Type,
}

#[inline]
fn pad_statement(ms: &MaybeString, pad: Option<usize>) -> super::SetAlignAndAddPadding {
    match pad {
        Some(pad) => super::SetAlignAndAddPadding::Number(pad as _),
        None => super::SetAlignAndAddPadding::AlignType(match ms {
            MaybeString::IsAString => Type::Basic("c_char".into()),
            MaybeString::NotAString(ty) => Type::from_lvl2(ty.clone()),
        }),
    }
}

impl Statement for AsBytesList {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        let s = syn::Stmt::Semi(
            let_statement(
                "block_len",
                Type::Basic("usize".into()),
                syn::Expr::Call(syn::ExprCall {
                    attrs: vec![],
                    func: Box::new(str_to_exprpath(match self.ty {
                        MaybeString::IsAString => "string_as_bytes",
                        MaybeString::NotAString(_) => "vector_as_bytes",
                    })),
                    paren_token: Default::default(),
                    args: vec![
                        syn::Expr::Reference(syn::ExprReference {
                            attrs: vec![],
                            and_token: Default::default(),
                            raw: Default::default(),
                            mutability: None,
                            expr: Box::new(item_field(str_to_exprpath("self"), &self.name)),
                        }),
                        bytes_slice(true),
                    ]
                    .into_iter()
                    .collect(),
                }),
                false,
            ),
            Default::default(),
        );

        let i = syn::Stmt::Semi(
            index_plus_equal(str_to_exprpath("block_len")),
            Default::default(),
        );

        let p = pad_statement(&self.ty, self.pad);

        iter::once(s)
            .chain(iter::once(i))
            .chain(p.to_syn_statement())
            .collect()
    }
}

impl Statement for FromBytesList {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        let s = syn::Stmt::Semi(
            syn::Expr::Let(syn::ExprLet {
                attrs: vec![],
                let_token: Default::default(),
                pat: syn::Pat::Type(syn::PatType {
                    attrs: vec![],
                    pat: Box::new(syn::Pat::Tuple(syn::PatTuple {
                        attrs: vec![],
                        paren_token: Default::default(),
                        elems: vec![
                            syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: syn::Ident::new(&self.name, Span::call_site()),
                                subpat: None,
                            }),
                            syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: syn::Ident::new("block_len", Span::call_site()),
                                subpat: None,
                            }),
                        ]
                        .into_iter()
                        .collect(),
                    })),
                    colon_token: Default::default(),
                    ty: Box::new(syn::Type::Tuple(syn::TypeTuple {
                        paren_token: Default::default(),
                        elems: vec![
                            match self.ty {
                                MaybeString::IsAString => Type::Basic("String".into()).to_syn_ty(),
                                MaybeString::NotAString(ref ty) => {
                                    Type::Vector(Box::new(Type::from_lvl2(ty.clone()))).to_syn_ty()
                                }
                            },
                            Type::Basic("usize".into()).to_syn_ty(),
                        ]
                        .into_iter()
                        .collect(),
                    })),
                }),
                eq_token: Default::default(),
                expr: Box::new(syn::Expr::Try(syn::ExprTry {
                    attrs: vec![],
                    expr: Box::new(syn::Expr::Call(syn::ExprCall {
                        attrs: vec![],
                        func: Box::new(str_to_exprpath(match self.ty {
                            MaybeString::IsAString => "string_from_bytes",
                            MaybeString::NotAString(_) => "vector_from_bytes",
                        })),
                        paren_token: Default::default(),
                        args: vec![bytes_slice(false), cast_to_usize(self.len.clone())]
                            .into_iter()
                            .collect(),
                    })),
                    question_token: Default::default(),
                })),
            }),
            Default::default(),
        );

        let i = syn::Stmt::Semi(
            index_plus_equal(str_to_exprpath("block_len")),
            Default::default(),
        );

        let p = pad_statement(&self.ty, self.pad);

        iter::once(s)
            .chain(iter::once(i))
            .chain(p.to_syn_statement())
            .collect()
    }
}

impl Statement for AppendLengthToIndex {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Semi(
            index_plus_equal(syn::Expr::Call(syn::ExprCall {
                attrs: vec![],
                func: Box::new(item_field(
                    syn::Expr::Paren(syn::ExprParen {
                        attrs: vec![],
                        paren_token: Default::default(),
                        expr: Box::new(syn::Expr::Cast(syn::ExprCast {
                            attrs: vec![],
                            expr: Box::new(syn::Expr::Call(syn::ExprCall {
                                attrs: vec![],
                                func: Box::new(item_field(
                                    item_field(str_to_exprpath("self"), &self.owner),
                                    "len",
                                )),
                                paren_token: Default::default(),
                                args: syn::punctuated::Punctuated::new(),
                            })),
                            as_token: Default::default(),
                            ty: Box::new(self.ty.to_syn_ty()),
                        })),
                    }),
                    "as_bytes",
                )),
                paren_token: Default::default(),
                args: iter::once(bytes_slice(true)).collect(),
            })),
            Default::default(),
        )]
    }
}
