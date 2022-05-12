// MIT/Apache2 License

use crate::{find_attribute, Docs, Expression};
use anyhow::{anyhow, Context, Error, Result};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use std::{io::BufRead, str::FromStr};

/// The contents of a structure, union, request or reply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StructureContents {
    /// The fields of the structure.
    pub fields: Vec<Field>,
    /// Documentation for the structure.
    pub docs: Option<Docs>,
    /// A reply to this request, if applicable.
    pub reply: Option<Box<StructureContents>>,
}

impl StructureContents {
    /// Parse the contents of a structure-like item.
    #[inline]
    pub(crate) fn parse(reader: &mut Reader<impl BufRead>, end: &[u8]) -> Result<Self> {
        let mut buffer = vec![];
        let mut docs = None;
        let mut fields = vec![];
        let mut reply = None;

        loop {
            match reader.read_event(&mut buffer)? {
                Event::Start(s) if s.name() == b"doc" => {
                    docs = Some(Docs::parse(reader).context("parsing documentation")?);
                }
                Event::Start(s) if s.name() == b"reply" => {
                    // this has a reply, parse it
                    reply = Some(Box::new(
                        StructureContents::parse(reader, b"reply").context("reading reply")?,
                    ));
                }
                Event::Start(s) => {
                    fields.push(Field::parse(s, reader, false)?);
                }
                Event::Empty(s) => {
                    fields.push(Field::parse(s, reader, true)?);
                }
                Event::End(e) if e.name() == end => {
                    return Ok(Self {
                        fields,
                        docs,
                        reply,
                    });
                }
                Event::Eof => {
                    return Err(anyhow!("unexpected end of file"));
                }
                _ => continue,
            }
        }
    }
}

/// A field of a structure, to be stored in [`StructureContents`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    /// Padding between fields.
    Pad {
        bytes: i64,
        ty: PaddingType,
        serialize: bool,
    },
    /// A single-item field containing a value.
    Field {
        ty: String,
        name: String,
        enum_: Option<String>,
        altenum: Option<String>,
        mask: Option<String>,
        altmask: Option<String>,
    },
    /// Overrides the length of the data structure.
    Length(Expression),
    /// A file descriptor included as part of a request.
    Fd(String),
    /// A list of values.
    List {
        ty: String,
        name: String,
        list_length: Option<Expression>,
    },
    /// A field whose value is defined by an expression.
    ExprField {
        ty: String,
        name: String,
        value: Expression,
    },
    /// A "value parameter" sum field, containing a mask, and a list
    /// of values.
    ValueParam {
        mask_ty: String,
        mask_name: String,
        list_name: String,
    },
    /// A "switch" case, containing a value to match against, its
    /// cases, and a list of fields.
    SwitchCase {
        name: String,
        switch_expr: Expression,
        cases: Vec<Case>,
    },
    /// Required starting alignment.
    RequiredStartAlign { align: i64, offset: i64 },
}

