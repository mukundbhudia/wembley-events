use chrono::{NaiveDate, Utc};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::{HttpClient, WembleyEvent, WembleyEvents};

/// Loads existing events from a local file path
pub fn load_existing_events_from_file(file_path: &str) -> Option<WembleyEvents> {
    let path = Path::new(file_path);

    if !path.exists() {
        return None;
    }

    match fs::read_to_string(path) {
        Ok(content) => {
            if let Ok(events) = serde_json::from_str::<Vec<WembleyEvent>>(&content) {
                let mut wembley_events = WembleyEvents::new();
                wembley_events.events = events.into_iter().enumerate().collect();
                Some(wembley_events)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Fetches existing events from the gh-pages deployment (via URL)
pub async fn fetch_existing_events(gh_pages_url: &str) -> Option<WembleyEvents> {
    match HttpClient::new(gh_pages_url).get_text_from_url().await {
        Ok(response) => {
            if let Ok(events) = serde_json::from_str::<Vec<WembleyEvent>>(&response.body) {
                let mut wembley_events = WembleyEvents::new();
                wembley_events.events = events.into_iter().enumerate().collect();
                Some(wembley_events)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Filters events to keep only those on or after today's date
pub fn filter_future_events(events: WembleyEvents) -> WembleyEvents {
    let today = Utc::now().date_naive();

    let filtered_events: Vec<WembleyEvent> = events
        .events
        .into_iter()
        .filter(|(_, event)| {
            if let Some(ymd) = event.date_to_ymd()
                && let Some(event_date) = NaiveDate::from_ymd_opt(ymd.year, ymd.month, ymd.day)
            {
                return event_date >= today;
            }
            false
        })
        .map(|(_, event)| event)
        .collect();

    WembleyEvents {
        events: filtered_events.into_iter().enumerate().collect(),
    }
}

/// Generates a unique key for an event based on title, date, and place
fn event_key(event: &WembleyEvent) -> String {
    format!(
        "{}|{}|{}",
        event.title.trim().to_lowercase(),
        event.date.trim().to_lowercase(),
        event.place.trim().to_lowercase()
    )
}

/// Merges existing and new events, removing duplicates
/// Keeps the version from new_events when duplicates are found
pub fn merge_and_deduplicate(
    existing_events: WembleyEvents,
    new_events: WembleyEvents,
) -> WembleyEvents {
    let mut seen_keys = HashSet::new();
    let mut merged = Vec::new();

    // First, add all new events (they take priority)
    for (_, event) in new_events.events.into_iter() {
        let key = event_key(&event);
        if seen_keys.insert(key) {
            merged.push(event);
        }
    }

    // Then, add existing events that aren't duplicates
    for (_, event) in existing_events.events.into_iter() {
        let key = event_key(&event);
        if seen_keys.insert(key) {
            merged.push(event);
        }
    }

    // Sort by date to maintain a consistent order
    merged.sort_by(|a, b| {
        let a_ymd = a.date_to_ymd();
        let b_ymd = b.date_to_ymd();

        match (a_ymd, b_ymd) {
            (Some(a_date), Some(b_date)) => {
                let a_naive = NaiveDate::from_ymd_opt(a_date.year, a_date.month, a_date.day);
                let b_naive = NaiveDate::from_ymd_opt(b_date.year, b_date.month, b_date.day);

                match (a_naive, b_naive) {
                    (Some(a_d), Some(b_d)) => a_d.cmp(&b_d),
                    _ => std::cmp::Ordering::Equal,
                }
            }
            _ => std::cmp::Ordering::Equal,
        }
    });

    WembleyEvents {
        events: merged.into_iter().enumerate().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_event(title: &str, date: &str, place: &str) -> WembleyEvent {
        WembleyEvent::new(
            date.to_string(),
            "Test time".to_string(),
            place.to_string(),
            title.to_string(),
            "Test description".to_string(),
            "https://example.com".to_string(),
        )
    }

    #[test]
    fn test_filter_future_events() {
        let mut events = WembleyEvents::new();
        events.events.insert(
            0,
            create_test_event("Future Event", "1 Jan 2030", "Test Place"),
        );
        events.events.insert(
            1,
            create_test_event("Past Event", "1 Jan 2020", "Test Place"),
        );

        let filtered = filter_future_events(events);

        assert_eq!(filtered.events.len(), 1);
        assert_eq!(filtered.events.get(&0).unwrap().title, "Future Event");
    }

    #[test]
    fn test_merge_and_deduplicate_no_duplicates() {
        let mut existing = WembleyEvents::new();
        existing
            .events
            .insert(0, create_test_event("Event 1", "1 Jan 2030", "Place 1"));

        let mut new = WembleyEvents::new();
        new.events
            .insert(0, create_test_event("Event 2", "2 Jan 2030", "Place 2"));

        let merged = merge_and_deduplicate(existing, new);

        assert_eq!(merged.events.len(), 2);
    }

    #[test]
    fn test_merge_and_deduplicate_with_duplicates() {
        let mut existing = WembleyEvents::new();
        existing
            .events
            .insert(0, create_test_event("Event 1", "1 Jan 2030", "Place 1"));

        let mut new = WembleyEvents::new();
        new.events
            .insert(0, create_test_event("Event 1", "1 Jan 2030", "Place 1"));
        new.events
            .insert(1, create_test_event("Event 2", "2 Jan 2030", "Place 2"));

        let merged = merge_and_deduplicate(existing, new);

        // Should only have 2 unique events
        assert_eq!(merged.events.len(), 2);
    }

    #[test]
    fn test_merge_prefers_new_events() {
        let mut existing = WembleyEvents::new();
        let mut old_event = create_test_event("Event 1", "1 Jan 2030", "Place 1");
        old_event.description = "Old description".to_string();
        existing.events.insert(0, old_event);

        let mut new = WembleyEvents::new();
        let mut new_event = create_test_event("Event 1", "1 Jan 2030", "Place 1");
        new_event.description = "New description".to_string();
        new.events.insert(0, new_event);

        let merged = merge_and_deduplicate(existing, new);

        assert_eq!(merged.events.len(), 1);
        assert_eq!(
            merged.events.get(&0).unwrap().description,
            "New description"
        );
    }

    #[test]
    fn test_event_key_case_insensitive() {
        let event1 = create_test_event("Event Title", "1 Jan 2030", "Place Name");
        let event2 = create_test_event("EVENT TITLE", "1 JAN 2030", "PLACE NAME");

        assert_eq!(event_key(&event1), event_key(&event2));
    }
}
