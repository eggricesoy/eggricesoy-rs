pub extern crate clap;
pub extern crate log;

#[path = "lib/http.rs"]
mod http;
#[path = "lib/logger.rs"]
mod logger;

use clap::{
  builder::{Arg, Command},
  value_parser, ArgAction, ArgMatches,
};
use log::{debug, error};
use std::thread::JoinHandle;

#[macro_export]
macro_rules! app {
  // Crate information is provided in macro so that the calling package's
  // information is used to populate application information.
  () => {
    eggricesoy::generate_app(
      env!("CARGO_PKG_NAME"),
      env!("CARGO_PKG_DESCRIPTION"),
      env!("CARGO_PKG_VERSION"),
    )
  };
}

pub type AnyError = Box<dyn std::error::Error>;

pub struct EggApp {
  pub matches: ArgMatches,
  pub http_handle: Option<JoinHandle<()>>,
}

pub fn generate_app(
  name: &'static str,
  description: &'static str,
  version: &'static str,
) -> Command {
  let default_log_file: &'static str = Box::leak(format!("/tmp/log/{}.log", name).into_boxed_str());
  let default_log_json_file: &'static str =
    Box::leak(format!("/tmp/log/{}.0.jsonlog", name).into_boxed_str());
  Command::new(name)
    .author("eggricesoy <eggrice.soy>")
    .about(description)
    .version(version)
    .args(&[
      Arg::new("log4rs-config")
        .help("log4rs configuration file. If read successfully, this overrides all configurations")
        .long("log4rs-config")
        .short('c')
        .hide_short_help(true)
        .action(ArgAction::Set),

      Arg::new("no-stderr")
        .help("If set, do not print log to stderr")
        .long("no-stderr")
        .hide_short_help(true)
        .action(ArgAction::SetTrue)
        .short('n'),

      Arg::new("log-file")
        .help("Log file path")
        .long("log-file")
        .short('f')
        .default_value(default_log_file)
        .hide_short_help(true)
        .action(ArgAction::Set),

      Arg::new("log-json")
        .help("Log json file path. Must end with .0.jsonlog, '0' will be incremented with log file size increase.")
        .long("log-json")
        .short('j')
        .default_value(default_log_json_file)
        .hide_short_help(true)
        .action(ArgAction::Set),

      Arg::new("log-level")
        .help("Minimum log level for both stderr and file")
        .value_parser(["trace", "debug", "info", "warn", "error"])
        .long("log-level")
        .short('l')
        .action(ArgAction::Set)
        .hide_short_help(true)
        .default_value("debug"),

      Arg::new("log-level-stderr")
        .help("Minimum log level for stderr")
        .value_parser(["trace", "debug", "info", "warn", "error"])
        .long("log-level-stderr")
        .short('s')
        .action(ArgAction::Set)
        .default_value("debug"),

      Arg::new("log-level-file")
        .help("Minimum log level for file")
        .value_parser(["trace", "debug", "info", "warn", "error"])
        .long("log-level-file")
        .action(ArgAction::Set)
        .hide_short_help(true)
        .default_value("info"),

      Arg::new("log-level-json")
        .help("Minimum log level for json")
        .value_parser(["trace", "debug", "info", "warn", "error"])
        .long("log-level-json")
        .action(ArgAction::Set)
        .hide_short_help(true)
        .default_value("info"),

      Arg::new("log-file-size")
        .help("Maximum log file size in bytes, any invalid values will default to 1MB")
        .long("log-file-size")
        .action(ArgAction::Set)
        .hide_short_help(true)
        .value_parser(value_parser!(u64))
        .default_value("1000000"),

      Arg::new("log-json-count")
        .help("Maximum json log file count, any invalid values will default to 10")
        .long("log-file-count")
        .action(ArgAction::Set)
        .hide_short_help(true)
        .value_parser(value_parser!(u32))
        .default_value("10"),

      Arg::new("http-ip")
        .help("IP to bind to for http health and basic info display")
        .long("http-ip")
        .action(ArgAction::Set)
        .hide_short_help(true)
        .default_value("0.0.0.0"),

      Arg::new("http-port")
        .help("Port to bind to for http health and basic info display")
        .long("http-port")
        .action(ArgAction::Set)
        .hide_short_help(true)
        .default_value("3000"),

      Arg::new("http-pool-size")
        .help("Number of threads to handle status HTTP requests.")
        .long("http-pool-size")
        .action(ArgAction::Set)
        .hide_short_help(true)
        .value_parser(value_parser!(usize))
        .default_value("3"),
    ])
}

pub fn init_app(app: Command) -> EggApp {
  debug!("Initializing app");

  let matches: ArgMatches = app.get_matches();
  logger::init_logger(&matches);
  match http::create_http_server(&matches) {
    Ok(handle) => EggApp {
      matches,
      http_handle: Some(handle),
    },
    Err(e) => {
      error!("Failed to create status http server: {}", e);
      EggApp {
        matches,
        http_handle: None,
      }
    }
  }
}