impl Field {
    /// Decode a `Field` from the starting element and the remaining
    /// XML.
    #[inline]
    fn parse(
        start: BytesStart<'_>,
        reader: &mut Reader<impl BufRead>,
        is_empty: bool,
    ) -> Result<Self> {
        let mut buffer = vec![];

        match start.name() {
            b"pad" => {
                let mut bytes = None;
                let mut ty = None;
                let mut serialize = true;

                for attr in start.attributes() {
                    let attr = attr?;
                    match attr.key {
                        b"bytes" => {
                            bytes = Some(i64::from_str(&attr.unescape_and_decode_value(reader)?)?);
                            ty = Some(PaddingType::SetNumberOfBytes);
                        }
                        b"align" => {
                            bytes = Some(i64::from_str(&attr.unescape_and_decode_value(reader)?)?);
                            ty = Some(PaddingType::ToAlignment);
                        }
                        b"serialize" => {
                            serialize = bool::from_str(&attr.unescape_and_decode_value(reader)?)?;
                        }
                        _ => {
                            return Err(anyhow!(
                                "Unknown attribute {}",
                                String::from_utf8_lossy(attr.key)
                            ))
                        }
                    }
                }

                Ok(Field::Pad {
                    bytes: bytes.ok_or_else(|| anyhow!("Missing bytes attribute"))?,
                    ty: ty.unwrap(),
                    serialize,
                })
            }
            b"field" => {
                let mut ty = None;
                let mut name = None;
                let mut enum_ = None;
                let mut altenum = None;
                let mut mask = None;
                let mut altmask = None;

                for attr in start.attributes() {
                    let attr = attr?;
                    match attr.key {
                        b"type" => {
                            ty = Some(attr.unescape_and_decode_value(reader)?);
                        }
                        b"name" => {
                            name = Some(attr.unescape_and_decode_value(reader)?);
                        }
                        b"enum" => {
                            enum_ = Some(attr.unescape_and_decode_value(reader)?);
                        }
                        b"altenum" => {
                            altenum = Some(attr.unescape_and_decode_value(reader)?);
                        }
                        b"mask" => {
                            mask = Some(attr.unescape_and_decode_value(reader)?);
                        }
                        b"altmask" => {
                            altmask = Some(attr.unescape_and_decode_value(reader)?);
                        }
                        _ => {
                            return Err(anyhow!(
                                "Unknown attribute {}",
                                String::from_utf8_lossy(attr.key)
                            ))
                        }
                    }
                }

                Ok(Field::Field {
                    ty: ty.ok_or_else(|| anyhow!("Missing type attribute"))?,
                    name: name.ok_or_else(|| anyhow!("Missing name attribute"))?,
                    enum_,
                    altenum,
                    mask,
                    altmask,
                })
            }
            b"length" => Ok(Field::Length(
                Expression::parse(reader).context("reading total length")?,
            )),
            b"fd" => {
                let name = find_attribute(&start, b"name", reader)?;

                Ok(Field::Fd(name))
            }
            b"list" => {
                let ty = find_attribute(&start, b"type", reader)?;
                let name = find_attribute(&start, b"name", reader)?;
                let list_length = if is_empty {
                    None
                } else {
                    Some(Expression::parse(reader).context("reading field list length")?)
                };

                Ok(Field::List {
                    ty,
                    name,
                    list_length,
                })
            }
            b"exprfield" => {
                let ty = find_attribute(&start, b"type", reader)?;
                let name = find_attribute(&start, b"name", reader)?;
                let value = Expression::parse(reader).context("reading exprfield expr")?;

                Ok(Field::ExprField { ty, name, value })
            }
            b"valueparam" => {
                let mask_ty = find_attribute(&start, b"value-mask-type", reader)?;
                let mask_name = find_attribute(&start, b"value-mask-name", reader)?;
                let list_name = find_attribute(&start, b"value-list-name", reader)?;

                Ok(Field::ValueParam {
                    mask_ty,
                    mask_name,
                    list_name,
                })
            }
            b"switch" => {
                let name = find_attribute(&start, b"name", reader)?;
                let switch_expr =
                    Expression::parse(reader).context("reading switch initial condition")?;
                let mut cases = Vec::new();

                // iterate over potential cases and add them to the list
                loop {
                    match reader.read_event(&mut buffer)? {
                        Event::Start(start) => {
                            if start.name() == b"case" || start.name() == b"bitcase" {
                                cases.push(Case::parse(start, reader)?);
                            }
                        }
                        Event::End(ref end) => {
                            if end.name() == b"switch" {
                                break;
                            }
                        }
                        Event::Eof => {
                            return Err(anyhow!("unexpected end of file"));
                        }
                        _ => {}
                    }
                }

                Ok(Field::SwitchCase {
                    name,
                    switch_expr,
                    cases,
                })
            }
            b"required_start_align" => {
                let bytes = find_attribute(&start, b"align", reader)
                    .and_then(|i| i64::from_str(&i).map_err(Error::from))?;
                let offset = find_attribute(&start, b"offset", reader)
                    .and_then(|i| i64::from_str(&i).map_err(Error::from))
                    .unwrap_or(0);
                Ok(Field::RequiredStartAlign {
                    align: bytes,
                    offset,
                })
            }
            name => {
                return Err(anyhow!(
                    "Unknown field type {}",
                    String::from_utf8_lossy(name)
                ))
            }
        }
    }
}

/// The type of padding to use.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PaddingType {
    SetNumberOfBytes,
    ToAlignment,
}

/// A case in a switch statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
    /// The type of case that this is.
    pub case_ty: CaseTy,
    /// The value to match against.
    pub value: Expression,
    /// The fields of the case.
    pub fields: Vec<Field>,
}

impl Case {
    #[inline]
    fn parse(bytes: BytesStart<'_>, reader: &mut Reader<impl BufRead>) -> Result<Self> {
        // parse the case type
        let case_ty = match bytes.name() {
            b"case" => CaseTy::Case,
            b"bitcase" => CaseTy::BitCase,
            _ => unreachable!(),
        };

        // parse the bitcase expression
        let value = Expression::parse(reader)
            .with_context(|| format!("Reading condition for a {:?}", case_ty))?;

        // parse the fields
        let mut buffer = vec![];
        let mut test_values = vec![];
        let mut fields = vec![];
        loop {
            match reader.read_event(&mut buffer)? {
                Event::Start(s) => {
                    if let Some(expr) = Expression::parse_with_start_elem(s.clone(), reader, false)?
                    {
                        test_values.push(expr);
                    } else {
                        fields.push(Field::parse(s, reader, false)?);
                    }
                }
                Event::Empty(s) => {
                    if let Some(expr) = Expression::parse_with_start_elem(s.clone(), reader, true)?
                    {
                        test_values.push(expr);
                    } else {
                        fields.push(Field::parse(s, reader, true)?);
                    }
                }
                Event::End(e) => {
                    if e.name() == b"case" || e.name() == b"bitcase" {
                        break;
                    }
                }
                Event::Eof => {
                    return Err(anyhow!("Unexpected EOF"));
                }
                _ => continue,
            }
        }

        Ok(Case {
            case_ty,
            value,
            fields,
        })
    }
}

