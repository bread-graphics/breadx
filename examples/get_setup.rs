// MIT/Apache2 License

//! This program is used to get a viable `Setup` from the server for use in dummy connections.

use breadx::{prelude::*, DisplayConnection, StaticSetup};
use std::{env, fs::File, io::prelude::*, iter};

fn main() {
    let display = DisplayConnection::create(None).unwrap();
    let mut bytes: Vec<u8> = iter::repeat(0).take(display.setup().size()).collect();
    display.setup().as_bytes(&mut bytes);

    println!("{}", bytes.len());
    println!(
        "Note: you are {} endian",
        if cfg!(target_endian = "little") {
            "little"
        } else {
            "big"
        }
    );

    let mut f = File::create(
        env::args_os()
            .nth(1)
            .expect("Pass in an argument for the file to write to"),
    )
    .unwrap();
    //f.write_all(&bytes).unwrap();
    write!(f, "{:#?}", display.setup()).unwrap();

    let _setup = StaticSetup::from_bytes(&bytes).unwrap();
}
