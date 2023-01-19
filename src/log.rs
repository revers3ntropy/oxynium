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
                "[perf] {:width$} in {:.2?}",
                $action,
                $time.elapsed(),
                width = 20
            );
        }
    };
}
