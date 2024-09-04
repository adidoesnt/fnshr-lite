use mongod::{Client, ClientBuilder};

pub struct MongoDB {
    pub client: Client,
}

impl MongoDB {
    pub fn new(uri: &str, db_name: &str) -> Self {
        let client: Client = ClientBuilder::new()
            .uri(uri)
            .database(db_name)
            .build()
            .expect("Failed to create MongoDB client");
        Self { client }
    }
}
