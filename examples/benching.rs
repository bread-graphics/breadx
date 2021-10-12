// MIT/Apache2 License

fn main() {
  env_logger::init();

  use breadx::{display::dummy::PreprogrammedConnection, BasicDisplay};
  let pc = PreprogrammedConnection::normal_setup(std::iter::empty());
  let _b = BasicDisplay::from_connection(pc, 0, Default::default()).unwrap();
}
