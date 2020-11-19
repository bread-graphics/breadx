// MIT/Apache2 License

use super::{
    syn_util::{int_litexpr_int, item_field, str_to_exprpath},
    Type,
};
use crate::lvl2::{BinaryOp, Expression, ExpressionItem, UnaryOp};
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
            with_self_fields: bool,
            cast: bool,
        ) -> syn::Expr {
            syn::Expr::Paren(syn::ExprParen {
                attrs: vec![],
                paren_token: Default::default(),
                expr: Box::new(match iter.next() {
                    None => panic!("Expected an operation where there was not one"),
                    Some(ExpressionItem::FieldRef(s)) => {
                        let b = if with_self_fields {
                            item_field(str_to_exprpath("self"), &s)
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
                                process_lli(iter, with_self_fields, cast),
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
                }),
            })
        }

        let mut i = self.iter().cloned();
        process_lli(&mut i, with_self_fields, cast)
    }
}
