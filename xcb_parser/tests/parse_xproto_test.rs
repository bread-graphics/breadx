//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

use anyhow::Context;
use std::{collections::HashMap, fs::read_dir, path::Path};
use xcb_parser::{read_xcb_from_file, Expression, ToplevelItem};

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const XPROTO_ROOT: &str = "../xcbproto/src";

#[test]
fn parse_all_protocol() {
    // iterate over the xproto directory, read in items and then parse them
    let protocol: HashMap<_, _> = read_dir(Path::new(MANIFEST_DIR).join(XPROTO_ROOT))
        .expect("Failed to read dir")
        .map(|x| x.expect("Failed to read dir entry"))
        .filter(|x| x.path().extension().map_or(false, |ext| ext == "xml"))
        .map(|x| x.path())
        .map(|x| (x.clone(), read_xcb_from_file(&x)))
        .map(|(path, iter)| {
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();
            let data = iter
                .expect("Failed to read xcb file")
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| format!("Failed while reading {}", &name))
                .expect("Failed to read entries");
            (name, data)
        })
        .collect();

    // test some operations on the "xproto" module
    let xproto = protocol.get("xproto").unwrap();
    assert!(xproto.iter().any(|x| match x {
        ToplevelItem::Struct(x) => x.name == "CHAR2B",
        _ => false,
    }));
    assert!(xproto.iter().any(|x| match x {
        ToplevelItem::Xidtype(x) => x == "WINDOW",
        _ => false,
    }));
    assert!(xproto.iter().any(|x| match x {
        ToplevelItem::Typedef { oldname, newname } => oldname == "CARD32" && newname == "VISUALID",
        _ => false,
    }));
    assert!(xproto.iter().any(|x| match x {
        ToplevelItem::Enum(x) =>
            x.name == "VisualClass" && {
                x.items
                    .iter()
                    .any(|v| v.name == "TrueColor" && v.value == Some(Expression::Value(4)))
            },
        _ => false,
    }));
    assert!(xproto.iter().any(|x| match x {
        ToplevelItem::Event(x) => {
            x.name == "ButtonPress"
                && x.number == 4
                && x.docs
                    .as_ref()
                    .unwrap()
                    .fields
                    .iter()
                    .any(|d| d.description.contains("event was generated"))
        }
        _ => false,
    }));
    assert!(xproto.iter().any(|x| match x {
        ToplevelItem::EventCopy(mc) =>
            mc.name == "KeyRelease" && mc.number == 3 && mc.ref_ == "KeyPress",
        _ => false,
    }));
}
