use std::error::Error;

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status: u16, body: String) -> Self {
        Self { status, body }
    }
}
pub struct HttpClient {
    url: String,
}

impl HttpClient {
    pub fn new(url: &str) -> Self {
        HttpClient { url: url.into() }
    }
    pub async fn get_text_from_url(&self) -> Result<HttpResponse, Box<dyn Error>> {
        let res = reqwest::get(&self.url).await?;
        Ok(HttpResponse::new(res.status().as_u16(), res.text().await?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn basic_http_access_test() {
        let http_response = HttpClient::new("https://httpbin.org/get")
            .get_text_from_url()
            .await
            .unwrap();
        assert_eq!(200, http_response.status);
    }
}
