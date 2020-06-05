// MIT/Apache2 License

use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
};

fn main() {
    // read in data to a JSON structure
    let mut args = env::args_os().skip(1);
    let mut infile = BufReader::new(File::open(args.next().unwrap()).unwrap());
    let mut outfile = BufWriter::new(File::create(args.next().unwrap()).unwrap());

    let mut buffer = String::new();
    infile.read_to_string(&mut buffer).unwrap();

    let data: Vec<Vec<serde_json::Value>> = serde_json::from_str(&buffer).unwrap();

    // begin writing to the outfile.
    outfile
        .write_all(
            "// Generated by breadx-keysym-generator, licensed under MIT/Apache2 License

#![allow(clippy::match_same_arms, clippy::too_many_lines)]

use crate::auto::xproto::Keysym;
use gluten_keyboard::Key;

#[inline]
#[must_use]
pub fn keysym_to_key(keysym: Keysym) -> Option<Key> {
    match keysym {
"
            .as_bytes(),
        )
        .unwrap();

    // write the data
    data.into_iter().for_each(|d| {
        if let (Some(num), Some(name)) = (d[0].as_str(), d[1].as_str()) {
            let num = u32::from_str_radix(&num[2..], 16).unwrap();

            outfile
                .write_all(format!("{} => Some(Key::{}),", num, name).as_bytes())
                .unwrap();
        }
    });

    outfile
        .write_all(
            "
        _ => None,
    }
}"
            .as_bytes(),
        )
        .unwrap();
}
