use std::str::{FromStr};

pub fn read_file<T: FromStr>(file_name: &str) -> Result<Vec<T>, AoCError> {
    let file_content = std::fs::read_to_string(file_name).map_err(|e| AoCError::new(e.to_string()))?;
    parse_iter(&mut file_content.lines())
}

pub fn parse_iter<T: FromStr>(lines: &mut dyn Iterator<Item = &str>) -> Result<Vec<T>, AoCError> {
    lines
        .map(|s| s.parse::<T>().map_err(|_| AoCError::new("Parsing error".to_owned())))
        .collect()
}

#[derive(Debug)]
pub struct AoCError {
    msg: String,
}

impl AoCError {
    pub fn new(msg: String) -> AoCError {
        AoCError { msg }
    }
}
