// MIT/Apache2 License

use super::NonenumTypenames;
use tinyvec::TinyVec;

/// A combination of two XID-based types.
#[derive(Debug, Default)]
pub struct XidUnion {
    pub name: String,
    pub members: TinyVec<[String; 2]>,
}

impl NonenumTypenames for XidUnion {
    #[inline]
    fn typename(&self) -> TinyVec<[String; 1]> {
        TinyVec::from([self.name.clone()])
    }
}
