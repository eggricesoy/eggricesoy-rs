pub extern crate clap;
pub extern crate log;

use clap::{App, AppSettings, Arg, ArgMatches};
use log::{debug, LevelFilter};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::rolling_file::policy::compound::roll::delete::DeleteRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;

#[macro_export(app)]
macro_rules! app {
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
      Arg::with_name("log-file-size")
        .help("Maximum log file size in bytes, any invalid values will default to 1MB")
        .long("log-file-size")
        .short("s")
        .takes_value(true)
        .required(true)
        .default_value("1000000"),
    )
    .setting(AppSettings::StrictUtf8)
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::VersionlessSubcommands)
}

fn str_to_levelfilter(string: &str) -> Result<LevelFilter, String> {
  match string {
    "trace" => Ok(LevelFilter::Trace),
    "debug" => Ok(LevelFilter::Debug),
    "info" => Ok(LevelFilter::Info),
    "warn" => Ok(LevelFilter::Warn),
    "error" => Ok(LevelFilter::Error),
    _ => Err(format!("Unsupported level filter: {}", string)),
  }
}

fn default_logger_config(matches: &ArgMatches) -> (Config, String) {
  let mut config_builder = Config::builder();
  let mut root_builder = Root::builder();
  let encoder_string = "{d} {h({l})} [{T}/{t}] {m}{n}";
  let mut error = String::new();

  if !matches.is_present("no-stderr") {
    let stderr = ConsoleAppender::builder()
      .target(Target::Stderr)
      .encoder(Box::new(PatternEncoder::new(encoder_string)))
      .build();
    config_builder = config_builder.appender(
      Appender::builder()
        .filter(Box::new(ThresholdFilter::new(
          str_to_levelfilter(matches.value_of("log-level-stderr").unwrap()).unwrap(),
        )))
        .build("stderr", Box::new(stderr)),
    );
    root_builder = root_builder.appender("stderr");
  }

  match matches.value_of("log-file") {
    Some(path) => {
      let file_size: u64 = matches
        .value_of("log-file-size")
        .unwrap()
        .parse()
        .unwrap_or(1000000);
      let policy = CompoundPolicy::new(
        Box::new(SizeTrigger::new(file_size)),
        Box::new(DeleteRoller::new()),
      );

      match RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(encoder_string)))
        .build(path, Box::new(policy))
      {
        Ok(file) => {
          config_builder = config_builder.appender(
            Appender::builder()
              .filter(Box::new(ThresholdFilter::new(
                str_to_levelfilter(matches.value_of("log-level-file").unwrap()).unwrap(),
              )))
              .build("file", Box::new(file)),
          );
          root_builder = root_builder.appender("file");
        }
        Err(e) => error = format!("Failed to create logger for {}", e),
      }
    }

    None => error = String::from("Not logging to file!"),
  }

  (
    config_builder
      .build(root_builder.build(
        str_to_levelfilter(matches.value_of("log-level").unwrap()).unwrap_or(LevelFilter::Debug),
      ))
      .unwrap(),
    error,
  )
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
    let config_and_err = default_logger_config(matches);
    log4rs_config = Some(config_and_err.0);
    parse_msg.push(config_and_err.1);
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
