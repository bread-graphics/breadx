// MIT/Apache2 License

use super::{
    get_mapped_lifetime, lifetime,
    syn_util::{int_litexpr_int, str_to_pathseg, str_to_ty},
};
use crate::lvl2::Type as Lvl2Type;
use proc_macro2::Span;
use std::{borrow::Cow, iter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    /// Ordinary type.
    Basic(Cow<'static, str>),
    /// A path-based type.
    Path { owner: Box<str>, name: Box<str> },
    /// Ordinary type, but with lifetimes.
    HasLifetime(Cow<'static, str>, Vec<String>),
    /// Array type.
    Array(Box<Type>, u64),
    /// Vector type. Should only be used for the Fd list.
    Vector(Box<Type>),
    /// TinyVec type.
    TinyVec(Box<Type>, u64),
    /// Option type.
    Opt(Box<Type>),
    /// crate::Result type
    Res(Box<Type>),
    /// Tuple container type.
    Tuple(Vec<Type>),
    /// Reference to another type.
    Ref(Box<Type>, bool, Option<&'static str>),
    /// Slice of a type.
    Slice(Box<Type>),
    /// Cow type; contained combined with lifetime.
    Cow(Box<Type>, String),
}

impl Default for Type {
    #[inline]
    fn default() -> Self {
        Self::Basic(Cow::Borrowed(
            "ThisShouldntHappenSinceTypeDefaultShouldAlwaysBeOverwritten",
        ))
    }
}

impl Type {
    #[inline]
    pub fn from_name(mut name: String) -> Self {
        match memchr::memchr(b':', name.as_bytes()) {
            None => Self::Basic(name.into()),
            Some(posn) => {
                let mut tname = name.split_off(posn);
                tname.remove(0); // get rid of the colon
                log::trace!(
                    "Importing external with owner {} and name {}",
                    &name,
                    &tname
                );
                Self::Path {
                    owner: name.to_lowercase().into_boxed_str(),
                    name: tname.into_boxed_str(),
                }
            }
        }
    }

    #[inline]
    pub fn to_syn_ty(&self) -> syn::Type {
        match self {
            Self::Basic(c) => str_to_ty(&*c),
            Self::Path { owner, name } => syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: vec![
                        str_to_pathseg("super"),
                        str_to_pathseg(owner),
                        str_to_pathseg(name),
                    ]
                    .into_iter()
                    .collect(),
                },
            }),
            Self::HasLifetime(name, lifetimes) => syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: iter::once(syn::PathSegment {
                        ident: syn::Ident::new(name, Span::call_site()),
                        arguments: syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: lifetimes
                                    .into_iter()
                                    .map(|lifetime| {
                                        syn::GenericArgument::Lifetime(syn::Lifetime {
                                            apostrophe: Span::call_site(),
                                            ident: syn::Ident::new(lifetime, Span::call_site()),
                                        })
                                    })
                                    .collect(),
                                gt_token: Default::default(),
                            },
                        ),
                    })
                    .collect(),
                },
            }),
            Self::Array(c, len) => syn::Type::Array(syn::TypeArray {
                bracket_token: Default::default(),
                elem: Box::new(c.to_syn_ty()),
                semi_token: Default::default(),
                len: int_litexpr_int(len),
            }),
            Self::Vector(c) => syn_container_type_ty("Vec", c.to_syn_ty()),
            Self::TinyVec(c, len) => syn_container_type_ty(
                "TinyVec",
                syn::Type::Array(syn::TypeArray {
                    bracket_token: Default::default(),
                    elem: Box::new(c.to_syn_ty()),
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
                            arguments: generic_argument(c),
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
            Self::Ref(r, is_mut, lifetime) => syn::Type::Reference(syn::TypeReference {
                and_token: Default::default(),
                lifetime: match lifetime {
                    None => None,
                    Some(lifetime) => Some(syn::Lifetime::new(lifetime, Span::call_site())),
                },
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
            Self::Cow(ty, lifetime) => syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: iter::once(syn::PathSegment {
                        ident: syn::Ident::new("Cow", Span::call_site()),
                        arguments: syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: vec![
                                    syn::GenericArgument::Lifetime(syn::Lifetime {
                                        apostrophe: Span::call_site(),
                                        ident: syn::Ident::new(&lifetime, Span::call_site()),
                                    }),
                                    syn::GenericArgument::Type(ty.to_syn_ty()),
                                ]
                                .into_iter()
                                .collect(),
                                gt_token: Default::default(),
                            },
                        ),
                    })
                    .collect(),
                },
            }),
        }
    }

    /// Convert an Lvl2 item to an Lvl3 item.
    #[inline]
    pub fn from_lvl2(ty: Lvl2Type) -> Self {
        match ty {
            Lvl2Type::BasicType(bt) => {
                let lifetimes = get_mapped_lifetime(&bt);
                match lifetimes {
                    0 => Self::from_name(bt.into_owned()),
                    _ => Self::HasLifetime(bt, (0..lifetimes).map(|_| lifetime()).collect()),
                }
            }
            Lvl2Type::Array(at, asize) => {
                let lifetimes = get_mapped_lifetime(&at);
                match lifetimes {
                    0 => Self::Array(Box::new(Type::Basic(at)), asize),
                    _ => Self::Array(
                        Box::new(Self::HasLifetime(
                            at,
                            (0..lifetimes).map(|_| lifetime()).collect(),
                        )),
                        asize,
                    ),
                }
            }
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
fn generic_argument(inner: &Type) -> syn::PathArguments {
    generic_argument_ty(inner.to_syn_ty())
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
