# eggricesoy-rs

Helper library for eggricesoy Rust binaries. This utilizes `clap`, `log` and
`log4rs`.

Feature list:

- [x] Generate `clap::App` with appropriate configuration.
- [x] Setup logger in production-ready style.
- [x] Turn up HTTP server for health checking.
- [] Display binary information (flags, etc) in HTTP server.
- [] Explicitly route `/health` for health check.

Usage:

`Cargo.toml`

```toml
[dependencies]
# Ensure ssh-agent is running
# ssh-add your key
eggricesoy = { git = "ssh://git@github.com/eggricesoy/eggricesoy-rs.git", tag = "1.2.0" }
```

`main.rs`

```rs
use eggricesoy;
use eggricesoy::clap::Arg;
use eggricesoy::log::{debug, info, warn};

fn main() {
  let app = eggricesoy::app!().arg(Arg::with_name("name").long("name").takes_value(true));
  let matches = eggricesoy::init_app(app).matches;
  debug!("This is a demo application!");
}
```

See https://github.com/eggricesoy/rust-example for an example.
