// MIT/Apache2 License

use super::StructureItem;

/// Represents a structure.
#[derive(Debug, Clone)]
pub struct Struct {
    pub name: Box<str>,
    pub brief: Option<Box<str>>,
    pub desc: Option<Box<str>>,
    pub fields: Vec<StructureItem>,
    pub fds: Vec<String>,
    pub special: StructSpecial,
}

/// Bits of a struct that represent something special.
#[derive(Debug, Clone)]
pub enum StructSpecial {
    /// Just a regular struct
    Regular,
    /// This is an event with an associated opcode.
    Event(u64),
    /// This is an error with an associated number.
    Error(u64),
    /// This is a request with an associated opcode and reply.
    /// Note: The reply is mandated to have struct type regular.
    Request(u64, Option<Box<Struct>>),
}
