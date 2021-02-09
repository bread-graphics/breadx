// MIT/Apache2 License

use super::{
    configure_fields, create_generator, safe_name, set_is_extension, EnumRepr, EnumReprGenerator,
    Expression, Field, Item as Lvl2Item, Item, List, MaybeString, Struct, StructSpecial,
    StructVariant, StructureItem, Type, XidType,
};
use crate::lvl1::{
    Item as Lvl1Item, NonenumTypenames, StructureItem as Lvl1StructureItem, XStruct,
};
use heck::{CamelCase, SnakeCase};
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
    iter, mem,
    ops::Deref,
    rc::Rc,
};
use tinyvec::{tiny_vec, TinyVec};

/// State used to construct level 2
pub struct Lvl2State {
    // type names of things that aren't enums, collected on the first pass
    nonenum_typenames: HashSet<Box<str>>,
    // enums that have not been identified to a specific type yet
    unresolved_enums: BTreeMap<String, EnumReprGenerator>,
    // a list of errors, kept here to help resolve errorcopies
    errors: BTreeMap<Box<str>, Struct>,
    // a list of events, kept here to help resolve eventcopies
    events: BTreeMap<Box<str>, Struct>,
    // output: list of XIDs
    pub xidtypes: Vec<Box<str>>,
}

#[inline]
fn enum_repr_conv(repr: String) -> Cow<'static, str> {
    match repr.as_str() {
        "Card8" | "Byte" | "Keycode" | "Op" | "Kind" => Cow::Borrowed("u8"),
        "Card16" | "LedClassSpec" | "IdSpec" | "DeviceId" => Cow::Borrowed("u16"),
        "Window" | "Colormap" | "Atom" | "Timestamp" | "Pixmap" | "Card32" => Cow::Borrowed("u32"),
        _ => Cow::Owned(repr),
    }
}

impl Lvl2State {
    #[inline]
    pub fn new() -> Self {
        Self {
            nonenum_typenames: HashSet::new(),
            unresolved_enums: BTreeMap::new(),
            errors: BTreeMap::<Box<str>, Struct>::new(),
            events: BTreeMap::new(),
            xidtypes: vec![],
        }
    }

    #[inline]
    pub fn register_typenames(&mut self, items: &[Lvl1Item]) {
        items.iter().for_each(|item| match item {
            Lvl1Item::Typedef(t) => {
                self.nonenum_typenames
                    .insert(t.newname.clone().into_boxed_str());
            }
            Lvl1Item::Xidtype(x) => {
                self.nonenum_typenames
                    .insert(x.name.clone().into_boxed_str());
            }
            Lvl1Item::XidUnion(x) => {
                self.nonenum_typenames
                    .insert(x.name.clone().into_boxed_str());
            }
            Lvl1Item::Struct(s) => {
                self.nonenum_typenames
                    .insert(s.name.clone().into_boxed_str());
            }
            Lvl1Item::Request(r) => {
                self.nonenum_typenames
                    .insert(r.base.name.clone().into_boxed_str());
            }
            Lvl1Item::Event(e) => {
                self.nonenum_typenames
                    .insert(e.base.name.clone().into_boxed_str());
            }
            Lvl1Item::Error(e) => {
                self.nonenum_typenames
                    .insert(e.base.name.clone().into_boxed_str());
            }
            _ => (),
        });
    }

    /// Tell if we have a name.
    #[inline]
    pub fn has_typename(&self, tn: &str) -> bool {
        self.nonenum_typenames.contains(tn)
    }

    /// Recognize the enums as unresolved.
    #[inline]
    pub fn load_enums(&mut self, items_source: &mut Vec<Lvl1Item>) {
        let items = mem::take(items_source);
        let items = items
            .into_iter()
            .filter_map(|i| match i {
                Lvl1Item::Enum(e) => {
                    let (mn, gen) = create_generator(self, e);
                    self.unresolved_enums.insert(mn, gen);
                    None
                }
                i => Some(i),
            })
            .collect();
        *items_source = items;
    }

