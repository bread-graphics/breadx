// MIT/Apache2 License

use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

#[inline]
pub fn get_mapped_lifetime(name: &str) -> usize {
    LIFETIME_MAP
        .lock()
        .unwrap()
        .map
        .get(name)
        .copied()
        .unwrap_or(0)
}

#[inline]
pub fn set_mapped_lifetime(name: &str, lifetimes: usize) {
    if let Some(_) = LIFETIME_MAP
        .lock()
        .unwrap()
        .map
        .insert(name.to_string(), lifetimes)
    {
        panic!()
    }
}

static LIFETIME_MAP: Lazy<Mutex<MapWrapper>> = Lazy::new(|| {
    Mutex::new(MapWrapper {
        map: HashMap::new(),
    })
});

struct MapWrapper {
    map: HashMap<String, usize>,
}

impl Drop for MapWrapper {
    #[inline]
    fn drop(&mut self) {
        println!("Lifetime map at drop time: {:?}", &self.map);
    }
}

#[inline]
pub fn lifetime() -> String {
    static LIFETIMES: Lazy<Mutex<LifetimeIter>> =
        Lazy::new(|| Mutex::new(LifetimeIter { inner: 0 }));
    LIFETIMES.lock().unwrap().next().unwrap()
}

struct LifetimeIter {
    inner: usize,
}

impl Iterator for LifetimeIter {
    type Item = String;

    #[inline]
    fn next(&mut self) -> Option<String> {
        loop {
            let mut next = self.inner;
            self.inner += 1;

            let mut result = String::new();
            loop {
                let modulo = next % 26;
                result.push_str(ALPHABET[modulo]);
                next /= 26;

                if next == 0 {
                    break;
                }
            }
            if result == "if" {
                continue;
            }
            return Some(result);
        }
    }
}

const ALPHABET: &[&'static str] = &[
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];
