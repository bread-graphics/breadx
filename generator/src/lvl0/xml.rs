// MIT/Apache2 License

//! XML Helper functions

use quick_xml::events::BytesStart;
use std::collections::BTreeMap;

/// Get the attributes from a byte item.
#[inline]
pub fn get_attributes<'a>(
    bs: &BytesStart<'a>,
    attrs: &[&[u8]],
    required: &[bool],
) -> Option<BTreeMap<Box<[u8]>, String>> {
    let res = bs.attributes().fold(BTreeMap::new(), |mut res, attr| {
        if let Ok(attr) = attr {
            // if we need the value
            if attrs.contains(&attr.key) {
                match String::from_utf8(attr.unescaped_value().unwrap().into_owned()) {
                    Ok(value) => {
                        res.insert(attr.key.into(), value);
                    }
                    Err(e) => log::error!("Found invalid UTF-8: {:?}", e.into_bytes()),
                }
            }
        }

        res
    });

    for attr in attrs
        .iter()
        .zip(required.iter())
        .filter(|(_a, required)| **required)
        .map(|(attr, _r)| attr)
    {
        if !res.contains_key(*attr) {
            log::error!(
                "Could not find required attribute: {}",
                std::str::from_utf8(attr).unwrap()
            );
            return None;
        }
    }

    Some(res)
}
