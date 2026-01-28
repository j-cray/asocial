use serde::Serialize;
use std::error::Error;

pub struct MastodonClient {
    base_url: String,
    access_token: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct StatusParams<'a> {
    status: &'a str,
}

impl MastodonClient {
    pub fn new(base_url: String, access_token: String) -> Self {
        Self {
            base_url,
            access_token,
            client: reqwest::Client::new(),
        }
    }

    pub async fn post_status(&self, content: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/v1/statuses", self.base_url);
        let params = StatusParams { status: content };

        let res = self.client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&params)
            .send()
            .await?;

        if res.status().is_success() {
             let text = res.text().await?;
             Ok(text)
        } else {
            Err(format!("Mastodon API Error: {}", res.status()).into())
        }
    }
}
