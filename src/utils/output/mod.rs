mod capture;
mod clipboard;
mod display;
mod theme;

pub use self::capture::with_captured_output;
pub use self::clipboard::copy_to_clipboard;
#[cfg(target_os = "android")]
pub use self::clipboard::copy_to_clipboard_termux;
pub use self::display::{
    build_output_path, cat_file, cat_file_with_line_numbers, cat_file_with_syntax,
    raw_ansi_allowed, write_file,
};
pub use self::theme::{
    format_separator, print_error, print_info, print_separator, print_success, print_warning,
};
