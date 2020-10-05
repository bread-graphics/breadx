// MIT/Apache2 License

use crate::xenum::UnresolvedEnum;
use tinyvec::TinyVec;

pub struct State {
  unresolved_enums: TinyVec<[UnresolvedEnum; 20]>,
  resolved: Vec<syn::Item>,
}

impl State {
  #[inline]
  pub fn new() -> Self {
    Self {
      unresolved_enums: TinyVec::new(),
      resolved: Vec::new(),
    }
  }

  #[inline]
  pub fn add_unresolved_enum(&mut self, ur: UnresolvedEnum) {
    self.unresolved_enums.push(ur);
  }

  #[inline]
  pub fn resolve_enum(&mut self, ty: &str, enum_name: &str) {
    if let Some(posn) = self
      .unresolved_enums
      .iter()
      .position(|ur| ur.name() == enum_name)
    {
      self
        .resolved
        .extend(self.unresolved_enums.remove(posn).resolve(ty));
      return;
    }
  }

  #[inline]
  pub fn resolved(self) -> Vec<syn::Item> {
    let Self {
      unresolved_enums,
      mut resolved,
    } = self;
    resolved.extend(
      unresolved_enums
        .into_iter()
        .flat_map(|ue| ue.resolve("u32").into_iter()),
    );
    resolved
  }
}
