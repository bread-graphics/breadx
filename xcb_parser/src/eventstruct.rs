// MIT/Apache2 License

//! Event structures, which are basically a sum type for events.

use crate::find_attribute;
use anyhow::Result;
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use std::io::BufRead;

/// An event struct sum type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventStruct {
    /// The name of this eventstruct.
    pub name: String,
    /// The allowed selectors for this eventstruct.
    pub allowed: Vec<Allowed>,
}

impl EventStruct {
    #[inline]
    pub(crate) fn parse(
        start_elem: BytesStart<'_>,
        reader: &mut Reader<impl BufRead>,
    ) -> Result<Self> {
        let name = find_attribute(&start_elem, b"name", reader)?;
        let mut allowed = Vec::new();
        let mut buffer = vec![];

        loop {
            match reader.read_event(&mut buffer)? {
                Event::Start(start) | Event::Empty(start) => {
                    if start.name() == b"allowed" {
                        allowed.push(Allowed::parse(start, reader)?);
                    }
                }
                Event::End(end) => {
                    if end.name() == b"eventstruct" {
                        break;
                    }
                }
                _ => {}
            }
        }

        Ok(Self { name, allowed })
    }
}

/// An allowed selector for an eventstruct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allowed {
    /// The extension used to identify this selector.
    pub extension: String,
    /// Whether or not we consider XGE events.
    pub xge: bool,
    /// The min opcode for this selector.
    pub opcode_min: i64,
    /// The max opcode for this selector.
    pub opcode_max: i64,
}

impl Allowed {
    #[inline]
    fn parse(start_elem: BytesStart<'_>, reader: &mut Reader<impl BufRead>) -> Result<Self> {
        let extension = find_attribute(&start_elem, b"extension", reader)?;
        let xge = find_attribute(&start_elem, b"xge", reader)?;
        let opcode_min = find_attribute(&start_elem, b"opcode-min", reader)?;
        let opcode_max = find_attribute(&start_elem, b"opcode-max", reader)?;

        Ok(Self {
            extension,
            xge: xge.parse::<bool>()?,
            opcode_min: opcode_min.parse::<i64>()?,
            opcode_max: opcode_max.parse::<i64>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_start_event;
    use quick_xml::Reader;

    #[test]
    fn test_eventstruct_parse() -> Result<()> {
        let mut reader = Reader::from_str(
            r#"
            <eventstruct name="GetEvent">
                <allowed extension="XGE" xge="true" opcode-min="0" opcode-max="20" />
            </eventstruct>"#,
        );

        let start_elem = read_start_event(&mut reader);
        let eventstruct = EventStruct::parse(start_elem, &mut reader)?;

        assert_eq!(eventstruct.name, "GetEvent");
        assert_eq!(eventstruct.allowed.len(), 1);
        assert_eq!(eventstruct.allowed[0].extension, "XGE");
        assert!(eventstruct.allowed[0].xge);
        assert_eq!(eventstruct.allowed[0].opcode_min, 0);
        assert_eq!(eventstruct.allowed[0].opcode_max, 20);

        Ok(())
    }
}
