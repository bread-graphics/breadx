// MIT/Apache2 License

/// Linked list of values that calculate an expression.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone)]
pub enum Expression {
    Value(i64),
    FieldReference(String),
    BinaryOp {
        op: String,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOp {
        op: String,
        target: Box<Expression>,
    },
}

impl Default for Expression {
    #[inline]
    fn default() -> Self {
        Expression::Value(0)
    }
}
