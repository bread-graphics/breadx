// MIT/Apache2 License

use std::borrow::Cow;

/// The type associated with a value.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Type {
    BasicType(Cow<'static, str>),
    Array(Cow<'static, str>, u64),
}

impl From<String> for Type {
    #[inline]
    fn from(s: String) -> Type {
        Type::BasicType(s.into())
    }
}

impl Default for Type {
    #[inline]
    fn default() -> Self {
        Self::BasicType("".into())
    }
}
