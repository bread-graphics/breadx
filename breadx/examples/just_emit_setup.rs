// MIT/Apache2 License

//! Less of an example of what to do and more of just a diagnostic tool
//! for what our `Setup` looks like.

#[cfg(feature = "std")]
use breadx::{display::DisplayConnection, prelude::*};

#[cfg(feature = "std")]
fn main() {
    let conn = DisplayConnection::connect(None).unwrap();
    println!("{:#?}", conn.setup());
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("This example requires the `std` feature.");
}
