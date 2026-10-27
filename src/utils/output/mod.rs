mod capture;
mod clipboard;
mod display;
mod theme;

pub use self::capture::{with_captured_output, with_captured_output_bytes};
pub use self::clipboard::copy_to_clipboard;
#[cfg(target_os = "android")]
pub use self::clipboard::copy_to_clipboard_termux;
pub use self::clipboard::normalize_newlines;
#[cfg(not(target_os = "android"))]
pub use self::clipboard::set_clipboard_text;
pub use self::display::{
    build_output_path, cat_file, cat_file_range, cat_file_with_line_numbers, cat_file_with_syntax,
    cat_file_with_syntax_range, parse_betw, parse_file_range, print_syntax_text, raw_ansi_allowed,
    read_file_as_string, slice_numbered_lines, write_file, FileLineRange,
};
pub use self::theme::{
    format_separator, print_error, print_info, print_separator, print_success, print_warning,
};
