#[macro_export]
macro_rules! debug {
    ($args:expr, $($arg:tt)*) => (
        if $args.comp_debug {
            println!("[debug] {}", format!($($arg)*));
        }
    );
}

#[macro_export]
macro_rules! perf {
    ($args:expr, $time:expr, $action:expr) => {
        if $args.comp_debug {
            println!(
                "[perf] {:width$} {:ms_width$} ({:.2?})",
                $action,
                $time.elapsed().as_micros(),
                $time.elapsed(),
                width = 40,
                ms_width = 6
            );
        }
    };
}
