// MIT/Apache2 License

use super::{
    syn_util::{int_litexpr_int, pub_vis, str_to_path},
    Asb, SizeSumPart, Statement, SumOfSizes, ToSyn, Trait, Type,
};
use crate::lvl2::TrueEnum;
use proc_macro2::{Span, TokenStream};
use std::{borrow::Cow, iter, str::FromStr};

/// Rust enum.
#[derive(Debug)]
pub struct REnum {
    pub name: Box<str>,
    pub underlying: Cow<'static, str>,
    pub variants: Box<[(Box<str>, i64)]>,
    pub asb: Asb,
}

impl ToSyn for REnum {
    #[inline]
    fn to_syn_item(self) -> Vec<syn::Item> {
        let s = syn::Item::Enum(syn::ItemEnum {
            attrs: vec![
                syn::Attribute {
                    pound_token: Default::default(),
                    style: syn::AttrStyle::Outer,
                    bracket_token: Default::default(),
                    path: str_to_path("repr"),
                    tokens: TokenStream::from_str(&format!("({})", &self.underlying)).unwrap(),
                },
                syn::Attribute {
                    pound_token: Default::default(),
                    style: syn::AttrStyle::Outer,
                    bracket_token: Default::default(),
                    path: str_to_path("derive"),
                    tokens: TokenStream::from_str(&format!(
                        "(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)"
                    ))
                    .unwrap(),
                },
            ],
            vis: pub_vis(),
            enum_token: Default::default(),
            ident: syn::Ident::new(&self.name, Span::call_site()),
            generics: Default::default(),
            brace_token: Default::default(),

            variants: self
                .variants
                .into_iter()
                .map(|(name, value)| syn::Variant {
                    attrs: vec![],
                    ident: syn::Ident::new(&name, Span::call_site()),
                    fields: syn::Fields::Unit,
                    discriminant: Some((Default::default(), int_litexpr_int(value))),
                })
                .collect(),
        });

        let name = self.name.clone();
        let asb = self.asb.to_syn_item(&name);
        iter::once(s)
            .chain(asb.into_iter())
            .chain(
                Trait::EnumDefault(self.variants.get(0).unwrap().0.clone()).to_syn_item(&self.name),
            )
            .collect()
    }
}

impl From<TrueEnum> for REnum {
    #[inline]
    fn from(te: TrueEnum) -> Self {
        let TrueEnum {
            name,
            underlying,
            variants,
        } = te;
        let name = name.into_boxed_str();
        let underlying = match underlying {
            crate::lvl2::Type::BasicType(c) => c,
            _ => unreachable!(),
        };

        let variants: Box<[(Box<str>, i64)]> = variants
            .into_iter()
            .map(|(name, value)| (name.into_boxed_str(), value))
            .collect();

        // generate the ASB setup
        let as_bytes_stmts = vec![super::MatchSelfToBytes {
            underlying: underlying.clone(),
        }
        .into()];

        let from_bytes_stmts = vec![super::MatchBytesToEnum {
            name: name.clone(),
            underlying: underlying.clone(),
            variants: variants.clone(),
        }
        .into()];

        REnum {
            name,
            underlying: underlying.clone(),
            variants,
            asb: Asb {
                is_none: false,
                as_bytes_stmts,
                from_bytes_stmts,
                fd_getting: None,
                size: SumOfSizes(vec![SizeSumPart::SizeofType(Type::Basic(underlying))]),
            },
        }
    }
}
