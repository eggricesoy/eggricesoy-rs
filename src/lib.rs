pub extern crate clap;
pub extern crate log;

#[path = "lib/http.rs"]
mod http;
#[path = "lib/logger.rs"]
mod logger;

use clap::{App, AppSettings, Arg, ArgMatches};
use log::{debug, error};
use std::thread::JoinHandle;

#[macro_export(app)]
macro_rules! app {
  // Crate information is provided in macro so that the calling package's
  // information is used to populate application information.
  () => {
    eggricesoy::generate_app(
      eggricesoy::clap::crate_name!(),
      eggricesoy::clap::crate_description!(),
      eggricesoy::clap::crate_version!(),
    )
  };
}

pub fn generate_app<'a, 'b>(name: &'b str, description: &'b str, version: &'b str) -> App<'a, 'b> {
  App::new(name)
    .author("eggricesoy <eggrice.soy>")
    .about(description)
    .version(version)
    .arg(
      Arg::with_name("log4rs-config")
        .help("log4rs configuration file. If read successfully, this overrides all configurations")
        .long("log4rs-config")
        .short("c")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("no-stderr")
        .help("If set, do not print log to stderr")
        .long("no-stderr")
        .short("n"),
    )
    .arg(
      Arg::with_name("log-file")
        .help("Log file path")
        .long("log-file")
        .short("f")
        .default_value(Box::leak(Box::new(format!("/tmp/log/{}.log", name))))
        .takes_value(true),
    )
    .arg(
      Arg::with_name("log-json")
        .help("Log json file path. Must end with .0.jsonlog, '0' will be incremented with log file size increase.")
        .long("log-json")
        .short("j")
        .default_value(Box::leak(Box::new(format!("/tmp/log/{}.0.jsonlog", name))))
        .takes_value(true),
    )
    .arg(
      Arg::with_name("log-level")
        .help("Minimum log level for both stderr and file")
        .possible_values(&["trace", "debug", "info", "warn", "error"])
        .long("log-level")
        .short("l")
        .takes_value(true)
        .required(true)
        .default_value("debug"),
    )
    .arg(
      Arg::with_name("log-level-stderr")
        .help("Minimum log level for stderr")
        .possible_values(&["trace", "debug", "info", "warn", "error"])
        .long("log-level-stderr")
        .takes_value(true)
        .required(true)
        .default_value("debug"),
    )
    .arg(
      Arg::with_name("log-level-file")
        .help("Minimum log level for file")
        .possible_values(&["trace", "debug", "info", "warn", "error"])
        .long("log-level-file")
        .takes_value(true)
        .required(true)
        .default_value("info"),
    )
    .arg(
      Arg::with_name("log-level-json")
        .help("Minimum log level for json")
        .possible_values(&["trace", "debug", "info", "warn", "error"])
        .long("log-level-json")
        .takes_value(true)
        .required(true)
        .default_value("info"),
    )
    .arg(
      Arg::with_name("log-file-size")
        .help("Maximum log file size in bytes, any invalid values will default to 1MB")
        .long("log-file-size")
        .takes_value(true)
        .required(true)
        .default_value("1000000"),
    )
    .arg(
      Arg::with_name("log-json-count")
        .help("Maximum json log file count, any invalid values will default to 10")
        .long("log-file-count")
        .takes_value(true)
        .required(true)
        .default_value("10"),
    )
    .arg(
      Arg::with_name("http-ip")
        .help("IP to bind to for http health and basic info display")
        .long("http-ip")
        .takes_value(true)
        .required(true)
        .default_value("0.0.0.0"),
    )
    .arg(
      Arg::with_name("http-port")
        .help("Port to bind to for http health and basic info display")
        .long("http-port")
        .takes_value(true)
        .required(true)
        .default_value("3000"),
    )
    .arg(
      Arg::with_name("http-pool-size")
        .help("Number of threads to handle status HTTP requests.")
        .long("http-pool-size")
        .takes_value(true)
        .required(true)
        .default_value("3"),
    )
    .setting(AppSettings::StrictUtf8)
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::VersionlessSubcommands)
}

pub fn init_app<'a, 'b>(app: App<'a, 'b>) -> (ArgMatches<'a>, Option<JoinHandle<()>>) {
  debug!("Initializing app");

  let matches: ArgMatches<'a> = app.get_matches();
  logger::init_logger(&matches);
  match http::create_http_server(&matches) {
    Ok(handle) => return (matches, Some(handle)),
    Err(e) => {
      error!("{}", e);
      return (matches, None);
    }
  }
}
