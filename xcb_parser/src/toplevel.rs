// MIT/Apache2 License

//! Top level items that appear in the XML file.

use crate::{
    find_attribute, read_text_or_cdata, Enum, EventStruct, Request, Struct, Union, XError, XEvent,
};
use anyhow::{anyhow, Result};
use quick_xml::{
    events::{BytesStart, Event as XmlEvent},
    Reader,
};
use std::io::BufRead;

/// A toplevel item, as provideby the XML file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToplevelItem {
    /// Type alias.
    Typedef { oldname: String, newname: String },
    /// Import from another module.
    Import(String),
    /// An XID type.
    Xidtype(String),
    /// An XID union.
    Xidunion(Xidunion),
    /// A structure.
    Struct(Struct),
    /// A union.
    Union(Union),
    /// A request.
    Request(Request),
    /// An error.
    Error(XError),
    /// An event.
    Event(XEvent),
    /// Copy an error.
    ErrorCopy(MakeCopy),
    /// Copy an event.
    EventCopy(MakeCopy),
    /// An eventstruct.
    EventStruct(EventStruct),
    /// An enumeration.
    Enum(Enum),
}

impl ToplevelItem {
    /// Get a representative type and name for this toplevel item.
    #[inline]
    pub fn name(&self) -> (&'static str, &str) {
        match self {
            Self::Typedef { newname, .. } => ("typedef", newname),
            Self::Import(import) => ("import", import),
            Self::Xidtype(name) => ("xidtype", name),
            Self::Xidunion(xidunion) => ("xidunion", &xidunion.name),
            Self::Struct(struct_) => ("struct", &struct_.name),
            Self::Union(union_) => ("union", &union_.name),
            Self::Request(request) => ("request", &request.name),
            Self::Error(error) => ("error", &error.name),
            Self::Event(event) => ("event", &event.name),
            Self::ErrorCopy(make_copy) => ("error", &make_copy.name),
            Self::EventCopy(make_copy) => ("event", &make_copy.name),
            Self::EventStruct(event_struct) => ("eventstruct", &event_struct.name),
            Self::Enum(enum_) => ("enum", &enum_.name),
        }
    }

    #[inline]
    pub(crate) fn parse(reader: &mut Reader<impl BufRead>) -> Result<Option<Self>> {
        let mut buffer = vec![];

        loop {
            // look for top-level tags and then call the corresponding
            // parse function
            let ev = reader.read_event(&mut buffer)?;
            let is_empty = matches!(ev, XmlEvent::Empty(_));
            match ev {
                XmlEvent::Start(start) | XmlEvent::Empty(start) => match start.name() {
                    b"struct" => {
                        return Ok(Some(ToplevelItem::Struct(Struct::parse(start, reader)?)));
                    }
                    b"union" => {
                        return Ok(Some(ToplevelItem::Union(Union::parse(start, reader)?)));
                    }
                    b"request" => {
                        return Ok(Some(ToplevelItem::Request(Request::parse(
                            start, reader, is_empty,
                        )?)));
                    }
                    b"error" => {
                        return Ok(Some(ToplevelItem::Error(XError::parse(
                            start, reader, is_empty,
                        )?)));
                    }
                    b"event" => {
                        return Ok(Some(ToplevelItem::Event(XEvent::parse(start, reader)?)));
                    }
                    b"errorcopy" => {
                        return Ok(Some(ToplevelItem::ErrorCopy(MakeCopy::parse(
                            start, reader,
                        )?)));
                    }
                    b"eventcopy" => {
                        return Ok(Some(ToplevelItem::EventCopy(MakeCopy::parse(
                            start, reader,
                        )?)));
                    }
                    b"import" => {
                        return Ok(Some(ToplevelItem::Import(read_text_or_cdata(reader)?)));
                    }
                    b"xidtype" => {
                        return Ok(Some(ToplevelItem::Xidtype(find_attribute(
                            &start, b"name", reader,
                        )?)));
                    }
                    b"xidunion" => {
                        return Ok(Some(ToplevelItem::Xidunion(Xidunion::parse(
                            start, reader,
                        )?)));
                    }
                    b"typedef" => {
                        let oldname = find_attribute(&start, b"oldname", reader)?;
                        let newname = find_attribute(&start, b"newname", reader)?;
                        return Ok(Some(ToplevelItem::Typedef { oldname, newname }));
                    }
                    b"enum" => {
                        return Ok(Some(ToplevelItem::Enum(Enum::parse(start, reader)?)));
                    }
                    b"eventstruct" => {
                        return Ok(Some(ToplevelItem::EventStruct(EventStruct::parse(
                            start, reader,
                        )?)));
                    }
                    name => {
                        tracing::warn!(
                            "Unknown name: {}",
                            std::str::from_utf8(name).unwrap_or("<not utf8>")
                        );
                    }
                },
                XmlEvent::End(e) if e.name() == b"xcb" => {
                    return Ok(None);
                }
                XmlEvent::Eof => {
                    return Err(anyhow!("Unexpected EOF"));
                }
                _ => continue,
            }
        }
    }
}

