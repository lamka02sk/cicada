use std::error::Error;
use serde_json::Value;
use crate::files::FileManager;

pub struct JsonFile {
    pub filename: String,
    content: Option<Value>
}

impl FileManager for JsonFile {

    type Parsed = Result<Value, Box<dyn Error>>;
    type Queried = Option<Value>;

    fn new(filename: &str) -> Self {
        JsonFile {
            filename: filename.to_string(),
            content: None
        }
    }

    fn filename(&self) -> &str {
        &self.filename
    }

    fn parse(&mut self) -> Self::Parsed {

        let file_content = serde_json::from_reader(self.get_reader()?);

        match file_content {
            Ok(result) => {
                self.content = Some(result);
                Ok(self.content.as_ref().unwrap().clone())
            },
            Err(error) => Err(Box::new(error))
        }

    }

    fn query(&self, path: &str) -> Self::Queried {

        let path = path.split(".");
        let mut result = self.content.as_ref();

        for location in path {

            if result.is_none() {
                return None;
            }

            result = result.unwrap().get(location);

        }

        match result {
            Some(result) => Some(result.clone()),
            None => None
        }

    }

}

#[cfg(test)]
mod test {

    use crate::{FileManager, JsonFile};

    fn get_test_file() -> JsonFile {
        JsonFile::new("../../testfiles/file.json")
    }

    #[test]
    fn query() {
        let mut file = get_test_file();
        assert!(file.parse().is_ok());
        assert!(file.query("test").is_some());
        assert!(file.query("x").is_none());
    }

}
