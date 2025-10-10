use std::time::Duration;

use crate::{RetryConfig, retry_request};

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
        let retry_config = RetryConfig {
            max_retries: 5,
            base_delay: Duration::from_millis(200),
            ..Default::default()
        };
        let http_request = || reqwest::get(&self.url);

        match retry_request(http_request, retry_config).await {
            Ok(res) => {
                let res_status = res.status().as_u16();

                if res_status >= 400 {
                    eprintln!("HTTP request returned error status: {}.", res_status);
                    return Err(HttpClientError::RequestError);
                }

                match res.text().await {
                    Ok(res_body) => {
                        if res_body.is_empty() {
                            eprintln!("HTTP response body is empty.");
                            Err(HttpClientError::ResponseBodyError)
                        } else {
                            Ok(HttpResponse::new(res_status, res_body))
                        }
                    }
                    Err(e) => {
                        eprintln!("HTTP response body error: {:#?}.", e);
                        Err(HttpClientError::ResponseBodyError)
                    }
                }
            }
            Err(e) => {
                eprintln!("HTTP request error after maximum retries: {:#?}.", e);
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
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    use super::*;

    async fn setup_mock_server(
        expected_http_status: u16,
        expected_times_called: u64,
    ) -> MockServer {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/get"))
            .respond_with(ResponseTemplate::new(expected_http_status).set_body_string("Hello"))
            .expect(expected_times_called)
            .mount(&mock_server)
            .await;

        mock_server
    }

    #[tokio::test]
    async fn basic_http_access_test() {
        let mock_server = setup_mock_server(200, 1).await;
        let endpoint = format!("{}/get", &mock_server.uri());

        let http_response = HttpClient::new(&endpoint).get_text_from_url().await;
        assert!(http_response.is_ok());
        let http_response = http_response.unwrap();

        assert_eq!(200, http_response.status);
    }

    #[tokio::test]
    async fn http_client_bad_endpoint() {
        let mock_server = setup_mock_server(404, 1).await;
        let endpoint = format!("{}/get", &mock_server.uri());

        let http_response = HttpClient::new(&endpoint).get_text_from_url().await;
        assert!(http_response.is_err());
        let http_response = http_response.unwrap_err();
        assert_eq!(http_response, HttpClientError::RequestError);
    }

    #[tokio::test]
    async fn http_client_server_error() {
        let mock_server = setup_mock_server(500, 5).await;
        let endpoint = format!("{}/get", &mock_server.uri());

        let http_response = HttpClient::new(&endpoint).get_text_from_url().await;

        assert!(http_response.is_err());
        let http_response_err = http_response.unwrap_err();
        assert_eq!(http_response_err, HttpClientError::RequestError);
    }

    #[tokio::test]
    async fn http_client_bad_protocol() {
        let http_response = HttpClient::new("htt://localhost").get_text_from_url().await;

        assert!(http_response.is_err());
        assert_eq!(http_response, Err(HttpClientError::RequestError));
    }

    #[tokio::test]
    async fn http_client_malformed_url() {
        let http_response = HttpClient::new("$^45fd456g*").get_text_from_url().await;

        assert!(http_response.is_err());
        assert_eq!(http_response, Err(HttpClientError::RequestError));
    }
}
