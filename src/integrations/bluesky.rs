use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

pub struct BlueskyClient {
    identifier: String,
    password: String,
    base_url: String,
    client: reqwest::Client,
    access_token: Option<String>,
    did: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateSessionRequest<'a> {
    identifier: &'a str,
    password: &'a str,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSessionResponse {
    access_jwt: String,
    did: String,
}

impl BlueskyClient {
    pub fn new(identifier: String, password: String) -> Self {
        Self {
            identifier,
            password,
            base_url: "https://bsky.social".to_string(), // Default to main server
            client: reqwest::Client::new(),
            access_token: None,
            did: None,
        }
    }

    pub async fn login(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let url = format!("{}/xrpc/com.atproto.server.createSession", self.base_url);
        let params = CreateSessionRequest {
            identifier: &self.identifier,
            password: &self.password,
        };

        let res = self.client
            .post(&url)
            .json(&params)
            .send()
            .await?;

        if res.status().is_success() {
            let session: CreateSessionResponse = res.json().await?;
            self.access_token = Some(session.access_jwt);
            self.did = Some(session.did);
            Ok(())
        } else {
            Err(format!("Bluesky Login Error: {}", res.status()).into())
        }
    }

    pub async fn post_record(&self, content: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let token = self.access_token.as_ref().ok_or("Not logged in")?;
        let did = self.did.as_ref().ok_or("No DID found")?;

        let url = format!("{}/xrpc/com.atproto.repo.createRecord", self.base_url);
        
        // Simplified record structure. Real implementation needs formatted date and potentially facets.
        let record = json!({
            "repo": did,
            "collection": "app.bsky.feed.post",
            "record": {
                "$type": "app.bsky.feed.post",
                "text": content,
                "createdAt": time::OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339)?
            }
        });

        let res = self.client
            .post(&url)
            .bearer_auth(token)
            .json(&record)
            .send()
            .await?;

        if res.status().is_success() {
             let text = res.text().await?;
             Ok(text)
        } else {
            Err(format!("Bluesky Post Error: {}", res.status()).into())
        }
    }
}
