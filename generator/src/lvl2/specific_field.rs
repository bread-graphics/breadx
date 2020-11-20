// MIT/Apache2 License

use super::{Field, StructureItem, Type};
use std::{
    hash::{Hash, Hasher},
    mem,
    ops::Deref,
};
use tinyvec::{ArrayVec, TinyVec};

pub enum StructVariant {
    No,
    Reply,
    Request,
    Error,
    Event,
}

/// Configure a set of fields, given its struct variant.
#[inline]
pub fn configure_fields(fields: &mut TinyVec<[StructureItem; 6]>, variant: StructVariant) {
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
    let opt_field =
        if let StructVariant::Request | StructVariant::Reply | StructVariant::Event = variant {
            if fields.is_empty() {
                Some(StructureItem::Padding { bytes: 1 })
            } else {
                Some(fields.remove(0))
            }
        } else {
            None
        };

    match variant {
        StructVariant::No => (),
        StructVariant::Reply => {
            let header: ArrayVec<[StructureItem; 4]> = ArrayVec::from([
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
        }
        StructVariant::Event => {
            let header: ArrayVec<[StructureItem; 3]> = ArrayVec::from([
                // type field, as always
                StructureItem::Field(Field {
                    name: "event_type".to_string(),
                    ty: Type::BasicType("u8".into()),
                    ..Default::default()
                }),
                opt_field.unwrap(),
                // sequence number
                // sequence number
                StructureItem::Field(Field {
                    name: "sequence".to_string(),
                    ty: Type::BasicType("u16".into()),
                    ..Default::default()
                }),
            ]);
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
