// MIT/Apache2 License

use super::NonenumTypenames;
use tinyvec::TinyVec;

#[derive(Debug, Default)]
pub struct Xidtype {
    pub name: String,
}

impl NonenumTypenames for Xidtype {
    #[inline]
    fn typename(&self) -> TinyVec<[String; 1]> {
        TinyVec::from([self.name.clone()])
    }
}
