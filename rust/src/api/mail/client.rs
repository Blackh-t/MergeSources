use super::{error::SGClienResult, mail_body::MailBox};
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Response,
};
static API_URL: &str = "https://api.sendgrid.com/api/mail.send.json?";

#[derive(Clone, Debug)]
pub struct SendGridClient {
    api_key: String,
    host: String,
    client: reqwest::Client,
}

fn build_body(mail_info: &MailBox) -> Result<String, serde_json::Error> {
    let converting = serde_json::to_string_pretty(mail_info)?;
    Ok(converting)
}

impl SendGridClient {
    pub fn new<S: Into<String>>(key: S) -> SendGridClient {
        // Init HTTP-request
        let builder = reqwest::ClientBuilder::new();
        let client = builder.build().unwrap();
        // Init SG-Client
        SendGridClient {
            api_key: key.into(),
            host: API_URL.to_string(),
            client,
        }
    }

    pub fn sets_host<S: Into<String>>(&mut self, api_url: S) {
        self.host = api_url.into()
    }

    pub async fn send(&self, mail_body: MailBox<'_>) -> SGClienResult<Response> {
        let parsed_mail = build_body(&mail_body)?;
        let reps = self
            .client
            .post(self.host.clone())
            .headers(self.headers().await?)
            .body(parsed_mail)
            .send()
            .await?;
        Ok(reps)
    }

    async fn headers(&self) -> SGClienResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key.clone()))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Ok(headers)
    }
}
