// MIT/Apache2 License

use super::XStruct;

#[derive(Default, Debug)]
pub struct XError {
    pub base: XStruct,
    pub number: u64,
}

#[derive(Default, Debug)]
pub struct XErrorCopy {
    pub name: String,
    pub number: usize,
    pub base: String,
}
