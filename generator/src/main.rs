// MIT/Apache2 License

#![recursion_limit = "256"]

mod lvl0;
mod lvl1;
mod lvl2;
mod lvl3;

use lvl3::ToSyn;
use quick_xml::{events::Event, Reader};
use quote::ToTokens;
use regex::Regex;
use std::{
    env,
    error::Error,
    fs,
    io::{prelude::*, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Error)
        .init();

    // open the file
    let fname = env::args_os()
        .nth(1)
        .unwrap_or_else(|| panic!("Generator requires at least 2 arguments"));
    let outname = env::args_os()
        .nth(2)
        .unwrap_or_else(|| panic!("Generator requires at least 2 arguments"));
    let file = fs::File::open(&fname)?;
    let file = BufReader::new(file);
    let mut outfile = fs::File::create(&outname)?;

    // open the XML reader
    let mut reader = Reader::from_reader(file);

    // Stage 1: Read from Level 0 representation (XML) into Level 1 representation. Result is a
    //          vector of Level 1 items.
    let mut buf = vec![];
    let mut lvl1_items = vec![];
    let mut lvl0_state: lvl0::Lvl0State = Default::default();
    let mut ext_name: Option<String> = None;

    loop {
        match reader.read_event(&mut buf) {
            Err(e) => panic!(
                "XML Error at position {}: {:?}",
                reader.buffer_position(),
                e
            ),
            Ok(Event::Eof) => break,
            Ok(event) => {
                if let Some(item) = lvl0_state.react_to_event(event, &mut ext_name) {
                    lvl1_items.push(item);
                }
            }
        }

        buf.clear();
    }

    assert!(matches!(lvl0_state, lvl0::Lvl0State::AwaitingTopLevel));

    // Stage 2: Normalize from Level 1 representation to Level 2 representation. This expands some of the copying,
    //          converts enums to what they're represented as in Rust, and preforms some other optimizations.
    let (lvl2_items, xidtypes) = lvl2::convert_series(lvl1_items, ext_name.is_some());

    // Stage 3: Normalize to a basic Rust representation.
    let lvl3_items: Vec<lvl3::Item> = lvl2_items
        .into_iter()
        .flat_map(|lvl2| lvl3::Item::from_lvl2(lvl2, &xidtypes, ext_name.as_deref()))
        .collect();

    // Stage 4: Convert to syn items
    let lvl4_items: Vec<syn::Item> = lvl3_items
        .into_iter()
        .flat_map(|lvl3| lvl3.to_syn_item())
        .collect();
    let lvl4_file = syn::File {
        shebang: None,
        attrs: vec![],
        items: lvl4_items,
    };

    // Stage 5: Convert to string and save to file
    let tokens = lvl4_file.into_token_stream();
    let output_pre_regex = format!("{}", tokens);

    // last minute modifications
    let re = Regex::new(r"\bBool\b").unwrap();
    let output_pr2 = re.replace_all(&output_pre_regex, "bool");

    let re = Regex::new(r"\bStr\b").unwrap();
    let output = re.replace_all(&output_pr2, "String");

    // monkey patch
    let re = Regex::new(r"\bPixmap<'.>\b").unwrap();
    println!("There are {} matches", re.find_iter(&output).count());
    let output2 = re.replace_all(&output, "Pixmap");
    let re = Regex::new(r"IdleNotifyEvent<'.>").unwrap();
    let output3 = re.replace_all(&output2, "IdleNotifyEvent");
    let re = Regex::new(r"impl<'.> IdleNotifyEvent").unwrap();
    let output4 = re.replace_all(&output3, "impl IdleNotifyEvent");
    let re = Regex::new(r"impl<'.> AsByteSequence for IdleNotifyEvent").unwrap();
    let output5 = re.replace_all(&output4, "impl AsByteSequence for IdleNotifyEvent");

    write!(
        outfile,
        "// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

{}",
        output5
    )?;

    Ok(())
}
