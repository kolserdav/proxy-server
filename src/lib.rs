//! Low level proxy server.
//! To implement request proxying, only standard [`TcpStream`] was used without additional libraries
//! # Examples
//! With default params:
//! ```no_run
//! use proxy_server::Builder;                                                                       
//!      
//! fn main() {
//!     Builder::new().bind(None).expect("Error in proxy");
//! }
//! ```
//! With custom params:
//! ```no_run
//! use proxy_server::{log::LogLevel, Builder};
//!
//! fn main() {
//!     Builder::new()
//!         .with_address("127.0.0.1:3000")
//!         .with_target("127.0.0.1:3001")
//!         .with_log_level(LogLevel::Warn)
//!         .with_threads(4)
//!         .bind(None)
//!         .expect("Error in proxy");
//! }
//! ```
//! With check and change target if needed on every request
//! ```no_run
//!
//! use proxy_server::{Builder, ChangeTarget};                  
//! fn get_actual_target(old: &str) -> &'static str {     
//!     let target1 = "127.0.0.1:3001";                   
//!     let target2 = "127.0.0.1:3003";
//!     let res = match old {                                 
//!         "127.0.0.1:3001" => target2,
//!         "127.0.0.1:3003" => target1,
//!         _ => target1,
//!         };
//!         res
//! }
//!
//! fn main() {
//!     let cb: ChangeTarget = |old| get_actual_target(old);                                          
//!     Builder::new()                          
//!         .bind(Some(cb))
//!         .expect("Error in proxy");
//! }
//! ```

use std::{
    convert::Infallible,
    io::{Error, ErrorKind, Result, Write},
    net::{TcpListener, TcpStream},
    str,
    thread::sleep,
    time::Duration,
};
mod thread_pool;
use thread_pool::ThreadPool;
pub mod http;
use http::{Http, Status};
pub mod log;
use log::{Log, LogLevel, LOG_LEVEL};
mod prelude;
use prelude::constants::*;
use prelude::*;

#[cfg(test)]
mod tests;

/// Callback function for change target on fly.
/// Use only fast method because this function if it provieded then run every request again.
pub type ChangeTarget = fn(&str) -> &'static str;

/// Structure for proxy server configuration
#[derive(Clone, Copy, Debug)]
pub struct Builder {
    pub address: &'static str,
    pub target: &'static str,
    pub log_level: LogLevel,
    pub threads: usize,
}

impl Builder {
    /// Create new proxy server builder
    pub fn new() -> Self {
        Self {
            address: PROXY_ADDRESS,
            target: TARGET_ADDRESS,
            log_level: LOG_LEVEL,
            threads: THREADS,
        }
    }

    /// Set proxy server address
    pub fn with_address(mut self, address: &'static str) -> Self {
        self.address = address;
        self
    }

    /// Set proxy server target address
    pub fn with_target(mut self, target: &'static str) -> Self {
        self.target = target;
        self
    }

    /// Set log level of proxy server
    pub fn with_log_level(mut self, log_level: LogLevel) -> Self {
        self.log_level = log_level;
        self
    }

    /// Set proxy server count of used threads
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    /// Proxy server listener releasing [`std::net::TcpListener`] via thread pool
    pub fn bind(mut self, cb: Option<ChangeTarget>) -> Result<Infallible> {
        let listener = TcpListener::bind(&self.address)?;

        let _log = Log::new(&self.log_level);
        println!(
            "Listening: {}; Target: {}; Chunk size: {}KB; Log level: {:?}",
            &self.address, &self.target, CHUNK_SIZE, &self.log_level
        );

        let pool = ThreadPool::new(self.threads);
        for stream in listener.incoming() {
            if let Some(func) = cb {
                self.target = func(&self.target);
            }
            let cl = Handler::new(self);
            pool.execute(|| {
                cl.handle_proxy(stream.expect("Error in incoming stream"))
                    .expect("Failed handle proxy");
            });
        }
        Err(Error::new(ErrorKind::Interrupted, "main thread crashed"))
    }
}

struct Handler {
    config: Builder,
}

impl Handler {
    fn new(config: Builder) -> Self {
        Self { config }
    }

    fn handle_proxy(self, client: TcpStream) -> Result<()> {
        let _log = Log::new(&self.config.log_level);

        _log.println(LogLevel::Info, "handle proxy", &client);

        let mut client = Http::from(client);
        let heads = client.read_headers()?;
        let heads_n = change_header_host(stringify_headers(&heads).as_str(), &self.config.target);
        if let None = heads_n {
            _log.println(LogLevel::Warn, "Header host is missing", &heads);
            client.set_status(Status::BadRequest)?;
            client.set_content_length(0)?;
            client.set_end_line()?;
            return Ok(());
        }
        let heads_n = heads_n.unwrap();
        _log.println(
            LogLevel::Info,
            "Request headers:",
            parse_headers(heads_n.clone()),
        );

        let http = Http::connect(&self.config.target);
        if let Err(e) = &http {
            _log.println(LogLevel::Warn, "Failed proxy", e);
            client.set_status(Status::BadGateway)?;
            client.set_content_length(0)?;
            client.set_end_line()?;
            client.flush()?;
            sleep(Duration::from_millis(100));
            return Ok(());
        }
        let mut http = http?;

        http.write(heads_n.as_bytes())?;

        if let Some(v) = get_content_length(&heads_n) {
            if v != 0 {
                let body = client.read_body()?;
                _log.println(
                    LogLevel::Info,
                    "Request body: ",
                    str::from_utf8(&body).unwrap(),
                );
                http.write(&body)?;
                http.write(&[0u8])?;
            }
        }

        let h = http.read_headers()?;
        let h_s = stringify_headers(&h);
        let heads = parse_headers(h_s);
        _log.println(LogLevel::Info, "Send headers to client", heads);
        client.write(&h).expect("failed send headers");

        client.tunnel(&mut http, &_log)?;

        Ok(())
    }
}
