#[derive(Debug)]
pub struct ScrapeError {
    msg: String,
}

impl ScrapeError {
    pub fn new(msg: String) -> Self {
        ScrapeError { msg }
    }
}

impl std::fmt::Display for ScrapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ScrapeError: {}", self.msg)
    }
}

impl std::error::Error for ScrapeError {
    fn description(&self) -> &str {
        &self.msg
    }
}
