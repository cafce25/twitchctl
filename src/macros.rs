#[macro_export]
macro_rules! exit {
    ($ec:expr, $($message:expr), +) => {
        {eprintln!($($message), +);
        std::process::exit($ec);}
    };
}

#[macro_export]
macro_rules! matches_any {
    ($value:expr, $first:expr) => {
        $value.eq_ignore_ascii_case($first)
    };
    ($value:expr, $first:expr, $($pattern:expr), +) => {
        $value.eq_ignore_ascii_case($first) || matches_any!($value, $($pattern), +)
    };
}

#[macro_export]
macro_rules! warning {
    ($($message:expr), +) => {
        eprintln!("[WARNING] {}", format!($($message), +));
    };
}
