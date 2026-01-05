#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log::logger::log_trace(format!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::logger::log_debug(format!($($arg)*));
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::logger::log_info(format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log::logger::log_warn(format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::logger::log_error(format!($($arg)*));
    };
}

#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {
        $crate::log::logger::log_critical(format!($($arg)*));
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        $crate::log::logger::log_success(format!($($arg)*));
    };
}