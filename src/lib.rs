pub extern crate clap;
pub extern crate log;

#[path = "lib/logger.rs"]
mod logger;

use clap::{App, AppSettings, Arg, ArgMatches};
use log::debug;
use log4rs::config::Config;

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
    .setting(AppSettings::StrictUtf8)
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::VersionlessSubcommands)
}

pub fn init_logger(matches: &ArgMatches) {
  let mut parse_msg: Vec<String> = Vec::new();
  let mut log4rs_config: Option<Config> = None;

  match matches.value_of("log4rs-config") {
    Some(config_path) => match log4rs::config::load_config_file(config_path, Default::default()) {
      Ok(config) => {
        log4rs_config = Some(config);
        parse_msg.push(format!("Initialized log4rs config {}", config_path));
      }
      Err(e) => {
        parse_msg.push(format!(
          "Failed to initialize log4rs config {}, {}",
          config_path, e
        ));
      }
    },
    None => {
      parse_msg.push(String::from("log4rs config not provided"));
    }
  }

  if log4rs_config.is_none() {
    let mut config_and_err = logger::default_logger_config(matches);
    log4rs_config = Some(config_and_err.0);
    parse_msg.append(&mut config_and_err.1);
  }

  log4rs::init_config(log4rs_config.unwrap()).unwrap();
  for msg in parse_msg {
    if msg.len() > 0 {
      debug!("{}", msg);
    }
  }
}

pub fn init_app<'a, 'b>(app: App<'a, 'b>) -> ArgMatches<'a> {
  let matches: ArgMatches<'a> = app.get_matches();
  init_logger(&matches);
  matches
}
