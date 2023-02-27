#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

/// Log level [`Info`, `Warn`, `Error`]
pub const LOG_LEVEL: LogLevel = LogLevel::Info;

impl LogLevel {
    fn as_num(&self) -> u8 {
        use LogLevel::*;
        match &self {
            Info => 0,
            Warn => 1,
            Error => 2,
        }
    }
}

#[derive(Clone)]
pub struct Log<'a> {
    level: &'a LogLevel,
}

impl<'a> Log<'a> {
    pub fn new(level: &LogLevel) -> Log {
        Log { level }
    }

    pub fn println<T, K>(&self, level: LogLevel, msg: K, arg: T)
    where
        T: std::fmt::Debug,
        K: std::fmt::Display,
    {
        if self.level.as_num() <= level.as_num() {
            println!("[ {:?} ] {}: {:?}", level, msg, arg);
        }
    }
}
