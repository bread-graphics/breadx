//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

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
