// MIT/Apache2 License

mod enumeration;
mod expr;
mod field;
mod item;
mod specific_field;
mod state;
mod structure;
mod ty;
mod xidtype;

pub use enumeration::*;
pub use expr::*;
pub use field::*;
pub use item::*;
pub use specific_field::*;
pub use state::*;
pub use structure::*;
pub use ty::*;
pub use xidtype::*;

#[inline]
pub fn safe_name(mut name: String) -> String {
    match name.as_str() {
        "type" => {
            name = "ty".to_owned();
        }
        "0" => {
            name = "Zero".to_owned();
        }
        "1" => {
            name = "One".to_owned();
        }
        "2" => {
            name = "Two".to_owned();
        }
        "3" => {
            name = "Three".to_owned();
        }
        "4" => {
            name = "Four".to_owned();
        }
        "5" => {
            name = "Five".to_owned();
        }
        "6" => {
            name = "Six".to_owned();
        }
        "7" => {
            name = "Seven".to_owned();
        }
        "8" => {
            name = "Eight".to_owned();
        }
        "9" => {
            name = "Nine".to_owned();
        }
        "10" => {
            name = "Ten".to_owned();
        }
        "11" => {
            name = "Eleven".to_owned();
        }
        "12" => {
            name = "Twelve".to_owned();
        }
        "13" => {
            name = "Thirteen".to_owned();
        }
        "14" => {
            name = "Fourteen".to_owned();
        }
        "15" => {
            name = "Fifteen".to_owned();
        }
        "async" => {
            name = "async_".to_owned();
        }
        "event_type" => {
            name = "event_type_".to_owned();
        }
        "bytes" => {
            name = "bytes_".to_owned();
        }
        "match" => {
            name = "match_".to_owned();
        }
        "Option" => {
            name = "Option_".to_owned();
        }
        _ => (),
    }

    if name.chars().all(|c| c.is_numeric()) {
        log::error!("Safe name still composed of all numbers: {}", &name);
    }

    name
}
