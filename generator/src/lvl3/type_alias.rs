// MIT/Apache2 License

use super::{
    syn_util::{pub_vis, str_to_ty},
    ToSyn,
};
use crate::lvl1::Typedef;
use proc_macro2::Span;

// convert a level 1 type alias into a syn typedef item
impl ToSyn for Typedef {
    #[inline]
    fn to_syn_item(self) -> Vec<syn::Item> {
        vec![syn::Item::Type(syn::ItemType {
            attrs: vec![],
            vis: pub_vis(),
            type_token: Default::default(),
            ident: syn::Ident::new(&self.newname, Span::call_site()),
            generics: Default::default(),
            eq_token: Default::default(),
            ty: Box::new(str_to_ty(&self.oldname)),
            semi_token: Default::default(),
        })]
    }
}
