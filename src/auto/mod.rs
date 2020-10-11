// MIT/Apache2 License

use std::{ffi::CString, os::raw::c_char};

pub mod prelude {
    pub use super::{clone_from_ptr, string_from_ptr, AsByteSequence};
    pub use crate::{client_message_data::ClientMessageData, Request, XidType, XID};
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
    fn size() -> usize;
    /// Append this item to a sequence of bytes.
    fn as_bytes(&self, bytes: &mut [u8]);
    /// Convert a sequence of bytes into this item.
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
}

/// Internal use helper functions that clones elements from a pointer and a specified length.
/// Used to automatically instantiate list parts of structs.
#[inline]
pub unsafe fn clone_from_ptr<T: AsByteSequence>(ptr: *const u8, len: u16) -> Vec<T> {
    use std::slice;
    let len = len as usize;

    // SAFETY: the caller confirms that ptr is valid and contains len elements. that's why this function
    //         is unsafe
    let elems = unsafe { slice::from_raw_parts(ptr as *const u8, len) };

    // use from_bytes to resolve the bytes
    let mut vector: Vec<T> = Vec::with_capacity(len);
    let mut index = 0;
    for _ in 0..len {
        // note: for now, just ignore invalid byte sequences
        match T::from_bytes(&elems[index..]) {
            Some(elem) => vector.push(elem),
            None => log::warn!("Invalid byte sequence occurred while processing vector"),
        }

        index += T::size();
    }

    vector
}

/// Internal use function to make it easier to convert the c-equivalent string to a Rust string.
#[inline]
pub unsafe fn string_from_ptr(ptr: *const u8, len: u16) -> String {
    // most of our logic is already implemented in clone_from_ptr
    let chars: Vec<c_char> = unsafe { clone_from_ptr(ptr, len) };

    // convert this vector to the equivalent CString item
    let cstr: CString =
        CString::new(chars.into_iter().map(|c| c as u8).collect()).unwrap_or_else(|e| {
            // strip all zeroes and try again
            let mut chars = e.into_vec();
            chars.retain(|x| *x != 0);
            CString::new(chars).expect("Invalid string received from X11")
        });

    // convert the cstring into a real string
    cstr.into_string().expect("Invalid UTF-8 received from X11")
}

impl AsByteSequence for u8 {
    #[inline]
    fn size() -> usize {
        1
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) {
        bytes[0] = *self;
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<u8> {
        bytes[0]
    }
}

impl AsByteSequence for i8 {
    #[inline]
    fn size() -> usize {
        1
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) {
        bytes[0] = *self as u8;
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<u8> {
        bytes[0] as i8
    }
}

impl AsByteSequence for bool {
    #[inline]
    fn size() -> usize {
        1
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) {
        bytes[0] = if *self { 1 } else { 0 };
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match *bytes[0] {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}

impl AsByteSequence for () {
    #[inline]
    fn size() -> usize {
        0
    }

    #[inline]
    fn as_bytes(&self, _bytes: &mut [u8]) {}

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(())
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
            fn as_bytes(&self, bytes: &mut [u8]) {
                let mut my_bytes = self.to_ne_bytes();
                ::std::mem::swap(&mut bytes[0..$sz], &mut my_bytes[0..$sz]);
            }

            #[inline]
            fn from_bytes(bytes: &[u8]) -> Option<Self> {
                let mut my_bytes = [0; $sz];
                my_bytes.copy_from_slice(bytes);
                Some(Self::from_ne_bytes(my_bytes))
            }
        }
    )*}
}

const USIZE_SIZE: usize = std::mem::size_of::<usize>();
const ISIZE_SIZE: usize = std::mem::size_of::<isize>();
impl_fundamental_num! {
    (i16, 2) (u16, 2) (i32, 4) (u32, 4) (i64, 8) (u64, 8)
    (usize, USIZE_SIZE) (isize, ISIZE_SIZE)
}

macro_rules! impl_array {
    ($($len:expr),*) => {$(
        impl<T: AsByteSequence> AsByteSequence for [T; $len] {
            #[inline]
            fn size() -> usize {
                T::size() * ($len)
            }

            #[inline]
            fn as_bytes(&self, bytes: &mut [u8]) {
                let mut index = 0;
                for i in 0..($len) {
                    self[i].as_bytes(&mut bytes[index..]);
                    index += T::size();
                }
            }

            #[inline]
            fn from_bytes(bytes: &[u8]) -> Option<Self> {
                let mut index = 0;
                let mut v: smallvec::SmallVec<Self> = Default::default();

                for i in 0..($len) {
                    v.push(T::from_bytes(&bytes[0..index])?);
                    index += T::size();
                }

                v.into_inner().ok()
            }
        }
    )*}
}

impl_array! {
    0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32
}

impl<T: ?Sized> AsByteSequence for *const T {
    #[inline]
    fn size() -> usize {
        <usize>::size()
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) {
        (self as *const () as usize).as_bytes(bytes)
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let raw: usize = usize::from_bytes(bytes);
        raw as *const () as *const T
    }
}

impl<T: ?Sized> AsByteSequence for *mut T {
    #[inline]
    fn size() -> usize {
        <usize>::size()
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) {
        (self as *const () as usize).as_bytes(bytes)
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let raw: usize = usize::from_bytes(bytes);
        raw as *const () as *mut T
    }
}

impl AsByteSequence for String {
    #[inline]
    fn size() -> usize {
        <*const c_char>::size()
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) {
        let cstr = CString::new(self.clone()).unwrap();
        cstr.into_raw().as_bytes(bytes)
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let raw = <*const c_char>::from_bytes(bytes)?;
        let len = unsafe { strlen(raw) };
        Some(unsafe { string_from_ptr(raw as *const (), len) })
    }
}

// quick strlen reimplementation
#[inline]
unsafe fn strlen(mut s: *const c_char) -> usize {
    let mut len = 0;
    while unsafe { s.read() } != 0 {
        s = unsafe { s.offset(1) };
        len += 1;
    }
    len
}

//pub mod glx;
//pub mod glx;
//pub mod render;
//pub mod shape;
//pub mod xfixes;
pub mod xproto;
