// MIT/Apache2 License

use crate::{find_attribute, Docs, Field, StructureContents};
use anyhow::{anyhow, Context, Result};
use quick_xml::{events::BytesStart, Reader};
use std::io::BufRead;

/// An ordinary structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
    /// The name of this structure.
    pub name: String,
    /// The fields in this structure.
    pub fields: Vec<Field>,
    /// Documentation for this structure.
    pub docs: Option<Docs>,
}

impl Struct {
    #[inline]
    pub(crate) fn parse(
        start_elem: BytesStart<'_>,
        reader: &mut Reader<impl BufRead>,
    ) -> Result<Self> {
        let name = find_attribute(&start_elem, b"name", reader)?;
        let StructureContents {
            fields,
            docs,
            reply,
        } = StructureContents::parse(reader, b"struct")
            .with_context(|| format!("reading struct {}", &name))?;

        if reply.is_some() {
            return Err(anyhow!("structs cannot have replies"));
        }

        Ok(Self { name, fields, docs })
    }
}

/// A union of several different types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Union {
    /// The name of this structure.
    pub name: String,
    /// The fields in this structure.
    pub fields: Vec<Field>,
    /// Documentation for this structure.
    pub docs: Option<Docs>,
}

impl Union {
    #[inline]
    pub(crate) fn parse(
        start_elem: BytesStart<'_>,
        reader: &mut Reader<impl BufRead>,
    ) -> Result<Self> {
        let name = find_attribute(&start_elem, b"name", reader)?;
        let StructureContents {
            fields,
            docs,
            reply,
        } = StructureContents::parse(reader, b"union")
            .with_context(|| format!("reading union {}", &name))?;

        if reply.is_some() {
            return Err(anyhow!("unions cannot have replies"));
        }

        Ok(Self { name, fields, docs })
    }
}

/// A request that can be sent across the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    /// The name of this structure.
    pub name: String,
    /// The fields in this structure.
    pub fields: Vec<Field>,
    /// Documentation for this structure.
    pub docs: Option<Docs>,
    /// The opcode for this request.
    pub opcode: i64,
    /// Are we able to combine adjacent structures effortlessly?
    pub combine_adjacent: bool,
    /// The reply to this request, if applicable.
    pub reply: Option<(Vec<Field>, Option<Docs>)>,
}

impl Request {
    #[inline]
    pub(crate) fn parse(
        start_elem: BytesStart<'_>,
        reader: &mut Reader<impl BufRead>,
        is_empty: bool,
    ) -> Result<Self> {
        let mut name = None;
        let mut opcode = None;
        let mut combine_adjacent = false;

        for attr in start_elem.attributes() {
            let attr = attr?;
            match attr.key {
                b"name" => name = Some(attr.unescape_and_decode_value(reader)?),
                b"opcode" => opcode = Some(attr.unescape_and_decode_value(reader)?),
                b"combine-adjacent" => {
                    combine_adjacent = attr.unescape_and_decode_value(reader)? == "true"
                }
                _ => continue,
            }
        }

        let (fields, docs, reply) = if is_empty {
            (vec![], None, None)
        } else {
            let StructureContents {
                fields,
                docs,
                reply,
            } = StructureContents::parse(reader, b"request")
                .with_context(|| format!("reading request {}", name.as_ref().unwrap()))?;
            (fields, docs, reply)
        };

        Ok(Self {
            name: name.ok_or_else(|| anyhow!("Missing name attribute"))?,
            fields,
            docs,
            opcode: opcode
                .ok_or_else(|| anyhow!("Missing opcode attribute"))?
                .parse()?,
            combine_adjacent,
            reply: reply.map(|sc| {
                let StructureContents { fields, docs, .. } = *sc;
                (fields, docs)
            }),
        })
    }
}

/// An event that can be sent across the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XEvent {
    /// The name of this structure.
    pub name: String,
    /// The fields in this structure.
    pub fields: Vec<Field>,
    /// Documentation for this structure.
    pub docs: Option<Docs>,
    /// The number necessary.
    pub number: i64,
    /// Do we omit the sequence number?
    pub no_sequence_number: bool,
    /// Is this an X Generic Event?
    pub xge: bool,
}

impl XEvent {
    #[inline]
    pub(crate) fn parse(
        start_elem: BytesStart<'_>,
        reader: &mut Reader<impl BufRead>,
    ) -> Result<Self> {
        let mut name = None;
        let mut number = None;
        let mut no_sequence_number = false;
        let mut xge = false;

        for attr in start_elem.attributes() {
            let attr = attr?;
            match attr.key {
                b"name" => name = Some(attr.unescape_and_decode_value(reader)?),
                b"number" => number = Some(attr.unescape_and_decode_value(reader)?),
                b"no-sequence-number" => {
                    no_sequence_number = attr.unescape_and_decode_value(reader)? == "true"
                }
                b"xge" => xge = attr.unescape_and_decode_value(reader)? == "true",
                _ => continue,
            }
        }

        let StructureContents {
            fields,
            docs,
            reply,
        } = StructureContents::parse(reader, b"event")
            .with_context(|| format!("reading event {}", name.as_ref().unwrap()))?;

        if reply.is_some() {
            return Err(anyhow!("events cannot have replies"));
        }

        Ok(Self {
            name: name.ok_or_else(|| anyhow!("Missing name attribute"))?,
            number: number
                .ok_or_else(|| anyhow!("Missing number attribute"))?
                .parse()?,
            no_sequence_number,
            xge,
            fields,
            docs,
        })
    }
}

/// An error that can be sent across the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XError {
    /// The name of this structure.
    pub name: String,
    /// The fields in this structure.
    pub fields: Vec<Field>,
    /// Documentation for this structure.
    pub docs: Option<Docs>,
    /// The error code for this error.
    pub number: i64,
}