/// Copy an event or error to another slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MakeCopy {
    /// The name of the new item.
    pub name: String,
    /// The name of the item to copy.
    pub ref_: String,
    /// The new number to use.
    pub number: i64,
}

impl MakeCopy {
    #[inline]
    fn parse(start_elem: BytesStart<'_>, reader: &mut Reader<impl BufRead>) -> Result<Self> {
        let name = find_attribute(&start_elem, b"name", reader)?;
        let ref_ = find_attribute(&start_elem, b"ref", reader)?;
        let number = find_attribute(&start_elem, b"number", reader)?;

        Ok(Self {
            name,
            ref_,
            number: number.parse()?,
        })
    }
}

/// An XID union.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xidunion {
    /// The name of the union.
    pub name: String,
    /// The values in the union.
    pub items: Vec<String>,
}

impl Xidunion {
    #[inline]
    fn parse(start_elem: BytesStart<'_>, reader: &mut Reader<impl BufRead>) -> Result<Self> {
        let name = find_attribute(&start_elem, b"name", reader)?;
        let mut buffer = vec![];
        let mut items = vec![];

        loop {
            match reader.read_event(&mut buffer)? {
                XmlEvent::Start(start) if start.name() == b"type" => {
                    items.push(read_text_or_cdata(reader)?);
                }
                XmlEvent::End(end) => {
                    if end.name() == b"xidunion" {
                        break;
                    }
                }
                XmlEvent::Eof => {
                    return Err(anyhow!("Unexpected EOF"));
                }
                _ => {}
            }
        }

        Ok(Self { name, items })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::Reader;

    #[test]
    fn test_parse_typedef() {
        let mut reader = Reader::from_str(
            r#"
            <typedef oldname="xcb_void_cookie_t" newname="xcb_void_cookie"/>
            "#,
        );
        reader.trim_text(true);

        let typedef = ToplevelItem::parse(&mut reader).unwrap().unwrap();
        assert_eq!(
            typedef,
            ToplevelItem::Typedef {
                oldname: "xcb_void_cookie_t".to_string(),
                newname: "xcb_void_cookie".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_import() {
        let mut reader = Reader::from_str(
            r#"
            <import>xinput</import>
            "#,
        );
        reader.trim_text(true);

        let import = ToplevelItem::parse(&mut reader).unwrap().unwrap();
        assert_eq!(import, ToplevelItem::Import("xinput".to_string()));
    }

    #[test]
    fn test_parse_xidtype() {
        let mut reader = Reader::from_str(
            r#"
            <xidtype name="Window"/>
            "#,
        );
        reader.trim_text(true);

        let xidtype = ToplevelItem::parse(&mut reader).unwrap().unwrap();
        assert_eq!(xidtype, ToplevelItem::Xidtype("Window".to_string()));
    }

    #[test]
    fn test_parse_xidunion() {
        let mut reader = Reader::from_str(
            r#"
            <xidunion name="xcb_window_t">
                <type>Window</type>
                <type>Drawable</type>
            </xidunion>
            "#,
        );
        reader.trim_text(true);

        let xidunion = ToplevelItem::parse(&mut reader).unwrap().unwrap();
        assert_eq!(
            xidunion,
            ToplevelItem::Xidunion(Xidunion {
                name: "xcb_window_t".to_string(),
                items: vec!["Window".to_string(), "Drawable".to_string()]
            })
        );
    }

    #[test]
    fn test_parse_errorcopy() {
        let mut reader = Reader::from_str(
            r#"
            <errorcopy name="Match" ref="Base" number="0"/>
            "#,
        );
        reader.trim_text(true);

        let errorcopy = ToplevelItem::parse(&mut reader).unwrap().unwrap();
        assert_eq!(
            errorcopy,
            ToplevelItem::ErrorCopy(MakeCopy {
                name: "Match".to_string(),
                ref_: "Base".to_string(),
                number: 0
            })
        );
    }

    #[test]
    fn test_parse_eventcopy() {
        let mut reader = Reader::from_str(
            r#"
            <eventcopy name="Event" ref="Base" number="23"/>
            "#,
        );
        reader.trim_text(true);

        let eventcopy = ToplevelItem::parse(&mut reader).unwrap().unwrap();
        assert_eq!(
            eventcopy,
            ToplevelItem::EventCopy(MakeCopy {
                name: "Event".to_string(),
                ref_: "Base".to_string(),
                number: 23
            })
        );
    }
}
