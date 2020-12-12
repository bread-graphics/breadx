// MIT/Apache2 License

use super::{
    syn_util::{int_litexpr_int, item_field, str_to_exprpath, str_to_path, str_to_pathseg},
    Type,
};
use crate::lvl2::{Expression, MaybeString, UseCondition};
use proc_macro2::{Span, TokenStream};
use std::{borrow::Cow, fmt, iter, ops::Deref, rc::Rc};

mod list;
pub use list::*;
mod padding;
pub use padding::*;

/// Translates into a syn statement.
pub trait Statement {
    /// Get the syn statement.
    fn to_syn_statement(&self) -> Vec<syn::Stmt>;
}

/// Create a let statement
#[inline]
pub fn let_statement(binding: &str, ty: Type, expr: syn::Expr, muta: bool) -> syn::Expr {
    syn::Expr::Let(syn::ExprLet {
        attrs: vec![],
        let_token: Default::default(),
        eq_token: Default::default(),
        pat: syn::Pat::Type(syn::PatType {
            attrs: vec![],
            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: if muta { Some(Default::default()) } else { None },
                ident: syn::Ident::new(binding, Span::call_site()),
                subpat: None,
            })),
            colon_token: Default::default(),
            ty: Box::new(ty.to_syn_ty()),
        }),
        expr: Box::new(expr),
    })
}

/// A statement to return "index".
#[derive(Copy, Clone, Debug)]
pub struct ReturnIndexStatement;

impl Statement for ReturnIndexStatement {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(str_to_exprpath("index"))]
    }
}

/// A statement to call `index += self.[0].as_bytes(&mut bytes[index..]);`
#[derive(Clone, Debug)]
pub struct AppendToIndexStatement {
    pub name: Box<str>,
    pub condition: Option<(Rc<UseCondition>, Box<str>)>,
}

impl Statement for AppendToIndexStatement {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        let inn = syn::Stmt::Semi(
            index_plus_equal(syn::Expr::Call(syn::ExprCall {
                attrs: vec![],
                func: Box::new(item_field(
                    item_field(str_to_exprpath("self"), &self.name),
                    "as_bytes",
                )),
                paren_token: Default::default(),
                args: iter::once(bytes_slice(true)).collect(),
            })),
            Default::default(),
        );
        vec![match self.condition {
            Some((ref condition, ref condname)) => syn::Stmt::Expr(syn::Expr::If(syn::ExprIf {
                attrs: vec![],
                if_token: Default::default(),
                cond: Box::new(condition.to_cond_expr(condname)),
                then_branch: syn::Block {
                    brace_token: Default::default(),
                    stmts: vec![inn],
                },
                else_branch: None,
            })),
            None => inn,
        }]
    }
}

/// Append a padding to "index" with a number of bytes.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct PadIndexStatement(pub usize);

impl Statement for PadIndexStatement {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Semi(
            index_plus_equal(int_litexpr_int(self.0)),
            Default::default(),
        )]
    }
}

#[inline]
fn index_plus_equal(e: syn::Expr) -> syn::Expr {
    syn::Expr::AssignOp(syn::ExprAssignOp {
        attrs: vec![],
        left: Box::new(str_to_exprpath("index")),
        op: syn::BinOp::AddEq(Default::default()),
        right: Box::new(e),
    })
}

/// A statement to create an index variable.
#[derive(Copy, Clone, Debug)]
pub struct CreateIndexVariable;

impl Statement for CreateIndexVariable {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Semi(
            let_statement(
                "index",
                Type::Basic("usize".into()),
                int_litexpr_int(0),
                true,
            ),
            Default::default(),
        )]
    }
}

/// A statement to initialize a condition variable.
#[derive(Debug, Clone)]
pub struct InitializeCondition {
    pub name: Box<str>,
    pub expression: Rc<Expression>,
    pub has_self: bool,
}

impl Statement for InitializeCondition {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        let b = self.expression.to_length_expr(self.has_self, false);

        vec![syn::Stmt::Semi(
            syn::Expr::Let(syn::ExprLet {
                attrs: vec![],
                let_token: Default::default(),
                pat: syn::Pat::Ident(syn::PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: syn::Ident::new(&self.name, Span::call_site()),
                    subpat: None,
                }),
                eq_token: Default::default(),
                expr: Box::new(b),
            }),
            Default::default(),
        )]
    }
}

/// A statement to create two variables: an item and a size, from a type and the index.
#[derive(Clone, Debug)]
pub struct LoadStatementVariable {
    pub name: Box<str>,
    pub ty: Type,
    pub use_slice: bool,
    pub condition: Option<(Rc<UseCondition>, Box<str>)>,
}

