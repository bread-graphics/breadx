// MIT/Apache2 License

use super::{safe_name, Expression, Type};
use crate::lvl1::StructureItem as Lvl1StructureItem;
use heck::{CamelCase, SnakeCase};
use std::{borrow::Cow, rc::Rc};
use tinyvec::TinyVec;

/// Check variant or ==
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConditionVariant {
    BitflagVariant,
    Equal,
}

/// A condition to use to tell whether or not to serialize or deserialize a certain
/// value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UseCondition {
    pub expr: Rc<Expression>,
    pub variant: ConditionVariant,
    pub enum_name: String,
    pub enum_value: String,
}

/// An ordinary data field.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub doc: Option<String>,
    pub condition: Option<Rc<UseCondition>>,
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
    pub list_length: Expression,
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
                        condition: None,
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
                    let list_length: Expression = list_length.into();
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
                        list_length: Expression::one_count(mask_name),
                        doc: None,
                        padding: None,
                    }),
                ])
            }
            Lvl1StructureItem::Switch(crate::lvl1::Switch { name, cases, expr }) => {
                let expr: Rc<Expression> = Rc::new(expr.into());
                cases
                    .into_iter()
                    .flat_map(|c| {
                        // create an item for the condition
                        let crate::lvl1::Case {
                            is_bitcase,
                            enum_ref,
                            enum_item,
                            fields,
                        } = c;
                        let cond = Rc::new(UseCondition {
                            expr: expr.clone(),
                            variant: if is_bitcase {
                                ConditionVariant::BitflagVariant
                            } else {
                                ConditionVariant::Equal
                            },
                            enum_name: enum_ref,
                            enum_value: enum_item,
                        });

                        fields.into_iter().flat_map(move |f| {
                            let cond = cond.clone();
                            StructureItem::from_lvl1(f, &mut Default::default())
                                .into_iter()
                                .map(move |mut f| {
                                    if let StructureItem::Field(Field {
                                        ref mut condition, ..
                                    }) = f
                                    {
                                        *condition = Some(cond.clone());
                                    }

                                    f
                                })
                        })
                    })
                    .collect()
            }
        }
    }
}
