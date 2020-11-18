// MIT/Apache2 License

use super::ListLength;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructureItem {
    Field(Field),
    Padding { bytes: usize, is_align: bool },
    List(List),
    ValueParam(ValueParam),
}

impl Default for StructureItem {
    #[inline]
    fn default() -> Self {
        Self::Field(Default::default())
    }
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub ty: String,
    pub name: String,
    pub mask: Option<String>,
    pub enumeration: Option<String>,
    pub alt_enum: Option<String>,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List {
    pub ty: String,
    pub name: String,
    pub list_length: ListLength,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueParam {
    pub mask_ty: String,
    pub mask_name: String,
    pub list_name: String,
}
