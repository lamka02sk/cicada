mod toml_file;
mod text_file;
mod json_file;

use std::io::{BufReader, Read, Result};
use std::fs::{File, OpenOptions};

pub use toml_file::TomlFile;
pub use text_file::TextFile;
pub use json_file::JsonFile;

pub trait FileManager {

    type Parsed;
    type Queried;

    fn new(filename: &str) -> Self;

    fn get_reader(&self) -> Result<BufReader<File>> {

        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(self.filename())?;

        Ok(BufReader::new(file))

    }

    fn get(&self) -> Result<String> {

        let mut content = String::new();
        let mut reader = self.get_reader()?;
        reader.read_to_string(&mut content)?;

        Ok(content)

    }

    fn filename(&self) -> &str;
    fn parse(&mut self) -> Self::Parsed;

    fn query(&self, path: &str) -> Self::Queried;

}