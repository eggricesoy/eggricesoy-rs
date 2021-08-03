extern crate eggricesoy;
use log::{debug, error, info, trace, warn};

fn main() {
  let app = eggricesoy::app!();
  eggricesoy::init_app(app);
  // for _ in 1..1000 {
  trace!("This is trace");
  debug!("This is debug");
  info!("This is info");
  warn!("This is warn");
  error!("This is error");
  // }
}
