#[macro_export]
macro_rules! die {
    ($($arg:tt)*) => ({
        use std::io::Write;
        (writeln!(&mut ::std::io::stderr(), $($arg)*)).expect("stderr");
        ::std::process::exit(1)
    })
}
