// This library intentionally does not use tokio or hyper
use clap::ArgMatches;
use log::{debug, error, info};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use threadpool::Builder;

fn handle_client(mut stream: TcpStream) {
  let mut buf = [0u8; 4096];
  match stream.read(&mut buf) {
    Ok(_) => {
      // Not using this right now
      // let req_str = String::from_utf8_lossy(&buf);
      // debug!("{}", req_str);
      let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>ok</body></html>\r\n";
      match stream.write(response) {
        Ok(_) => debug!("Response sent"),
        Err(e) => error!("Failed sending response: {}", e),
      }
    }
    Err(e) => debug!("Unable to read stream: {}", e),
  }
}

pub fn create_http_server(matches: &ArgMatches) -> Result<thread::JoinHandle<()>, std::io::Error> {
  let bind_ip = matches.value_of("http-ip").unwrap_or("0.0.0.0");
  let bind_port = matches.value_of("http-port").unwrap_or("3000");
  let pool_size: usize = matches
    .value_of("http-pool-size")
    .unwrap_or("3")
    .parse()
    .unwrap_or(3);

  let bind_addr = format!("{}:{}", bind_ip, bind_port);
  debug!("Creating TCP listener at {}", bind_addr);

  let listener = TcpListener::bind(bind_addr)?;
  info!("HTTP listener initialized");
  let builder = thread::Builder::new().name("status-http".to_string());

  let lambda_handler = move || {
    let pool = Builder::new()
      .num_threads(pool_size)
      .thread_name("status-http.handler".to_string())
      .build();

    for stream in listener.incoming() {
      match stream {
        Ok(stream) => {
          // This is intentional, we don't want to accidentally kill the job
          // when /health is spammed with requests. So we should have a fixed
          // pool size
          pool.execute(move || {
            handle_client(stream);
          })
        }
        Err(e) => {
          error!("Unable to connect: {}", e);
        }
      }
    }
  };

  builder.spawn(lambda_handler)
}
