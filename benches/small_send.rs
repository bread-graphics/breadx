// MIT/Apache2 License

#![feature(test)]
extern crate test;

use test::Bencher;

#[bench]
fn send_small_request(b: &mut Bencher) {
    use breadx::{
        auto::xproto::GetInputFocusRequest,
        display::dummy::{PreprogrammedConnection, Transaction},
        prelude::*,
        BasicDisplay,
    };

    b.iter(|| {
        let mut conn = BasicDisplay::from_connection(
            PreprogrammedConnection::normal_setup(Some(Transaction::request(
                GetInputFocusRequest::default(),
            ))),
            0,
            Default::default(),
        )
        .expect("Connection is broken");
        conn.set_checked(false);
        conn.send_request(GetInputFocusRequest::default())
    });
}
