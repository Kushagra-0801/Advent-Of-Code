use std::path::PathBuf;

pub struct PythonTemplate;

impl super::Template for PythonTemplate {
    fn init_at_path(&self, path: PathBuf, day: u8, year: u16) -> std::io::Result<()> {
        todo!()
    }
}
