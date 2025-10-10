use chrono::{DateTime, Local};
use reqwest::{Error, StatusCode};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug)]
pub enum RetryError {
    MaxRetriesExceeded,
    NonRetryableError(Error),
}

#[derive(Clone)]
pub struct RetryConfig {
    pub max_retries: usize,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

pub async fn retry_request<F, Fut>(
    operation: F,
    config: RetryConfig,
) -> Result<reqwest::Response, RetryError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<reqwest::Response, Error>>,
{
    let mut attempts = 0;
    let mut delay = config.base_delay;

    loop {
        match operation().await {
            Ok(response) => {
                println!("Received response with status: {}", response.status());

                match should_retry_status(response.status()) {
                    true => {
                        attempts += 1;
                        let now: DateTime<Local> = Local::now();
                        eprintln!(
                            "Retrying request... attempt: {} of {} at {}",
                            attempts, config.max_retries, now
                        );

                        if attempts >= config.max_retries {
                            return Err(RetryError::MaxRetriesExceeded);
                        }
                    }
                    false => return Ok(response),
                }
            }
            Err(err) => {
                eprintln!("Request error: {:#?}", err);
                if !should_retry_error(&err) {
                    return Err(RetryError::NonRetryableError(err));
                }

                attempts += 1;
                let now: DateTime<Local> = Local::now();
                eprintln!(
                    "Retrying request... attempt: {} of {} at {}",
                    attempts, config.max_retries, now
                );

                if attempts >= config.max_retries {
                    eprintln!(
                        "Max retries exceeded after {} attempts.",
                        config.max_retries
                    );
                    return Err(RetryError::MaxRetriesExceeded);
                }
            }
        }

        // Apply exponential backoff with a jitter
        sleep(delay).await;
        delay = std::cmp::min(
            Duration::from_secs_f64(delay.as_secs_f64() * config.backoff_multiplier),
            config.max_delay,
        );
    }
}

fn should_retry_status(status: StatusCode) -> bool {
    matches!(
        status.as_u16(),
        429 |     // Too Many Requests
        500..=599 // Server errors
    )
}

fn should_retry_error(error: &Error) -> bool {
    error.is_timeout() || error.is_connect() || error.is_request()
}
