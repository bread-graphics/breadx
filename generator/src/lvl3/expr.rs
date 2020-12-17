// MIT/Apache2 License

use super::{
    syn_util::{int_litexpr_int, item_field, str_to_exprpath, str_to_pathseg, str_to_ty},
    Type,
};
use crate::lvl2::{BinaryOp, Expression, ExpressionItem, UnaryOp};
use proc_macro2::Span;
use std::iter;

#[inline]
pub fn cast_to_usize(e: syn::Expr) -> syn::Expr {
    syn::Expr::Cast(syn::ExprCast {
        attrs: vec![],
        expr: Box::new(e),
        as_token: Default::default(),
        ty: Box::new(Type::Basic("usize".into()).to_syn_ty()),
    })
}

impl Expression {
    #[inline]
    pub fn to_length_expr(&self, with_self_fields: bool, cast: bool) -> syn::Expr {
        #[inline]
        fn process_lli<T: Iterator<Item = ExpressionItem>>(
            iter: &mut T,
            with_self_fields: Option<&str>,
            cast: bool,
        ) -> syn::Expr {
            syn::Expr::Paren(syn::ExprParen {
                attrs: vec![],
                paren_token: Default::default(),
                expr: Box::new(match iter.next() {
                    None => panic!("Expected an operation where there was not one"),
                    Some(ExpressionItem::FieldRef(s)) => {
                        let b = if let Some(t) = with_self_fields {
                            item_field(str_to_exprpath(t), &s)
                        } else {
                            str_to_exprpath(&s)
                        };
                        if cast {
                            cast_to_usize(b)
                        } else {
                            b
                        }
                    }
                    Some(ExpressionItem::Value(i)) => int_litexpr_int(i),
                    Some(ExpressionItem::BinaryOp(b)) => {
                        let e1 = process_lli(iter, with_self_fields, cast);
                        let e2 = process_lli(iter, with_self_fields, cast);

                        syn::Expr::Binary(syn::ExprBinary {
                            attrs: vec![],
                            left: Box::new(e1),
                            op: match b {
                                BinaryOp::Add => syn::BinOp::Add(Default::default()),
                                BinaryOp::Sub => syn::BinOp::Sub(Default::default()),
                                BinaryOp::Mult => syn::BinOp::Mul(Default::default()),
                                BinaryOp::Div => syn::BinOp::Div(Default::default()),
                                BinaryOp::And => syn::BinOp::BitAnd(Default::default()),
                            },
                            right: Box::new(e2),
                        })
                    }
                    Some(ExpressionItem::UnaryOp(u)) => match u {
                        UnaryOp::OneCount => syn::Expr::Call(syn::ExprCall {
                            attrs: vec![],
                            func: Box::new(item_field(
                                process_lli(iter, with_self_fields, false),
                                "count_ones",
                            )),
                            paren_token: Default::default(),
                            args: syn::punctuated::Punctuated::new(),
                        }),
                        u => syn::Expr::Unary(syn::ExprUnary {
                            attrs: vec![],
                            expr: Box::new(process_lli(iter, with_self_fields, cast)),
                            op: match u {
                                UnaryOp::Not => syn::UnOp::Not(Default::default()),
                                _ => unreachable!(),
                            },
                        }),
                    },
                    Some(ExpressionItem::Remainder) => syn::Expr::Binary(syn::ExprBinary {
                        attrs: vec![],
                        left: Box::new(syn::Expr::Paren(syn::ExprParen {
                            attrs: vec![],
                            paren_token: Default::default(),
                            expr: Box::new(syn::Expr::Binary(syn::ExprBinary {
                                attrs: vec![],
                                left: Box::new(cast_to_usize(str_to_exprpath("length"))),
                                op: syn::BinOp::Mul(Default::default()),
                                right: Box::new(int_litexpr_int(4)),
                            })),
                        })),
                        op: syn::BinOp::Sub(Default::default()),
                        right: Box::new(str_to_exprpath("index")),
                    }),
                    Some(ExpressionItem::SumOf(slist, uses_extended)) => {
                        let sexpr = if uses_extended {
                            process_lli(iter, Some("a"), cast)
                        } else {
                            let mut once = std::iter::once(ExpressionItem::ListExprRef);
                            process_lli(&mut once, Some("a"), cast)
                        };

                        syn::Expr::MethodCall(syn::ExprMethodCall {
                            attrs: vec![],
                            receiver: Box::new({
                                let iterator = syn::Expr::Call(syn::ExprCall {
                                    attrs: vec![],
                                    func: Box::new(item_field(
                                        if let Some(a) = with_self_fields {
                                            item_field(str_to_exprpath(a), &slist)
                                        } else {
                                            str_to_exprpath(&slist)
                                        },
                                        "iter",
                                    )),
                                    paren_token: Default::default(),
                                    args: syn::punctuated::Punctuated::new(),
                                });
                                let mapper = syn::Expr::Closure(syn::ExprClosure {
                                    attrs: vec![],
                                    asyncness: None,
                                    movability: None,
                                    capture: None,
                                    or1_token: Default::default(),
                                    inputs: iter::once(syn::Pat::Ident(syn::PatIdent {
                                        attrs: vec![],
                                        by_ref: None,
                                        mutability: None,
                                        ident: syn::Ident::new("a", Span::call_site()),
                                        subpat: None,
                                    }))
                                    .collect(),
                                    or2_token: Default::default(),
                                    output: syn::ReturnType::Default,
                                    body: Box::new(cast_to_usize(sexpr)),
                                });

                                syn::Expr::Call(syn::ExprCall {
                                    attrs: vec![],
                                    func: Box::new(item_field(iterator, "map")),
                                    paren_token: Default::default(),
                                    args: iter::once(mapper).collect(),
                                })
                            }),
                            dot_token: Default::default(),
                            method: syn::Ident::new("sum", Span::call_site()),
                            turbofish: Some(syn::MethodTurbofish {
                                colon2_token: Default::default(),
                                lt_token: Default::default(),
                                gt_token: Default::default(),
                                args: iter::once(syn::GenericMethodArgument::Type(str_to_ty(
                                    "usize",
                                )))
                                .collect(),
                            }),
                            paren_token: Default::default(),
                            args: syn::punctuated::Punctuated::new(),
                        })
                    }
                    Some(ExpressionItem::ListExprRef) => {
                        // assume we're inside of a sumof loop, where 'a' is the item
                        let base_call = syn::Expr::Call(syn::ExprCall {
                            attrs: vec![],
                            args: iter::once(syn::Expr::Unary(syn::ExprUnary {
                                attrs: vec![],
                                op: syn::UnOp::Deref(Default::default()),
                                expr: Box::new(str_to_exprpath(with_self_fields.unwrap())),
                            }))
                            .collect(),
                            paren_token: Default::default(),
                            func: Box::new(syn::Expr::Path(syn::ExprPath {
                                attrs: vec![],
                                qself: None,
                                path: syn::Path {
                                    leading_colon: None,
                                    segments: vec![
                                        syn::PathSegment {
                                            ident: syn::Ident::new("TryInto", Span::call_site()),
                                            arguments: syn::PathArguments::AngleBracketed(
                                                syn::AngleBracketedGenericArguments {
                                                    colon2_token: Some(Default::default()),
                                                    lt_token: Default::default(),
                                                    gt_token: Default::default(),
                                                    args: iter::once(syn::GenericArgument::Type(
                                                        str_to_ty("usize"),
                                                    ))
                                                    .collect(),
                                                },
                                            ),
                                        },
                                        str_to_pathseg("try_into"),
                                    ]
                                    .into_iter()
                                    .collect(),
                                },
                            })),
                        });
                        syn::Expr::Call(syn::ExprCall {
                            attrs: vec![],
                            args: iter::once(syn::Expr::Lit(syn::ExprLit {
                                attrs: vec![],
                                lit: syn::Lit::Str(syn::LitStr::new(
                                    "Unable to cast type to usize",
                                    Span::call_site(),
                                )),
                            }))
                            .collect(),
                            paren_token: Default::default(),
                            func: Box::new(item_field(base_call, "expect")),
                        })
                    }
                }),
            })
        }

        let mut i = self.iter().cloned();
        process_lli(
            &mut i,
            if with_self_fields { Some("self") } else { None },
            cast,
        )
    }
}
