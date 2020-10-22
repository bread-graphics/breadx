// MIT/Apache2 License

use crate::xenum::UnresolvedEnum;
use std::collections::HashMap;
use tinyvec::TinyVec;
use treexml::Element;

pub struct State {
  unresolved_enums: TinyVec<[UnresolvedEnum; 20]>,
  resolved: Vec<syn::Item>,
  events: HashMap<String, (usize, Vec<syn::Item>)>,
  scanned_names: Vec<NameHints>,
}

struct NameHints {
  name: String,
  type_hint: TypeHint,
}

#[derive(Copy, Clone)]
pub enum TypeHint {
  No,
  Enum,
  Xidtype,
}

impl State {
  #[inline]
  pub fn new() -> Self {
    Self {
      unresolved_enums: TinyVec::new(),
      resolved: Vec::new(),
      events: HashMap::new(),
      scanned_names: Vec::new(),
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
  pub fn add_event(&mut self, event_name: String, event_items: Vec<syn::Item>, number: usize) {
    self.events.insert(event_name, (number, event_items));
  }

  #[inline]
  pub fn clone_event(&mut self, old_name: &str, new_name: String, new_number: usize) {
    let (_, mut evn) = self
      .events
      .get(old_name)
      .unwrap_or_else(|| panic!("Event not found: {}", old_name))
      .clone();
    crate::xevent::replace_name(&new_name, &mut evn);
    self.events.insert(new_name, (new_number, evn));
  }

  #[inline]
  pub fn generate_name_hints(&mut self, elements: &[Element]) {
    self
      .scanned_names
      .extend(elements.iter().filter_map(NameHints::from_element));
  }

  #[inline]
  pub fn name_hint(&self, name: &str) -> Option<TypeHint> {
    self.scanned_names.iter().find_map(|x| {
      if x.name.as_str() == &name.to_lowercase() {
        Some(x.type_hint)
      } else {
        None
      }
    })
  }

  #[inline]
  pub fn resolved(self) -> Vec<syn::Item> {
    let Self {
      unresolved_enums,
      mut resolved,
      events,
      ..
    } = self;
    resolved.extend(
      unresolved_enums
        .into_iter()
        .map(|ue| ue.resolve("u32"))
        .chain(events.into_iter().map(|(_k, (_n, v))| v))
        .flat_map(|ue| ue.into_iter()),
    );
    resolved
  }
}

impl NameHints {
  #[inline]
  fn from_element(elem: &Element) -> Option<Self> {
    let name = elem.attributes.get("name")?.to_lowercase();
    let type_hint = match elem.name.as_str() {
      "xidtype" => TypeHint::Xidtype,
      "enum" => TypeHint::Enum,
      _ => TypeHint::No,
    };

    Some(Self { name, type_hint })
  }
}
