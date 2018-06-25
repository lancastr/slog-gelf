#[macro_use] extern crate slog;
extern crate slog_async;
extern crate slog_gelf;

use slog::Drain;

fn main() {
    let drain = slog_gelf::Gelf::new().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());

    info!(log,
        "An example log message: {} {}", "argument", 666;
        "45342" => "34523",
        "12323" => "53245",
        "65462" => "84534",
        "92423" => "23424",
    );
}