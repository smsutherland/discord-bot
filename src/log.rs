use std::{sync::atomic::{AtomicU8, Ordering}, fmt::Display};

macro_rules! make_log {
    ($name:ident, $name_file:ident, $name_mod:ident, $level:ident) => {
        make_log!($name, $name_file, $name_mod, $level, $);
    };

    ($name:ident, $name_file:ident, $name_mod:ident, $level:ident, $dol:tt) => {
        #[macro_export]
        macro_rules! $name {
            ($dol($arg:tt)*) => {
                $crate::log::log($crate::log::LogLevel::$level, &format!("{}", format!($dol($arg)*)))
            }
        }
        #[macro_export]
        macro_rules! $name_file {
            ($dol($arg:tt)*) => {
                $crate::log::log($crate::log::LogLevel::$level, &format!("[{}:{}]: {}", file!(), line!(), format!($dol($arg)*)))
            }
        }
        #[macro_export]
        macro_rules! $name_mod {
            ($dol($arg:tt)*) => {
                $crate::log::log($crate::log::LogLevel::$level, &format!("[{}]: {}", module_path!(), format!($dol($arg)*)))
            }
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum LogLevel {
    Emerg = 0,
    Alert,
    Crit,
    Err,
    Warning,
    Notice,
    Info,
    Debug,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let log_string = format!("{:?}", self).to_uppercase();
        write!(f, "{}", log_string)
    }
}

#[cfg(debug_assertions)]
static LOG_LEVEL: AtomicU8 = AtomicU8::new(LogLevel::Debug as u8);
#[cfg(not(debug_assertions))]
static LOG_LEVEL: AtomicU8 = AtomicU8::new(ERR);

make_log!(emergency, emergency_file, emergency_mod, Emerg);
make_log!(alert, alert_file, alert_mod, Alert);
make_log!(crit, crit_file, crit_mod, Crit);
make_log!(error, error_file, error_mod, Err);
make_log!(warning, warning_file, warning_mod, Warning);
make_log!(notice, notice_file, notice_mod, Notice);
make_log!(info, info_file, info_mod, Info);
make_log!(debug, debug_file, debug_mod, Debug);

#[macro_export]
macro_rules! log {
    ($level:ident, $($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::$level, &format!("{}", format!($($arg)*)))
    }
}
#[macro_export]
macro_rules! log_file {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::$level, &format!("[{}:{}]: {}", file!(), line!(), format!($($arg)*)))
    }
}
#[macro_export]
macro_rules! log_mod {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::$level, &format!("[{}]: {}", module_path!(), format!($($arg)*)))
    }
}

pub fn set_log_level(new_log_level: LogLevel) {
    LOG_LEVEL.store(new_log_level as u8, Ordering::Relaxed);
}

pub fn get_log_level() -> LogLevel {
    unsafe { std::mem::transmute(LOG_LEVEL.load(Ordering::Relaxed)) }
}

pub fn log(level: LogLevel, msg: &str) {
    if level <= get_log_level() {
        println!("{:<7} | {}", format!("{}", level), msg);
    }
}

#[cfg(test)]
mod log_testing {
    use super::*;
    #[test]
    fn a() {
        debug!("a{}", 12);
        debug_file!("a{}", 12);
        debug_mod!("a{}", 12);
    }

    #[test]
    fn b() {
        println!("{:?}", get_log_level());
        set_log_level(LogLevel::Warning);
        println!("{:?}", get_log_level());

        log!(Err, "{}", 12);
        log!(Warning, "{}", 12);
    }
}
