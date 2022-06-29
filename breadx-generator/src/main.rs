// MIT/Apache2 License

use anyhow::Result;
use std::{
    env,
    fmt::Write as _,
    fs,
    io::{BufWriter, Write},
    path::Path,
};
use xcb_parser::Header;

mod items;

fn main() -> Result<()> {
    // setup tracing
    tracing_subscriber::fmt::init();

    // read in files
    let mut args = env::args_os();
    let root_path = args.by_ref().nth(1).map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("../xcbproto/src"),
        |arg| arg.into(),
    );

    tracing::info!("Reading files from {:?}", root_path);

    // read all filenames into a Vec<PathBuf>
    let mut filepaths = fs::read_dir(root_path)?
        .map(|entry| entry.map(|entry| entry.path()))
        .inspect(|entry| {
            if let Ok(entry) = entry {
                tracing::debug!("Parsing file {:?}", entry);
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    // sort filenames deterministically
    filepaths.sort();

    // begin writing to the output file
    let output_file = args.next().map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("../breadx/src/automatically_generated.rs"),
        |arg| arg.into(),
    );

    tracing::info!("Writing to {:?}", output_file);

    let mut output = BufWriter::new(fs::File::create(output_file)?);

    // write the header
    output.write_all(
        r#"
// This file is automatically generated by the `breadx-generator` crate.
// Do not edit this file directly.

//! Contains automatically generated items.

use crate::{Result, display::{Cookie, Display, DisplayExt}};
use alloc::borrow::Cow;
#[allow(unused_imports)]
use alloc::vec::Vec;
use core::borrow::Borrow;
use __private::Sealed;

cfg_async! {
    use crate::{display::{AsyncDisplay, AsyncDisplayExt}, futures};
    use __private::Sealed2;
}
"#
        .as_bytes(),
    )?;

    // generate list to contain header info
    let mut headers = Vec::new();

    // generate items for each request
    let (sync_items, async_items) = filepaths
        .into_iter()
        .map(|path| process_file(&path, &mut headers))
        .filter_map(|res| match res {
            Ok(res) => Some(res),
            Err(err) => {
                tracing::error!("{}", err);
                None
            }
        })
        .unzip::<_, _, Vec<String>, Vec<String>>();

    // begin writing the impl blocks
    output.write_all(
        r#"
pub trait DisplayFunctionsExt : Display + Sealed {"#
            .as_bytes(),
    )?;

    for sync_item in sync_items {
        // indent it by 4 spaces
        let sync_item = sync_item.replace('\n', "\n    ");
        write!(output, "\n    {}", sync_item)?;
    }

    output.write_all(
        r#"
}

#[cfg(feature = "async")]
pub trait AsyncDisplayFunctionsExt : AsyncDisplay + Sealed2 {"#
            .as_bytes(),
    )?;

    for async_item in async_items {
        // indent it by 4 spaces
        let async_item = async_item.replace('\n', "\n    ");
        write!(output, "\n    {}", async_item)?;
    }

    // finish it up
    output.write_all(
        r#"}

impl<D: Display + ?Sized> DisplayFunctionsExt for D {}

#[cfg(feature = "async")]
impl<D: AsyncDisplay + ?Sized> AsyncDisplayFunctionsExt for D {}

mod __private {
    use crate::display::Display;

    pub trait Sealed {
        fn __sealed_trait_marker() {}
    }

    impl<D: Display + ?Sized> Sealed for D {}

    cfg_async! {
        use crate::display::AsyncDisplay;

        pub trait Sealed2 {
            fn __sealed_trait_marker() {}
        }

        impl<D: AsyncDisplay + ?Sized> Sealed2 for D {}
    }
}
"#
        .as_bytes(),
    )?;

    // add types module
    output.write_all(generate_types(&headers).as_bytes())?;

    Ok(())
}

fn process_file(filepath: &Path, headers: &mut Vec<Header>) -> Result<(String, String)> {
    // open the file using xcb_parser
    let parser = xcb_parser::read_xcb_from_file(filepath)?;
    let header = parser.header().clone();
    headers.push(header.clone());

    // calculate all of the items
    let item_pairs = parser
        .filter_map(|item| {
            item.map(|item| {
                // create a tuple of the sync item and async item
                let sync_item = items::generate_item(&header, &item, items::Mode::Sync);
                let async_item = items::generate_item(&header, &item, items::Mode::Async);

                sync_item.map(move |sync_item| (sync_item, async_item.unwrap_or_else(|| "".into())))
            })
            .transpose()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(item_pairs.into_iter().unzip())
}

fn generate_types(headers: &[Header]) -> String {
    // list of header imports
    let mut base = r#"

#[allow(dead_code, unused_imports)]
mod types {
    pub(crate) type Card8 = u8;
    pub(crate) type Card16 = u16;
    pub(crate) type Card32 = u32;
    pub(crate) type Card64 = u64;
    pub(crate) type Bool = bool;
    pub(crate) type Char = u8;
    pub(crate) type Byte = u8;
    pub(crate) type Int8 = i8;
    pub(crate) type Int16 = i16;
    pub(crate) type Int32 = i32;
    pub(crate) type Float = f32;
    pub(crate) type Double = f64;
    pub(crate) type Void = u8;

    pub(crate) use crate::Fd;
"#
    .to_string();

    // append header imports to this mess
    for header in headers {
        if !always_available(&header.header) {
            writeln!(base, r#"    #[cfg(feature = "{}")]"#, header.header).unwrap();
        }
        writeln!(
            base,
            "    pub(crate) use crate::protocol::{}::{{self, *}};",
            header.header
        )
        .unwrap();
    }

    // finish it up
    base.push('}');
    base
}

pub fn always_available(name: &str) -> bool {
    matches!(name, "xproto" | "bigreq" | "xc_misc" | "ge")
}
