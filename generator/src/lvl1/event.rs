// MIT/Apache2 License

use super::{NonenumTypenames, XStruct};
use tinyvec::TinyVec;

#[derive(Default, Debug)]
pub struct Event {
    pub base: XStruct,
    pub opcode: u64,
    pub skip_sequence: bool,
}

#[derive(Default, Debug)]
pub struct EventCopy {
    pub name: String,
    pub opcode: u64,
    pub base: String,
}

impl NonenumTypenames for Event {
    #[inline]
    fn typename(&self) -> TinyVec<[String; 1]> {
        TinyVec::from([self.base.name.clone()])
    }
}

impl NonenumTypenames for EventCopy {
    #[inline]
    fn typename(&self) -> TinyVec<[String; 1]> {
        TinyVec::from([self.name.clone()])
    }
}
