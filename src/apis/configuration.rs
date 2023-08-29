#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub client: reqwest::Client
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "http://localhost".to_owned(),
            client: reqwest::Client::new()

        }
    }
}