    /// Instantly resolve all enums.
    #[inline]
    pub fn resolve_enums(&mut self) -> Vec<Lvl2Item> {
        let unresolved_enums = std::mem::replace(&mut self.unresolved_enums, BTreeMap::new());
        unresolved_enums
            .into_iter()
            .map(|(_, ue)| Lvl2Item::Enum((ue)(Type::BasicType("i32".into()))))
            .collect()
    }

    /// Convert a set of fields.
    #[inline]
    fn convert_fields(
        &mut self,
        mut fields: TinyVec<[crate::lvl1::StructureItem; 6]>,
        variant: StructVariant,
        fds: &mut Vec<String>,
    ) -> (TinyVec<[StructureItem; 6]>, TinyVec<[Lvl2Item; 1]>) {
        let mut side_effect_enums = TinyVec::new();
        let mut align_indices: BTreeMap<usize, usize> = BTreeMap::new();
        let mut index: usize = 0;

        fields.retain(|i| {
            if let Lvl1StructureItem::Padding {
                is_align: true,
                bytes,
            } = i
            {
                align_indices.insert(index - 1, *bytes);
                false
            } else {
                index += 1;
                true
            }
        });

        let mut fields = fields
            .into_iter()
            .enumerate()
            .flat_map(|(i, f)| {
                let mut resolution = None;
                let res = StructureItem::from_lvl1(f, &mut resolution, fds);

                if let Some((idname, ty)) = resolution {
                    if let Some(gen) = self.unresolved_enums.remove(&idname) {
                        side_effect_enums
                            .push(Item::Enum((gen)(Type::BasicType(enum_repr_conv(ty)))));
                    }
                }
                res.into_iter().map(move |r| (i, r))
            })
            .map(|(i, mut f)| {
                if let StructureItem::List(ref mut l) = f {
                    if let Some(b) = align_indices.get(&i) {
                        l.padding = Some(*b);
                    }
                }

                f
            })
            .collect();

        // configure the fields
        configure_fields(&mut fields, variant);

        // normalize the fields
        normalize_fields(&mut fields);

        // uniqueify the fields
        uniquify_fields(&mut fields);

        (fields, side_effect_enums)
    }

