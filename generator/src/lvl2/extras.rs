// MIT/Apache2 License

use super::StructureItem;
use tinyvec::TinyVec;

/// Request extras.
#[inline]
pub fn modify_with_request_extras(fields: &mut TinyVec<[StructureItem; 6]>) {
    // if the first field can be optimized, take it out
}
