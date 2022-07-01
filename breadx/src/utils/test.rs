//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

#![cfg(test)]

/// Set up a `tracing` subscriber for testing purposes.
#[allow(dead_code)]
pub(crate) fn setup_tracing() {
    tracing_subscriber::fmt::fmt()
        //.with_max_level(tracing::Level::TRACE)
        .try_init()
        .ok();
}