impl Statement for LoadStatementVariable {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        let inn = syn::Stmt::Semi(
            syn::Expr::Let(syn::ExprLet {
                attrs: vec![],
                let_token: Default::default(),
                eq_token: Default::default(),
                pat: syn::Pat::Type(syn::PatType {
                    attrs: vec![],
                    pat: Box::new(syn::Pat::Tuple(syn::PatTuple {
                        attrs: vec![],
                        paren_token: Default::default(),
                        elems: vec![
                            syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: syn::Ident::new(&*self.name, Span::call_site()),
                                subpat: None,
                            }),
                            syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: syn::Ident::new("sz", Span::call_site()),
                                subpat: None,
                            }),
                        ]
                        .into_iter()
                        .collect(),
                    })),
                    colon_token: Default::default(),
                    ty: Box::new(
                        Type::Tuple(vec![self.ty.clone(), Type::Basic("usize".into())]).to_syn_ty(),
                    ),
                }),
                expr: Box::new({
                    let a = syn::Expr::Call(syn::ExprCall {
                        attrs: vec![],
                        func: Box::new(syn::Expr::Path(syn::ExprPath {
                            attrs: vec![],
                            qself: Some(syn::QSelf {
                                lt_token: Default::default(),
                                gt_token: Default::default(),
                                ty: Box::new(self.ty.to_syn_ty()),
                                as_token: None,
                                position: 0,
                            }),
                            path: syn::Path {
                                leading_colon: Some(Default::default()),
                                segments: iter::once(str_to_pathseg("from_bytes")).collect(),
                            },
                        })),
                        paren_token: Default::default(),
                        args: iter::once(if self.use_slice {
                            bytes_slice(false)
                        } else {
                            str_to_exprpath("bytes")
                        })
                        .collect(),
                    });

                    syn::Expr::Try(syn::ExprTry {
                        attrs: vec![],
                        question_token: Default::default(),
                        expr: Box::new(a),
                    })
                }),
            }),
            Default::default(),
        );

        let do_increment = match self.use_slice {
            true => IncrementIndex::Sz.to_syn_statement(),
            false => vec![],
        };

        match self.condition {
            Some((ref condition, ref condname)) => vec![syn::Stmt::Semi(
                let_statement(
                    &self.name,
                    self.ty.clone(),
                    syn::Expr::If(syn::ExprIf {
                        attrs: vec![],
                        if_token: Default::default(),
                        cond: Box::new(condition.to_cond_expr(&condname)),
                        then_branch: syn::Block {
                            brace_token: Default::default(),
                            stmts: iter::once(inn)
                                .chain(do_increment)
                                .chain(iter::once(syn::Stmt::Expr(str_to_exprpath(&self.name))))
                                .collect(),
                        },
                        else_branch: Some((
                            Default::default(),
                            Box::new(syn::Expr::Block(syn::ExprBlock {
                                attrs: vec![],
                                label: None,
                                block: syn::Block {
                                    brace_token: Default::default(),
                                    stmts: vec![syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
                                        attrs: vec![],
                                        func: Box::new(syn::Expr::Path(syn::ExprPath {
                                            attrs: vec![],
                                            qself: None,
                                            path: syn::Path {
                                                segments: vec![
                                                    str_to_pathseg("Default"),
                                                    str_to_pathseg("default"),
                                                ]
                                                .into_iter()
                                                .collect(),
                                                leading_colon: None,
                                            },
                                        })),
                                        paren_token: Default::default(),
                                        args: syn::punctuated::Punctuated::new(),
                                    }))],
                                },
                            })),
                        )),
                    }),
                    false,
                ),
                Default::default(),
            )],
            None => iter::once(inn).chain(do_increment).collect(),
        }
    }
}

#[inline]
pub fn bytes_slice(is_mut: bool) -> syn::Expr {
    syn::Expr::Reference(syn::ExprReference {
        attrs: vec![],
        and_token: Default::default(),
        raw: Default::default(),
        mutability: if is_mut {
            Some(Default::default())
        } else {
            None
        },
        expr: Box::new(syn::Expr::Index(syn::ExprIndex {
            attrs: vec![],
            expr: Box::new(str_to_exprpath("bytes")),
            bracket_token: Default::default(),
            index: Box::new(syn::Expr::Range(syn::ExprRange {
                attrs: vec![],
                from: Some(Box::new(str_to_exprpath("index"))),
                limits: syn::RangeLimits::HalfOpen(Default::default()),
                to: None,
            })),
        })),
    })
}

