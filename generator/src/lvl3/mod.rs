// MIT/Apache2 License

mod asb;
mod bitflags;
mod expr;
mod field;
mod import;
mod item;
mod method;
mod renum;
mod rstruct;
mod rtrait;
mod statement;
mod switch;
mod ty;
mod type_alias;
mod xidtype;

pub mod syn_util;

pub use asb::*;
pub use bitflags::*;
pub use expr::*;
pub use field::*;
pub use import::*;
pub use item::*;
pub use method::*;
pub use renum::*;
pub use rstruct::*;
pub use rtrait::*;
pub use statement::*;
pub use switch::*;
pub use ty::*;
pub use type_alias::*;
pub use xidtype::*;

pub trait ToSyn {
    fn to_syn_item(self) -> Vec<syn::Item>;
}
