// utils/src/lib.rs

#[cfg(feature = "dev_print")]
#[macro_export]
macro_rules! dev_print {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

#[cfg(not(feature = "dev_print"))]
#[macro_export]
macro_rules! dev_print {
    ($($arg:tt)*) => {
        // No-op if dev_print feature is not enabled
    };
}
