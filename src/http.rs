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

    #[tokio::test]
    async fn http_client_bad_endpoint() {
        let http_response = HttpClient::new("https://httpbin.org/ge")
            .get_text_from_url()
            .await
            .unwrap();
        assert_eq!(404, http_response.status);
    }

    #[tokio::test]
    #[should_panic(expected = "cannot_be_a_base")]
    async fn http_client_bad_protocol() {
        let _http_response = HttpClient::new("htt://httpbin.org/get")
            .get_text_from_url()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "RelativeUrlWithoutBase")]
    async fn http_client_malformed_url() {
        let _http_response = HttpClient::new("$^45fd456g*")
            .get_text_from_url()
            .await
            .unwrap();
    }
}
