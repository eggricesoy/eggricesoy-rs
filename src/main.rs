extern crate eggricesoy;
use log::{debug, error, info, trace, warn};

fn main() {
  let app = eggricesoy::app!();
  let option_handle = eggricesoy::init_app(app).1;
  // for _ in 1..1000 {
  trace!("This is trace");
  debug!("This is debug");
  info!("This is info");
  warn!("This is warn");
  error!("This is error");
  // }
  match option_handle {
    Some(handle) => {
      info!("Joining handle..");
      handle.join();
    }
    None => {
      info!("No handle to join");
    }
  }
}
