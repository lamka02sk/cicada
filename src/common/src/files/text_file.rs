use crate::files::FileManager;
use std::io::Result;

pub struct TextFile(String);

impl FileManager for TextFile {

    type Parsed = Result<String>;
    type Queried = ();

    fn new(filename: &str) -> Self {
        TextFile(filename.to_string())
    }

    fn filename(&self) -> &str {
        &self.0
    }

    fn parse(&mut self) -> Self::Parsed {
        self.get()
    }

    fn query(&self, _: &str) -> Self::Queried {
        ()
    }

}