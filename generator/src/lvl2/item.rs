// MIT/Apache2 License

use super::{EnumRepr, Struct, XidType};
use crate::lvl1::{Import, Typedef};

/// Level 2 Items include a subset of Level 1's items, along with resolved versions of those items.
#[derive(Debug)]
pub enum Item {
    Import(Import),
    Typedef(Typedef),
    XidType(XidType),
    Enum(EnumRepr),
    Struct(Struct),
}

impl Default for Item {
    #[inline]
    fn default() -> Self {
        Self::Import(Default::default())
    }
}
