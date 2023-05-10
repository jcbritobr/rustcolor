/*!
# macros
This module implements macros to automate the printer module
and remove the boilerplate when coding.
*/

/// Prints in terminal an output with fg green and bg black.
#[macro_export]
macro_rules! info {
    ($text:expr) => {
        println!("{}", $text.info());
    };
}

/// Prints in terminal an output with fg blue and bg black.
#[macro_export]
macro_rules! primary {
    ($text:expr) => {
        println!("{}", $text.primary());
    };
}

/// Prints in terminal an output with fg yellow and bg black.
#[macro_export]
macro_rules! warn {
    ($text:expr) => {
        println!("{}", $text.warn());
    };
}

/// Prints in terminal an output with fg red and bg black.
#[macro_export]
macro_rules! danger {
    ($text:expr) => {
        println!("{}", $text.danger());
    };
}

/// Prints in terminal an output with fg white and bg red.
#[macro_export]
macro_rules! error {
    ($text:expr) => {
        println!("{}", $text.error());
    };
}

/// Prints in terminal an output with fg red, bg black and blink tag.
#[macro_export]
macro_rules! blink {
    ($text:expr) => {
        println!("{}", $text.blink());
    };
}

/// Prints in terminal an output with fg yellow, bg black and underline tag.
#[macro_export]
macro_rules! underline {
    ($text:expr) => {
        println!("{}", $text.underline());
    };
}
