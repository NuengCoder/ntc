mod simple;
mod rich;
mod welcome;
mod edit;
mod hyper;

pub use edit::{edit_file, edit_file_with_session};
pub use simple::{generate_template, init_file};
pub use rich::{generate_super_template, init_super_file};
pub use welcome::generate_welcome_content;
pub use hyper::init_hyper_project;
