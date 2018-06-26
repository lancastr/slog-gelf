use slog;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Level {
    Emergency       = 0,
    Alert           = 1,
    Critical        = 2,
    Error           = 3,
    Warning         = 4,
    Notice          = 5,
    Informational   = 6,
    Debug           = 7,
}

impl From<slog::Level> for Level {
    fn from(level: slog::Level) -> Level {
        match level {
            slog::Level::Critical => Level::Critical,
            slog::Level::Error => Level::Error,
            slog::Level::Warning => Level::Warning,
            slog::Level::Info => Level::Informational,
            slog::Level::Debug => Level::Debug,
            slog::Level::Trace => Level::Debug,
        }
    }
}