/// The type of case that this is.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CaseTy {
    /// The value case.
    Case,
    /// The bit case.
    BitCase,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_start_event;
    use quick_xml::Reader;

    #[test]
    fn test_parse_field() {
        let mut reader = Reader::from_str(
            r#"
            <field name="foo" type="int32" />
            "#,
        );
        let start = read_start_event(&mut reader);
        let field = Field::parse(start, &mut reader, true).unwrap();
        assert_eq!(
            field,
            Field::Field {
                ty: "int32".into(),
                name: "foo".into(),
                enum_: None,
                altenum: None,
                mask: None,
                altmask: None,
            }
        );
    }

    #[test]
    fn test_parse_pad() {
        let mut reader = Reader::from_str(
            r#"
            <pad bytes="4" />
            "#,
        );
        let start = read_start_event(&mut reader);
        let field = Field::parse(start, &mut reader, true).unwrap();
        assert_eq!(
            field,
            Field::Pad {
                bytes: 4,
                ty: PaddingType::SetNumberOfBytes,
                serialize: false,
            }
        );
    }

    #[test]
    fn test_parse_pad_serialize() {
        let mut reader = Reader::from_str(
            r#"
            <pad bytes="4" serialize="true" />
            "#,
        );
        let start = read_start_event(&mut reader);
        let field = Field::parse(start, &mut reader, true).unwrap();
        assert_eq!(
            field,
            Field::Pad {
                bytes: 4,
                ty: PaddingType::SetNumberOfBytes,
                serialize: true,
            }
        );
    }

    #[test]
    fn test_parse_pad_align() {
        let mut reader = Reader::from_str(
            r#"
            <pad align="4" />
            "#,
        );
        let start = read_start_event(&mut reader);
        let field = Field::parse(start, &mut reader, true).unwrap();
        assert_eq!(
            field,
            Field::Pad {
                bytes: 4,
                ty: PaddingType::ToAlignment,
                serialize: false,
            }
        );
    }

    #[test]
    fn test_parse_list() {
        let mut reader = Reader::from_str(
            r#"
            <list type="int32" name="foo">
                <fieldref>bar</fieldref>
            </list>
            "#,
        );
        let start = read_start_event(&mut reader);
        let field = Field::parse(start, &mut reader, false).unwrap();
        assert_eq!(
            field,
            Field::List {
                ty: "int32".into(),
                name: "foo".into(),
                list_length: Some(Expression::Fieldref("bar".into())),
            }
        );
    }

    #[test]
    fn test_parse_length() {
        let mut reader = Reader::from_str(
            r#"
            <length>
                <fieldref>bar</fieldref>
            </length>
            "#,
        );
        let start = read_start_event(&mut reader);
        let field = Field::parse(start, &mut reader, false).unwrap();
        assert_eq!(field, Field::Length(Expression::Fieldref("bar".into()),));
    }

    #[test]
    fn test_parse_switch() {
        let mut reader = Reader::from_str(
            r#"
            <switch name="foo">
                <fieldref>bar</fieldref>
                <case>
                    <value>42</value>
                    <field name="baz" type="int32" />
                </case>
                <bitcase>
                    <value>1</value>
                    <field name="qux" type="int64" />
                </bitcase>
            </switch>
            "#,
        );
        let start = read_start_event(&mut reader);
        let field = Field::parse(start, &mut reader, false).unwrap();
        assert_eq!(
            field,
            Field::SwitchCase {
                name: "foo".into(),
                switch_expr: Expression::Fieldref("bar".into()),
                cases: vec![
                    Case {
                        case_ty: CaseTy::Case,
                        value: Expression::Value(42),
                        fields: vec![Field::Field {
                            ty: "int32".into(),
                            name: "baz".into(),
                            enum_: None,
                            altenum: None,
                            mask: None,
                            altmask: None,
                        }],
                    },
                    Case {
                        case_ty: CaseTy::BitCase,
                        value: Expression::Value(1),
                        fields: vec![Field::Field {
                            ty: "int64".into(),
                            name: "qux".into(),
                            enum_: None,
                            altenum: None,
                            mask: None,
                            altmask: None,
                        }],
                    }
                ],
            }
        );
    }

    #[test]
    fn test_parse_valueparam() {
        let mut reader = Reader::from_str(
            r#"
            <valueparam
                value-mask-type="int32"
                value-mask-name="foo"
                value-list-name="bar" />
            "#,
        );
        let start = read_start_event(&mut reader);
        let field = Field::parse(start, &mut reader, true).unwrap();
        assert_eq!(
            field,
            Field::ValueParam {
                mask_ty: "int32".into(),
                mask_name: "foo".into(),
                list_name: "bar".into(),
            }
        );
    }

    #[test]
    fn test_parse_fd() {
        let mut reader = Reader::from_str(
            r#"
            <fd name="foo" />
            "#,
        );
        let start = read_start_event(&mut reader);
        let field = Field::parse(start, &mut reader, true).unwrap();
        assert_eq!(field, Field::Fd("foo".into()));
    }

    #[test]
    fn test_parse_exprfield() {
        let mut reader = Reader::from_str(
            r#"
            <exprfield name="foo" type="int32">
                <fieldref>bar</fieldref>
            </exprfield>
            "#,
        );
        let start = read_start_event(&mut reader);
        let field = Field::parse(start, &mut reader, false).unwrap();
        assert_eq!(
            field,
            Field::ExprField {
                ty: "int32".into(),
                name: "foo".into(),
                value: Expression::Fieldref("bar".into()),
            }
        );
    }

    #[test]
    fn test_parse_structure_contents() {
        let mut reader = Reader::from_str(
            r#"
            <struct>
                <field name="foo" type="int32" />
                <field name="bar" type="int64" />
            </struct>"#,
        );
        let _ = read_start_event(&mut reader);
        let contents = StructureContents::parse(&mut reader, b"struct").unwrap();

        assert_eq!(
            contents,
            StructureContents {
                fields: vec![
                    Field::Field {
                        ty: "int32".into(),
                        name: "foo".into(),
                        enum_: None,
                        altenum: None,
                        mask: None,
                        altmask: None,
                    },
                    Field::Field {
                        ty: "int64".into(),
                        name: "bar".into(),
                        enum_: None,
                        altenum: None,
                        mask: None,
                        altmask: None,
                    },
                ],
                docs: None,
                reply: None,
            }
        );
    }

    #[test]
    fn test_parse_structure_contents_with_reply() {
        let mut reader = Reader::from_str(
            r#"
            <struct>
                <field name="foo" type="int32" />
                <field name="bar" type="int64" />
                <reply>
                    <field name="baz" type="int32" />
                </reply>
            </struct>
            "#,
        );
        let _ = read_start_event(&mut reader);
        let contents = StructureContents::parse(&mut reader, b"struct").unwrap();

        assert_eq!(
            contents,
            StructureContents {
                fields: vec![
                    Field::Field {
                        ty: "int32".into(),
                        name: "foo".into(),
                        enum_: None,
                        altenum: None,
                        mask: None,
                        altmask: None,
                    },
                    Field::Field {
                        ty: "int64".into(),
                        name: "bar".into(),
                        enum_: None,
                        altenum: None,
                        mask: None,
                        altmask: None,
                    },
                ],
                docs: None,
                reply: Some(Box::new(StructureContents {
                    fields: vec![Field::Field {
                        ty: "int32".into(),
                        name: "baz".into(),
                        enum_: None,
                        altenum: None,
                        mask: None,
                        altmask: None,
                    }],
                    docs: None,
                    reply: None
                })),
            }
        );
    }

    #[test]
    fn test_parse_structure_contents_with_docs() {
        let mut reader = Reader::from_str(
            r#"
            <struct>
                <field name="foo" type="int32" />
                <field name="bar" type="int64" />
                <doc>
                    <brief>Something</brief>
                </doc>
            </struct>
            "#,
        );

        let _ = read_start_event(&mut reader);
        let contents = StructureContents::parse(&mut reader, b"struct").unwrap();

        assert_eq!(
            contents,
            StructureContents {
                fields: vec![
                    Field::Field {
                        ty: "int32".into(),
                        name: "foo".into(),
                        enum_: None,
                        altenum: None,
                        mask: None,
                        altmask: None,
                    },
                    Field::Field {
                        ty: "int64".into(),
                        name: "bar".into(),
                        enum_: None,
                        altenum: None,
                        mask: None,
                        altmask: None,
                    },
                ],
                docs: Some(Docs {
                    brief: Some("Something".into()),
                    description: None,
                    example: None,
                    fields: vec![],
                    errors: vec![],
                    see_also: vec![],
                }),
                reply: None,
            }
        );
    }
}
