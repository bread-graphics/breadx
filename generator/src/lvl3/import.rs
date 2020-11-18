// MIT/Apache2 License

use super::ToSyn;
use crate::lvl1::Import;
use proc_macro2::Span;

impl ToSyn for Import {
    #[inline]
    fn to_syn_item(self) -> Vec<syn::Item> {
        vec![syn::Item::Use(syn::ItemUse {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            use_token: Default::default(),
            leading_colon: None,
            semi_token: Default::default(),
            tree: syn::UseTree::Path(syn::UsePath {
                ident: syn::Ident::new("super", Span::call_site()),
                colon2_token: Default::default(),
                tree: Box::new(syn::UseTree::Path(syn::UsePath {
                    ident: syn::Ident::new(&self.0, Span::call_site()),
                    colon2_token: Default::default(),
                    tree: Box::new(syn::UseTree::Glob(syn::UseGlob {
                        star_token: Default::default(),
                    })),
                })),
            }),
        })]
    }
}
