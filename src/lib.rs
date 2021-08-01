use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches};
use log::{debug, warn, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config};
use simple_logger::SimpleLogger;

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

pub fn init_logger(matches: &ArgMatches) {
  let mut use_simple_logger = !matches.is_present("log4rs-config");

  let stdout = ConsoleAppender::builder().build();
  let config = Config::builder().appender(Appender::builder().build("stdout", Box::new(stdout)));

  // Attempt to initialize log4rs
  if !use_simple_logger {
    let log4rs_config = matches
      .value_of("log4rs-config")
      .expect("Failed to get log4rs-config.");
    debug!("Initializing log4rs config at {}", log4rs_config);
    match log4rs::init_file(log4rs_config, Default::default()) {
      Ok(_) => {
        debug!("log4rs initialized.");
      }
      Err(e) => {
        use_simple_logger = true;
        SimpleLogger::new().init().unwrap();
        warn!("Failed to initialize log4rs: {}", e);
      }
    }
  } else {
    SimpleLogger::new().init().unwrap();
  }

  if use_simple_logger {
    log::set_max_level(LevelFilter::Debug);
    warn!("Using SimpleLogger, this only outputs to stdout/stderr.");
  }
}

pub fn init_app<'a, 'b>(app: App<'a, 'b>) -> ArgMatches<'a> {
  let matches: ArgMatches<'a> = app.get_matches();
  init_logger(&matches);
  matches
}
