// MIT/Apache2 License

use super::Docs;
use tinyvec::TinyVec;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum EnumData {
    Value(i64),
    Bit(i64),
}

impl Default for EnumData {
    #[inline]
    fn default() -> Self {
        Self::Value(0)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumVariant {
    pub name: String,
    pub data: EnumData,
}

#[derive(Default, Debug)]
pub struct XEnum {
    pub name: String,
    pub variants: TinyVec<[EnumVariant; 4]>,
    pub docs: Option<Docs>,
}
