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

    fn get_writer(&self, append: bool) -> Result<File> {

        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(append)
            .open(self.filename())

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

#[cfg(test)]
mod test {

    use std::io::{BufRead, Write};
    use crate::FileManager;

    struct TestFile(String);
    impl FileManager for TestFile {

        type Parsed = ();
        type Queried = ();

        fn new(filename: &str) -> Self {
            TestFile(filename.to_string())
        }

        fn filename(&self) -> &str {
            &self.0
        }

        fn parse(&mut self) -> Self::Parsed {
            ()
        }

        fn query(&self, _: &str) -> Self::Queried {
            ()
        }

    }

    fn get_test_file() -> TestFile {
        TestFile::new("../../testfiles/file.txt")
    }

    fn get_lines() -> usize {
        let reader = get_test_file().get_reader();
        reader.unwrap().lines().map(|line| line.unwrap()).collect::<Vec<String>>().len()
    }

    #[test]
    fn get_reader() {
        let reader = get_test_file().get_reader();
        assert!(reader.is_ok());
    }

    #[test]
    fn get_writer() {
        let writer = get_test_file().get_writer(false);
        assert!(writer.is_ok());
    }

    #[test]
    fn write() {

        let mut lines = get_lines();

        if lines < 1 {
            lines = 1;
        }

        let writer = get_test_file().get_writer(false);
        assert!(writer.unwrap().write("lorem ipsum\n".as_bytes()).is_ok());

        assert_eq!(lines, get_lines());

    }

    #[test]
    fn append() {

        let lines = get_lines();

        let writer = get_test_file().get_writer(true);
        assert!(writer.unwrap().write("lorem ipsum\n".as_bytes()).is_ok());

        assert_eq!(lines, get_lines() - 1);

    }

    #[test]
    fn get_contents() {
        let file = get_test_file();
        assert!(file.get().is_ok());
    }

}