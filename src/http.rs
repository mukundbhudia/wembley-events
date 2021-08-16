use std::error::Error;

pub struct HttpClient {
    url: String,
}

impl HttpClient {
    pub fn new(url: &str) -> Self {
        HttpClient { url: url.into() }
    }
    pub async fn get_text_from_url(&self) -> Result<String, Box<dyn Error>> {
        let res = reqwest::get(&self.url).await?;
        Ok(res.text().await?)
    }
}

// TODO: consider async http client test for a basic URL (e.g.: https://httpbin.org/get)
