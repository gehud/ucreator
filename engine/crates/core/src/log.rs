#[repr(usize)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error
}

pub const PREFIXES: &'static [&'static str] = &[
    "TRACE",
    "DEBUG",
    "INFO",
    "WARN",
    "ERROR"
];

pub const COLORS: &'static [u8] = &[
    37,
    34,
    32,
    33,
    31
];

#[macro_export]
macro_rules! __log__log__ {
    ($lvl:expr, $($arg:expr),+) => {
        let idx = $lvl as $crate::log::Level as usize;
        print!("\x1b[{}m[{}]: ", $crate::log::COLORS[idx], $crate::log::PREFIXES[idx]);
        print!($($arg),+);
        println!("\x1b[0m");
    };
}

#[macro_export]
macro_rules! __log__trace__ {
    ($($arg:expr),+) => {
        $crate::log::log!($crate::log::Level::Trace, $($arg),+);
    };
}

#[macro_export]
macro_rules! __log__debug__ {
    ($($arg:expr),+) => {
        $crate::log::log!($crate::log::Level::Debug, $($arg),+);
    };
}

#[macro_export]
macro_rules! __log__info__ {
    ($($arg:expr),+) => {
        $crate::log::log!($crate::log::Level::Info, $($arg),+);
    };
}

#[macro_export]
macro_rules! __log__warn__ {
    ($($arg:expr),+) => {
        $crate::log::log!($crate::log::Level::Warn, $($arg),+);
    };
}

#[macro_export]
macro_rules! __log__error__ {
    ($($arg:expr),+) => {
        $crate::log::log!($crate::log::Level::Error, $($arg),+);
    };
}

pub use crate::__log__log__ as log;
pub use crate::__log__trace__ as trace;
pub use crate::__log__debug__ as debug;
pub use crate::__log__info__ as info;
pub use crate::__log__warn__ as warn;
pub use crate::__log__error__ as error;
