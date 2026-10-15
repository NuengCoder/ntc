mod edit;
mod hyper;
mod rich;
mod simple;
mod welcome;

pub use edit::{edit_file, edit_file_with_session};
pub use hyper::init_hyper_project;
pub use rich::{generate_super_template, init_super_file};
pub use simple::{generate_template, init_file};
pub use welcome::{
    generate_welcome_content, is_stale_welcome, refresh_welcome_file, WELCOME_VERSION,
};
