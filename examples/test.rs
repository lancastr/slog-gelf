#[macro_use] extern crate slog;
extern crate slog_async;
extern crate slog_gelf;
extern crate hostname;

use slog::Drain;

fn main() {
    let hostname = hostname::get_hostname().unwrap_or("unhostnamed".to_string());

    let drain = slog_gelf::Gelf::new(&hostname, "192.168.0.101:12201").unwrap().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!("holy" => "shit"));

    info!(log,
        "An example log message: {}{:X}", "0xDEAD", 0xBEEF;
        "45342" => "34523",
        "12323" => "53245",
        "65462" => "84534",
        "92423" => "23424",
    );
}