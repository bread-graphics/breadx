// MIT/Apache2 License

use super::{
    syn_util::{item_field, pub_vis, str_to_path, str_to_ty},
    Asb, InputParameter, Method, ParameterUsage, RStruct, SizeSumPart, SumOfSizes, Trait, Type,
};
use crate::lvl2::{Bitflags, Field, StructureItem, Type as Lvl2Type};
use heck::ShoutySnakeCase;
use proc_macro2::Span;
use std::{iter, rc::Rc};

/// Convert a bitflags conversion to a rust struct.
pub fn bitflags_to_lvl3(bitflags: Bitflags) -> Vec<RStruct> {
    let Bitflags {
        name,
        underlying,
        bits,
    } = bitflags;

    let mut rstruct = RStruct {
        name: name.clone().into_boxed_str(),
        derives: vec![
            "Copy",
            "Clone",
            "Debug",
            "Default",
            "PartialEq",
            "Eq",
            "PartialOrd",
            "Ord",
        ],
        fds: vec![],
        is_transparent: true,
        fields: vec![StructureItem::Field(Field {
            name: "inner".to_string(),
            ty: underlying.clone(),
            ..Default::default()
        })],
        methods: Vec::with_capacity(bits.len() * 2),
        other_impl_items: Vec::with_capacity(bits.len() + 1),
        traits: vec![
            Trait::BitflagsNot(name.clone().into_boxed_str()),
            Trait::BitflagsAnd(name.clone().into_boxed_str()),
            Trait::BitflagsOr(name.clone().into_boxed_str()),
            Trait::BitflagsXor(name.clone().into_boxed_str()),
        ],
        asb: Default::default(),
    };

    // iterate over the bits
    bits.iter().for_each(|(bitname, bitval)| {
        // getter
        rstruct.methods.push({
            let mut method = Method::new(
                bitname.to_string().into(),
                Some(ParameterUsage::Ref),
                vec![],
                Some(Type::Basic("bool".into())),
            );

            method
                .statements
                .push(super::ExtractBit(*bitval as _).into());
            method
        });

        // setter
        rstruct.methods.push({
            let mut method = Method::new(
                format!("set_{}", &bitname).into(),
                Some(ParameterUsage::MutRef),
                vec![InputParameter {
                    name: "val".into(),
                    ty: Type::Basic("bool".into()),
                    usage: ParameterUsage::Owned,
                }],
                Some(Type::Ref(Box::new(Type::Basic("Self".into())), true, None)),
            );

            method.statements.extend(vec![
                super::InsertBit {
                    bit: *bitval as _,
                    is_self: true,
                    val: "val".into(),
                }
                .into(),
                super::JustReturnSelf.into(),
            ]);
            method
        });

        // const
        rstruct.other_impl_items.push(bitflags_const_field(
            &bitname.to_shouty_snake_case(),
            1u64 << (*bitval as u64),
        ));
    });

    // also have a new() method with each
    let mut new_method = Method::new(
        "new".into(),
        None,
        bits.iter()
            .map(|(bitname, bitval)| {
                InputParameter {
                    name: bitname.to_string().into(),
                    ty: Type::Basic("bool".into()),
                    usage: ParameterUsage::Owned,
                }
                .into()
            })
            .collect(),
        Some(Type::Basic("Self".into())),
    );
    let underlying = Type::from_lvl2(underlying);
    new_method.statements.extend(
        iter::once(super::DefineInnerAccumulator(underlying.clone()).into()).chain(
            bits.iter()
                .map(|(bitname, bitval)| {
                    super::InsertBit {
                        bit: *bitval as _,
                        is_self: false,
                        val: bitname.clone().into_boxed_str(),
                    }
                    .into()
                })
                .chain(iter::once(
                    super::ReturnBitflag(name.clone().into_boxed_str()).into(),
                )),
        ),
    );
    rstruct.methods.push(new_method);

    // also have a count_ones() method
    let mut count_ones = Method::new(
        "count_ones".into(),
        Some(ParameterUsage::Ref),
        vec![].into_iter().collect(),
        Some(Type::Basic("usize".into())),
    );
    count_ones.statements.push(
        super::ForwardToInner(Rc::new(|i| {
            syn::Expr::Cast(syn::ExprCast {
                attrs: vec![],
                expr: Box::new(syn::Expr::Call(syn::ExprCall {
                    attrs: vec![],
                    func: Box::new(item_field(i, "count_ones")),
                    paren_token: Default::default(),
                    args: syn::punctuated::Punctuated::new(),
                })),
                as_token: Default::default(),
                ty: Box::new(str_to_ty("usize")),
            })
        }))
        .into(),
    );
    rstruct.methods.push(count_ones);

    // also have an "all" const
    rstruct
        .other_impl_items
        .push(bitflags_const_field("COMPLETE", {
            bits.iter()
                .fold(0u64, |d, (_, bitval)| d | (1 << *bitval as u64))
        }));

    // asb should forward to inner
    rstruct.asb.size = SumOfSizes(vec![SizeSumPart::SizeofField("inner".into())]);
    rstruct
        .asb
        .as_bytes_stmts
        .push(super::ForwardAsBytes("inner".into()).into());
    rstruct.asb.from_bytes_stmts.push(
        super::ForwardFromBytes {
            inner_ty: underlying.clone(),
            self_name: name.into_boxed_str(),
        }
        .into(),
    );

    vec![rstruct]
}

#[inline]
fn bitflags_const_field(name: &str, val: u64) -> syn::ImplItem {
    syn::ImplItem::Const(syn::ImplItemConst {
        attrs: vec![],
        vis: pub_vis(),
        defaultness: None,
        const_token: Default::default(),
        ident: syn::Ident::new(name, Span::call_site()),
        colon_token: Default::default(),
        ty: str_to_ty("Self"),
        eq_token: Default::default(),
        expr: syn::Expr::Struct(syn::ExprStruct {
            attrs: vec![],
            path: str_to_path("Self"),
            brace_token: Default::default(),
            fields: iter::once(syn::FieldValue {
                attrs: vec![],
                member: syn::Member::Named(syn::Ident::new("inner", Span::call_site())),
                colon_token: Some(Default::default()),
                expr: syn::Expr::Lit(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Int(syn::LitInt::new(&format!("{}", val), Span::call_site())),
                }),
            })
            .collect(),
            dot2_token: None,
            rest: None,
        }),
        semi_token: Default::default(),
    })
}
