# `slog-gelf` - A [`GELF`][gelf] integration for [`slog-rs`][slog-rs]
[![Build Status](https://travis-ci.org/lancastr/slog-gelf.svg?branch=master)](https://travis-ci.org/lancastr/slog-gelf)

### How to use

```rust
#[macro_use]
extern crate slog;
extern crate hostname;
extern crate slog_async;
extern crate slog_gelf;

use slog::Drain;

fn main() {
    let hostname = hostname::get_hostname().unwrap();

    let drain = slog_gelf::Gelf::new(&hostname, "192.168.0.1011:12201")
        .unwrap()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!("key" => "value"));

    info!(log,
        "An example log message";
        "k1" => "v1",
        "k2" => "v2",
    );
}

```
[gelf]: http://docs.graylog.org/en/3.0/pages/gelf.html
[slog-rs]: //github.com/slog-rs/slog
