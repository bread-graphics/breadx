// MIT/Apache2 License

//! A simple parser for the [XML-XCB](https://xcb.freedesktop.org/XmlXcb/) format.
//!
//! This crate provides a mechanism for taking in XML XCB files and parsing them into
//! the represented constructs. Simply use the [`read_xcb`] method to create an iterator
//! that can be used to parse an XCB file.
//!
//! # Example
//!
//! ```
//! # const XCB_FILE: &str = r#"
//! # <?xml version="1.0" encoding="UTF-8"?>
//! # <xcb header="xproto">
//! #   <import>xproto</import>
//! #   <import>randr</import>
//! # </xcb>
//! # "#;
//! #
//! # fn get_xcb_file() -> &'static [u8] {
//! #    XCB_FILE.as_bytes()
//! # }
//!
//! use xcb_parser::{ToplevelItem, read_xcb};
//!
//! # fn main() -> anyhow::Result<()> {
//! // Create a reader for the XCB file.
//! let mut reader = read_xcb(get_xcb_file())?;
//!
//! // Get information from the header.
//! let header = reader.header();
//! println!("The file's name is {}", header.header);
//!
//! // Begin reading constructs from the file.
//! for construct in reader {
//!    match construct? {
//!        ToplevelItem::Import(import) => println!("Imported {}", import),
//!        ToplevelItem::Struct(s) => {
//!            println!("Struct {}", s.name);
//!        },
//!        _ => {},
//!    }
//! }
//! # Ok(())
//! # }
//! ```

mod docs;
mod eventstruct;
mod expression;
mod fields;
mod toplevel;
mod xenum;
mod xstruct;

pub use docs::*;
pub use eventstruct::*;
pub use expression::*;
pub use fields::{Case, CaseTy, Field, PaddingType};
pub use toplevel::*;
pub use xenum::*;
pub use xstruct::*;

pub(crate) use fields::StructureContents;

use anyhow::{anyhow, Context, Error, Result};
use quick_xml::{
    events::{BytesStart, Event as XmlEvent},
    Reader,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Read XML-XCB constructs from a `BufRead` implementor.
///
/// See the crate root for more documentation on this method.
///
/// # Errors
///
/// This function will return an error if the XML file is malformed or
/// if it cannot properly read the XCB header.
#[inline]
pub fn read_xcb<B: BufRead>(br: B) -> Result<XmlXcbIterator<B>> {
    XmlXcbIterator::new(Reader::from_reader(br))
}

/// Read XML-XCB constructs from a file as given by the path.
///
/// This is a convenience wrapper that opens a file and then forwards to
/// [`read_xcb`].
///
/// # Errors
///
/// In addition to errors that may be returned by [`read_xcb`], this function
/// can return an error if the file cannot be opened.
#[inline]
pub fn read_xcb_from_file<P: AsRef<Path>>(path: P) -> Result<XmlXcbIterator<BufReader<File>>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    read_xcb(br)
}

/// An iterator that processes the contents of an XCB-XML
/// file.
///
/// This is the return type of [`read_xcb`] and [`read_xcb_from_file`].
/// It is intended that `header()` should be called first to identify the
/// header, then the iterator can be used to iterate over the contents of
/// the file.
///
/// # Errors
///
/// If a construct cannot be parsed, an error will be returned by
/// `Iterator::next()`.
pub struct XmlXcbIterator<B: BufRead> {
    reader: Reader<B>,
    header: Header,
}

impl<B: BufRead> XmlXcbIterator<B> {
    #[inline]
    fn new(mut reader: Reader<B>) -> Result<Self> {
        let mut buffer = vec![];
        let header;

        reader.trim_text(true);

        loop {
            match reader.read_event(&mut buffer)? {
                XmlEvent::Start(s) if s.name() == b"xcb" => {
                    header = Header::parse(s, &mut reader)?;
                    break;
                }
                XmlEvent::Eof => {
                    return Err(anyhow!("unexpected end of file"));
                }
                _ => continue,
            }
        }

        Ok(Self { reader, header })
    }

    /// Get the header for this file.
    #[inline]
    #[must_use]
    pub fn header(&self) -> &Header {
        &self.header
    }
}

impl<B: BufRead> Iterator for XmlXcbIterator<B> {
    type Item = Result<ToplevelItem>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        ToplevelItem::parse(&mut self.reader).transpose()
    }
}

/// The header for an XCB-XML file.
#[derive(Debug, Clone, Default)]
pub struct Header {
    /// The header of the file.
    pub header: String,
    /// The "Xname" of an extension.
    pub extension_xname: Option<String>,
    /// The name of the extension.
    pub extension_name: Option<String>,
    /// Is this a multiworld extension?
    pub extension_multiworld: Option<bool>,
    /// Major version.
    pub major_version: Option<u64>,
    /// Minor version.
    pub minor_version: Option<u64>,
}

