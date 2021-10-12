// MIT/Apache2 License

#![feature(test)]
extern crate test;

use test::Bencher;

#[bench]
fn send_large_request(b: &mut Bencher) {
    use breadx::{
        auto::xproto::{CoordMode, Drawable, Gcontext, Point, PolyLineRequest},
        display::dummy::{PreprogrammedConnection, Transaction},
        prelude::*,
        BasicDisplay,
    };
    use std::{borrow::Cow, vec::Vec};

    let polypoints = (0..100)
        .map(|i| Point { x: i, y: i })
        .collect::<Vec<Point>>();
    let plr = PolyLineRequest {
        coordinate_mode: CoordMode::default(),
        drawable: Drawable::const_from_xid(0),
        gc: Gcontext::const_from_xid(0),
        points: Cow::Borrowed(&polypoints),
        ..Default::default()
    };

    b.iter(|| {
        let mut conn = BasicDisplay::from_connection(
            PreprogrammedConnection::normal_setup(Some(Transaction::request(plr.clone()))),
            0,
            Default::default(),
        )
        .expect("Connection is broken");
        conn.set_checked(false);
        conn.send_request(plr.clone())
    });
}
