// MIT/Apache2 License

use super::{Expression, Switch};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum StructureItem {
    Field(Field),
    Padding { bytes: usize, is_align: bool },
    List(List),
    ValueParam(ValueParam),
    Switch(Switch),
    Fd { name: String },
    RequiredStartAlign { align: usize },
}

impl Default for StructureItem {
    #[inline]
    fn default() -> Self {
        Self::Field(Default::default())
    }
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Field {
    pub ty: String,
    pub name: String,
    pub mask: Option<String>,
    pub enumeration: Option<String>,
    pub alt_enum: Option<String>,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct List {
    pub ty: String,
    pub name: String,
    pub list_length: Expression,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ValueParam {
    pub mask_ty: String,
    pub mask_name: String,
    pub list_name: String,
}
