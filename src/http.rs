#[derive(Debug, PartialEq)]
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

    pub async fn get_text_from_url(&self) -> Result<HttpResponse, HttpClientError> {
        match reqwest::get(&self.url).await {
            Ok(res) => {
                let res_status = res.status().as_u16();
                match res.text().await {
                    Ok(res_body) => Ok(HttpResponse::new(res_status, res_body)),
                    Err(e) => {
                        eprintln!("HTTP response body error: {:#?}", e);
                        Err(HttpClientError::ResponseBodyError)
                    }
                }
            }
            Err(e) => {
                eprintln!("HTTP request error: {:#?}", e);
                Err(HttpClientError::RequestError)
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum HttpClientError {
    RequestError,
    ResponseBodyError,
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
    async fn http_client_bad_protocol() {
        let http_response = HttpClient::new("htt://httpbin.org/get")
            .get_text_from_url()
            .await;

        assert_eq!(http_response, Err(HttpClientError::RequestError));
    }

    #[tokio::test]
    async fn http_client_malformed_url() {
        let http_response = HttpClient::new("$^45fd456g*").get_text_from_url().await;

        assert_eq!(http_response, Err(HttpClientError::RequestError));
    }
}
