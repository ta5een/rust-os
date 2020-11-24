use crate::vga_buffer::*;

/// Print to stdout.
#[macro_export]
macro_rules! print {
    ($val:expr) => {
        $crate::vga_buffer::macros::_print(format_args!("{}", $val))
    };
    ($($arg:tt)*) => {
        $crate::vga_buffer::macros::_print(format_args!($($arg)*))
    };
}

/// Print to stdout, appending a new line.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($val:expr) => ($crate::print!("{}\n", $val));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Print to stderr.
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        $crate::vga_buffer::macros::_eprint(format_args!($($arg)*))
    };
}

/// Print to stderr, appending a new line.
#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

/// Prints the given formatted string to the VGA text buffer through the global
/// `WRITER` instance (representing stdout).
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    #[cfg(not(test))]
    // writer::writer().write_fmt(args).unwrap();
    writer::WRITER.lock().write_fmt(args).unwrap();
}

/// Prints the given formatted string to the VGA text buffer through the global
/// `ERR_WRITER` instance (representing stderr).
#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    use core::fmt::Write;
    // writer::ewriter().write_fmt(args).unwrap();
    writer::EWRITER.lock().write_fmt(args).unwrap();
}
