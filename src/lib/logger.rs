use clap::ArgMatches;
use log::{debug, LevelFilter};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::rolling_file::policy::compound::roll::delete::DeleteRoller;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::json::JsonEncoder;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;

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

pub fn default_logger_config(matches: &ArgMatches) -> (Config, Vec<String>) {
  let mut config_builder = Config::builder();
  let mut root_builder = Root::builder();
  let encoder_string = "{d} {h({l})} [{T}/{t}] {m}{n}";
  let mut msgs: Vec<String> = Vec::new();

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

  match matches.value_of("log-json") {
    Some(first_file) => {
      let pattern = first_file.replace(".0.jsonlog", ".{}.jsonlog");
      let file_size: u64 = matches
        .value_of("log-file-size")
        .unwrap()
        .parse()
        .unwrap_or(1000000);
      let file_count: u32 = matches
        .value_of("log-json-count")
        .unwrap()
        .parse()
        .unwrap_or(10);
      let policy = CompoundPolicy::new(
        Box::new(SizeTrigger::new(file_size)),
        Box::new(
          FixedWindowRoller::builder()
            .build(&pattern, file_count)
            .unwrap(),
        ),
      );

      match RollingFileAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build(first_file, Box::new(policy))
      {
        Ok(file) => {
          config_builder = config_builder.appender(
            Appender::builder()
              .filter(Box::new(ThresholdFilter::new(
                str_to_levelfilter(matches.value_of("log-level-json").unwrap()).unwrap(),
              )))
              .build("json", Box::new(file)),
          );
          root_builder = root_builder.appender("json");
        }
        Err(e) => msgs.push(format!("Failed to create logger for {}", e)),
      }
    }
    None => msgs.push(String::from("Not logging json file!")),
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
        Err(e) => msgs.push(format!("Failed to create logger for {}", e)),
      }
    }

    None => msgs.push(String::from("Not logging to file!")),
  }

  (
    config_builder
      .build(root_builder.build(
        str_to_levelfilter(matches.value_of("log-level").unwrap()).unwrap_or(LevelFilter::Debug),
      ))
      .unwrap(),
    msgs,
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
    let mut config_and_err = default_logger_config(matches);
    log4rs_config = Some(config_and_err.0);
    parse_msg.append(&mut config_and_err.1);
  }

  log4rs::init_config(log4rs_config.unwrap()).unwrap();
  for msg in parse_msg {
    if !msg.is_empty() {
      debug!("{}", msg);
    }
  }
}
