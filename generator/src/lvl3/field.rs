// MIT/Apache2 License

use super::{syn_util::pub_vis, Type};
use crate::lvl2::{Field, List, MaybeString, StructureItem};
use proc_macro2::Span;
use std::iter;

impl StructureItem {
    #[inline]
    pub fn to_syn_field(&self) -> Option<syn::Field> {
        match self {
            StructureItem::Field(Field { name, ty, doc, .. }) => Some(syn::Field {
                attrs: vec![],
                vis: pub_vis(),
                ident: Some(syn::Ident::new(name, Span::call_site())),
                colon_token: Some(Default::default()),
                ty: Type::from_lvl2(ty.clone()).to_syn_ty(),
            }),
            StructureItem::List(List { name, ty, doc, .. }) => Some(syn::Field {
                attrs: vec![],
                vis: pub_vis(),
                ident: Some(syn::Ident::new(name, Span::call_site())),
                colon_token: Some(Default::default()),
                ty: match ty {
                    MaybeString::IsAString => Type::Basic("String".into()).to_syn_ty(),
                    MaybeString::NotAString(ty) => syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path {
                            leading_colon: None,
                            segments: iter::once(syn::PathSegment {
                                ident: syn::Ident::new("Vec", Span::call_site()),
                                arguments: syn::PathArguments::AngleBracketed(
                                    syn::AngleBracketedGenericArguments {
                                        colon2_token: None,
                                        lt_token: Default::default(),
                                        args: iter::once(syn::GenericArgument::Type(
                                            Type::from_lvl2(ty.clone()).to_syn_ty(),
                                        ))
                                        .collect(),
                                        gt_token: Default::default(),
                                    },
                                ),
                            })
                            .collect(),
                        },
                    }),
                },
            }),
            _ => None,
        }
    }
}