impl Header {
    #[inline]
    fn parse(start_elem: BytesStart<'_>, reader: &mut Reader<impl BufRead>) -> Result<Self> {
        let mut header = None;
        let mut extension_xname = None;
        let mut extension_name = None;
        let mut extension_multiworld = None;
        let mut major_version = None;
        let mut minor_version = None;

        for attr in start_elem.attributes() {
            let attr = attr?;
            match attr.key {
                b"header" => header = Some(attr.unescape_and_decode_value(reader)?),
                b"extension-xname" => {
                    extension_xname = Some(attr.unescape_and_decode_value(reader)?)
                }
                b"extension-name" => extension_name = Some(attr.unescape_and_decode_value(reader)?),
                b"extension-multiworld" => {
                    extension_multiworld = Some(attr.unescape_and_decode_value(reader)?)
                }
                b"major-version" => major_version = Some(attr.unescape_and_decode_value(reader)?),
                b"minor-version" => minor_version = Some(attr.unescape_and_decode_value(reader)?),
                _ => continue,
            }
        }

        Ok(Self {
            header: header.ok_or_else(|| anyhow!("missing header"))?,
            extension_xname,
            extension_name,
            extension_multiworld: extension_multiworld.map(|s| s == "true"),
            major_version: major_version.and_then(|v| v.parse().ok()),
            minor_version: minor_version.and_then(|v| v.parse().ok()),
        })
    }
}

/// Read an event from a reader, assuming it is text or CDATA.
#[inline]
pub(crate) fn read_text_or_cdata<B: BufRead>(reader: &mut Reader<B>) -> Result<String> {
    let mut buf = vec![];
    match reader.read_event(&mut buf)? {
        XmlEvent::Text(text) => Ok(text.unescape_and_decode(reader)?),
        XmlEvent::CData(cdata) => Ok(cdata.unescape_and_decode(reader)?),
        XmlEvent::End(_) => Ok(String::new()),
        _ => Err(anyhow!("expected text or cdata")),
    }
}

/// Read in the first start event we encounter.
#[cfg(test)]
#[inline]
pub(crate) fn read_start_event(reader: &mut Reader<impl BufRead>) -> BytesStart<'static> {
    let mut buffer = vec![];
    loop {
        match reader.read_event(&mut buffer) {
            Ok(XmlEvent::Start(s)) | Ok(XmlEvent::Empty(s)) => return s.into_owned(),
            Ok(XmlEvent::Eof) => panic!("Unexpected EOF"),
            Err(e) => panic!("{}", e),
            _ => continue,
        }
    }
}

/// Find a specific attribute in the start element.
#[inline]
pub(crate) fn find_attribute<'a, B: BufRead>(
    start: &'a BytesStart,
    name: &[u8],
    reader: &mut Reader<B>,
) -> Result<String> {
    start
        .attributes()
        .find(|attr| match attr {
            Ok(attr) => attr.key == name,
            Err(_) => false,
        })
        .ok_or_else(|| anyhow!("missing attribute {}", String::from_utf8_lossy(name)))
        .and_then(|attr| attr.map_err(Error::from))
        .and_then(|attr| attr.unescape_and_decode_value(reader).map_err(Error::from))
        .with_context(|| format!("reading attribute {}", String::from_utf8_lossy(name)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::Reader;

    #[test]
    fn test_read_header() {
        let mut reader = Reader::from_str(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <xcb header="xproto">
            </xcb>"#,
        );
        let start_elem = read_start_event(&mut reader);
        let header = Header::parse(start_elem, &mut reader).unwrap();
        assert_eq!(header.header, "xproto");
        assert_eq!(header.extension_xname, None);
        assert_eq!(header.extension_name, None);
        assert_eq!(header.extension_multiworld, None);
        assert_eq!(header.major_version, None);
        assert_eq!(header.minor_version, None);
    }

    #[test]
    fn test_read_header_with_extension() {
        let mut reader = Reader::from_str(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <xcb header="xproto" extension_xname="xname" extension_name="extension" extension_multiworld="true" major_version="1" minor_version="2">
            </xcb>"#,
        );
        let start_elem = read_start_event(&mut reader);
        let header = Header::parse(start_elem, &mut reader).unwrap();
        assert_eq!(header.header, "xproto");
        assert_eq!(header.extension_xname, Some("xname".to_string()));
        assert_eq!(header.extension_name, Some("extension".to_string()));
        assert_eq!(header.extension_multiworld, Some(true));
        assert_eq!(header.major_version, Some(1));
        assert_eq!(header.minor_version, Some(2));
    }

    #[test]
    fn test_read_xcb() {
        let reader = Reader::from_str(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <xcb header="xproto">
              <typedef oldname="foo" newname="bar" />
              <struct name="baz">
                <field name="qux" type="int" />
              </struct>
            </xcb>"#,
        );

        let mut iter = XmlXcbIterator::new(reader).unwrap();
        assert_eq!(
            iter.next().unwrap().unwrap(),
            ToplevelItem::Typedef {
                oldname: "foo".to_string(),
                newname: "bar".to_string(),
            }
        );
        assert_eq!(
            iter.next().unwrap().unwrap(),
            ToplevelItem::Struct(Struct {
                name: "baz".to_string(),
                docs: None,
                fields: vec![Field::Field {
                    name: "qux".to_string(),
                    ty: "int".to_string(),
                    enum_: None,
                    altenum: None,
                    mask: None,
                    altmask: None,
                }],
            })
        );
        assert!(iter.next().is_none());
    }
}
