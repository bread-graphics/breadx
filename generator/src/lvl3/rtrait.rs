// MIT/Apache2 License

use super::{
    get_mapped_lifetime,
    syn_util::{
        int_litexpr_int, item_field, str_to_exprpath, str_to_path, str_to_pathseg, str_to_ty,
    },
    InputParameter, Method, ParameterUsage, ToSyn, Type,
};
use proc_macro2::Span;
use std::{borrow::Cow, iter, ops::Deref, rc::Rc};

#[derive(Debug)]
pub struct Trait {
    pub specifics: TraitSpecifics,
    pub lifetimes: Vec<String>,
}

#[derive(Debug)]
pub enum TraitSpecifics {
    Event(u64),
    Error(u64),
    Request(u64, Type, Option<String>, bool),
    Xid,
    EnumDefault(Box<str>),
    FromXid(Box<str>),
    BitflagsNot(Box<str>),
    BitflagsAnd(Box<str>),
    BitflagsOr(Box<str>),
    BitflagsXor(Box<str>),
}

impl From<TraitSpecifics> for Trait {
    #[inline]
    fn from(ts: TraitSpecifics) -> Trait {
        Trait {
            specifics: ts,
            lifetimes: vec![],
        }
    }
}

#[inline]
fn opcode_const(op: u64) -> syn::ImplItem {
    syn::ImplItem::Const(syn::ImplItemConst {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        defaultness: None,
        const_token: Default::default(),
        ident: syn::Ident::new("OPCODE", Span::call_site()),
        colon_token: Default::default(),
        ty: Type::Basic("u8".into()).to_syn_ty(),
        eq_token: Default::default(),
        expr: int_litexpr_int(op),
        semi_token: Default::default(),
    })
}

#[inline]
fn ref_const(val: bool) -> syn::ImplItem {
    syn::ImplItem::Const(syn::ImplItemConst {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        defaultness: None,
        const_token: Default::default(),
        ident: syn::Ident::new("REPLY_EXPECTS_FDS", Span::call_site()),
        colon_token: Default::default(),
        ty: Type::Basic("bool".into()).to_syn_ty(),
        eq_token: Default::default(),
        expr: syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Bool(syn::LitBool {
                value: val,
                span: Span::call_site(),
            }),
        }),
        semi_token: Default::default(),
    })
}

#[inline]
fn extension_const(ext: Option<&str>) -> syn::ImplItem {
    syn::ImplItem::Const(syn::ImplItemConst {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        defaultness: None,
        const_token: Default::default(),
        ident: syn::Ident::new("EXTENSION", Span::call_site()),
        colon_token: Default::default(),
        ty: Type::Opt(Box::new(Type::Ref(
            Box::new(Type::Basic("str".into())),
            false,
            Some("'static"),
        )))
        .to_syn_ty(),
        eq_token: Default::default(),
        expr: match ext {
            None => str_to_exprpath("None"),
            Some(ext) => syn::Expr::Call(syn::ExprCall {
                attrs: vec![],
                func: Box::new(str_to_exprpath("Some")),
                paren_token: Default::default(),
                args: iter::once(syn::Expr::Lit(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Str(syn::LitStr::new(ext, Span::call_site())),
                }))
                .collect(),
            }),
        },
        semi_token: Default::default(),
    })
}

#[inline]
fn bitflags_output_ty(ty: &str) -> syn::ImplItem {
    syn::ImplItem::Type(syn::ImplItemType {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        defaultness: None,
        type_token: Default::default(),
        ident: syn::Ident::new("Output", Span::call_site()),
        generics: Default::default(),
        eq_token: Default::default(),
        semi_token: Default::default(),
        ty: str_to_ty(ty),
    })
}

