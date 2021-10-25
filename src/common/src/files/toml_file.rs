use std::error::Error;
use toml::Value;
use crate::files::FileManager;

pub struct TomlFile {
    pub filename: String,
    content: Option<Value>
}

impl FileManager for TomlFile {

    type Parsed = Result<Value, Box<dyn Error>>;
    type Queried = Option<Value>;

    fn new(filename: &str) -> Self {
        TomlFile {
            filename: filename.to_string(),
            content: None
        }
    }

    fn filename(&self) -> &str {
        &self.filename
    }

    fn parse(&mut self) -> Self::Parsed {

        let file_content = self.get()?.parse::<Value>();

        match file_content {
            Ok(result) => {
                self.content = Some(result.clone());
                Ok(result)
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