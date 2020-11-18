// MIT/Apache2 License

use super::{
    syn_util::{inliner, pub_vis},
    Statement, SumStatement, Type,
};
use proc_macro2::Span;
use std::{borrow::Cow, fmt};

/// An input parameter for a method.
#[derive(Debug, Clone)]
pub struct InputParameter {
    pub name: Cow<'static, str>,
    pub ty: Type,
    pub usage: ParameterUsage,
}

/// Type of usage for a parameter.
#[derive(Debug, Copy, Clone)]
pub enum ParameterUsage {
    Owned,
    Ref,
    MutRef,
}

/// A method in an impl block.
#[derive(Clone, Debug)]
pub struct Method {
    pub name: Cow<'static, str>,
    pub is_const: bool,
    pub self_parameter: Option<ParameterUsage>,
    pub parameters: Vec<InputParameter>,
    pub statements: Vec<SumStatement>,
    pub return_type: Option<Type>,
}

impl Method {
    /// Create a new method.
    #[inline]
    pub fn new(
        name: Cow<'static, str>,
        self_parameter: Option<ParameterUsage>,
        parameters: Vec<InputParameter>,
        return_type: Option<Type>,
    ) -> Method {
        Self {
            name,
            is_const: false,
            self_parameter,
            parameters,
            statements: vec![],
            return_type,
        }
    }

    /// Convert this method to a syn ImplItem.
    #[inline]
    pub fn to_syn_impl_item(&self, in_trait: bool) -> syn::ImplItem {
        syn::ImplItem::Method(syn::ImplItemMethod {
            attrs: vec![inliner()],
            vis: if in_trait {
                syn::Visibility::Inherited
            } else {
                pub_vis()
            },
            defaultness: None,
            sig: syn::Signature {
                constness: if self.is_const {
                    Some(Default::default())
                } else {
                    None
                },
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: syn::Ident::new(&self.name, Span::call_site()),
                generics: Default::default(),
                paren_token: Default::default(),
                inputs: self.inputs().into_iter().collect(),
                variadic: None,
                output: match self.return_type {
                    Some(ref ty) => {
                        syn::ReturnType::Type(Default::default(), Box::new(ty.to_syn_ty()))
                    }
                    None => syn::ReturnType::Default,
                },
            },
            block: syn::Block {
                brace_token: Default::default(),
                stmts: self
                    .statements
                    .iter()
                    .flat_map(|s| s.to_syn_statement())
                    .collect(),
            },
        })
    }

    /// Get an iterator of fnargs.
    #[inline]
    fn inputs(&self) -> Vec<syn::FnArg> {
        self.self_parameter
            .as_ref()
            .map(|c| {
                syn::FnArg::Receiver(syn::Receiver {
                    attrs: vec![],
                    reference: if let ParameterUsage::Owned = c {
                        None
                    } else {
                        Some((Default::default(), None))
                    },
                    mutability: if let ParameterUsage::MutRef = c {
                        Some(Default::default())
                    } else {
                        None
                    },
                    self_token: Default::default(),
                })
            })
            .into_iter()
            .chain(self.parameters.iter().map(|p| {
                syn::FnArg::Typed(syn::PatType {
                    attrs: vec![],
                    pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                        attrs: vec![],
                        by_ref: None,
                        mutability: None,
                        ident: syn::Ident::new(&*p.name, Span::call_site()),
                        subpat: None,
                    })),
                    colon_token: Default::default(),
                    ty: Box::new({
                        let ty = p.ty.to_syn_ty();
                        match p.usage {
                            ParameterUsage::Owned => ty,
                            _ => syn::Type::Reference(syn::TypeReference {
                                and_token: Default::default(),
                                lifetime: None,
                                mutability: match p.usage {
                                    ParameterUsage::MutRef => Some(Default::default()),
                                    _ => None,
                                },
                                elem: Box::new(ty),
                            }),
                        }
                    }),
                })
            }))
            .collect()
    }
}
