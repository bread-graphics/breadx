// MIT/Apache2 License

mod enumeration;
mod field;
mod item;
mod length;
mod specific_field;
mod state;
mod structure;
mod ty;
mod xidtype;

pub use enumeration::*;
pub use field::*;
pub use item::*;
pub use length::*;
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
        _ => (),
    }

    name
}