/// Increment the "index" variable by either a set number or "sz".
#[derive(Copy, Clone, Debug)]
pub enum IncrementIndex {
    Number(usize),
    Sz,
}

impl Statement for IncrementIndex {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Semi(
            syn::Expr::AssignOp(syn::ExprAssignOp {
                attrs: vec![],
                left: Box::new(str_to_exprpath("index")),
                op: syn::BinOp::AddEq(Default::default()),
                right: Box::new(match self {
                    Self::Number(i) => int_litexpr_int(*i),
                    Self::Sz => str_to_exprpath("sz"),
                }),
            }),
            Default::default(),
        )]
    }
}

/// Return a struct containing all of the listed fields.
#[derive(Clone, Debug)]
pub struct ReturnStruct {
    pub sname: Box<str>,
    pub fields: Vec<Cow<'static, str>>,
    pub last_index: &'static str,
    pub fds: Vec<String>,
}

impl Statement for ReturnStruct {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        let struct_expr = syn::Expr::Struct(syn::ExprStruct {
            attrs: vec![],
            path: str_to_path(&*self.sname),
            brace_token: Default::default(),
            fields: self
                .fields
                .iter()
                .map(|f| syn::FieldValue {
                    attrs: vec![],
                    member: syn::Member::Named(syn::Ident::new(&*f, Span::call_site())),
                    colon_token: Some(Default::default()),
                    expr: str_to_exprpath(&*f),
                })
                .chain(self.fds.iter().map(|f| syn::FieldValue {
                    attrs: vec![],
                    member: syn::Member::Named(syn::Ident::new(&*f, Span::call_site())),
                    colon_token: Some(Default::default()),
                    expr: syn::Expr::Macro(syn::ExprMacro {
                        attrs: vec![],
                        mac: syn::Macro {
                            path: str_to_path("vec"),
                            bang_token: Default::default(),
                            delimiter: syn::MacroDelimiter::Bracket(Default::default()),
                            tokens: TokenStream::new(),
                        },
                    }),
                }))
                .collect(),
            dot2_token: None,
            rest: None,
        });
        vec![syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            func: Box::new(str_to_exprpath("Some")),
            paren_token: Default::default(),
            args: iter::once(syn::Expr::Tuple(syn::ExprTuple {
                attrs: vec![],
                paren_token: Default::default(),
                elems: vec![struct_expr, str_to_exprpath(self.last_index)]
                    .into_iter()
                    .collect(),
            }))
            .collect(),
        }))]
    }
}

/// Match a series of variants.
#[derive(Clone, Debug)]
pub struct MatchSelfToBytes {
    pub underlying: Cow<'static, str>,
}

impl Statement for MatchSelfToBytes {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            paren_token: Default::default(),
            args: iter::once(str_to_exprpath("bytes")).collect(),
            func: Box::new(syn::Expr::Field(syn::ExprField {
                attrs: vec![],
                dot_token: Default::default(),
                member: syn::Member::Named(syn::Ident::new("as_bytes", Span::call_site())),
                base: Box::new(syn::Expr::Paren(syn::ExprParen {
                    attrs: vec![],
                    paren_token: Default::default(),
                    expr: Box::new(syn::Expr::Cast(syn::ExprCast {
                        attrs: vec![],
                        expr: Box::new(syn::Expr::Unary(syn::ExprUnary {
                            attrs: vec![],
                            op: syn::UnOp::Deref(Default::default()),
                            expr: Box::new(str_to_exprpath("self")),
                        })),
                        as_token: Default::default(),
                        ty: Box::new(Type::Basic(self.underlying.clone()).to_syn_ty()),
                    })),
                })),
            })),
        }))]
    }
}

/// Match a number to a series of variants.
#[derive(Clone, Debug)]
pub struct MatchBytesToEnum {
    pub name: Box<str>,
    pub underlying: Cow<'static, str>,
    pub variants: Box<[(Box<str>, i64)]>,
}

impl Statement for MatchBytesToEnum {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        let load = LoadStatementVariable {
            name: "underlying".into(),
            ty: Type::Basic(self.underlying.clone()),
            use_slice: false,
            condition: None,
        }
        .to_syn_statement();

