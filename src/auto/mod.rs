// MIT/Apache2 License

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::mem;
use cstr_core::CString;
use cty::c_char;

pub mod prelude {
    pub use super::{
        string_as_bytes, string_from_bytes, vector_as_bytes, vector_from_bytes, AsByteSequence,
    };
    pub use crate::{client_message_data::ClientMessageData, Request, XidType, XID};
    pub use alloc::{string::String, vec::Vec};
    pub type Card8 = u8;
    pub type Card16 = u16;
    pub type Card32 = u32;
    pub type Byte = u8;
    pub type Int8 = i8;
    pub type Int16 = i16;
    pub type Int32 = i32;
}

/// Internal use helper trait. This represents an item that can be converted to and from a series
/// of bytes.
pub trait AsByteSequence: Sized {
    /// The number of bytes needed to contain this item.
    /// Not guaranteed to be exact for allocated types.
    fn size() -> usize;
    /// Append this item to a sequence of bytes.
    fn as_bytes(&self, bytes: &mut [u8]) -> usize;
    /// Convert a sequence of bytes into this item.
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)>;
    /// Returns true if the first element of this object is a u8, and can be
    /// optimized into the padding in headers.
    fn includes_optimization() -> bool;
}

/// Internal use helper functions to build a vector of elements from a pointer to the bytes and the
/// desired length.
/// TODO: specialize this somewhat
#[inline]
pub fn vector_from_bytes<T: AsByteSequence>(bytes: &[u8], len: usize) -> Option<(Vec<T>, usize)> {
    log::trace!("Deserializing vector of byte length {} from bytes", len);

    // allocate the vector
    let mut items: Vec<T> = Vec::with_capacity(len);

    // pull items from the bytes vector and create elements
    let mut current_index = 0;
    while current_index < len {
        let (item, sz) = T::from_bytes(&bytes[current_index..])?;
        items.push(item);
        current_index += sz;
    }

    Some((items, current_index))
}

/// Internal use function to make it easier to convert the c-equivalent string to a Rust string.
#[inline]
pub fn string_from_bytes(bytes: &[u8], len: usize) -> Option<(String, usize)> {
    log::trace!("Deserializing string of length {} from bytes", len);

    let chars: Vec<u8> = bytes.iter().take(len).copied().collect();

    // convert this vector to the equivalent CString item
    let cstr: CString = match CString::new(chars) {
        Ok(cstr) => cstr,
        Err(e) => {
            // strip all zeroes and try again
            let mut chars = e.into_vec();
            chars.retain(|x| *x != 0);
            CString::new(chars).ok()?
        }
    };

    // convert the cstring into a real string
    match cstr.into_string() {
        Ok(s) => Some((s, len)),
        Err(estr) => {
            log::warn!("Encountered invalid UTF-8, redoing with substitution.");
            let mut cstr = estr.into_cstring().into_bytes();
            cstr.iter_mut().for_each(|b| {
                if *b > 127 {
                    *b = b'?';
                }
            });
            Some((CString::new(cstr).ok()?.into_string().ok()?, len as usize))
        }
    }
}

/// Internal use function to convert a vector of AsByteSequence types to bytes.
/// TODO: specialization
#[inline]
pub fn vector_as_bytes<T: AsByteSequence>(vector: &[T], bytes: &mut [u8]) -> usize {
    let mut current_index = 0;

    vector.iter().for_each(|item| {
        item.as_bytes(&mut bytes[current_index..]);
        current_index += T::size();
    });

    current_index
}

/// Internal use function to convert a String to bytes.
#[inline]
pub fn string_as_bytes(string: &str, bytes: &mut [u8]) -> usize {
    vector_as_bytes(string.as_bytes(), bytes)
}

impl AsByteSequence for u8 {
    #[inline]
    fn size() -> usize {
        1
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        bytes[0] = *self;
        1
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(u8, usize)> {
        Some((bytes[0], 1))
    }

    #[inline]
    fn includes_optimization() -> bool {
        // not applicable
        false
    }
}

impl AsByteSequence for i8 {
    #[inline]
    fn size() -> usize {
        1
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        bytes[0] = *self as u8;
        1
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(i8, usize)> {
        Some((bytes[0] as i8, 1))
    }

    #[inline]
    fn includes_optimization() -> bool {
        // not applicable
        false
    }
}

impl AsByteSequence for bool {
    #[inline]
    fn size() -> usize {
        1
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        bytes[0] = if *self { 1 } else { 0 };
        1
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        match bytes[0] {
            0 => Some((false, 1)),
            _ => Some((true, 1)),
        }
    }

