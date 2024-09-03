use mongod::{Client, ClientBuilder};

pub struct Database {
    _client: Client,
}

impl Database {
    pub fn new(uri: &str) -> Self {
        let _client: Client = ClientBuilder::new()
            .uri(uri)
            .build()
            .expect("Failed to create MongoDB _client");
        Self { _client }
    }
}