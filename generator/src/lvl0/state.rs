// MIT/Apache2 License

use super::xml::get_attributes;
use crate::lvl1::{Docs, EnumVariant, Item, ListLength, StructureItem, XStruct};
use quick_xml::events::{attributes::Attribute, BytesStart, Event};
use std::{mem, ops::DerefMut};
use tinyvec::{tiny_vec, TinyVec};

/// The current state of the parser. Describes which element is currently being parsed.
#[derive(Debug)]
pub enum Lvl0State {
    /// Awaiting for a top-level tag.
    AwaitingTopLevel,
    /// Reading in the text of an import.
    Import,
    /// Reading in XidUnion types.
    XidUnion(String, TinyVec<[String; 2]>),
    /// Reading in individual XidUnion type.
    XidUnionType(String, TinyVec<[String; 2]>),
    /// Reading in enum variants.
    Enum(String, TinyVec<[EnumVariant; 4]>, Option<Docs>),
    /// Reading in from an item.
    EnumItem(String, Box<Lvl0State>),
    /// Reading in from a value or bits tag. True if bits, false if value.
    EnumItemValue(bool, Box<Lvl0State>),
    /// Reading in structural fields for anything that needs that.
    StructLike(StructLike, TinyVec<[StructureItem; 6]>, Option<Docs>),
    /// Calculating the length of a list. Name, type, and handle to the previous item.
    ListField(String, String, Option<ListLength>, Box<Lvl0State>),
    /// Calculating the length of a list, given a fieldref.
    ListLengthFieldRef(Box<Lvl0State>),
    /// Calculating the length of a list, given a value.
    ListLengthValue(Box<Lvl0State>),
    /// Calculating the length of a list, given a unary operation.
    ListLengthUnary(String, Option<ListLength>, Box<Lvl0State>),
    /// Calculating the length of a list, given a binary operation.
    ListLengthBinary(
        String,
        Option<ListLength>,
        Option<ListLength>,
        Box<Lvl0State>,
    ),
    /// Beginning documentation of an item.
    Docs(Docs, Box<Lvl0State>),
}

impl Default for Lvl0State {
    #[inline]
    fn default() -> Self {
        Self::AwaitingTopLevel
    }
}

/// A target of structural fields.
#[derive(Debug)]
pub enum StructLike {
    /// Reading in a basic struct.
    Struct(String),
    /// Reading in an event.
    Event(String, u64),
    /// Reading in an error.
    Error(String, u64),
    /// Reading in a request.
    Request(String, u64, Option<XStruct>),
    /// Reading in a reply belonging to a request.
    Reply(Box<Lvl0State>),
}

impl StructLike {
    /// Get the end tag we're waiting for.
    #[inline]
    pub fn end_tag(&self) -> &'static [u8] {
        match self {
            Self::Struct(_) => b"struct",
            Self::Event(_, _) => b"event",
            Self::Error(_, _) => b"error",
            Self::Request(_, _, _) => b"request",
            Self::Reply(_) => b"reply",
        }
    }
}

impl Lvl0State {
    /// Get the mutable reference to the list length variable.
    #[inline]
    fn list_length_ref(&mut self) -> &mut Option<ListLength> {
        match self {
            Self::ListField(_, _, ll, _) => ll,
            Self::ListLengthUnary(_, ll, _) => ll,
            Self::ListLengthBinary(_, ll1, ll2, _) => {
                if ll1.is_none() {
                    ll1
                } else {
                    ll2
                }
            }
            _ => unreachable!("Called list_length_ref on non LL-item"),
        }
    }

