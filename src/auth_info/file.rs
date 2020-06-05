// MIT/Apache2 License

use std::{env, path::PathBuf};

#[inline]
pub(crate) fn xauth_file() -> Option<PathBuf> {
    match env::var_os("XAUTHORITY") {
        Some(xauth) => Some(xauth.into()),
        None => env::var_os("HOME").map(|home| {
            let mut res: PathBuf = home.into();
            res.push(".Xauthority");
            res
        }),
    }
}
