// MIT/Apache2 License

use super::{safe_name, ListLength, Type};
use crate::lvl1::StructureItem as Lvl1StructureItem;
use heck::{CamelCase, SnakeCase};
use std::borrow::Cow;
use tinyvec::TinyVec;

/// An ordinary data field.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub doc: Option<String>,
}

/// The list might be a string in disguise!
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaybeString {
    NotAString(Type),
    IsAString,
}

/// A data field that is a list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List {
    pub name: String,
    pub ty: MaybeString,
    pub doc: Option<String>,
    pub list_length: ListLength,
    pub padding: Option<usize>,
}

/// An item in a structure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructureItem {
    Field(Field),
    Padding { bytes: usize },
    List(List),
    LenSlot { ty: Type, owning_list: String },
}

impl Default for StructureItem {
    #[inline]
    fn default() -> Self {
        Self::Padding { bytes: 0 }
    }
}

impl StructureItem {
    /// Raise a level 1 structure item to a level 2 structure item. If the type is of an enum it also provides
    /// an enum resolution.
    #[inline]
    pub fn from_lvl1(
        lvl1: Lvl1StructureItem,
        resolution: &mut Option<(String, String)>,
    ) -> TinyVec<[Self; 1]> {
        match lvl1 {
            Lvl1StructureItem::Field(f) => {
                TinyVec::from([StructureItem::Field({
                    let crate::lvl1::Field {
                        ty,
                        name,
                        mask,
                        enumeration,
                        alt_enum,
                    } = f;

                    // if mask or enum is set, that's a setting resolution
                    let ty = match (mask, enumeration) {
                        (Some(e), _) | (_, Some(e)) => {
                            *resolution = Some((e.clone(), ty.to_camel_case()));
                            e
                        }
                        _ => ty,
                    };

                    let ty = ty.to_camel_case();
                    let name = safe_name(name.to_snake_case());

                    // is alt_enum is set, it's a resolution that doesn't change our type
                    if let Some(alt_enum) = alt_enum {
                        *resolution = Some((alt_enum, ty.clone()));
                    }

                    Field {
                        name,
                        ty: Type::BasicType(ty.into()),
                        doc: None,
                    }
                })])
            }
            Lvl1StructureItem::Padding { bytes, .. } => TinyVec::from([Self::Padding { bytes }]),
            Lvl1StructureItem::List(l) => {
                TinyVec::from([StructureItem::List({
                    let crate::lvl1::List {
                        ty,
                        name,
                        list_length,
                    } = l;

                    // conver the list length to a postfix-oriented version rather than a linked list version
                    let list_length: ListLength = list_length.into();
                    let name = safe_name(name.clone());

                    List {
                        name,
                        ty: match ty.as_str() {
                            "char" => MaybeString::IsAString,
                            _ => MaybeString::NotAString(ty.to_camel_case().into()),
                        },
                        doc: None,
                        list_length,
                        padding: None,
                    }
                })])
            }
            Lvl1StructureItem::ValueParam(v) => {
                let crate::lvl1::ValueParam {
                    mask_ty,
                    mask_name,
                    list_name,
                } = v;

                TinyVec::Heap(vec![
                    StructureItem::Field(Field {
                        name: mask_name.clone(),
                        ty: mask_ty.to_camel_case().into(),
                        ..Default::default()
                    }),
                    StructureItem::List(List {
                        name: list_name,
                        ty: MaybeString::NotAString(Type::BasicType("u32".into())),
                        list_length: ListLength::one_count(mask_name),
                        doc: None,
                        padding: None,
                    }),
                ])
            }
        }
    }
}
