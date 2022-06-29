// MIT/Apache2 License

//! Less of an example of what to do and more of just a diagnostic tool
//! for what our `Setup` looks like.

use breadx::{display::DisplayConnection, prelude::*};

fn main() {
    let conn = DisplayConnection::connect(None).unwrap();
    println!("{:#?}", conn.setup());
}
