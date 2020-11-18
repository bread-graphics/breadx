// MIT/Apache2 License

use super::{NonenumTypenames, StructureItem, XStruct};
use tinyvec::TinyVec;

#[derive(Default, Debug)]
pub struct Request {
    pub base: XStruct,
    pub opcode: u64,
    pub reply: Option<XStruct>,
}

impl NonenumTypenames for Request {
    #[inline]
    fn typename(&self) -> TinyVec<[String; 1]> {
        TinyVec::Heap(vec![
            format!("{}Request", &self.base.name),
            format!("{}Reply", &self.base.name),
        ])
    }
}
