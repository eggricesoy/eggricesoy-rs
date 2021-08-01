extern crate eggricesoy;
use log::debug;

fn main() {
  let app = eggricesoy::app();
  eggricesoy::init_app(app);
  debug!("okay!");
}
