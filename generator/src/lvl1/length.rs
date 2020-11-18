// MIT/Apache2 License

/// Linked list of values that calculate a list length.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ListLength {
    Value(i64),
    FieldReference(String),
    BinaryOp {
        op: String,
        left: Box<ListLength>,
        right: Box<ListLength>,
    },
    UnaryOp {
        op: String,
        target: Box<ListLength>,
    },
}

impl Default for ListLength {
    #[inline]
    fn default() -> Self {
        ListLength::Value(0)
    }
}
