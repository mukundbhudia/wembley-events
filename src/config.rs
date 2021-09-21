extern crate dotenv;

use dotenv::dotenv;
use std::env;
pub struct Config {
    pub calendar_url: String,
    pub calendar_save_path: String,
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

        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            calendar_url: "https://www.brent.gov.uk/events-and-whats-on-calendar/?eventCat=Wembley+Stadium+events".into(),
            calendar_save_path: "output/wembley-events.ics".into() 
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
    }

    #[test]
    #[ignore = "needs .env file"]
    fn test_config_should_never_be_empty() {
        let config = Config::new().load_from_dotenv();
        assert!(!config.calendar_url.is_empty());
        assert!(!config.calendar_save_path.is_empty());
    }
}
