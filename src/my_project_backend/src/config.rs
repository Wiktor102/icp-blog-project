#[derive(Clone)]
pub struct Config {
    pub max_tags_count: u8,
    pub max_title_length: u8,
    pub max_content_length: u16,
    pub tags: Vec<String>,
}

impl Config {
    pub fn new() -> Self{
        Self {
            max_tags_count: 2,
            max_content_length: 2000,
            max_title_length: 64,
            tags: vec!["rust".to_string(), "webassembly".to_string(), "ic".to_string()],
        }
    }
}
