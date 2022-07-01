//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

/// A type that can be used in the "void" position.
pub trait Void {
    /// The raw bytes of this item.
    fn bytes(&self) -> &[u8];
}

impl<T: bytemuck::NoUninit> Void for T {
    fn bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

impl<T: bytemuck::NoUninit> Void for [T] {
    fn bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }
}

impl Void for str {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