    /// React to a quick-xml event. It may return a Level 1 Item representing what it has read in.
    #[inline]
    pub fn react_to_event<'a>(&mut self, event: Event<'a>) -> Option<Item> {
        match self {
            Self::AwaitingTopLevel => {
                // we're waiting for a top-level event
                // let's read it in
                match event {
                    Event::Start(b) => {
                        // tell what kind of tag we're reading in, and roll based on that,
                        self.match_toplevel(b, false)
                    }
                    Event::Empty(b) => self.match_toplevel(b, true),
                    _ => None,
                }
            }
            Self::Import => {
                // we're waiting for text
                match event {
                    Event::Text(t) => {
                        let target = std::str::from_utf8(&*t.unescaped().ok()?).ok()?.to_owned();
                        *self = Self::AwaitingTopLevel;
                        Some(Item::Import(crate::lvl1::Import(target)))
                    }
                    _ => None,
                }
            }
            Self::XidUnion(name, sublist) => {
                // await for the start of the type element or the end of the list
                match event {
                    Event::Start(b) | Event::Empty(b) => {
                        // if we've encountered a new type, recurse into XidUnionType
                        if let b"type" = b.name() {
                            let name = mem::take(name);
                            let sublist = mem::take(sublist);
                            *self = Self::XidUnionType(name, sublist);
                        }

                        None
                    }
                    Event::End(e) => {
                        if let b"xidunion" = e.name() {
                            let name = mem::take(name);
                            let sublist = mem::take(sublist);
                            *self = Self::AwaitingTopLevel;
                            Some(Item::XidUnion(crate::lvl1::XidUnion {
                                name,
                                members: sublist,
                            }))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            Self::XidUnionType(name, sublist) => {
                if let Event::Text(t) = event {
                    sublist.push(std::str::from_utf8(&*t.unescaped().ok()?).ok()?.to_owned());
                    let name = mem::take(name);
                    let sublist = mem::take(sublist);
                    *self = Self::XidUnion(name, sublist);
                }

                None
            }
            Self::Enum(name, variants, docs) => {
                // searching for variants, the <item> tag
                match event {
                    Event::Start(b) | Event::Empty(b) => {
                        if b.name() == b"item" {
                            let name = get_attributes(&b, &[b"name".as_ref()], &[true])?
                                .remove(b"name".as_ref())?;
                            let mut state = Self::EnumItem(name, Box::new(Default::default()));
                            mem::swap(self, &mut state);
                            if let Self::EnumItem(_, base) = self {
                                *base = Box::new(state);
                            }
                        } else if b.name() == b"doc" {
                            // begin doc mode
                            let mut base = mem::replace(
                                self,
                                Self::Docs(Default::default(), Box::new(Default::default())),
                            );
                            if let Self::Docs(_, sbase) = self {
                                mem::swap(&mut base, &mut *sbase);
                            }
                        }
                    }
                    Event::End(e) => {
                        if e.name() == b"enum" {
                            let name = mem::take(name);
                            let variants = mem::take(variants);
                            let docs = mem::take(docs);
                            *self = Self::AwaitingTopLevel;
                            return Some(Item::Enum(crate::lvl1::XEnum {
                                name,
                                variants,
                                docs,
                            }));
                        }
                    }
                    _ => (),
                }

                None
            }
            Self::EnumItem(name, base) => {
                // searching for <value> or <bits>
                if let Event::Start(b) = event {
                    let t = match b.name() {
                        b"value" => false,
                        b"bit" => true,
                        _ => return None,
                    };

                    let mut state = Self::EnumItemValue(t, Box::new(Default::default()));
                    mem::swap(self, &mut state);
                    if let Self::EnumItemValue(_, base) = self {
                        *base = Box::new(state);
                    }
                }

                None
            }
            Self::EnumItemValue(bits, base) => {
                // searching for text in the bits
                if let Event::Text(t) = event {
                    if let Self::EnumItem(vname, base) = base.deref_mut() {
                        if let Self::Enum(ename, variants, docs) = base.deref_mut() {
                            let number = std::str::from_utf8(&*t.unescaped().ok()?)
                                .ok()?
                                .parse()
                                .ok()?;
                            let variant = EnumVariant {
                                name: mem::take(vname),
                                data: match *bits {
                                    true => crate::lvl1::EnumData::Bit(number),
                                    false => crate::lvl1::EnumData::Value(number),
                                },
                            };

                            variants.push(variant);
                            let ename = mem::take(ename);
                            let variants = mem::take(variants);
                            let docs = mem::take(docs);

                            *self = Self::Enum(ename, variants, docs);
                        }
                    }
                }

                None
            }
            Self::StructLike(sl, fields, docs) => {
                if let Event::End(ref e) = event {
                    match sl {
                        StructLike::Request(name, opcode, reply) => {
                            if e.name() == b"request" {
                                let name = mem::take(name);
                                let opcode = mem::take(opcode);
                                let fields = mem::take(fields);
                                let reply = mem::take(reply);
                                let docs = mem::take(docs);

                                *self = Self::AwaitingTopLevel;
                                return Some(Item::Request(crate::lvl1::Request {
                                    base: crate::lvl1::XStruct { name, fields, docs },
                                    opcode,
                                    reply,
                                }));
                            }
                        }

                        StructLike::Reply(base) => {
                            if e.name() == b"reply" {
                                let fields = mem::take(fields);
                                let docs = mem::take(docs);
                                let mut reply = Some(XStruct {
                                    name: String::new(),
                                    fields,
                                    docs,
                                });

                                let base = mem::take(base);
                                *self = *base;

                                if let Self::StructLike(StructLike::Request(_, _, repl), _, _) =
                                    self
                                {
                                    *repl = reply;
                                }

                                return None;
                            }
                        }

                        StructLike::Struct(name) => {
                            if e.name() == b"struct" {
                                let name = mem::take(name);
                                let fields = mem::take(fields);
                                let docs = mem::take(docs);

                                *self = Self::AwaitingTopLevel;
                                return Some(Item::Struct(XStruct { name, fields, docs }));
                            }
                        }
                        StructLike::Event(name, number) => {
                            if e.name() == b"event" {
                                let name = mem::take(name);
                                let fields = mem::take(fields);
                                let docs = mem::take(docs);
                                let opcode = *number;

                                *self = Self::AwaitingTopLevel;
                                return Some(Item::Event(crate::lvl1::Event {
                                    base: XStruct { name, fields, docs },
                                    opcode,
                                }));
                            }
                        }
                        StructLike::Error(name, number) => {
                            if e.name() == b"error" {
                                let name = mem::take(name);
                                let fields = mem::take(fields);
                                let docs = mem::take(docs);
                                let number = *number;

                                *self = Self::AwaitingTopLevel;
                                return Some(Item::Error(crate::lvl1::XError {
                                    base: XStruct { name, fields, docs },
                                    number,
                                }));
                            }
                        }
                    }
                }

                self.struct_like(event)
            }
            Self::ListField(name, ty, ll, base) => {
                // we are constructing a list field
                match event {
                    Event::Start(b) => self.handle_list_item(b)?,
                    Event::End(e) => {
                        if e.name() == b"list" {
                            let mut base = mem::take(base);
                            let ll = ll.take().expect("Failed to construct list");

                            let listfield = StructureItem::List(crate::lvl1::List {
                                name: mem::take(name),
                                ty: mem::take(ty),
                                list_length: ll,
                            });

                            if let Self::StructLike(_, fields, _) = &mut *base {
                                fields.push(listfield);
                            }

                            *self = *base;
                        }
                    }
                    _ => (),
                }

                None
            }
            Self::ListLengthFieldRef(base) => {
                // check for text
                match event {
                    Event::Text(t) => {
                        let t = std::str::from_utf8(&*t.unescaped().ok()?).ok()?.to_owned();
                        let mut base = mem::take(base);
                        *base.list_length_ref() = Some(crate::lvl1::ListLength::FieldReference(t));
                        *self = *base;
                    }
                    _ => (),
                }

                None
            }
            Self::ListLengthValue(base) => {
                // check for a number
                match event {
                    Event::Text(t) => {
                        let v = std::str::from_utf8(&*t.unescaped().ok()?)
                            .ok()?
                            .parse()
                            .ok()?;
                        let mut base = mem::take(base);
                        *base.list_length_ref() = Some(crate::lvl1::ListLength::Value(v));
                        *self = *base;
                    }
                    _ => (),
                }

                None
            }
            Self::ListLengthBinary(op, ll1, ll2, base) => {
                match event {
                    Event::Start(b) => self.handle_list_item(b)?,
                    Event::End(e) => {
                        if e.name() == b"op" {
                            if ll2.is_some() {
                                let op = mem::take(op);
                                let ll1 = ll1.take().expect("Failed to construct LL operation");
                                let ll2 = ll2.take().expect("Failed to construct LL operation");
                                let mut base = mem::take(base);

                                // read our operation into the base

                                *base.list_length_ref() = Some(crate::lvl1::ListLength::BinaryOp {
                                    op,
                                    left: Box::new(ll1),
                                    right: Box::new(ll2),
                                });

                                *self = *base;
                            } else {
                                log::error!("Failed to construct binary LL operation");
                                return None;
                            }
                        }
                    }
                    _ => (),
                }

                None
            }
            Self::ListLengthUnary(op, ll, base) => {
                match event {
                    Event::Start(b) => self.handle_list_item(b)?,
                    Event::End(e) => {
                        if e.name() == b"unop" {
                            let op = mem::take(op);
                            let ll = ll.take().expect("Failed to construct unary operation");
                            let mut base = mem::take(base);

                            // read out operation into the base
                            *base.list_length_ref() = Some(crate::lvl1::ListLength::UnaryOp {
                                op,
                                target: Box::new(ll),
                            });

                            *self = *base;
                        }
                    }
                    _ => (),
                }

                None
            }
            Self::Docs(docs, base) => {
                // TODO: not skip the docs
                match event {
                    Event::End(e) => {
                        if e.name() == b"doc" {
                            let mut mbase: Self = Default::default();
                            mem::swap(&mut mbase, &mut *base);
                            *self = mbase;
                        }
                    }
                    _ => (),
                }

                None
            }
        }
    }

    #[inline]
    fn struct_like<'a>(&mut self, event: Event<'a>) -> Option<Item> {
        if let Self::StructLike(_, fields, _) = self {
            // see if this is a field or padding
            match event {
                Event::Start(ref b) | Event::Empty(ref b) => {
                    match b.name() {
                        b"field" => {
                            // this is an ordinary field, get the name and type
                            let mut map = get_attributes(
                                &b,
                                &[
                                    b"name".as_ref(),
                                    b"type".as_ref(),
                                    b"enum".as_ref(),
                                    b"mask".as_ref(),
                                    b"altenum".as_ref(),
                                ],
                                &[true, true, false, false, false],
                            )?;
                            let name = map.remove(b"name".as_ref()).unwrap();
                            let ty = map.remove(b"type".as_ref()).unwrap();
                            let enumeration = map.remove(b"enum".as_ref());
                            let mask = map.remove(b"mask".as_ref());
                            let alt_enum = map.remove(b"altenum".as_ref());

                            fields.push(StructureItem::Field(crate::lvl1::Field {
                                name,
                                ty,
                                enumeration,
                                mask,
                                alt_enum,
                            }));
                        }
                        b"pad" => {
                            // this is padding, get the number of bytes
                            let mut map = get_attributes(
                                &b,
                                &[b"bytes".as_ref(), b"align".as_ref()],
                                &[false, false],
                            )?;
                            let mut align_to = false;
                            let bytes = if let Some(bytes) = map
                                .remove(b"bytes".as_ref())
                                .and_then(|x| x.parse::<usize>().ok())
                            {
                                bytes
                            } else if let Some(bytes) = map
                                .remove(b"align".as_ref())
                                .and_then(|x| x.parse::<usize>().ok())
                            {
                                align_to = true;
                                bytes
                            } else {
                                panic!("Bad padding")
                            };
                            fields.push(StructureItem::Padding {
                                bytes,
                                is_align: align_to,
                            });
                        }
                        b"valueparam" => {
                            // this is a mask value parameter
                            let mut map = get_attributes(
                                &b,
                                &[
                                    b"value-mask-type".as_ref(),
                                    b"value-mask-name".as_ref(),
                                    b"value-list-name".as_ref(),
                                ],
                                &[true, true, true],
                            )?;
                            let mask_ty = map.remove(b"value-mask-type".as_ref()).unwrap();
                            let mask_name = map.remove(b"value-mask-name".as_ref()).unwrap();
                            let list_name = map.remove(b"value-list-name".as_ref()).unwrap();

                            fields.push(StructureItem::ValueParam(crate::lvl1::ValueParam {
                                mask_ty,
                                mask_name,
                                list_name,
                            }));
                        }
                        b"list" => {
                            // we're starting a list field, shame.
                            let mut map = get_attributes(
                                &b,
                                &[b"name".as_ref(), b"type".as_ref()],
                                &[true, true],
                            )?;
                            let name = map.remove(b"name".as_ref()).unwrap();
                            let ty = map.remove(b"type".as_ref()).unwrap();

                            if let Event::Empty(_) = event {
                                fields.push(StructureItem::List(crate::lvl1::List {
                                    name,
                                    ty,
                                    list_length: Default::default(),
                                }));
                            } else {
                                let base = Box::new(mem::take(self));
                                *self = Self::ListField(name, ty, None, base);
                            }
                        }
                        b"doc" => {
                            // begin documentation mode
                            let mut base = mem::replace(
                                self,
                                Self::Docs(Default::default(), Box::new(Default::default())),
                            );
                            if let Self::Docs(_, sbase) = self {
                                mem::swap(&mut base, &mut *sbase);
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        None
    }

    #[inline]
    fn handle_list_item<'a>(&mut self, b: BytesStart<'a>) -> Option<()> {
        match b.name() {
            // replace self with various list types
            b"fieldref" => {
                let mut base =
                    mem::replace(self, Self::ListLengthFieldRef(Box::new(Default::default())));
                if let Self::ListLengthFieldRef(sbase) = self {
                    mem::swap(&mut base, &mut *sbase);
                }
            }
            b"value" => {
                let mut base =
                    mem::replace(self, Self::ListLengthValue(Box::new(Default::default())));
                if let Self::ListLengthValue(sbase) = self {
                    mem::swap(&mut base, &mut *sbase);
                }
            }
            b"op" => {
                let op = get_attributes(&b, &[b"op".as_ref()], &[true])?
                    .remove(b"op".as_ref())
                    .unwrap();
                let mut base = mem::replace(
                    self,
                    Self::ListLengthBinary(op, None, None, Box::new(Default::default())),
                );
                if let Self::ListLengthBinary(_, _, _, sbase) = self {
                    mem::swap(&mut base, &mut *sbase);
                }
            }
            b"unop" => {
                let op = get_attributes(&b, &[b"op".as_ref()], &[true])?
                    .remove(b"op".as_ref())
                    .unwrap();
                let mut base = mem::replace(
                    self,
                    Self::ListLengthUnary(op, None, Box::new(Default::default())),
                );
                if let Self::ListLengthUnary(_, _, sbase) = self {
                    mem::swap(&mut base, &mut *sbase);
                }
            }
            _ => {
                log::warn!(
                    "Unexpected list length item for {:#?}: {}",
                    self,
                    std::str::from_utf8(b.name()).unwrap()
                );
            }
        }

        None
    }

    #[inline]
    fn match_toplevel<'a>(&mut self, b: BytesStart<'a>, is_empty: bool) -> Option<Item> {
        match b.name() {
            b"typedef" => {
                // type definition element
                let mut map = get_attributes(
                    &b,
                    &[b"oldname".as_ref(), b"newname".as_ref()],
                    &[true, true],
                )?;
                let oldname = map.remove(b"oldname".as_ref()).unwrap();
                let newname = map.remove(b"newname".as_ref()).unwrap();
                Some(Item::Typedef(crate::lvl1::Typedef { oldname, newname }))
            }
            b"xidtype" => {
                let name = get_attributes(&b, &[b"name".as_ref()], &[true])?
                    .remove(b"name".as_ref())
                    .unwrap();
                Some(Item::Xidtype(crate::lvl1::Xidtype { name }))
            }
            b"import" => {
                // change machine to await import input
                *self = Lvl0State::Import;
                None
            }
            b"xidunion" => {
                // change machine to await XidUnion input types
                let name = get_attributes(&b, &[b"name".as_ref()], &[true])?
                    .remove(b"name".as_ref())
                    .unwrap();
                *self = Lvl0State::XidUnion(name, tiny_vec![]);
                None
            }
            b"struct" => {
                // change state machine to await field inputs for structs
                let name = get_attributes(&b, &[b"name".as_ref()], &[true])?
                    .remove(b"name".as_ref())
                    .unwrap();
                *self = Lvl0State::StructLike(StructLike::Struct(name), tiny_vec![], None);
                None
            }
            b"enum" => {
                // change state machine to await enum variants
                let name = get_attributes(&b, &[b"name".as_ref()], &[true])?
                    .remove(b"name".as_ref())
                    .unwrap();
                *self = Lvl0State::Enum(name, tiny_vec![], None);
                None
            }
            b"event" => {
                // change state machine to await event inputs
                let mut map =
                    get_attributes(&b, &[b"name".as_ref(), b"number".as_ref()], &[true, true])?;
                let name = map.remove(b"name".as_ref()).unwrap();
                let number = map.remove(b"number".as_ref()).unwrap().parse().ok()?;

                *self = Lvl0State::StructLike(StructLike::Event(name, number), tiny_vec![], None);
                None
            }
            b"error" => {
                // change state machine to await error inputs
                let mut map =
                    get_attributes(&b, &[b"name".as_ref(), b"number".as_ref()], &[true, true])?;
                let name = map.remove(b"name".as_ref()).unwrap();
                let number = map.remove(b"number".as_ref()).unwrap().parse().ok()?;

                *self = Lvl0State::StructLike(StructLike::Error(name, number), tiny_vec![], None);
                None
            }
            b"request" => {
                // change state machine to await request inputs
                let mut map =
                    get_attributes(&b, &[b"name".as_ref(), b"opcode".as_ref()], &[true, true])?;
                let name = map.remove(b"name".as_ref()).unwrap();
                let opcode = map.remove(b"opcode".as_ref()).unwrap().parse().ok()?;

                if !is_empty {
                    *self = Lvl0State::StructLike(
                        StructLike::Request(name, opcode, None),
                        tiny_vec![],
                        None,
                    );
                    None
                } else {
                    Some(Item::Request(crate::lvl1::Request {
                        base: XStruct {
                            name,
                            fields: Default::default(),
                            docs: None,
                        },
                        opcode,
                        reply: None,
                    }))
                }
            }
            b"eventcopy" => {
                // read in eventcopy tag
                let mut map = get_attributes(
                    &b,
                    &[b"name".as_ref(), b"number".as_ref(), b"ref".as_ref()],
                    &[true, true, true],
                )?;
                let name = map.remove(b"name".as_ref()).unwrap();
                let number = map.remove(b"number".as_ref()).unwrap().parse().ok()?;
                let base = map.remove(b"ref".as_ref()).unwrap();

                Some(Item::EventCopy(crate::lvl1::EventCopy {
                    name,
                    opcode: number,
                    base,
                }))
            }
            b"errorcopy" => {
                // read in errorcopy tag
                let mut map = get_attributes(
                    &b,
                    &[b"name".as_ref(), b"number".as_ref(), b"ref".as_ref()],
                    &[true, true, true],
                )?;
                let name = map.remove(b"name".as_ref()).unwrap();
                let number = map.remove(b"number".as_ref()).unwrap().parse().ok()?;
                let base = map.remove(b"ref".as_ref()).unwrap();

                Some(Item::ErrorCopy(crate::lvl1::XErrorCopy {
                    name,
                    number,
                    base,
                }))
            }
            name => {
                log::warn!(
                    "Found invalid tag at top level: {}",
                    String::from_utf8_lossy(name)
                );
                None
            }
        }
    }
}
