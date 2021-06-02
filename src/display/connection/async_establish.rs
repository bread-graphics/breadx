// MIT/Apache2 License

use super::{create_setup, AsyncConnection};
use crate::auth_info::AuthInfo;
use tinyvec::TinyVec;

#[inline]
pub(crate) async fn establish_connection_async<C: AsyncConnection>(
    conn: &mut C,
    auth_info: Option<AuthInfo>,
) -> crate::Result<(Setup, XidGenerator)> {
    let setup = create_setup(match auth {
        Some(auth) => auth,
        None => AuthInfo::get_async().await,
    });

    // write setup request
    let mut _fds: Vec<Fd> = vec![];
    let mut bytes: TinyVec<[u8; 32]> = iter::repeat(0).take(setup.size()).collect();
    let len = setup.as_bytes(&mut bytes);

    conn.send_packet_async(&bytes[0..len], &mut _fds).await?;

    // read setup
    let mut bytes: TinyVec<[u8; 8]> = iter::repeat(0).take(8).collect();
    conn.read_packet_async(&mut bytes, &mut _fds).await?;

    // figure out whether or not it succeeded
    match bytes[0] {
        0 => return Err(crate::BreadError::FailedToConnect),
        2 => return Err(crate::BreadError::FailedToAuthorize),
        _ => (),
    }

    // read in the rest of the setup
    let length = u16::from_ne_bytes([bytes[6], bytes[7]]) as usize * 4;
    bytes.extend(iter::repeat(0).take(length));
    conn.read_packet_async(&mut bytes[8..], &mut _fds).await?;

    let (setup, _) =
        Setup::from_bytes(&bytes).ok_or(crate::BreadError::BadObjectRead(Some("Setup")))?;
    let xid = XidGenerator::new(setup.resource_id_base, setup.resource_id_mask);
    Ok((setup, xid))
}