        load.into_iter()
            .chain(vec![syn::Stmt::Expr(syn::Expr::Match(syn::ExprMatch {
                attrs: vec![],
                match_token: Default::default(),
                expr: Box::new(str_to_exprpath("underlying")),
                brace_token: Default::default(),
                arms: self
                    .variants
                    .iter()
                    .map(|(name, value)| syn::Arm {
                        attrs: vec![],
                        pat: syn::Pat::Lit(syn::PatLit {
                            attrs: vec![],
                            expr: Box::new(int_litexpr_int(value)),
                        }),
                        guard: None,
                        fat_arrow_token: Default::default(),
                        body: Box::new(syn::Expr::Call(syn::ExprCall {
                            attrs: vec![],
                            func: Box::new(str_to_exprpath("Some")),
                            paren_token: Default::default(),
                            args: iter::once(syn::Expr::Tuple(syn::ExprTuple {
                                attrs: vec![],
                                paren_token: Default::default(),
                                elems: vec![
                                    syn::Expr::Path(syn::ExprPath {
                                        attrs: vec![],
                                        qself: None,
                                        path: syn::Path {
                                            segments: vec![
                                                str_to_pathseg("Self"),
                                                str_to_pathseg(name),
                                            ]
                                            .into_iter()
                                            .collect(),
                                            leading_colon: None,
                                        },
                                    }),
                                    str_to_exprpath("sz"),
                                ]
                                .into_iter()
                                .collect(),
                            }))
                            .collect(),
                        })),
                        comma: Some(Default::default()),
                    })
                    .chain(iter::once(syn::Arm {
                        attrs: vec![],
                        pat: syn::Pat::Wild(syn::PatWild {
                            attrs: vec![],
                            underscore_token: Default::default(),
                        }),
                        guard: None,
                        fat_arrow_token: Default::default(),
                        body: Box::new(str_to_exprpath("None")),
                        comma: None,
                    }))
                    .collect(),
            }))])
            .collect()
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct ExprWrapper(pub syn::Expr);

impl Statement for ExprWrapper {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(self.0.clone())]
    }
}

impl fmt::Debug for ExprWrapper {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExprWrapper")
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GetXidStatement;

impl Statement for GetXidStatement {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(item_field(str_to_exprpath("self"), "xid"))]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CreateXidTypeStatement;

impl Statement for CreateXidTypeStatement {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(syn::Expr::Struct(syn::ExprStruct {
            attrs: vec![],
            path: str_to_path("Self"),
            brace_token: Default::default(),
            fields: iter::once(syn::FieldValue {
                attrs: vec![],
                member: syn::Member::Named(syn::Ident::new("xid", Span::call_site())),
                colon_token: Some(Default::default()),
                expr: str_to_exprpath("xid"),
            })
            .collect(),
            dot2_token: None,
            rest: None,
        }))]
    }
}

#[derive(Debug, Clone)]
pub enum SizeSumPart {
    Bytes(usize),
    SizeofField(Box<str>),
    SizeofType(Type),
    ListTimesSize(Box<str>, MaybeString, Option<usize>),
}

