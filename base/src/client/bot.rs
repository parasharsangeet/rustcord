use std::collections::HashMap;

use serde_json::json;

pub struct Client {
    app_id: u64,
}

impl Client {
    pub fn new(app_id: u64) -> Self {
        Self { app_id }
    }

    pub async fn start(&self) {
        let client = reqwest::Client::new();

        let name = "hello";
        let description = "Send a random adorable animal photo";

        let mut map = HashMap::new();

        map.insert("name", name);
        map.insert("description", description);

        let resp = client
            .post(format!(
                "https://discord.com/api/v10/applications/{}/commands",
                self.app_id
            ))
            .json(&map)
            .header(
                "Authorization",
                "Bot MTA1ODc2MzQ0ODYyNTkzODUwNA.GRBUIj.LLQwKc8YKfbSa19kggS4_PTNkggz8V089ZqLUM",
            )
            .send()
            .await
            .expect("Failed to register Slash Command");
    }
}