impl Trait {
    #[inline]
    pub fn to_syn_item(self, tyname: &str) -> Vec<syn::Item> {
        vec![syn::Item::Impl(syn::ItemImpl {
            attrs: vec![],
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: match self.lifetimes.len() {
                0 => Default::default(),
                _ => syn::Generics {
                    lt_token: Some(Default::default()),
                    params: self
                        .lifetimes
                        .iter()
                        .map(|lifetime| {
                            syn::GenericParam::Lifetime(syn::LifetimeDef {
                                attrs: vec![],
                                lifetime: syn::Lifetime {
                                    apostrophe: Span::call_site(),
                                    ident: syn::Ident::new(lifetime, Span::call_site()),
                                },
                                colon_token: None,
                                bounds: syn::punctuated::Punctuated::new(),
                            })
                        })
                        .collect(),
                    gt_token: Some(Default::default()),
                    where_clause: None,
                },
            },
            trait_: Some((
                None,
                match &self.specifics {
                    TraitSpecifics::Event(_) => syn::Path {
                        leading_colon: None,
                        segments: vec![
                            str_to_pathseg("crate"),
                            str_to_pathseg("auto"),
                            str_to_pathseg("Event"),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    TraitSpecifics::Error(_) => syn::Path {
                        leading_colon: None,
                        segments: vec![
                            str_to_pathseg("crate"),
                            str_to_pathseg("auto"),
                            str_to_pathseg("Error"),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    TraitSpecifics::Request(_, _, _, _) => str_to_path("Request"),
                    TraitSpecifics::Xid => str_to_path("XidType"),
                    TraitSpecifics::EnumDefault(_) => str_to_path("Default"),
                    TraitSpecifics::FromXid(ref from) => syn::Path {
                        leading_colon: None,
                        segments: vec![syn::PathSegment {
                            ident: syn::Ident::new("From", Span::call_site()),
                            arguments: syn::PathArguments::AngleBracketed(
                                syn::AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Default::default(),
                                    args: iter::once(syn::GenericArgument::Type(
                                        Type::from_name(from.to_string()).to_syn_ty(),
                                    ))
                                    .collect(),
                                    gt_token: Default::default(),
                                },
                            ),
                        }]
                        .into_iter()
                        .collect(),
                    },
                    TraitSpecifics::BitflagsNot(_) => syn::Path {
                        leading_colon: None,
                        segments: vec![
                            str_to_pathseg("core"),
                            str_to_pathseg("ops"),
                            str_to_pathseg("Not"),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    TraitSpecifics::BitflagsAnd(_) => syn::Path {
                        leading_colon: None,
                        segments: vec![
                            str_to_pathseg("core"),
                            str_to_pathseg("ops"),
                            str_to_pathseg("BitAnd"),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    TraitSpecifics::BitflagsOr(_) => syn::Path {
                        leading_colon: None,
                        segments: vec![
                            str_to_pathseg("core"),
                            str_to_pathseg("ops"),
                            str_to_pathseg("BitOr"),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    TraitSpecifics::BitflagsXor(_) => syn::Path {
                        leading_colon: None,
                        segments: vec![
                            str_to_pathseg("core"),
                            str_to_pathseg("ops"),
                            str_to_pathseg("BitXor"),
                        ]
                        .into_iter()
                        .collect(),
                    },
                },
                Default::default(),
            )),
            self_ty: Box::new(syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: iter::once(syn::PathSegment {
                        ident: syn::Ident::new(tyname, Span::call_site()),
                        arguments: match self.lifetimes.len() {
                            0 => syn::PathArguments::None,
                            _ => syn::PathArguments::AngleBracketed(
                                syn::AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Default::default(),
                                    args: self
                                        .lifetimes
                                        .iter()
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
                        },
                    })
                    .collect(),
                },
            })),
            brace_token: Default::default(),
            items: match &self.specifics {
                TraitSpecifics::Event(opcode) => vec![opcode_const(*opcode)],
                TraitSpecifics::Error(opcode) => vec![opcode_const(*opcode)],
                TraitSpecifics::Request(opcode, reply_name, ext_name, expects_fds) => vec![
                    opcode_const(*opcode),
                    extension_const(ext_name.as_deref()),
                    ref_const(*expects_fds),
                    syn::ImplItem::Type(syn::ImplItemType {
                        attrs: vec![],
                        vis: syn::Visibility::Inherited,
                        defaultness: None,
                        type_token: Default::default(),
                        ident: syn::Ident::new("Reply", Span::call_site()),
                        generics: Default::default(),
                        eq_token: Default::default(),
                        ty: match reply_name.clone() {
                            Type::Basic(name) => Type::HasLifetime(
                                name.clone(),
                                iter::repeat("static".to_string())
                                    .take(get_mapped_lifetime(&name))
                                    .collect(),
                            ),
                            ty => ty,
                        }
                        .to_syn_ty(),
                        semi_token: Default::default(),
                    }),
                ],
                TraitSpecifics::Xid => vec![
                    {
                        let mut method = Method::new(
                            "xid".into(),
                            Some(ParameterUsage::Ref),
                            vec![],
                            Some(Type::Basic("XID".into())),
                        );
                        method.statements = vec![super::GetXidStatement.into()];
                        method.to_syn_impl_item(true)
                    },
                    {
                        let mut method = Method::new(
                            "from_xid".into(),
                            None,
                            vec![InputParameter {
                                name: "xid".into(),
                                ty: Type::Basic("XID".into()),
                                usage: ParameterUsage::Owned,
                            }],
                            Some(Type::Basic("Self".into())),
                        );
                        method.statements = vec![super::CreateXidTypeStatement.into()];
                        method.to_syn_impl_item(true)
                    },
                ],
                TraitSpecifics::EnumDefault(variant) => vec![{
                    let mut method = Method::new(
                        "default".into(),
                        None,
                        vec![],
                        Some(Type::from_name(tyname.to_string().into())),
                    );
                    method.statements = vec![super::ReturnEnumVariant(
                        tyname.to_string().into_boxed_str(),
                        variant.clone(),
                    )
                    .into()];
                    method.to_syn_impl_item(true)
                }],
                TraitSpecifics::FromXid(from) => vec![{
                    let mut method = Method::new(
                        "from".into(),
                        None,
                        vec![InputParameter {
                            name: "base".into(),
                            ty: Type::from_name(from.deref().to_string().into()),
                            usage: ParameterUsage::Owned,
                        }],
                        Some(Type::Basic("Self".into())),
                    );
                    method.statements = vec![super::ConvertXids {
                        oldname: from.clone(),
                        newname: tyname.to_string().into_boxed_str(),
                    }
                    .into()];
                    method.to_syn_impl_item(true)
                }],
                TraitSpecifics::BitflagsNot(ty) => vec![bitflags_output_ty(&ty), {
                    let mut method = Method::new(
                        "not".into(),
                        Some(ParameterUsage::Owned),
                        vec![],
                        Some(Type::Basic(ty.to_string().into())),
                    );
                    let ty = ty.clone();
                    method.statements = vec![super::ForwardToInner(Rc::new(move |i| {
                        syn::Expr::Struct(syn::ExprStruct {
                            attrs: vec![],
                            path: str_to_path(&ty),
                            brace_token: Default::default(),
                            fields: iter::once(syn::FieldValue {
                                attrs: vec![],
                                member: syn::Member::Named(syn::Ident::new(
                                    "inner",
                                    Span::call_site(),
                                )),
                                colon_token: Some(Default::default()),
                                expr: syn::Expr::Unary(syn::ExprUnary {
                                    attrs: vec![],
                                    op: syn::UnOp::Not(Default::default()),
                                    expr: Box::new(i),
                                }),
                            })
                            .collect(),
                            dot2_token: None,
                            rest: None,
                        })
                    }))
                    .into()];
                    method.to_syn_impl_item(true)
                }],

                TraitSpecifics::BitflagsAnd(ty) => vec![bitflags_output_ty(&ty), {
                    let mut method = Method::new(
                        "bitand".into(),
                        Some(ParameterUsage::Owned),
                        vec![InputParameter {
                            name: "rhs".into(),
                            ty: Type::Basic(ty.to_string().into()),
                            usage: ParameterUsage::Owned,
                        }],
                        Some(Type::Basic(ty.to_string().into())),
                    );
                    let ty = ty.clone();
                    method.statements = vec![super::ForwardToInner(Rc::new(move |i| {
                        syn::Expr::Struct(syn::ExprStruct {
                            attrs: vec![],
                            path: str_to_path(&ty),
                            brace_token: Default::default(),
                            fields: iter::once(syn::FieldValue {
                                attrs: vec![],
                                member: syn::Member::Named(syn::Ident::new(
                                    "inner",
                                    Span::call_site(),
                                )),
                                colon_token: Some(Default::default()),
                                expr: syn::Expr::Binary(syn::ExprBinary {
                                    attrs: vec![],
                                    op: syn::BinOp::BitAnd(Default::default()),
                                    left: Box::new(i),
                                    right: Box::new(item_field(str_to_exprpath("rhs"), "inner")),
                                }),
                            })
                            .collect(),
                            dot2_token: None,
                            rest: None,
                        })
                    }))
                    .into()];
                    method.to_syn_impl_item(true)
                }],
                TraitSpecifics::BitflagsOr(ty) => vec![bitflags_output_ty(&ty), {
                    let mut method = Method::new(
                        "bitor".into(),
                        Some(ParameterUsage::Owned),
                        vec![InputParameter {
                            name: "rhs".into(),
                            ty: Type::Basic(ty.to_string().into()),
                            usage: ParameterUsage::Owned,
                        }],
                        Some(Type::Basic(ty.to_string().into())),
                    );
                    let ty = ty.clone();
                    method.statements = vec![super::ForwardToInner(Rc::new(move |i| {
                        syn::Expr::Struct(syn::ExprStruct {
                            attrs: vec![],
                            path: str_to_path(&ty),
                            brace_token: Default::default(),
                            fields: iter::once(syn::FieldValue {
                                attrs: vec![],
                                member: syn::Member::Named(syn::Ident::new(
                                    "inner",
                                    Span::call_site(),
                                )),
                                colon_token: Some(Default::default()),
                                expr: syn::Expr::Binary(syn::ExprBinary {
                                    attrs: vec![],
                                    op: syn::BinOp::BitOr(Default::default()),
                                    left: Box::new(i),
                                    right: Box::new(item_field(str_to_exprpath("rhs"), "inner")),
                                }),
                            })
                            .collect(),
                            dot2_token: None,
                            rest: None,
                        })
                    }))
                    .into()];
                    method.to_syn_impl_item(true)
                }],
                TraitSpecifics::BitflagsXor(ty) => vec![bitflags_output_ty(&ty), {
                    let mut method = Method::new(
                        "bitxor".into(),
                        Some(ParameterUsage::Owned),
                        vec![InputParameter {
                            name: "rhs".into(),
                            ty: Type::Basic(ty.to_string().into()),
                            usage: ParameterUsage::Owned,
                        }],
                        Some(Type::Basic(ty.to_string().into())),
                    );
                    method.statements = {
                        let ty = ty.clone();
                        vec![super::ForwardToInner(Rc::new(move |i| {
                            syn::Expr::Struct(syn::ExprStruct {
                                attrs: vec![],
                                path: str_to_path(&ty),
                                brace_token: Default::default(),
                                fields: iter::once(syn::FieldValue {
                                    attrs: vec![],
                                    member: syn::Member::Named(syn::Ident::new(
                                        "inner",
                                        Span::call_site(),
                                    )),
                                    colon_token: Some(Default::default()),
                                    expr: syn::Expr::Binary(syn::ExprBinary {
                                        attrs: vec![],
                                        op: syn::BinOp::BitXor(Default::default()),
                                        left: Box::new(i),
                                        right: Box::new(item_field(
                                            str_to_exprpath("rhs"),
                                            "inner",
                                        )),
                                    }),
                                })
                                .collect(),
                                dot2_token: None,
                                rest: None,
                            })
                        }))
                        .into()]
                    };
                    method.to_syn_impl_item(true)
                }],
            },
        })]
    }
}
