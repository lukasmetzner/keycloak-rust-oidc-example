use std::fmt;

#[derive(Debug)]
pub struct AlgNotFoundError {
    pub message: String,
}

impl fmt::Display for AlgNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AlgNotFoundError {}

impl AlgNotFoundError {
    pub fn new(alg_name: &str) -> AlgNotFoundError {
        AlgNotFoundError {
            message: format!("{} algorithm not found!", alg_name),
        }
    }
}
