// MIT/Apache2 License

use super::{
    syn_util::{derive_attrs, pub_vis, repr_transparent, str_to_exprpath, str_to_ty},
    Asb, Method, SizeSumPart, Statement, SumOfSizes, SumStatement, ToSyn, Trait, Type,
};
use crate::lvl2::{
    Expression, Field, List, MaybeString, Struct as Lvl2Struct, StructSpecial, StructureItem,
    Type as Lvl2Type, UseCondition,
};
use proc_macro2::Span;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    iter,
    ops::Deref,
    rc::Rc,
};
use tinyvec::ArrayVec;

/// Rust structure.
#[derive(Debug)]
pub struct RStruct {
    pub name: Box<str>,
    pub derives: Vec<&'static str>,
    pub is_transparent: bool,
    pub fields: Vec<StructureItem>,
    pub methods: Vec<Method>,
    pub traits: Vec<Trait>,
    pub fds: Vec<String>,
    pub asb: Asb,
}

#[inline]
fn cond_vars(
    condition: &Option<Rc<UseCondition>>,
    conds: &mut HashMap<Rc<Expression>, Box<str>>,
    last_cond_index: &mut usize,
    use_self: bool,
) -> (Option<(Rc<UseCondition>, Box<str>)>, Option<SumStatement>) {
    match condition {
        None => (None, None),
        Some(condition) => {
            if let Some(condname) = conds.get(&condition.expr) {
                (Some((condition.clone(), condname.clone())), None)
            } else {
                let condname = format!("cond{}", *last_cond_index).into_boxed_str();
                *last_cond_index += 1;
                conds.insert(condition.expr.clone(), condname.clone());
                (
                    Some((condition.clone(), condname.clone())),
                    Some(
                        super::InitializeCondition {
                            name: condname,
                            expression: condition.expr.clone(),
                            has_self: use_self,
                        }
                        .into(),
                    ),
                )
            }
        }
    }
}

impl RStruct {
    /// Populate the ASB structure based on field information.
    #[inline]
    pub fn populate_asb(&mut self) {
        self.populate_as_bytes();
        self.populate_from_bytes();

        // populate size
        self.asb.size = SumOfSizes(
            self.fields
                .iter()
                .map(|f| match f {
                    StructureItem::Field(Field { name, .. }) => {
                        SizeSumPart::SizeofField(name.clone().into_boxed_str())
                    }
                    StructureItem::Padding { bytes } => SizeSumPart::Bytes(*bytes),
                    StructureItem::List(List {
                        name, ty, padding, ..
                    }) => SizeSumPart::ListTimesSize(
                        name.clone().into_boxed_str(),
                        ty.clone(),
                        *padding,
                    ),
                    StructureItem::LenSlot { ty, .. } => {
                        SizeSumPart::SizeofType(Type::from_lvl2(ty.clone()))
                    }
                })
                .collect(),
        );
        self.asb.fd_getting = self.fds.get(0).map(|s| s.to_owned());
    }

    /// Populate the as_bytes statements.
    #[inline]
    pub fn populate_as_bytes(&mut self) {
        let mut last_cond_index: usize = 0;
        let mut conds: HashMap<Rc<Expression>, Box<str>> = HashMap::new();
        let stmts = iter::once(super::CreateIndexVariable.into())
            .chain(self.fields.iter().flat_map(|f| match f {
                StructureItem::Field(Field {
                    name, condition, ..
                }) => {
                    let (cond_pass, cond_init) =
                        cond_vars(condition, &mut conds, &mut last_cond_index, true);

                    cond_init
                        .into_iter()
                        .chain(iter::once(
                            super::AppendToIndexStatement {
                                name: name.clone().into_boxed_str(),
                                condition: cond_pass,
                            }
                            .into(),
                        ))
                        .collect()
                }
                StructureItem::Padding { bytes } => {
                    vec![super::PadIndexStatement(*bytes).into()]
                }
                StructureItem::LenSlot { owning_list, ty } => {
                    vec![super::AppendLengthToIndex {
                        owner: owning_list.clone().into_boxed_str(),
                        ty: Type::from_lvl2(ty.clone()),
                    }
                    .into()]
                }

                StructureItem::List(List {
                    name, ty, padding, ..
                }) => vec![super::AsBytesList {
                            name: name.clone().into_boxed_str(),
                            ty: ty.clone(),
                            pad: *padding,
                        }
                        .into()],
            }))
            .chain(iter::once(super::ReturnIndexStatement.into()))
            .collect();
        self.asb.as_bytes_stmts = stmts;
    }

