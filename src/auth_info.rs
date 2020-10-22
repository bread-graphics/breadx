// MIT/Apache2 License

use alloc::string::String;

#[derive(Default, Debug)]
pub struct AuthInfo {
    pub name: String,
    pub data: String,
}
