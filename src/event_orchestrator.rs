use std::process;

use crate::{
    Config, HttpClient, WembleyEvents,
    event_merger::{
        fetch_existing_events, filter_future_events, load_existing_events_from_file,
        merge_and_deduplicate,
    },
};

/// Orchestrates fetching, merging, and building the final event list
pub struct EventOrchestrator {
    config: Config,
}

impl EventOrchestrator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Fetches and processes all events, returning the final merged result
    pub async fn fetch_and_merge_events(&self) -> WembleyEvents {
        let existing_events = self.fetch_existing_events_if_configured().await;
        let new_events = self.fetch_new_events_from_api().await;
        self.merge_events(existing_events, new_events)
    }

    /// Fetches existing events from gh-pages if configured
    /// Prefers local file path over URL if both are configured
    async fn fetch_existing_events_if_configured(&self) -> Option<WembleyEvents> {
        // Try local file path first
        if let Some(file_path) = &self.config.gh_pages_json_file_path {
            println!("Loading existing events from local file: {}", file_path);
            if let Some(events) = load_existing_events_from_file(file_path) {
                println!("Found {} existing events.", events.events.len());
                let filtered = filter_future_events(events);
                println!(
                    "Kept {} future events after filtering.",
                    filtered.events.len()
                );
                return Some(filtered);
            } else {
                println!("Failed to load from local file. Will try URL if configured.");
            }
        }

        // Fall back to URL if local file not available or failed
        if let Some(gh_pages_url) = &self.config.gh_pages_json_url {
            println!("Fetching existing events from URL: {}", gh_pages_url);
            match fetch_existing_events(gh_pages_url).await {
                Some(events) => {
                    println!("Found {} existing events.", events.events.len());
                    let filtered = filter_future_events(events);
                    println!(
                        "Kept {} future events after filtering.",
                        filtered.events.len()
                    );
                    Some(filtered)
                }
                None => {
                    println!("No existing events found or failed to fetch. Starting fresh.");
                    None
                }
            }
        } else {
            None
        }
    }

    /// Fetches new events from the API
    async fn fetch_new_events_from_api(&self) -> WembleyEvents {
        let full_url = format!(
            "{}{}",
            &self.config.calendar_url, &self.config.serpapi_api_key
        );

        let res = match HttpClient::new(&full_url).get_text_from_url().await {
            Ok(res) => res,
            Err(_) => {
                eprintln!("Failed to fetch calendar data from API.");
                process::exit(1);
            }
        };

        let new_events = WembleyEvents::new().build_events_from_html(res.body);
        println!("Fetched {} new events from API.", new_events.events.len());
        new_events
    }

    /// Merges existing and new events if existing events are available
    fn merge_events(
        &self,
        existing_events: Option<WembleyEvents>,
        new_events: WembleyEvents,
    ) -> WembleyEvents {
        match existing_events {
            Some(existing) => {
                println!("Merging existing and new events...");
                let merged = merge_and_deduplicate(existing, new_events);
                println!("Total events after merge: {}", merged.events.len());
                merged
            }
            None => {
                println!("No existing events to merge. Using new events only.");
                new_events
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator_creation() {
        let config = Config::new();
        let orchestrator = EventOrchestrator::new(config);
        assert!(orchestrator.config.gh_pages_json_url.is_none());
    }

    #[test]
    fn test_merge_events_no_existing() {
        let config = Config::new();
        let orchestrator = EventOrchestrator::new(config);
        let new_events = WembleyEvents::new();

        let result = orchestrator.merge_events(None, new_events.clone());
        assert_eq!(result.events.len(), new_events.events.len());
    }

    #[test]
    fn test_merge_events_with_existing() {
        let config = Config::new();
        let orchestrator = EventOrchestrator::new(config);

        let mut existing = WembleyEvents::new();
        existing.events.insert(
            0,
            crate::WembleyEvent::new(
                "1 Jan 2030".to_string(),
                "Test".to_string(),
                "Place".to_string(),
                "Event 1".to_string(),
                "Desc".to_string(),
                "https://example.com".to_string(),
            ),
        );

        let mut new = WembleyEvents::new();
        new.events.insert(
            0,
            crate::WembleyEvent::new(
                "2 Jan 2030".to_string(),
                "Test".to_string(),
                "Place".to_string(),
                "Event 2".to_string(),
                "Desc".to_string(),
                "https://example.com".to_string(),
            ),
        );

        let result = orchestrator.merge_events(Some(existing), new);
        assert_eq!(result.events.len(), 2);
    }
}