impl SizeSumPart {
    #[inline]
    pub fn to_expr(&self) -> syn::Expr {
        match self {
            Self::Bytes(bytes) => int_litexpr_int(bytes),
            Self::SizeofType(ty) => syn::Expr::Call(syn::ExprCall {
                attrs: vec![],
                func: Box::new(syn::Expr::Path(syn::ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: syn::Path {
                        leading_colon: Some(Default::default()),
                        segments: vec![
                            str_to_pathseg("core"),
                            str_to_pathseg("mem"),
                            syn::PathSegment {
                                ident: syn::Ident::new("size_of", Span::call_site()),
                                arguments: syn::PathArguments::AngleBracketed(
                                    syn::AngleBracketedGenericArguments {
                                        colon2_token: Some(Default::default()),
                                        lt_token: Default::default(),
                                        args: iter::once(syn::GenericArgument::Type(
                                            ty.to_syn_ty(),
                                        ))
                                        .collect(),
                                        gt_token: Default::default(),
                                    },
                                ),
                            },
                        ]
                        .into_iter()
                        .collect(),
                    },
                })),
                paren_token: Default::default(),
                args: syn::punctuated::Punctuated::new(),
            }),
            Self::SizeofField(fname) => syn::Expr::Call(syn::ExprCall {
                attrs: vec![],
                func: Box::new(item_field(
                    item_field(str_to_exprpath("self"), fname),
                    "size",
                )),
                paren_token: Default::default(),
                args: syn::punctuated::Punctuated::new(),
            }),
            Self::ListTimesSize(fname, ty, pad) => {
                let list_size = match ty {
                    MaybeString::NotAString(_) => syn::Expr::Call(syn::ExprCall {
                        attrs: vec![],
                        func: Box::new(item_field(
                            syn::Expr::Call(syn::ExprCall {
                                attrs: vec![],
                                func: Box::new(item_field(
                                    syn::Expr::Call(syn::ExprCall {
                                        attrs: vec![],
                                        func: Box::new(item_field(
                                            item_field(str_to_exprpath("self"), fname),
                                            "iter",
                                        )),
                                        paren_token: Default::default(),
                                        args: syn::punctuated::Punctuated::new(),
                                    }),
                                    "map",
                                )),
                                paren_token: Default::default(),
                                args: iter::once(syn::Expr::Closure(syn::ExprClosure {
                                    attrs: vec![],
                                    asyncness: None,
                                    movability: None,
                                    capture: None,
                                    or1_token: Default::default(),
                                    inputs: iter::once(syn::Pat::Ident(syn::PatIdent {
                                        attrs: vec![],
                                        by_ref: None,
                                        mutability: None,
                                        ident: syn::Ident::new("i", Span::call_site()),
                                        subpat: None,
                                    }))
                                    .collect(),
                                    or2_token: Default::default(),
                                    output: syn::ReturnType::Default,
                                    body: Box::new(syn::Expr::Call(syn::ExprCall {
                                        attrs: vec![],
                                        func: Box::new(item_field(str_to_exprpath("i"), "size")),
                                        paren_token: Default::default(),
                                        args: syn::punctuated::Punctuated::new(),
                                    })),
                                }))
                                .collect(),
                            }),
                            "sum",
                        )),
                        paren_token: Default::default(),
                        args: syn::punctuated::Punctuated::new(),
                    }),
                    MaybeString::IsAString => syn::Expr::Call(syn::ExprCall {
                        attrs: vec![],
                        func: Box::new(item_field(
                            item_field(str_to_exprpath("self"), fname),
                            "len",
                        )),
                        paren_token: Default::default(),
                        args: syn::punctuated::Punctuated::new(),
                    }),
                };

                let pad_size = syn::Expr::Call(syn::ExprCall {
                    attrs: vec![],
                    func: Box::new(str_to_exprpath("buffer_pad")),
                    paren_token: Default::default(),
                    args: vec![
                        str_to_exprpath("block_len"),
                        match pad {
                            Some(p) => int_litexpr_int(p),
                            None => {
                                let align_ty = match ty {
                                    MaybeString::IsAString => Type::Basic("c_char".into()),
                                    MaybeString::NotAString(ty) => Type::from_lvl2(ty.clone()),
                                };
                                get_pad_align(&align_ty)
                            }
                        },
                    ]
                    .into_iter()
                    .collect(),
                });

                let block_len_let =
                    let_statement("block_len", Type::Basic("usize".into()), list_size, false);
                let pad_let = let_statement("pad", Type::Basic("usize".into()), pad_size, false);

                let adder = syn::Expr::Binary(syn::ExprBinary {
                    attrs: vec![],
                    left: Box::new(str_to_exprpath("block_len")),
                    op: syn::BinOp::Add(Default::default()),
                    right: Box::new(str_to_exprpath("pad")),
                });

                syn::Expr::Block(syn::ExprBlock {
                    attrs: vec![],
                    label: None,
                    block: syn::Block {
                        brace_token: Default::default(),
                        stmts: vec![
                            syn::Stmt::Semi(block_len_let, Default::default()),
                            syn::Stmt::Semi(pad_let, Default::default()),
                            syn::Stmt::Expr(adder),
                        ],
                    },
                })
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SumOfSizes(pub Vec<SizeSumPart>);

impl Statement for SumOfSizes {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![{
            let first = match self.0.get(0) {
                Some(f) => f.to_expr(),
                None => return vec![],
            };
            syn::Stmt::Expr(self.0.iter().skip(1).fold(first, |sum, current| {
                syn::Expr::Binary(syn::ExprBinary {
                    attrs: vec![],
                    left: Box::new(sum),
                    op: syn::BinOp::Add(Default::default()),
                    right: Box::new(current.to_expr()),
                })
            }))
        }]
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReturnEnumVariant(pub Box<str>, pub Box<str>);

impl Statement for ReturnEnumVariant {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(syn::Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: vec![str_to_pathseg(&self.0), str_to_pathseg(&self.1)]
                    .into_iter()
                    .collect(),
            },
        }))]
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ExtractBit(pub usize);

impl Statement for ExtractBit {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(syn::Expr::Binary(syn::ExprBinary {
            attrs: vec![],
            left: Box::new(syn::Expr::Binary(syn::ExprBinary {
                attrs: vec![],
                left: Box::new(item_field(str_to_exprpath("self"), "inner")),
                op: syn::BinOp::BitAnd(Default::default()),
                right: Box::new(syn::Expr::Paren(syn::ExprParen {
                    attrs: vec![],
                    paren_token: Default::default(),
                    expr: Box::new(syn::Expr::Binary(syn::ExprBinary {
                        attrs: vec![],
                        left: Box::new(int_litexpr_int(1)),
                        op: syn::BinOp::Shl(Default::default()),
                        right: Box::new(int_litexpr_int(self.0)),
                    })),
                })),
            })),
            op: syn::BinOp::Ne(Default::default()),
            right: Box::new(int_litexpr_int(0)),
        }))]
    }
}

