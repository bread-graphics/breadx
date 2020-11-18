// MIT/Apache2 License

use tinyvec::TinyVec;

/// The representation of either an XID type or an XID union type.
#[derive(Debug)]
pub struct XidType {
    pub name: Box<str>,
    pub from_impls: TinyVec<[Option<Box<str>>; 2]>, // variants are always Some
}
