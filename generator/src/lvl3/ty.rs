// MIT/Apache2 License

use super::syn_util::{int_litexpr_int, str_to_pathseg, str_to_ty};
use crate::lvl2::Type as Lvl2Type;
use proc_macro2::Span;
use std::{borrow::Cow, iter};

#[derive(Clone, Debug)]
pub enum Type {
    /// Ordinary type.
    Basic(Cow<'static, str>),
    /// Array type.
    Array(Cow<'static, str>, u64),
    /// Vector type.
    Vector(Box<Type>),
    /// TinyVec type.
    TinyVec(Cow<'static, str>, u64),
    /// Option type.
    Opt(Box<Type>),
    /// crate::Result type
    Res(Cow<'static, str>),
    /// Tuple container type.
    Tuple(Vec<Type>),
    /// Reference to another type.
    Ref(Box<Type>, bool),
    /// Slice of a type.
    Slice(Box<Type>),
}

impl Type {
    #[inline]
    pub fn to_syn_ty(&self) -> syn::Type {
        match self {
            Self::Basic(c) => str_to_ty(&*c),
            Self::Array(c, len) => syn::Type::Array(syn::TypeArray {
                bracket_token: Default::default(),
                elem: Box::new(str_to_ty(&*c)),
                semi_token: Default::default(),
                len: int_litexpr_int(len),
            }),
            Self::Vector(c) => syn_container_type_ty("Vec", c.to_syn_ty()),
            Self::TinyVec(c, len) => syn_container_type_ty(
                "TinyVec",
                syn::Type::Array(syn::TypeArray {
                    bracket_token: Default::default(),
                    elem: Box::new(str_to_ty(&*c)),
                    semi_token: Default::default(),
                    len: int_litexpr_int(len),
                }),
            ),
            Self::Opt(c) => syn_container_type_ty("Option", c.to_syn_ty()),
            Self::Res(c) => syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: vec![
                        str_to_pathseg("crate"),
                        syn::PathSegment {
                            ident: syn::Ident::new("Result", Span::call_site()),
                            arguments: generic_argument(&*c),
                        },
                    ]
                    .into_iter()
                    .collect(),
                },
            }),
            Self::Tuple(tys) => syn::Type::Tuple(syn::TypeTuple {
                paren_token: Default::default(),
                elems: tys.iter().map(|t| t.to_syn_ty()).collect(),
            }),
            Self::Ref(r, is_mut) => syn::Type::Reference(syn::TypeReference {
                and_token: Default::default(),
                lifetime: None,
                mutability: if *is_mut {
                    Some(Default::default())
                } else {
                    None
                },
                elem: Box::new(r.to_syn_ty()),
            }),
            Self::Slice(r) => syn::Type::Slice(syn::TypeSlice {
                bracket_token: Default::default(),
                elem: Box::new(r.to_syn_ty()),
            }),
        }
    }

    /// Convert an Lvl2 item to an Lvl3 item.
    #[inline]
    pub fn from_lvl2(ty: Lvl2Type) -> Self {
        match ty {
            Lvl2Type::BasicType(bt) => Self::Basic(bt),
            Lvl2Type::Array(at, asize) => Self::Array(at, asize),
        }
    }
}

#[inline]
fn syn_container_type_ty(name: &str, inner: syn::Type) -> syn::Type {
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: iter::once(syn::PathSegment {
                ident: syn::Ident::new(name, Span::call_site()),
                arguments: generic_argument_ty(inner),
            })
            .collect(),
        },
    })
}

#[inline]
fn generic_argument(inner: &str) -> syn::PathArguments {
    generic_argument_ty(str_to_ty(inner))
}

#[inline]
fn generic_argument_ty(ty: syn::Type) -> syn::PathArguments {
    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: iter::once(syn::GenericArgument::Type(ty)).collect(),
        gt_token: Default::default(),
    })
}
