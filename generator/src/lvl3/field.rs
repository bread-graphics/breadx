// MIT/Apache2 License

use super::{get_mapped_lifetime, lifetime, syn_util::pub_vis, Type};
use crate::lvl2::{
    Expression, Field as Lvl2Field, List as Lvl2List, MaybeString as Lvl2Ms,
    StructureItem as Lvl2StructureItem, UseCondition,
};
use proc_macro2::Span;
use std::{iter, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructureItem {
    Field(Field),
    List(List),
    Padding { bytes: usize },
    LenSlot { ty: Type, owning_list: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub doc: Option<String>,
    pub condition: Option<Rc<UseCondition>>,
}

impl Field {
    #[inline]
    fn lifetimes(&self) -> &[String] {
        match &self.ty {
            Type::HasLifetime(_, lifetimes) => lifetimes,
            _ => &[],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    pub name: String,
    pub ty: MaybeString,
    pub doc: Option<String>,
    pub list_length: Expression,
    pub padding: Option<usize>,
    pub cow_lifetime: String,
}

impl List {
    #[inline]
    fn lifetimes(&self) -> impl Iterator<Item = &String> {
        iter::once(&self.cow_lifetime).chain(match &self.ty {
            MaybeString::NotAString(Type::HasLifetime(_, lifetimes)) => lifetimes.iter(),
            _ => (&[]).iter(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaybeString {
    IsAString,
    NotAString(Type),
}

impl StructureItem {
    #[inline]
    pub fn lifetimes(&self) -> Vec<String> {
        match self {
            Self::Field(f) => f.lifetimes().to_vec(),
            Self::List(l) => l.lifetimes().cloned().collect(),
            _ => vec![],
        }
    }

    #[inline]
    pub fn from_lvl2(inner: Lvl2StructureItem) -> Self {
        match inner {
            Lvl2StructureItem::Field(Lvl2Field {
                name,
                ty,
                doc,
                condition,
            }) => Self::Field(Field {
                name,
                ty: Type::from_lvl2(ty),
                doc,
                condition,
            }),
            Lvl2StructureItem::Padding { bytes } => Self::Padding { bytes },
            Lvl2StructureItem::List(Lvl2List {
                name,
                ty,
                doc,
                list_length,
                padding,
            }) => Self::List(List {
                name,
                ty: match ty {
                    Lvl2Ms::IsAString => MaybeString::IsAString,
                    Lvl2Ms::NotAString(ty) => MaybeString::NotAString(Type::from_lvl2(ty)),
                },
                doc,
                list_length,
                padding,
                cow_lifetime: lifetime(),
            }),
            Lvl2StructureItem::LenSlot { ty, owning_list } => StructureItem::LenSlot {
                ty: Type::from_lvl2(ty),
                owning_list,
            },
        }
    }

    #[inline]
    pub fn to_syn_field(&self) -> Option<syn::Field> {
        match self {
            StructureItem::Field(Field { name, ty, doc, .. }) => Some(syn::Field {
                attrs: vec![],
                vis: pub_vis(),
                ident: Some(syn::Ident::new(&name, Span::call_site())),
                colon_token: Some(Default::default()),
                ty: ty.to_syn_ty(),
            }),
            StructureItem::List(List {
                name,
                ty,
                doc,
                cow_lifetime,
                ..
            }) => Some(syn::Field {
                attrs: vec![],
                vis: pub_vis(),
                ident: Some(syn::Ident::new(&name, Span::call_site())),
                colon_token: Some(Default::default()),
                ty: match ty {
                    MaybeString::IsAString => {
                        Type::Cow(Box::new(Type::Basic("str".into())), cow_lifetime.clone())
                            .to_syn_ty()
                    }
                    MaybeString::NotAString(ty) => Type::Cow(
                        Box::new(Type::Slice(Box::new(ty.clone()))),
                        cow_lifetime.clone(),
                    )
                    .to_syn_ty(),
                },
            }),
            _ => None,
        }
    }
}
