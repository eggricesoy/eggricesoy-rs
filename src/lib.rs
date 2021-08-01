use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches};
use log::{debug, warn, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Deserializers, Root};

pub fn app<'a, 'b>() -> App<'a, 'b> {
  App::new(crate_name!())
    .author("eggricesoy <eggrice.soy>")
    .about(crate_description!())
    .version(crate_version!())
    .arg(
      Arg::with_name("log4rs-config")
        .takes_value(true)
        .long("log4rs-config")
        .help("log4rs configuration file")
        .short("l"),
    )
    .setting(AppSettings::StrictUtf8)
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::VersionlessSubcommands)
}

fn default_logger_config() -> Config {
  let stdout = ConsoleAppender::builder().build();

  Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
    .unwrap()
}

pub fn init_logger(matches: &ArgMatches) {
  let parse_msg;
  let log4rs_config;
  let deserializer = Deserializers::new();

  match matches.value_of("log4rs-config") {
    Some(config_path) => match log4rs::config::load_config_file(config_path, deserializer) {
      Ok(config) => {
        log4rs_config = config;
        parse_msg = format!("Initialized log4rs config {}", config_path);
      }
      Err(e) => {
        log4rs_config = default_logger_config();
        parse_msg = format!(
          "Failed to initialize log4rs config {}, printing to stdout: {}",
          config_path, e
        );
      }
    },
    None => {
      log4rs_config = default_logger_config();
      parse_msg = String::from("--log4rs-config not provided.")
    }
  }

  let encoder = log4rs::encode::pattern::PatternEncoder::new("[] {d} {l} {t} - {m}{n}");
  log4rs_config.borrow_mut();
  log4rs::init_config(log4rs_config).unwrap();
  debug!("{}", parse_msg);
}

pub fn init_app<'a, 'b>(app: App<'a, 'b>) -> ArgMatches<'a> {
  let matches: ArgMatches<'a> = app.get_matches();
  init_logger(&matches);
  matches
}