    /// Convert a Lvl1 Item to an Lvl2 Item.
    #[inline]
    pub fn convert_item(&mut self, item: Lvl1Item) -> Option<TinyVec<[Item; 1]>> {
        let mut fds: Vec<String> = vec![];
        match item {
            // imports and typedefs are directly used in lvl2
            Lvl1Item::Import(i) => Some(TinyVec::from([Item::Import(i)])),
            // however, make sure typedef names are camel-case'd
            Lvl1Item::Typedef(crate::lvl1::Typedef { oldname, newname }) => {
                Some(TinyVec::from([Item::Typedef(crate::lvl1::Typedef {
                    oldname: oldname.to_camel_case(),
                    newname: newname.to_camel_case(),
                })]))
            }
            // xidtypes are basically the same
            Lvl1Item::Xidtype(crate::lvl1::Xidtype { name }) => {
                let name = safe_name(name.to_camel_case()).into_boxed_str();
                self.xidtypes.push(name.clone());
                Some(TinyVec::from([Item::XidType(XidType {
                    name,
                    from_impls: tiny_vec![],
                })]))
            }
            // xidunions translate to xidtypes as well
            Lvl1Item::XidUnion(crate::lvl1::XidUnion { name, members }) => {
                let name = safe_name(name.to_camel_case()).into_boxed_str();
                self.xidtypes.push(name.clone());
                Some(TinyVec::from([Item::XidType(XidType {
                    name,
                    from_impls: members
                        .into_iter()
                        .map(|m| Some(m.to_camel_case().into_boxed_str()))
                        .collect(),
                })]))
            }
            // enums should aleady be handled
            Lvl1Item::Enum(_) => unreachable!(),
            // structs translate pretty directly
            Lvl1Item::Struct(XStruct { name, fields, docs }) => {
                let (fields, se) = self.convert_fields(fields, StructVariant::No, &mut fds);
                let (brief, desc) = (None, None);
                let name = safe_name(name.to_camel_case()).into_boxed_str();
                let mut tv = TinyVec::from([Item::Struct(Struct {
                    name,
                    brief,
                    fds,
                    desc,
                    fields: fields.to_vec(),
                    special: StructSpecial::Regular,
                })]);
                tv.extend(se);
                Some(tv)
            }
            // requests translate to structs
            Lvl1Item::Request(crate::lvl1::Request {
                base: XStruct { name, fields, docs },
                opcode,
                reply,
            }) => {
                let (fields, mut se) =
                    self.convert_fields(fields, StructVariant::Request, &mut fds);
                let (brief, desc) = (None, None);
                let name = safe_name(name.to_camel_case());

                let reply = match reply {
                    Some(XStruct { name, fields, docs }) => {
                        let mut fds2: Vec<String> = vec![];
                        let (fields2, se2) =
                            self.convert_fields(fields, StructVariant::Reply, &mut fds2);
                        let (brief, desc) = (None, None);

                        se.extend(se2);

                        crate::any_field_length(&fields2);

                        Some(Box::new(Struct {
                            name: name.to_camel_case().into_boxed_str(),
                            brief,
                            desc,
                            fds: fds2,
                            fields: fields2.to_vec(),
                            special: StructSpecial::Regular,
                        }))
                    }
                    None => None,
                };

                let mut tv = TinyVec::from([Item::Struct(Struct {
                    name: name.into_boxed_str(),
                    brief,
                    desc,
                    fds,
                    fields: fields.to_vec(),
                    special: StructSpecial::Request(opcode, reply),
                })]);
                tv.extend(se);
                Some(tv)
            }
            Lvl1Item::Event(crate::lvl1::Event {
                base: XStruct { name, docs, fields },
                opcode,
                skip_sequence,
            }) => {
                let (brief, desc) = (None, None);
                let (fields, se) =
                    self.convert_fields(fields, StructVariant::Event(skip_sequence), &mut fds);
                let sname = safe_name(name.to_camel_case()).into_boxed_str();

                self.events.insert(
                    name.into_boxed_str(),
                    Struct {
                        name: sname,
                        brief,
                        desc,
                        fds,
                        fields: fields.to_vec(),
                        special: StructSpecial::Event(opcode, skip_sequence),
                    },
                );
                Some(se)
            }
            Lvl1Item::EventCopy(crate::lvl1::EventCopy { name, opcode, base }) => {
                if let Some(event) = self.events.get(base.as_str()) {
                    let mut event = event.clone();
                    if let StructSpecial::Event(ref mut o, _) = event.special {
                        *o = opcode;
                    }
                    event.name = safe_name(name.to_camel_case()).into_boxed_str();
                    self.events.insert(name.into_boxed_str(), event);
                }
                None
            }
            Lvl1Item::Error(crate::lvl1::XError {
                base: XStruct { name, fields, docs },
                number,
            }) => {
                let (brief, desc) = (None, None);
                let (fields, se) = self.convert_fields(fields, StructVariant::Error, &mut fds);
                let sname = safe_name(name.to_camel_case()).into_boxed_str();
                self.errors.insert(
                    name.into_boxed_str(),
                    Struct {
                        name: sname,
                        fields: fields.to_vec(),
                        brief,
                        fds,
                        desc,
                        special: StructSpecial::Error(number),
                    },
                );
                Some(se)
            }
            Lvl1Item::ErrorCopy(crate::lvl1::XErrorCopy { name, number, base }) => {
                if let Some(error) = self.errors.get(base.as_str()) {
                    let mut error = error.clone();
                    if let StructSpecial::Error(ref mut o) = error.special {
                        *o = number as _;
                    }
                    error.name = safe_name(name.to_camel_case()).into_boxed_str();
                    self.errors.insert(name.into_boxed_str(), error);
                }

                None
            }
        }
    }
}

