// MIT/Apache2 License

/// A top-level item on the XML description file.
#[derive(Debug)]
pub enum Item {
    Import(super::Import),
    Typedef(super::Typedef),
    Xidtype(super::Xidtype),
    XidUnion(super::XidUnion),
    Enum(super::XEnum),
    Struct(super::XStruct),
    Request(super::Request),
    Event(super::Event),
    Error(super::XError),
    EventCopy(super::EventCopy),
    ErrorCopy(super::XErrorCopy),
}
