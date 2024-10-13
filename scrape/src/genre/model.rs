use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Genre {
    id: String,
    url: String,
}

impl Genre {
    pub fn new(id: String, url: String) -> Self {
        Self { id, url }
    }
}
