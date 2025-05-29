#![doc = include_str!("../README.md")]

use anyhow::{Result, anyhow};
use reqwest::{Client, StatusCode as Status, header};
use secrecy::{ExposeSecret, SecretString};

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case", tag = "event_type")]
pub enum Event {
    Trigger {
        incident_key: Option<String>,
        description: String,
    },
    #[allow(dead_code)] // Not all binaries create Acknowledge events
    Acknowledge {
        incident_key: String,
        description: Option<String>,
    },
    Resolve {
        incident_key: String,
        description: Option<String>,
    },
}

#[derive(Clone, Debug)]
pub struct PagerdutyClient {
    authorization: SecretString,
    service_key: String,
}

impl PagerdutyClient {
    pub fn new(api_token: SecretString, service_key: String) -> Self {
        Self {
            authorization: format!("Token token={}", api_token.expose_secret()).into(),
            service_key,
        }
    }
}

impl PagerdutyClient {
    /// Sends the event to pagerduty.
    ///
    /// If the variant is `Trigger`, this will page whoever is on call
    /// (potentially waking them up at 3 AM).
    pub async fn send(&self, event: &Event) -> Result<()> {
        let service_key = &self.service_key;

        let response = Client::new()
            .post("https://events.pagerduty.com/generic/2010-04-15/create_event.json")
            .header(header::ACCEPT, "application/vnd.pagerduty+json;version=2")
            .header(header::AUTHORIZATION, self.authorization.expose_secret())
            .json(&FullEvent { service_key, event })
            .send()
            .await?;

        match response.status() {
            s if s.is_success() => Ok(()),
            Status::BAD_REQUEST => {
                let error = response.json::<InvalidEvent>().await?;
                Err(anyhow!("pagerduty error: {:?}", error))
            }
            Status::FORBIDDEN => Err(anyhow!("rate limited by pagerduty")),
            n => Err(anyhow!(
                "Got a non 200 response code from pagerduty: {} with {:?}",
                n,
                response
            )),
        }
    }
}

#[derive(serde::Serialize, Debug)]
struct FullEvent<'a> {
    service_key: &'a str,
    #[serde(flatten)]
    event: &'a Event,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)] // workaround for a potential bug in Rust v1.57.0-beta.1
struct InvalidEvent {
    message: String,
    errors: Vec<String>,
}
