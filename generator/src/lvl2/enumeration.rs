// MIT/Apache2 License

use super::{safe_name, Lvl2State, Type};
use crate::lvl1::{EnumData, EnumVariant, XEnum as Lvl1Enum};
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use tinyvec::TinyVec;

/// A true enum, where one value corresponds to one variant.
#[derive(Debug)]
pub struct TrueEnum {
    pub name: String,
    pub underlying: Type,
    pub variants: Vec<(String, i64)>,
}

/// A bitflags enum, where one value corresponds to one bit.
#[derive(Debug)]
pub struct Bitflags {
    pub name: String,
    pub underlying: Type,
    pub bits: Vec<(String, u64)>,
}

/// A list of constant items.
#[derive(Debug)]
pub struct ConstItems {
    pub items: Vec<(String, i128)>,
    pub underlying: Type,
}

/// A representation of the enum types.
#[derive(Debug)]
pub enum EnumRepr {
    TrueEnum(TrueEnum),
    Bitflags(Bitflags),
    ConstItems(ConstItems),
}

/// A generator for an EnumRepr type.
pub type EnumReprGenerator = Box<dyn FnOnce(Type) -> EnumRepr>;

/// Create a new EnumReprGenerator from a level 1 enum.
pub fn create_generator(state: &Lvl2State, mut lvl1: Lvl1Enum) -> (String, EnumReprGenerator) {
    let matching_name = lvl1.name.clone();
    let capsname = lvl1.name.to_uppercase();

    // Case 1: The enumeration has the same name as another type, or two of its cases have
    //         identical values. In this case, convert the enum to a list of constant items.
    if state.has_typename(&lvl1.name)
        || state.has_typename(&capsname)
        || state.has_typename(&lvl1.name.to_camel_case())
    {
        // create the pairs
        let Lvl1Enum { name, variants, .. } = lvl1;
        let underlying = name.to_camel_case();
        let name = name.to_shouty_snake_case();
        let items: Vec<(String, i128)> = variants
            .into_iter()
            .map(|v| {
                let name = format!("{}_{}", &name, v.name.to_shouty_snake_case());
                let value = match v.data {
                    EnumData::Value(v) => v as i128,
                    EnumData::Bit(b) => 1 << b,
                };

                (name, value)
            })
            .collect();

        return (
            matching_name,
            Box::new(move |_| {
                EnumRepr::ConstItems(ConstItems {
                    items,
                    underlying: Type::BasicType(underlying.into()),
                })
            }),
        );
    }

    // Remove duplicates from the set.

    #[inline]
    fn val_of_em(em: &EnumVariant) -> u64 {
        match em.data {
            EnumData::Value(v) => v as _,
            EnumData::Bit(i) => 1 << i,
        }
    }

    // test for unique
    let Lvl1Enum {
        name, mut variants, ..
    } = lvl1;
    variants.move_to_the_heap();
    let mut variants = match variants {
        TinyVec::Heap(v) => v,
        _ => unreachable!(),
    };
    variants.sort_unstable_by_key(val_of_em);
    variants.dedup_by_key(|em| val_of_em(em));

    // Case 2: The enumeration contains any combination of "bits" items. In this case, we assume
    //         that it is a bitflag.
    // Case 3: If nothing else, we assume it is a true enum.
    match variants.iter().any(|v| matches!(&v.data, EnumData::Bit(_))) {
        true => {
            let name = name.to_camel_case();
            let bits = variants
                .into_iter()
                .filter_map(|v| {
                    let data = match v.data {
                        // on the zero case, just ignore it
                        EnumData::Value(0) => return None,
                        // on any other value case, error out
                        EnumData::Value(_) => return Some(Err(())),
                        EnumData::Bit(b) => b as u64,
                    };
                    let name = safe_name(v.name.to_snake_case());
                    Some(Ok((name, data)))
                })
                .collect::<Result<Vec<_>, ()>>()
                .expect("Invalid bitflag declaration");

            return (
                matching_name,
                Box::new(move |underlying| {
                    EnumRepr::Bitflags(Bitflags {
                        name,
                        bits,
                        underlying,
                    })
                }),
            );
        }
        false => {
            let name = name.to_camel_case();
            let variants = variants
                .into_iter()
                .map(|v| {
                    let EnumVariant { name, data } = v;

                    let name = safe_name(name.to_camel_case());
                    let data = match data {
                        EnumData::Value(v) => v,
                        _ => unreachable!(),
                    };

                    (name, data)
                })
                .collect();

            return (
                matching_name,
                Box::new(move |underlying| {
                    EnumRepr::TrueEnum(TrueEnum {
                        name,
                        variants,
                        underlying,
                    })
                }),
            );
        }
    }
}
