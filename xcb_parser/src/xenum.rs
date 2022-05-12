// MIT/Apache2 License

use crate::{find_attribute, Docs, Expression};
use anyhow::{anyhow, Result};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use std::io::BufRead;

/// An enumeration defined in the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
    /// The name of the enum.
    pub name: String,
    /// The values in the enum.
    pub items: Vec<EnumItem>,
    /// The documentation for the num.
    pub docs: Option<Docs>,
}

impl Enum {
    #[inline]
    pub(crate) fn parse(
        start_elem: BytesStart<'_>,
        reader: &mut Reader<impl BufRead>,
    ) -> Result<Self> {
        let name = find_attribute(&start_elem, b"name", reader)?;
        let mut items = Vec::new();
        let mut buffer = vec![];
        let mut docs = None;

        loop {
            match reader.read_event(&mut buffer)? {
                Event::Start(start) => {
                    if start.name() == b"item" {
                        items.push(EnumItem::parse(start, reader)?);
                    } else if start.name() == b"docs" {
                        docs = Some(Docs::parse(reader)?);
                    }
                }
                Event::End(end) => {
                    if end.name() == b"enum" {
                        break;
                    }
                }
                Event::Eof => {
                    return Err(anyhow!("parse enum: Unexpected EOF"));
                }
                _ => {}
            }
        }

        Ok(Self { name, items, docs })
    }
}

/// An item in an enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumItem {
    /// The name of the item.
    pub name: String,
    /// The value of the item.
    pub value: Option<Expression>,
}

impl EnumItem {
    #[inline]
    fn parse(start_elem: BytesStart<'_>, reader: &mut Reader<impl BufRead>) -> Result<Self> {
        let name = find_attribute(&start_elem, b"name", reader)?;
        let value = Expression::parse(reader)?;

        // TODO: optional value... but I don't think it's ever
        // actually done in the protocol, so I don't think it
        // matters

        Ok(Self {
            name,
            value: Some(value),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_start_event;
    use quick_xml::Reader;

    #[test]
    fn test_parse_enum() {
        let mut reader = Reader::from_str(
            r#"
            <enum name="foo">
                <item name="bar">
                    <value>1</value>
                </item>
                <item name="baz">
                    <bit>2</bit>
                </item>
            </enum>
            "#,
        );

        let start_elem = read_start_event(&mut reader);
        let enum_ = Enum::parse(start_elem, &mut reader).unwrap();
        assert_eq!(enum_.name, "foo");
        assert_eq!(enum_.items.len(), 2);
        assert_eq!(enum_.items[0].name, "bar");
        assert_eq!(enum_.items[0].value, Some(Expression::Value(1)));
        assert_eq!(enum_.items[1].name, "baz");
        assert_eq!(enum_.items[1].value, Some(Expression::Bit(2)));
    }
}
