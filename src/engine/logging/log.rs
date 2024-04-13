static mut ACTIVE_LOG_LEVEL: LogLevel = LogLevel::DEBUG;

/// Sets the minimum log level to be outputted.
pub fn set_active_log_level(level: LogLevel) {
    unsafe { ACTIVE_LOG_LEVEL = level };
}

pub fn log<S: AsRef<str>>(level: LogLevel, message: S) {
    match (level, unsafe { ACTIVE_LOG_LEVEL }) {
        (LogLevel::DEBUG, LogLevel::DEBUG) => {}
        (LogLevel::DEBUG, LogLevel::WARNING) => return,
        (LogLevel::DEBUG, LogLevel::ERROR) => return,
        (LogLevel::WARNING, LogLevel::DEBUG) => {}
        (LogLevel::WARNING, LogLevel::WARNING) => {}
        (LogLevel::WARNING, LogLevel::ERROR) => return,
        (LogLevel::ERROR, LogLevel::DEBUG) => {}
        (LogLevel::ERROR, LogLevel::WARNING) => {}
        (LogLevel::ERROR, LogLevel::ERROR) => {}
        (_, LogLevel::NONE) => return,
        (LogLevel::NONE, _) => return,
    };

    let level_str = get_log_level_str(level);
    println!("[{}] {}", level_str, message.as_ref());
}

pub fn debug<S: AsRef<str>>(message: S) {
    log(LogLevel::DEBUG, message);
}

pub fn warning<S: AsRef<str>>(message: S) {
    log(LogLevel::WARNING, message);
}

pub fn error<S: AsRef<str>>(message: S) {
    log(LogLevel::ERROR, message);
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum LogLevel {
    DEBUG,
    WARNING,
    ERROR,
    NONE,
}

const fn get_log_level_str(level: LogLevel) -> &'static str {
    match level {
        LogLevel::DEBUG => "DEBUG",
        LogLevel::WARNING => "WARNING",
        LogLevel::ERROR => "ERROR",
        LogLevel::NONE => "NONE",
    }
}