    /// Populate the from_bytes statements.
    #[inline]
    pub fn populate_from_bytes(&mut self) {
        let mut len_map = HashMap::<String, Box<str>>::with_capacity(self.fields.len());
        let mut i: u32 = 0;
        let mut cond_map = HashMap::<Rc<Expression>, Box<str>>::new();
        let mut last_cond_index: usize = 0;

        let stmts = vec![
            super::CreateIndexVariable.into(),
            super::DeserTraceMarker(self.name.clone()).into(),
        ]
        .into_iter()
        .chain(self.fields.iter().flat_map(|f| match f {
            StructureItem::Field(Field {
                name,
                ty,
                condition,
                ..
            }) => {
                let (cond_pass, cond_init) =
                    cond_vars(condition, &mut cond_map, &mut last_cond_index, false);

                cond_init
                    .into_iter()
                    .chain(iter::once(
                        super::LoadStatementVariable {
                            name: name.clone().into(),
                            ty: Type::from_lvl2(ty.clone()),
                            use_slice: true,
                            condition: cond_pass,
                        }
                        .into(),
                    ))
                    .collect()
            }
            StructureItem::Padding { bytes } => {
                vec![super::IncrementIndex::Number(*bytes).into()]
            }
            StructureItem::LenSlot { ty, owning_list } => {
                // create a random name
                let len_name = format!("len{}", i);
                i += 1;

                len_map.insert(owning_list.clone(), len_name.clone().into_boxed_str());

                vec![super::LoadStatementVariable {
                    name: len_name.into(),
                    ty: Type::from_lvl2(ty.clone()),
                    use_slice: true,
                    condition: None,
                }
                .into()]
            }
            StructureItem::List(List {
                name,
                ty,
                list_length,
                padding,
                ..
            }) => {
                // if the list length is a single item, get that length slot
                let length_expr = if let Some(_) = list_length.single_item() {
                    match len_map.get(name) {
                        Some(name) => str_to_exprpath(name),
                        None => list_length.to_length_expr(false, true),
                    }
                } else {
                    // just get the length expr
                    list_length.to_length_expr(false, true)
                };

                vec![super::FromBytesList {
                    name: name.clone().into_boxed_str(),
                    ty: ty.clone(),
                    len: length_expr,
                    pad: padding.clone(),
                }
                .into()]
            }
        }))
        .chain(iter::once(
            super::ReturnStruct {
                last_index: "index",
                sname: self.name.clone(),
                fields: self
                    .fields
                    .iter()
                    .filter_map(|f| match f {
                        StructureItem::Field(Field { name, .. }) => Some(name.clone().into()),
                        StructureItem::List(List { name, .. }) => Some(name.clone().into()),
                        _ => None,
                    })
                    .collect(),
            }
            .into(),
        ))
        .collect();
        self.asb.from_bytes_stmts = stmts;
    }
}

impl ToSyn for RStruct {
    #[inline]
    fn to_syn_item(self) -> Vec<syn::Item> {
        let s = syn::Item::Struct(syn::ItemStruct {
            attrs: (match self.is_transparent {
                false => None,
                true => Some(repr_transparent()),
            })
            .into_iter()
            .chain(match self.derives.len() {
                0 => None,
                _ => Some(derive_attrs(&self.derives)),
            })
            .collect(),
            vis: pub_vis(),
            struct_token: Default::default(),
            ident: syn::Ident::new(&self.name, Span::call_site()),
            generics: Default::default(),
            fields: syn::FieldsNamed {
                brace_token: Default::default(),
                named: self
                    .fields
                    .iter()
                    .filter_map(|f| f.to_syn_field())
                    .chain(self.fds.iter().map(|fd| syn::Field {
                        attrs: vec![],
                        vis: pub_vis(),
                        ident: Some(syn::Ident::new(fd, Span::call_site())),
                        colon_token: Some(Default::default()),
                        ty: Type::Vector(Box::new(Type::Basic("Fd".into()))).to_syn_ty(),
                    }))
                    .collect(),
            }
            .into(),
            semi_token: None,
        });

        let methods = syn::Item::Impl(syn::ItemImpl {
            attrs: vec![],
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: Default::default(),
            trait_: None,
            self_ty: Box::new(str_to_ty(&self.name)),
            brace_token: Default::default(),
            items: self
                .methods
                .iter()
                .map(|m| m.to_syn_impl_item(false))
                .collect(),
        });

        let Self {
            name, asb, traits, ..
        } = self;
        let asb = asb.to_syn_item(&name);
        let mut s = vec![s, methods];
        s.extend(asb);
        s.extend(traits.into_iter().flat_map(|t| t.to_syn_item(&name)));
        s
    }
}

// recursive entry point
#[inline]
fn from_lvl2(s: Lvl2Struct, is_reply: bool, ext_name: Option<&str>) -> (RStruct, Option<RStruct>) {
    // disassemble the structure
    let Lvl2Struct {
        mut name,
        brief,
        desc,
        fields,
        fds,
        special,
    } = s;
    let mut traits = vec![];

    // special-dependent stuff
    let other: Option<RStruct> = if is_reply {
        None
    } else {
        match special {
            StructSpecial::Regular => None,
            StructSpecial::Event(opcode) => {
                traits.push(Trait::Event(opcode));
                name = format!("{}Event", name).into_boxed_str();
                None
            }
            StructSpecial::Error(opcode) => {
                traits.push(Trait::Error(opcode));
                name = format!("{}Error", name).into_boxed_str();
                None
            }
            StructSpecial::Request(opcode, reply) => {
                let reply_name = format!("{}Reply", &name);
                traits.push(Trait::Request(
                    opcode,
                    match reply {
                        Some(ref reply) => Type::Basic(reply_name.clone().into()),
                        None => Type::Tuple(vec![]),
                    },
                    ext_name.map(|s| s.to_string()),
                    match reply {
                        Some(ref reply) => !reply.fds.is_empty(),
                        None => false,
                    },
                ));
                name = format!("{}Request", name).into_boxed_str();
                match reply {
                    Some(mut reply) => {
                        reply.name = reply_name.into_boxed_str();
                        let (reply, _) = from_lvl2(*reply, true, ext_name);
                        Some(reply)
                    }
                    None => None,
                }
            }
        }
    };

    let res = RStruct {
        name,
        derives: vec!["Clone", "Debug", "Default"],
        is_transparent: fields.len() == 1,
        fields,
        fds,
        methods: vec![],
        traits,
        asb: Default::default(),
    };

    (res, other)
}

// From a level 2 struct
// Note: this potentially produces 2 items
impl RStruct {
    #[inline]
    pub fn from_prev(s: Lvl2Struct, ext_name: Option<&str>) -> (Self, Option<Self>) {
        from_lvl2(s, false, ext_name)
    }
}
