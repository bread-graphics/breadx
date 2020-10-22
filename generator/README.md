# flutterbug_generator

This sub-crate generates X11-interfacing code for `flutterbug` from the `xproto` XML description files.

## Rules

This is how this crate translates certain XML tags into Rust structures and concepts.

### typedef

A simple `typedef` statement in C that translates to:

```
pub type [oldname] = [newname];
```

### xidtype

Creates a wrapper type around the `XID` type, and adds an implementation for the `XidType` trait

```
#[derive(Copy, Clone, Default, Debug)]
pub struct [name] {
    inner: XID
}

impl XidType for [name] {
    #[inline]
    fn xid(&self) -> XID {
        self.inner
    }

    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { inner: xid }
    }
}

impl [name] {
    #[inline]
    pub(crate) const fn const_from_xid(xid: XID) -> Self {
        Self { xid }
    }
}
```

The last impl exists for convenience in instantiating const versions of these `XidType` items.

### struct

Creates a Rust structure that, among other things, implements the `AsByteSequence` trait to allow it to be sent across or read from byte streams. The field tags are treated as follows:

* `field` - Creates a field with name `name` and type `type`.
* `pad` - Modifies the `AsByteSequence` implementation to skip certain bytes when writing or reading.
* `list` - Behavior depends on its length.
 * If the length is fixed, this becomes a field of type `[type; length]`.
 * If the length depends on a single field, this becomes a field of type `Vec<type>` and the single field is "absorbed" into the list
 * If the length depends on a series of operations on fixed values or fields, this becomes a field of type `Vec<type>` whose length is assumed to be equal to the result of the operation.

```
#[derive(Clone, Default, Debug)]
pub struct [name] {
    [fieldName1]: [fieldType1],
    [fieldName2]: [fieldType2],
}

impl AsByteSequence for [name] {
    #[inline]
    fn size() -> usize {
        [fieldType1]::size() + [padBytes] + [fieldType2]::size()
    } 

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index = 0;
        index += [fieldType1]::as_bytes(&mut bytes[index..]);
        index += [padBytes];
        index += [fieldType2]::as_bytes(&mut bytes[index..]);
        index
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index = 0;
        let (fieldName1, sz) = fieldType1::from_bytes(&bytes[index..]);
        index += sz;
        index += [padBytes];
        let (fieldName2, sz) = fieldType2::from_bytes(&bytes[index..]);
        index += sz;
        Some((Self { fieldName1, fieldName2 }, index))
    }
}
```

### enum

Because of the variety of uses for enums in C, there are several available translations used in Rust.

#### True Enum

If all of the elements of the enum are unique, non-bitfield elements, it creates a true Rust enum.

```
#[repr([representation])]
pub enum [name] {
    [Variant1] = [Value1], 
    [Variant2] = [Value2],
}

impl AsByteSequence for [name] {
    #[inline]
    fn size() -> usize {
        [representation]::size()
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (self as [representation]).as_bytes(bytes)
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (repr, sz) = [representation]::from_bytes(bytes)
        Some((match repr {
            [Value1] => Self::[Variant1],
            [Value2] => Self::[Variant2],
            _ => return None,
        }, sz))
    }
}
```

#### Bitfield
