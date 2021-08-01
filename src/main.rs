extern crate eggricesoy;
use log::{debug, error, info, trace, warn};

fn main() {
  let app = eggricesoy::app();
  eggricesoy::init_app(app);
  trace!("This is trace");
  debug!("This is debug");
  info!("This is info");
  warn!("This is warn");
  error!("This is error");
}
