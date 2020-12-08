// MIT/Apache2 License

use super::{
    syn_util::{int_litexpr_int, pub_vis, str_to_pathseg},
    REnum, RStruct, ToSyn, Type,
};
use crate::{
    lvl1::{Import, Typedef},
    lvl2::{Bitflags, ConstItems, EnumRepr, Item as Lvl2Item, TrueEnum},
};
use heck::CamelCase;
use proc_macro2::Span;
use std::iter;

#[derive(Debug)]
pub enum Item {
    Import(Import),
    Typedef(Typedef),
    RStruct(RStruct),
    REnum(REnum),
    ConstItem {
        name: Box<str>,
        ty: Type,
        val: i128,
        is_xidtype: bool,
    },
}

impl ToSyn for Item {
    #[inline]
    fn to_syn_item(self) -> Vec<syn::Item> {
        match self {
            Self::Import(i) => i.to_syn_item(),
            Self::Typedef(t) => t.to_syn_item(),
            Self::RStruct(rs) => rs.to_syn_item(),
            Self::REnum(re) => re.to_syn_item(),
            Self::ConstItem {
                name,
                ty,
                val,
                is_xidtype,
            } => vec![syn::Item::Const(syn::ItemConst {
                attrs: vec![],
                vis: pub_vis(),
                const_token: Default::default(),
                ident: syn::Ident::new(&name, Span::call_site()),
                colon_token: Default::default(),
                ty: Box::new(ty.to_syn_ty()),
                eq_token: Default::default(),
                semi_token: Default::default(),
                expr: Box::new({
                    let literal = int_litexpr_int(val);
                    if is_xidtype {
                        syn::Expr::Call(syn::ExprCall {
                            attrs: vec![],
                            func: Box::new(syn::Expr::Path(syn::ExprPath {
                                attrs: vec![],
                                qself: Some(syn::QSelf {
                                    lt_token: Default::default(),
                                    ty: Box::new(ty.to_syn_ty()),
                                    position: 0,
                                    as_token: Default::default(),
                                    gt_token: Default::default(),
                                }),
                                path: syn::Path {
                                    leading_colon: Some(Default::default()),
                                    segments: iter::once(str_to_pathseg("const_from_xid"))
                                        .collect(),
                                },
                            })),
                            paren_token: Default::default(),
                            args: iter::once(literal).collect(),
                        })
                    } else {
                        literal
                    }
                }),
            })],
        }
    }
}

impl Item {
    #[inline]
    pub fn from_lvl2(lvl2: Lvl2Item, xids: &[Box<str>], ext_name: Option<&str>) -> Vec<Self> {
        match lvl2 {
            Lvl2Item::Import(i) => vec![Item::Import(i)],
            Lvl2Item::Typedef(t) => vec![Item::Typedef(t)],
            Lvl2Item::Struct(s) => {
                let (mut rs1, mut rs2) = RStruct::from_prev(s, ext_name);
                rs1.populate_asb();
                if let Some(ref mut rs2) = rs2 {
                    rs2.populate_asb();
                }
                iter::once(rs1)
                    .chain(rs2.into_iter())
                    .map(|rs| Item::RStruct(rs))
                    .collect()
            }
            Lvl2Item::Enum(EnumRepr::ConstItems(ConstItems { items, underlying })) => items
                .into_iter()
                .map(|(iname, val)| Self::ConstItem {
                    ty: Type::from_lvl2(underlying.clone()),
                    val,
                    is_xidtype: xids.iter().any(|xid| {
                        let x = xid.to_uppercase();
                        let i = iname.split('_').next().unwrap();
                        //                        log::warn!("Comparing {} against {}", &x, &i);
                        x == i
                    }),
                    name: iname.into_boxed_str(),
                })
                .collect(),
            Lvl2Item::Enum(EnumRepr::Bitflags(bitflags)) => super::bitflags_to_lvl3(bitflags)
                .into_iter()
                .map(|r| Item::RStruct(r))
                .collect(),
            Lvl2Item::Enum(EnumRepr::TrueEnum(te)) => vec![Item::REnum(REnum::from(te))],
            Lvl2Item::XidType(xty) => vec![Item::RStruct(RStruct::from(xty))],
            _ => vec![],
        }
    }
}