#[derive(Debug, Clone)]
pub struct InsertBit {
    pub bit: usize,
    pub val: Box<str>,
    pub is_self: bool,
}

impl Statement for InsertBit {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        let shl = syn::Expr::Binary(syn::ExprBinary {
            attrs: vec![],
            left: Box::new(int_litexpr_int(1)),
            op: syn::BinOp::Shl(Default::default()),
            right: Box::new(int_litexpr_int(self.bit)),
        });

        let inner = if self.is_self {
            item_field(str_to_exprpath("self"), "inner")
        } else {
            str_to_exprpath("inner")
        };

        vec![syn::Stmt::Expr(syn::Expr::If(syn::ExprIf {
            attrs: vec![],
            if_token: Default::default(),
            cond: Box::new(str_to_exprpath(&self.val)),
            then_branch: syn::Block {
                // OR self.inner with 1 << bit
                brace_token: Default::default(),
                stmts: vec![syn::Stmt::Semi(
                    syn::Expr::AssignOp(syn::ExprAssignOp {
                        attrs: vec![],
                        left: Box::new(inner.clone()),
                        op: syn::BinOp::BitOrEq(Default::default()),
                        right: Box::new(shl.clone()),
                    }),
                    Default::default(),
                )],
            },
            else_branch: Some((
                Default::default(),
                Box::new(syn::Expr::Block(syn::ExprBlock {
                    attrs: vec![],
                    label: None,
                    block: syn::Block {
                        brace_token: Default::default(),
                        stmts: vec![
                            // AND self.inner with !(1 << bit)
                            syn::Stmt::Semi(
                                syn::Expr::AssignOp(syn::ExprAssignOp {
                                    attrs: vec![],
                                    left: Box::new(inner),
                                    op: syn::BinOp::BitAndEq(Default::default()),
                                    right: Box::new(syn::Expr::Unary(syn::ExprUnary {
                                        attrs: vec![],
                                        op: syn::UnOp::Not(Default::default()),
                                        expr: Box::new(syn::Expr::Paren(syn::ExprParen {
                                            attrs: vec![],
                                            paren_token: Default::default(),
                                            expr: Box::new(shl),
                                        })),
                                    })),
                                }),
                                Default::default(),
                            ),
                        ],
                    },
                })),
            )),
        }))]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct JustReturnSelf;

impl Statement for JustReturnSelf {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(str_to_exprpath("self"))]
    }
}

#[derive(Debug, Clone)]
pub struct DefineInnerAccumulator(pub Type);

impl Statement for DefineInnerAccumulator {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Semi(
            let_statement("inner", self.0.clone(), int_litexpr_int(0), true),
            Default::default(),
        )]
    }
}

#[derive(Debug, Clone)]
pub struct ReturnBitflag(pub Box<str>);

impl Statement for ReturnBitflag {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(syn::Expr::Struct(syn::ExprStruct {
            attrs: vec![],
            path: str_to_path(&self.0),
            brace_token: Default::default(),
            fields: iter::once(syn::FieldValue {
                attrs: vec![],
                member: syn::Member::Named(syn::Ident::new("inner", Span::call_site())),
                colon_token: Some(Default::default()),
                expr: str_to_exprpath("inner"),
            })
            .collect(),
            dot2_token: None,
            rest: None,
        }))]
    }
}

#[derive(Debug, Clone)]
pub struct ForwardAsBytes(pub Cow<'static, str>);

impl Statement for ForwardAsBytes {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            func: Box::new(item_field(
                item_field(str_to_exprpath("self"), &*self.0),
                "as_bytes",
            )),
            paren_token: Default::default(),
            args: iter::once(str_to_exprpath("bytes")).collect(),
        }))]
    }
}

#[derive(Debug, Clone)]
pub struct ForwardFromBytes {
    pub inner_ty: Type,
    pub self_name: Box<str>,
}

