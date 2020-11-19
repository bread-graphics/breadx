// MIT/Apache2 License

use super::syn_util::{item_field, str_to_exprpath, str_to_pathseg};
use crate::lvl2::{ConditionVariant, UseCondition};
use heck::{CamelCase, SnakeCase};

impl UseCondition {
    #[inline]
    pub fn to_cond_expr(&self, exprname: &str) -> syn::Expr {
        match self.variant {
            ConditionVariant::BitflagVariant => syn::Expr::Call(syn::ExprCall {
                attrs: vec![],
                func: Box::new(item_field(
                    str_to_exprpath(exprname),
                    &self.enum_value.to_snake_case(),
                )),
                paren_token: Default::default(),
                args: syn::punctuated::Punctuated::new(),
            }),
            ConditionVariant::Equal => syn::Expr::Binary(syn::ExprBinary {
                attrs: vec![],
                left: Box::new(str_to_exprpath(exprname)),
                op: syn::BinOp::Eq(Default::default()),
                right: Box::new(syn::Expr::Path(syn::ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: syn::Path {
                        leading_colon: None,
                        segments: vec![
                            str_to_pathseg(&self.enum_name.to_camel_case()),
                            str_to_pathseg(&self.enum_value.to_camel_case()),
                        ]
                        .into_iter()
                        .collect(),
                    },
                })),
            }),
        }
    }
}
