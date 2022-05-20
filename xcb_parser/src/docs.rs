// MIT/Apache2 License

use crate::read_text_or_cdata;
use anyhow::{anyhow, Context, Result};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use std::io::BufRead;

/// Documentation for an X11 item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Docs {
    /// A brief description of the item.
    pub brief: Option<String>,
    /// A longer description of the item.
    pub description: Option<String>,
    /// An example describing the item, in C code.
    pub example: Option<String>,
    /// A list of fields in the docs.
    pub fields: Vec<DocField>,
    /// A list of errors that can occur.
    pub errors: Vec<DocError>,
    /// A list of references to other items.
    pub see_also: Vec<DocReference>,
}

impl Docs {
    /// Parse these docs.
    ///
    /// Thie assumes we are already in the docs block.
    #[inline]
    pub(crate) fn parse<B: BufRead>(reader: &mut Reader<B>) -> Result<Self> {
        let mut buffer = vec![];

        let mut brief = None;
        let mut description = None;
        let mut example = None;
        let mut fields = vec![];
        let mut errors = vec![];
        let mut see_also = vec![];

        loop {
            match reader.read_event(&mut buffer)? {
                Event::Start(s) if s.name() == b"brief" => {
                    brief = Some(read_text_or_cdata(reader)?);
                }
                Event::Start(s) if s.name() == b"description" => {
                    description = Some(read_text_or_cdata(reader)?);
                }
                Event::Start(s) if s.name() == b"example" => {
                    example = Some(read_text_or_cdata(reader)?);
                }
                Event::Start(s) if s.name() == b"field" => {
                    fields.push(DocField::parse(s, reader)?);
                }
                Event::Start(s) if s.name() == b"error" => {
                    errors.push(DocError::parse(s, reader)?);
                }
                Event::Empty(e) if e.name() == b"see" => {
                    see_also.push(DocReference::parse(e, reader)?);
                }
                Event::End(e) if e.name() == b"doc" => {
                    return Ok(Docs {
                        brief,
                        description,
                        example,
                        fields,
                        errors,
                        see_also,
                    });
                }
                Event::Eof => return Err(anyhow!("unexpected end of file")),
                _ => continue,
            }
        }
    }
}

/// A field in the documentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocField {
    /// The name of the field.
    pub name: String,
    /// A description of the field.
    pub description: String,
}

impl DocField {
    #[inline]
    fn parse<B: BufRead>(start_event: BytesStart<'_>, reader: &mut Reader<B>) -> Result<Self> {
        let mut name = None;
        for attr in start_event.attributes() {
            let attr = attr?;
            match attr.key {
                b"name" => name = Some(attr.unescape_and_decode_value(reader)?),
                key => {
                    return Err(anyhow!(
                        "Unexpected attribute {}",
                        String::from_utf8_lossy(key)
                    ))
                    .context("reading doc attrs")
                }
            }
        }

        let description = read_text_or_cdata(reader)?;

        Ok(DocField {
            name: name.ok_or_else(|| anyhow!("Missing name attribute"))?,
            description,
        })
    }
}

/// An error that can occur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocError {
    /// The type of the error.
    pub ty: String,
    /// A description of the error.
    pub description: String,
}

impl DocError {
    #[inline]
    fn parse<B: BufRead>(start_event: BytesStart<'_>, reader: &mut Reader<B>) -> Result<Self> {
        let mut ty = None;

        for attr in start_event.attributes() {
            let attr = attr?;
            match attr.key {
                b"type" => ty = Some(attr.unescape_and_decode_value(reader)?),
                _ => {
                    return Err(anyhow!(
                        "unexpected attribute {}",
                        String::from_utf8_lossy(attr.key)
                    ))
                }
            }
        }

        let description = read_text_or_cdata(reader)?;

        Ok(DocError {
            ty: ty.ok_or_else(|| anyhow!("missing error type"))?,
            description,
        })
    }
}

/// A reference to another item in the documentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocReference {
    /// The type of the reference.
    pub ty: String,
    /// The name of the reference.
    pub name: String,
}

impl DocReference {
    #[inline]
    fn parse<B: BufRead>(event: BytesStart<'_>, reader: &mut Reader<B>) -> Result<Self> {
        let mut name = None;
        let mut ty = None;
        for attr in event.attributes() {
            let attr = attr?;
            match attr.key {
                b"name" => name = Some(attr.unescape_and_decode_value(reader)?),
                b"type" => ty = Some(attr.unescape_and_decode_value(reader)?),
                key => {
                    return Err(anyhow!(
                        "unknown attribute: {}",
                        String::from_utf8_lossy(key)
                    ))
                }
            }
        }
        Ok(DocReference {
            ty: ty.ok_or_else(|| anyhow!("missing type attribute"))?,
            name: name.ok_or_else(|| anyhow!("missing name attribute"))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::{events::Event, Reader};

    #[test]
    fn test_parse_docs() {
        let mut reader = Reader::from_str(
            r#"
            <doc>
                <brief>This is a brief description.</brief>
                <description>This is a longer description.</description>
                <example>This is an example.</example>
                <field name="foo">This is a field.</field>
                <error type="bar"><![CDATA[This is an error.]]></error>
                <see type="baz" name="qux" />
            </doc>"#,
        );
        reader.trim_text(true);

        let docs = Docs::parse(&mut reader).unwrap();

        assert_eq!(docs.brief, Some("This is a brief description.".into()));
        assert_eq!(
            docs.description,
            Some("This is a longer description.".into())
        );
        assert_eq!(docs.example, Some("This is an example.".into()));
        assert_eq!(docs.fields[0].name, "foo");
        assert_eq!(docs.fields[0].description, "This is a field.");
        assert_eq!(docs.errors[0].ty, "bar");
        assert_eq!(docs.errors[0].description, "This is an error.");
        assert_eq!(docs.see_also[0].ty, "baz");
        assert_eq!(docs.see_also[0].name, "qux");
    }

    #[test]
    fn test_parse_field() {
        let mut reader = Reader::from_str(
            r#"
            <field name="foo">This is a field.</field>"#,
        );

        let mut buf = vec![];
        let start_elem = loop {
            match reader.read_event(&mut buf).unwrap() {
                Event::Start(e) => break e,
                _ => continue,
            }
        };
        let field = DocField::parse(start_elem, &mut reader).unwrap();

        assert_eq!(field.name, "foo");
        assert_eq!(field.description, "This is a field.");
    }

    #[test]
    fn test_parse_error() {
        let mut reader = Reader::from_str(
            r#"
            <error type="bar">This is an error.</error>"#,
        );

        let mut buf = vec![];
        let start_elem = loop {
            match reader.read_event(&mut buf).unwrap() {
                Event::Start(e) => break e,
                _ => continue,
            }
        };
        let error = DocError::parse(start_elem, &mut reader).unwrap();

        assert_eq!(error.ty, "bar");
        assert_eq!(error.description, "This is an error.");
    }

    #[test]
    fn test_parse_reference() {
        let mut reader = Reader::from_str(
            r#"
            <see type="baz" name="qux" />"#,
        );

        let mut buf = vec![];
        let start_elem = loop {
            match reader.read_event(&mut buf).unwrap() {
                Event::Empty(e) => break e,
                _ => continue,
            }
        };
        let reference = DocReference::parse(start_elem, &mut reader).unwrap();

        assert_eq!(reference.ty, "baz");
        assert_eq!(reference.name, "qux");
    }
}
