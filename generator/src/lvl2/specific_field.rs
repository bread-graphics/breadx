// MIT/Apache2 License

use super::{Field, StructureItem, Type};
use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    mem,
    ops::Deref,
    sync::atomic::{AtomicBool, Ordering},
};
use tinyvec::{ArrayVec, TinyVec};

pub enum StructVariant {
    No,
    Reply,
    Request,
    Error,
    Event(bool),
}

static IS_EXTENSION: AtomicBool = AtomicBool::new(false);

#[inline]
pub fn set_is_extension(val: bool) {
    IS_EXTENSION.store(val, Ordering::Release)
}

#[inline]
fn doesnt_need_pad_pop(field: &StructureItem) -> bool {
    let s = match field {
        StructureItem::Field(Field { ty, .. }) => match ty {
            Type::BasicType(Cow::Borrowed(s)) => s,
            Type::BasicType(Cow::Owned(s)) => s.as_str(),
            _ => return false,
        },
        _ => return false,
    };

    match s {
        "u32" | "Card32" | "CARD32" | "Drawable" | "DRAWABLE" | "Window" | "WINDOW" => true,
        _ => false,
    }
}

/// Configure a set of fields, given its struct variant.
#[inline]
pub fn configure_fields(fields: &mut TinyVec<[StructureItem; 6]>, variant: StructVariant) {
    let is_extension = IS_EXTENSION.load(Ordering::Acquire);

    // firstly, ensure that all fields are unique
    let mut hashes: Vec<u64> = Vec::with_capacity(fields.len());
    fields.retain(|field| {
        let mut hasher: ahash::AHasher = Default::default();
        field.hash(&mut hasher);
        let hash = hasher.finish();

        if hashes.contains(&hash) {
            false
        } else {
            hashes.push(hash);
            true
        }
    });

    // secondly, if the first field has a size of one, we could potentially optimize it
    const ONE_PAD: Option<StructureItem> = Some(StructureItem::Padding { bytes: 1 });
    let opt_field = if let StructVariant::Request
    | StructVariant::Reply
    | StructVariant::Event(false) = variant
    {
        if fields.is_empty()
            || matches!(fields[0], StructureItem::List(_))
            || (is_extension && matches!(variant, StructVariant::Request))
            || doesnt_need_pad_pop(&fields[0])
        {
            ONE_PAD
        } else {
            Some(fields.remove(0))
        }
    } else {
        ONE_PAD
    };

    match variant {
        StructVariant::No => (),
        StructVariant::Reply => {
            let mut header: ArrayVec<[StructureItem; 4]> = ArrayVec::from([
                // type field, this is always X_Reply
                StructureItem::Field(Field {
                    name: "reply_type".to_string(),
                    ty: Type::BasicType("u8".into()),
                    ..Default::default()
                }),
                // either a padding field or the optimized byte
                opt_field.unwrap(),
                // sequence number
                StructureItem::Field(Field {
                    name: "sequence".to_string(),
                    ty: Type::BasicType("u16".into()),
                    ..Default::default()
                }),
                // length in four-bytes
                StructureItem::Field(Field {
                    name: "length".to_string(),
                    ty: Type::BasicType("u32".into()),
                    ..Default::default()
                }),
            ]);

            let f = mem::take(fields);
            *fields = header.into_iter().chain(f.into_iter()).collect();

            if !fields.iter().any(|f| {
                if let StructureItem::Field(Field { name, .. }) = f {
                    name == "length"
                } else {
                    false
                }
            }) {
                panic!("how");
            }
        }
        StructVariant::Event(skip_sequence) => {
            let mut header: ArrayVec<[StructureItem; 3]> = ArrayVec::from_array_len(
                [
                    // type field, as always
                    StructureItem::Field(Field {
                        name: "event_type".to_string(),
                        ty: Type::BasicType("u8".into()),
                        ..Default::default()
                    }),
                    Default::default(),
                    Default::default(),
                ],
                1,
            );

            // sequence number
            if !skip_sequence {
                header.extend(vec![
                    opt_field.unwrap(),
                    StructureItem::Field(Field {
                        name: "sequence".to_string(),
                        ty: Type::BasicType("u16".into()),
                        ..Default::default()
                    }),
                ]);
            }

            let f = mem::take(fields);
            *fields = header.into_iter().chain(f.into_iter()).collect();
        }
        StructVariant::Error => {
            fields.insert(
                0,
                StructureItem::Field(Field {
                    name: "sequence".to_string(),
                    ty: Type::BasicType("u16".into()),
                    ..Default::default()
                }),
            );
            fields.insert(
                0,
                StructureItem::Field(Field {
                    name: "minor_code".to_string(),
                    ty: Type::BasicType("u8".into()),
                    ..Default::default()
                }),
            );
            fields.insert(
                0,
                StructureItem::Field(Field {
                    name: "major_code".to_string(),
                    ty: Type::BasicType("u8".into()),
                    ..Default::default()
                }),
            );
            fields.insert(
                0,
                StructureItem::Field(Field {
                    name: "error_code".to_string(),
                    ty: Type::BasicType("u8".into()),
                    ..Default::default()
                }),
            );
            fields.insert(
                0,
                StructureItem::Field(Field {
                    name: "_error_type".to_string(),
                    ty: Type::BasicType("u8".into()),
                    ..Default::default()
                }),
            );
        }
        StructVariant::Request => {
            let header: ArrayVec<[StructureItem; 3]> = ArrayVec::from([
                // header type
                StructureItem::Field(Field {
                    name: "req_type".into(),
                    ty: Type::BasicType("u8".into()),
                    ..Default::default()
                }),
                // either the optimized data or a byte of padding
                opt_field.unwrap(),
                // length of the request
                StructureItem::Field(Field {
                    name: "length".into(),
                    ty: Type::BasicType("u16".into()),
                    ..Default::default()
                }),
            ]);
            let f = mem::take(fields);
            *fields = header.into_iter().chain(f.into_iter()).collect();
        }
    }
}
