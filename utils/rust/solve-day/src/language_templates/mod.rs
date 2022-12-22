pub use python_template::PythonTemplate;
pub use rust_template::RustTemplate;
use std::path::PathBuf;

pub trait Template {
    /// Create a new solution template at the given path
    fn init_at_path(&self, path: PathBuf, day: u8, year: u16) -> std::io::Result<()>;
}

mod python_template;
mod rust_template;
