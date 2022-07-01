//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

//! This module contains runtime support for the most common runtimes:
//! [`tokio`] and [`async-std`].
//!
//! [`tokio`]: crate::rt_support::tokio_support
//! [`async-std`]: crate::rt_support::async_std_support

#[cfg(feature = "async-std-support")]
pub mod async_std_support;
#[cfg(all(feature = "tokio-support", unix))]
pub mod tokio_support;
