extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub calendar_url: String,
    pub calendar_save_path: String,
    pub calendar_json_save_path: Option<String>,
    pub ticketmaster_api_key: String,
    pub gh_pages_json_url: Option<String>,
    pub gh_pages_json_file_path: Option<String>,
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

        if let Ok(ticketmaster_api_key) = env::var("TICKETMASTER_API_KEY") {
            if ticketmaster_api_key.is_empty() {
                eprintln!("Empty TICKETMASTER_API_KEY environment variable. Using default.");
            } else {
                self.ticketmaster_api_key = ticketmaster_api_key;
            }
        } else {
            eprintln!("Missing TICKETMASTER_API_KEY environment variable. Using default.");
        }

        self.calendar_json_save_path = env::var("CALENDAR_JSON_SAVE_PATH").ok();
        self.gh_pages_json_url = env::var("GH_PAGES_JSON_URL").ok();
        self.gh_pages_json_file_path = env::var("GH_PAGES_JSON_FILE_PATH").ok();

        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            calendar_url: "https://app.ticketmaster.com/discovery/v2/events.json".into(),
            calendar_save_path: "output/wembley-events.ics".into(),
            calendar_json_save_path: None,
            ticketmaster_api_key: String::default(),
            gh_pages_json_url: None,
            gh_pages_json_file_path: None,
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
        assert_eq!(config.ticketmaster_api_key, Config::default().ticketmaster_api_key);
    }

    #[test]
    #[ignore = "needs a local `.env` file to run"]
    fn test_config_should_never_be_empty() {
        let config = Config::new().load_from_dotenv();
        assert!(!config.calendar_url.is_empty());
        assert!(!config.calendar_save_path.is_empty());
        assert!(config.calendar_json_save_path.is_some());
        assert!(!config.ticketmaster_api_key.is_empty());
    }
}
