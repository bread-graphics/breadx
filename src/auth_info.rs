// MIT/Apache2 License

use alloc::string::String;

/// Information needed to authorize a user to use an X11 connection.
#[derive(Default, Debug)]
pub struct AuthInfo {
    pub name: String,
    pub data: String,
}