impl XError {
    #[inline]
    pub(crate) fn parse(
        start_elem: BytesStart<'_>,
        reader: &mut Reader<impl BufRead>,
        is_empty: bool,
    ) -> Result<Self> {
        let mut name = None;
        let mut number = None;

        for attr in start_elem.attributes() {
            let attr = attr?;
            match attr.key {
                b"name" => name = Some(attr.unescape_and_decode_value(reader)?),
                b"number" => number = Some(attr.unescape_and_decode_value(reader)?),
                _ => continue,
            }
        }

        let (fields, docs, reply) = if is_empty {
            (vec![], None, None)
        } else {
            let StructureContents {
                fields,
                docs,
                reply,
            } = StructureContents::parse(reader, b"error")
                .with_context(|| format!("reading error {}", name.as_ref().unwrap()))?;

            (fields, docs, reply)
        };

        if reply.is_some() {
            return Err(anyhow!("errors cannot have replies"));
        }

        Ok(Self {
            name: name.ok_or_else(|| anyhow!("Missing name attribute"))?,
            number: number
                .ok_or_else(|| anyhow!("Missing number attribute"))?
                .parse()?,
            fields,
            docs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_start_event;
    use quick_xml::Reader;

    #[test]
    fn test_parse_struct() {
        let mut reader = Reader::from_str(
            r#"
            <struct name="foo">
                <field name="bar" type="int" />
                <field name="baz" type="int" />
            </struct>"#,
        );

        let start_elem = read_start_event(&mut reader);
        let struct_ = Struct::parse(start_elem, &mut reader).unwrap();
        assert_eq!(struct_.name, "foo");
        assert_eq!(struct_.fields.len(), 2);
    }

    #[test]
    fn test_parse_union() {
        let mut reader = Reader::from_str(
            r#"
            <union name="foo">
                <field name="bar" type="int" />
                <field name="baz" type="int" />
            </union>"#,
        );

        let start_elem = read_start_event(&mut reader);
        let union = Union::parse(start_elem, &mut reader).unwrap();
        assert_eq!(union.name, "foo");
        assert_eq!(union.fields.len(), 2);
    }

    #[test]
    fn test_parse_event() {
        let mut reader = Reader::from_str(
            r#"
            <event name="foo" number="1">
                <field name="bar" type="int" />
                <field name="baz" type="int" />
            </event>"#,
        );

        let start_elem = read_start_event(&mut reader);
        let event = XEvent::parse(start_elem, &mut reader).unwrap();
        assert_eq!(event.name, "foo");
        assert_eq!(event.number, 1);
        assert!(!event.xge);
        assert!(!event.no_sequence_number);
        assert_eq!(event.fields.len(), 2);
    }

    #[test]
    fn test_parse_error() {
        let mut reader = Reader::from_str(
            r#"
            <error name="foo" number="1">
                <field name="bar" type="int" />
                <pad bytes="42" />
            </error>"#,
        );

        let start_elem = read_start_event(&mut reader);
        let error = XError::parse(start_elem, &mut reader, false).unwrap();
        assert_eq!(error.name, "foo");
        assert_eq!(error.number, 1);
        assert_eq!(error.fields.len(), 2);
    }

    #[test]
    fn test_parse_error_empty() {
        let mut reader = Reader::from_str(
            r#"
            <error name="foo" number="1" />"#,
        );

        let start_elem = read_start_event(&mut reader);
        let error = XError::parse(start_elem, &mut reader, true).unwrap();
        assert_eq!(error.name, "foo");
        assert_eq!(error.number, 1);
        assert_eq!(error.fields.len(), 0);
    }

    #[test]
    fn test_parse_request() {
        let mut reader = Reader::from_str(
            r#"
            <request name="GetProperty" opcode="0">
                <field name="window" type="Window" />
                <field name="property" type="Atom" />
                <field name="type" type="Atom" />
                <field name="long_offset" type="Card32" />
                <field name="long_length" type="Card32" />
                <field name="delete" type="Bool" />
            </request>"#,
        );
        let start_elem = read_start_event(&mut reader);
        let req = Request::parse(start_elem, &mut reader, false).unwrap();

        assert_eq!(req.name, "GetProperty");
        assert_eq!(req.opcode, 0);
        assert!(!req.combine_adjacent);
        assert_eq!(req.fields.len(), 6);
        assert_eq!(req.reply, None);
    }

    #[test]
    fn test_parse_request_with_reply() {
        let mut reader = Reader::from_str(
            r#"
            <request name="GetProperty" opcode="0" combine-adjacent="true">
                <field name="window" type="Window" />
                <reply>
                    <field name="type" type="Atom" />
                    <pad bytes="4" />
                    <field name="data" type="Void" />
                </reply>
            </request>"#,
        );

        let start_elem = read_start_event(&mut reader);
        let req = Request::parse(start_elem, &mut reader, false).unwrap();

        assert_eq!(req.name, "GetProperty");
        assert_eq!(req.opcode, 0);
        assert!(req.combine_adjacent);
        assert_eq!(req.fields.len(), 1);
        assert_eq!(req.reply.unwrap().0.len(), 3);
    }

    #[test]
    fn test_parse_request_empty() {
        let mut reader = Reader::from_str(
            r#"
            <request name="GetProperty" opcode="0" />"#,
        );

        let start_elem = read_start_event(&mut reader);
        let req = Request::parse(start_elem, &mut reader, true).unwrap();

        assert_eq!(req.name, "GetProperty");
        assert_eq!(req.opcode, 0);
        assert!(!req.combine_adjacent);
        assert_eq!(req.fields.len(), 0);
        assert_eq!(req.reply, None);
    }
}
