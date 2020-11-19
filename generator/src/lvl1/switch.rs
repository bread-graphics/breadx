// MIT/Apache2 License

use super::{Expression, Field, StructureItem};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Switch {
    pub name: String,
    pub cases: Vec<Case>,
    pub expr: Expression,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Case {
    pub is_bitcase: bool,
    pub enum_ref: String,
    pub enum_item: String,
    pub fields: Vec<StructureItem>,
}
