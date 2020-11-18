// MIT/Apache2 License

use super::{Docs, StructureItem};
use tinyvec::TinyVec;

#[derive(Default, Debug)]
pub struct XStruct {
    pub name: String,
    pub fields: TinyVec<[StructureItem; 6]>,
    pub docs: Option<Docs>,
}

/// Gives a typename or typenames and isn't an enum.
pub trait NonenumTypenames {
    /// Typename or typenames.
    fn typename(&self) -> TinyVec<[String; 1]>;
}

impl NonenumTypenames for XStruct {
    #[inline]
    fn typename(&self) -> TinyVec<[String; 1]> {
        TinyVec::from([self.name.clone()])
    }
}
