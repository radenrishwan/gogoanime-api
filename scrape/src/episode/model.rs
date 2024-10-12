use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Episode {
    urls: Vec<String>,
}

impl Episode {
    pub fn new(urls: Vec<String>) -> Self {
        Self { urls }
    }
}