impl Statement for ForwardFromBytes {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        let load = LoadStatementVariable {
            name: "inner".into(),
            ty: self.inner_ty.clone(),
            use_slice: false,
            condition: None,
        }
        .to_syn_statement();

        load.into_iter()
            .chain(
                ReturnStruct {
                    sname: self.self_name.clone(),
                    fields: vec!["inner".into()],
                    last_index: "sz",
                    fds: vec![],
                }
                .to_syn_statement(),
            )
            .collect()
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct DeserTraceMarker(pub Box<str>);

impl Statement for DeserTraceMarker {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Semi(
            syn::Expr::Macro(syn::ExprMacro {
                attrs: vec![],
                mac: syn::Macro {
                    path: syn::Path {
                        leading_colon: None,
                        segments: vec![str_to_pathseg("log"), str_to_pathseg("trace")]
                            .into_iter()
                            .collect(),
                    },
                    bang_token: Default::default(),
                    delimiter: syn::MacroDelimiter::Paren(Default::default()),
                    tokens: format!("\"Deserializing {} from byte buffer\"", &self.0)
                        .parse()
                        .unwrap(),
                },
            }),
            Default::default(),
        )]
    }
}

#[derive(Debug, Clone)]
pub struct ConvertXids {
    pub oldname: Box<str>,
    pub newname: Box<str>,
}

impl Statement for ConvertXids {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            func: Box::new(syn::Expr::Path(syn::ExprPath {
                attrs: vec![],
                qself: Some(syn::QSelf {
                    lt_token: Default::default(),
                    ty: Box::new(Type::Basic(self.newname.deref().to_string().into()).to_syn_ty()),
                    position: 0,
                    as_token: None,
                    gt_token: Default::default(),
                }),
                path: syn::Path {
                    leading_colon: Some(Default::default()),
                    segments: iter::once(str_to_pathseg("const_from_xid")).collect(),
                },
            })),
            paren_token: Default::default(),
            args: iter::once(item_field(str_to_exprpath("base"), "xid")).collect(),
        }))]
    }
}

#[derive(Debug, Clone)]
pub struct GetFdRef(pub String);

impl Statement for GetFdRef {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        vec![syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            func: Box::new(str_to_exprpath("Some")),
            paren_token: Default::default(),
            args: iter::once(syn::Expr::Reference(syn::ExprReference {
                attrs: vec![],
                and_token: Default::default(),
                raw: Default::default(),
                mutability: Some(Default::default()),
                expr: Box::new(item_field(str_to_exprpath("self"), &self.0)),
            }))
            .collect(),
        }))]
    }
}

#[derive(Debug, Clone)]
pub enum SumStatement {
    ReturnIndex(ReturnIndexStatement),
    AppendToIndex(AppendToIndexStatement),
    PadIndex(PadIndexStatement),
    CreateIndexVariable(CreateIndexVariable),
    LoadStatementVariable(LoadStatementVariable),
    IncrementIndex(IncrementIndex),
    ReturnStruct(ReturnStruct),
    MatchSelfToBytes(MatchSelfToBytes),
    MatchBytesToEnum(MatchBytesToEnum),
    ExprWrapper(ExprWrapper),
    GetXid(GetXidStatement),
    CreateXidType(CreateXidTypeStatement),
    SumOfSizes(SumOfSizes),
    ReturnEnumVariant(ReturnEnumVariant),
    ExtractBit(ExtractBit),
    InsertBit(InsertBit),
    JustReturnSelf(JustReturnSelf),
    DefineInnerAccumulator(DefineInnerAccumulator),
    ReturnBitflag(ReturnBitflag),
    ForwardAsBytes(ForwardAsBytes),
    ForwardFromBytes(ForwardFromBytes),
    CreateAlignToAndBlockLen(CreateAlignToAndBlockLen),
    SetAlignAndAddPadding(SetAlignAndAddPadding),
    FromBytesList(FromBytesList),
    AsBytesList(AsBytesList),
    AppendLengthToIndex(AppendLengthToIndex),
    InitializeCondition(InitializeCondition),
    DeserTraceMarker(DeserTraceMarker),
    ConvertXids(ConvertXids),
    GetFdRef(GetFdRef),
}

macro_rules! sst_from_impl {
    ($ename: ident, $sname: ty) => {
        impl From<$sname> for SumStatement {
            #[inline]
            fn from(s: $sname) -> Self {
                Self::$ename(s)
            }
        }
    };
}

