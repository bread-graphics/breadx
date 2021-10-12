// MIT/Apache2 License

#![feature(test)]

extern crate test;
use test::Bencher;

#[bench]
fn benchmark_parse_setup(b: &mut Bencher) {
    use breadx::{display::dummy::PreprogrammedConnection, BasicDisplay};

    b.iter(|| {
        let bh = PreprogrammedConnection::normal_setup(std::iter::empty());
        BasicDisplay::from_connection(bh, 0, Default::default()).unwrap();
    });
}
