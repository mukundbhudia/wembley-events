extern crate dotenv;

use dotenv::dotenv;
use std::env;
pub struct Config {
    pub calendar_url: String,
    pub calendar_save_path: String,
    pub calendar_json_save_path: Option<String>,
    pub serpapi_api_key: String,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_dotenv(mut self) -> Self {
        dotenv().ok();

        if let Ok(calendar_url) = env::var("CALENDAR_URL") {
            self.calendar_url = calendar_url;
        } else {
            eprintln!("Missing CALENDAR_URL environment variable. Using default.");
        }

        if let Ok(calendar_save_path) = env::var("CALENDAR_SAVE_PATH") {
            self.calendar_save_path = calendar_save_path;
        } else {
            eprintln!("Missing CALENDAR_SAVE_PATH environment variable. Using default.");
        }

        if let Ok(serpapi_api_key) = env::var("SERPAPI_API_KEY") {
            if serpapi_api_key.is_empty() {
                eprintln!("Empty SERPAPI_API_KEY environment variable. Using default.");
            } else {
                self.serpapi_api_key = serpapi_api_key;
            }
        } else {
            eprintln!("Missing SERPAPI_API_KEY environment variable. Using default.");
        }

        self.calendar_json_save_path = env::var("CALENDAR_JSON_SAVE_PATH").ok();

        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            calendar_url: "https://www.brent.gov.uk/events-and-whats-on-calendar/?eventCat=Wembley+Stadium+events".into(),
            calendar_save_path: "output/wembley-events.ics".into(),
            calendar_json_save_path: None,
            serpapi_api_key: String::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::new();
        assert_eq!(config.calendar_url, Config::default().calendar_url);
        assert_eq!(
            config.calendar_save_path,
            Config::default().calendar_save_path
        );
        assert!(config.calendar_json_save_path.is_none());
        assert_eq!(config.serpapi_api_key, Config::default().serpapi_api_key);
    }

    #[test]
    #[ignore = "needs a local `.env` file to run"]
    fn test_config_should_never_be_empty() {
        let config = Config::new().load_from_dotenv();
        assert!(!config.calendar_url.is_empty());
        assert!(!config.calendar_save_path.is_empty());
        assert!(config.calendar_json_save_path.is_some());
        assert!(!config.serpapi_api_key.is_empty());
    }
}