sst_from_impl! { ReturnIndex, ReturnIndexStatement }
sst_from_impl! { AppendToIndex, AppendToIndexStatement }
sst_from_impl! { PadIndex, PadIndexStatement }
sst_from_impl! { CreateIndexVariable, CreateIndexVariable }
sst_from_impl! { LoadStatementVariable, LoadStatementVariable }
sst_from_impl! { IncrementIndex, IncrementIndex }
sst_from_impl! { ReturnStruct, ReturnStruct }
sst_from_impl! { MatchSelfToBytes, MatchSelfToBytes }
sst_from_impl! { MatchBytesToEnum, MatchBytesToEnum }
sst_from_impl! { ExprWrapper, ExprWrapper }
sst_from_impl! { GetXid, GetXidStatement }
sst_from_impl! { CreateXidType, CreateXidTypeStatement }
sst_from_impl! { SumOfSizes, SumOfSizes }
sst_from_impl! { ReturnEnumVariant, ReturnEnumVariant }
sst_from_impl! { ExtractBit, ExtractBit }
sst_from_impl! { InsertBit, InsertBit }
sst_from_impl! { JustReturnSelf, JustReturnSelf }
sst_from_impl! { DefineInnerAccumulator, DefineInnerAccumulator }
sst_from_impl! { ReturnBitflag, ReturnBitflag }
sst_from_impl! { ForwardAsBytes, ForwardAsBytes }
sst_from_impl! { ForwardFromBytes, ForwardFromBytes }
sst_from_impl! { CreateAlignToAndBlockLen, CreateAlignToAndBlockLen }
sst_from_impl! { SetAlignAndAddPadding, SetAlignAndAddPadding }
sst_from_impl! { FromBytesList, FromBytesList }
sst_from_impl! { AsBytesList, AsBytesList }
sst_from_impl! { AppendLengthToIndex, AppendLengthToIndex }
sst_from_impl! { InitializeCondition, InitializeCondition }
sst_from_impl! { DeserTraceMarker, DeserTraceMarker }
sst_from_impl! { ConvertXids, ConvertXids }
sst_from_impl! { GetFdRef, GetFdRef }

impl Statement for SumStatement {
    #[inline]
    fn to_syn_statement(&self) -> Vec<syn::Stmt> {
        match self {
            Self::ReturnIndex(rr) => rr.to_syn_statement(),
            Self::AppendToIndex(atrs) => atrs.to_syn_statement(),
            Self::PadIndex(prs) => prs.to_syn_statement(),
            Self::CreateIndexVariable(civ) => civ.to_syn_statement(),
            Self::LoadStatementVariable(lsv) => lsv.to_syn_statement(),
            Self::IncrementIndex(ii) => ii.to_syn_statement(),
            Self::ReturnStruct(rs) => rs.to_syn_statement(),
            Self::MatchSelfToBytes(mstb) => mstb.to_syn_statement(),
            Self::MatchBytesToEnum(mbte) => mbte.to_syn_statement(),
            Self::ExprWrapper(ew) => ew.to_syn_statement(),
            Self::GetXid(gx) => gx.to_syn_statement(),
            Self::CreateXidType(cxt) => cxt.to_syn_statement(),
            Self::SumOfSizes(ss) => ss.to_syn_statement(),
            Self::ReturnEnumVariant(rev) => rev.to_syn_statement(),
            Self::ExtractBit(eb) => eb.to_syn_statement(),
            Self::InsertBit(ib) => ib.to_syn_statement(),
            Self::JustReturnSelf(jrs) => jrs.to_syn_statement(),
            Self::DefineInnerAccumulator(dia) => dia.to_syn_statement(),
            Self::ReturnBitflag(rb) => rb.to_syn_statement(),
            Self::ForwardAsBytes(fab) => fab.to_syn_statement(),
            Self::ForwardFromBytes(ffb) => ffb.to_syn_statement(),
            Self::CreateAlignToAndBlockLen(c) => c.to_syn_statement(),
            Self::SetAlignAndAddPadding(saaap) => saaap.to_syn_statement(),
            Self::FromBytesList(fbl) => fbl.to_syn_statement(),
            Self::AsBytesList(asl) => asl.to_syn_statement(),
            Self::AppendLengthToIndex(ai) => ai.to_syn_statement(),
            Self::InitializeCondition(ic) => ic.to_syn_statement(),
            Self::DeserTraceMarker(dtm) => dtm.to_syn_statement(),
            Self::ConvertXids(cx) => cx.to_syn_statement(),
            Self::GetFdRef(gfr) => gfr.to_syn_statement(),
        }
    }
}
