use std::str::FromStr;

pub fn read_file<T: FromStr>(file_name: &str) -> Result<Vec<T>, AoCError> {
    std::fs::read_to_string(file_name).map_err(|e| AoCError::new(e.to_string()))?
        .lines()
        .map(|s| s.parse::<T>().map_err(|_| AoCError::new("Parsing error".to_owned())))
        .collect()
}

#[derive(Debug)]
pub struct AoCError {
    msg: String,
}

impl AoCError {
    fn new(msg: String) -> AoCError {
        AoCError { msg }
    }
}
