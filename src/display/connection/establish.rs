// MIT/Apache2 License

use super::Connection;
use crate::{
    auth_info::AuthInfo,
    auto::xproto::{Setup, SetupRequest},
    xid::XidGenerator,
    Fd,
};
use core::iter;
use tinyvec::TinyVec;

#[inline]
const fn endian_byte() -> u8 {
    // Excerpt from the X Window System Protocol
    //
    // The client must send an initial byte of data to identify the byte order to be employed.
    // The value of the byte must be octal 102 or 154. The value 102 (ASCII uppercase B) means
    // values are transmitted most significant byte first, and value 154 (ASCII lowercase l)
    // means values are transmitted least significant byte first.
    #[cfg(not(target_endian = "little"))]
    {
        const BE_SIGNIFIER: u8 = b'B';
        BE_SIGNIFIER
    }
    #[cfg(target_endian = "little")]
    {
        const LE_SIGNIFIER: u8 = b'l';
        LE_SIGNIFIER
    }
}

#[inline]
pub(crate) fn create_setup(auth: AuthInfo) -> SetupRequest {
    let AuthInfo { name, data, .. } = auth;
    SetupRequest {
        byte_order: endian_byte(),
        protocol_major_version: 11,
        protocol_minor_version: 0,
        authorization_protocol_name: name,
        authorization_protocol_data: data,
    }
}

#[inline]
pub(crate) fn establish_connection<C: Connection>(
    conn: &mut C,
    auth_info: Option<AuthInfo>,
) -> crate::Result<(Setup, XidGenerator)> {
    let setup = create_setup(match auth {
        Some(auth) => auth,
        None => AuthInfo::get(),
    });

    // write setup request
    let mut _fds: Vec<Fd> = vec![];
    let mut bytes: TinyVec<[u8; 32]> = iter::repeat(0).take(setup.size()).collect();
    let len = setup.as_bytes(&mut bytes);

    conn.send_packet(&bytes[0..len], &mut _fds)?;

    // read setup
    let mut bytes: TinyVec<[u8; 8]> = iter::repeat(0).take(8).collect();
    conn.read_packet(&mut bytes, &mut _fds)?;

    // figure out whether or not it succeeded
    match bytes[0] {
        0 => return Err(crate::BreadError::FailedToConnect),
        2 => return Err(crate::BreadError::FailedToAuthorize),
        _ => (),
    }

    // read in the rest of the setup
    let length = u16::from_ne_bytes([bytes[6], bytes[7]]) as usize * 4;
    bytes.extend(iter::repeat(0).take(length));
    conn.read_packet(&mut bytes[8..], &mut _fds)?;

    let (setup, _) =
        Setup::from_bytes(&bytes).ok_or(crate::BreadError::BadObjectRead(Some("Setup")))?;
    let xid = XidGenerator::new(setup.resource_id_base, setup.resource_id_mask);
    Ok((setup, xid))
}