    #[inline]
    fn includes_optimization() -> bool {
        false
    }
}

impl AsByteSequence for () {
    #[inline]
    fn size() -> usize {
        0
    }

    #[inline]
    fn as_bytes(&self, _bytes: &mut [u8]) -> usize {
        0
    }

    #[inline]
    fn from_bytes(_bytes: &[u8]) -> Option<(Self, usize)> {
        Some(((), 0))
    }

    #[inline]
    fn includes_optimization() -> bool {
        false
    }
}

macro_rules! impl_fundamental_num {
    ($(($t:ty, $sz:expr))*) => {$(
        impl AsByteSequence for $t {
            #[inline]
            fn size() -> usize {
                $sz
            }

            #[inline]
            fn as_bytes(&self, bytes: &mut [u8]) -> usize {
                let my_bytes = self.to_ne_bytes();
                (&mut bytes[0..$sz]).copy_from_slice(&my_bytes);
                $sz
            }

            #[inline]
            fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
                log::trace!("Deserializing {} from bytes", stringify!($t));

                cfg_if::cfg_if! {
                    if #[cfg(debug_assertions)] {
                        if bytes.len() < $sz {
                            log::error!("There are {} bytes in the slice, expected {}", bytes.len(), $sz);
                            return None;
                        }
                    }
                }

                let mut my_bytes = [0; $sz];
                my_bytes.copy_from_slice(&bytes[0..$sz]);
                Some((Self::from_ne_bytes(my_bytes), $sz))
            }

            #[inline]
            fn includes_optimization() -> bool {
                false
            }
        }
    )*}
}

const USIZE_SIZE: usize = mem::size_of::<usize>();
const ISIZE_SIZE: usize = mem::size_of::<isize>();
impl_fundamental_num! {
    (i16, 2) (u16, 2) (i32, 4) (u32, 4) (i64, 8) (u64, 8) (i128, 16) (u128, 16)
    (usize, USIZE_SIZE) (isize, ISIZE_SIZE)
}

macro_rules! impl_array {
    ($($len:expr),*) => {$(
        impl<T: AsByteSequence + Default> AsByteSequence for [T; $len] {
            #[inline]
            fn size() -> usize {
                T::size() * ($len)
            }

            #[inline]
            fn as_bytes(&self, bytes: &mut [u8]) -> usize {
                let mut index = 0;
                for i in 0..($len) {
                    index += self[i].as_bytes(&mut bytes[index..]);
                }
                index
            }

            #[inline]
            fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
                let mut index = 0;
                let mut v: tinyvec::ArrayVec<Self> = Default::default();
                log::trace!("Deserializing an array of length {}", $len);

                for i in 0..($len) {
                    let (item, sz) = T::from_bytes(&bytes[index..])?;
                    if let Some(_) = v.try_push(item) {
                        const SZ: usize = $len;
                        log::error!("Array of {} elements overflowed", SZ);
                        return None;
                    }
                    index += sz;
                }

                Some((v.into_inner(), index))
            }

            #[inline]
            fn includes_optimization() -> bool {
                false
            }
        }
    )*}
}

impl_array! {
    0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32
}

impl AsByteSequence for String {
    #[inline]
    fn size() -> usize {
        // the idea of sizing kind of breaks down here
        // so just assume all strings are at least 64 characters long. this shouldn't cause significant
        // issues except on embedded systems
        // note: this isn't used for as_bytes or from_bytes so
        // it shouldn't cause significant breakage
        <c_char>::size() * 64
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        bytes.copy_from_slice(String::as_bytes(self));
        bytes[self.len()] = 0;
        self.len() + 1
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        log::trace!("Deserializing string of unknown length");

        let end = match memchr::memrchr(0, bytes) {
            Some(posn) => posn - 1,
            None => bytes.len() - 1,
        };

        let s = String::from_utf8_lossy(&bytes[..end]).to_string();
        let len = s.len();
        Some((s, len))
    }

    #[inline]
    fn includes_optimization() -> bool {
        false
    }
}

//pub mod glx;
//pub mod glx;
//pub mod render;
//pub mod shape;
//pub mod xfixes;
pub mod xc_misc;
pub mod xproto;
