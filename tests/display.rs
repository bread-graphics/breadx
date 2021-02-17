// MIT/Apache2 License

#[path = "./common/void_conn.rs"]
mod void_conn;

use breadx::{auto::AsByteSequence, Display, Setup};
use void_conn::VoidConnection;

#[inline]
fn setup() -> Setup {
    let mut s = Setup {
        status: 1,
        protocol_major_version: 11,
        protocol_minor_version: 0,
        release_number: 12009000,
        resource_id_base: 67108864,
        resource_id_mask: 2097181,
        maximum_request_length: std::u16::MAX as u32,
        vendor: "Dummy Connection".to_string(),
        ..Default::default()
    };

    s.length = (s.size() as u16 / 4) - 2;
    s
}

macro_rules! harness {
    ($conn: ident, $tt: tt) => {{
        let conn = VoidConnection::from_setup(setup());
        let mut $conn = Display::from_connection(conn, None).unwrap();

        {
            $tt
        }
    }};
}

#[test]
fn open() {
    harness!(_conn, {});
}
