use std::path::PathBuf;

pub struct PythonTemplate;

impl super::Template for PythonTemplate {
    fn init_at_path(&self, _path: PathBuf, _day: u8, _year: u16) -> std::io::Result<()> {
        todo!()
    }
}