/// Go over each field an eliminate unnecessary ones, such as fields expressing the
/// length of vectors.
#[inline]
fn normalize_fields(fields: &mut [StructureItem]) {
    for i in 0..fields.len() {
        // if the item is a non-zero fixed-size list, replace it with an array type
        if let StructureItem::List(l) = &mut fields[i] {
            if let Some(array_length) = l.list_length.fixed_size() {
                if array_length != 0 {
                    let list = if let StructureItem::List(list) = mem::take(&mut fields[i]) {
                        list
                    } else {
                        unreachable!()
                    };
                    let List { name, ty, .. } = list;
                    fields[i] = StructureItem::Field(Field {
                        name,
                        ty: Type::Array(
                            match ty {
                                MaybeString::NotAString(Type::BasicType(t)) => t,
                                MaybeString::IsAString => "c_char".into(),
                                _ => unreachable!("Should be a normal field"),
                            },
                            array_length as u64,
                        ),
                        ..Default::default()
                    });
                } else {
                    l.list_length = Expression::remainder();
                }
            } else if let Some(item) = l.list_length.single_item() {
                if item == "length" {
                    continue;
                }

                let lname = l.name.clone();
                let item = item.to_owned();

                // check and make sure none of the other lists in this series have this item
                // as a single item
                if fields.iter().any(|f| {
                    if let StructureItem::List(l) = f {
                        if l.name != lname && l.list_length.involves_field(&item) {
                            return true;
                        }
                    }

                    false
                }) {
                    log::info!("Found duplicate single item: {}", &item);
                    continue;
                }

                // if this is a single-item list length, axe that single item and
                // just use Vec::len() to calculate length
                fields.iter_mut().any(move |f| {
                    if let StructureItem::Field(Field { name, ty, .. }) = f {
                        if name.as_str() == item {
                            let ty = mem::take(ty);
                            *f = StructureItem::LenSlot {
                                ty,
                                owning_list: lname.to_snake_case(),
                            };

                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });
            }
        }
    }
}

/// Iterate over a set of fields and ensure all of the names are unique (for bitcases).
#[inline]
pub fn uniquify_fields(fields: &mut [StructureItem]) {
    for i in 0..fields.len() {
        let fname = match fields[i] {
            StructureItem::Field(Field { ref name, .. })
            | StructureItem::List(List { ref name, .. }) => name.clone(),
            _ => continue,
        };

        fields.iter_mut().fold(0, |mut field_index, field| {
            match field {
                StructureItem::Field(Field { ref mut name, .. })
                | StructureItem::List(List { ref mut name, .. }) => {
                    if name.as_str() == fname.as_str() {
                        *name = fname
                            .chars()
                            .chain(iter::repeat('_').take(field_index))
                            .collect::<String>();
                        field_index += 1;
                    }
                }
                _ => (),
            }

            field_index
        });
    }
}

/// Convert a series of Level 2 items to Level 1 items.
#[inline]
pub fn convert_series(
    mut series: Vec<Lvl1Item>,
    is_extension: bool,
) -> (Vec<Lvl2Item>, Vec<Box<str>>) {
    set_is_extension(is_extension);

    // first, glob all of the enums
    let mut state = Lvl2State::new();
    state.register_typenames(&series);
    state.load_enums(&mut series);

    // then, preform conversions
    let mut res: Vec<Lvl2Item> = series
        .into_iter()
        .filter_map(|i| state.convert_item(i))
        .flatten()
        .collect();

    // now, take all of the aux. stuff
    let errors = mem::replace(&mut state.errors, BTreeMap::new());
    let events = mem::replace(&mut state.events, BTreeMap::new());

    res.extend(state.resolve_enums());
    res.extend(errors.into_iter().map(|(_k, v)| Item::Struct(v)));
    res.extend(events.into_iter().map(|(_k, v)| Item::Struct(v)));

    (res, state.xidtypes)
}
