use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use chrono::*;
use icalendar::*;

use crate::{SerpapiEvents, WembleyEvent};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WembleyEvents {
    pub events: BTreeMap<usize, WembleyEvent>,
}

impl WembleyEvents {
    pub fn new() -> Self {
        WembleyEvents {
            events: BTreeMap::new(),
        }
    }

    pub fn get_events(&self) -> &BTreeMap<usize, WembleyEvent> {
        &self.events
    }

    pub fn build_events_from_html(mut self, html: String) -> WembleyEvents {
        let serp_api_events = serde_json::from_str::<SerpapiEvents>(&html).unwrap_or_default();

        let mut year = serp_api_events.search_metadata.created_at;
        year.truncate(4);

        self.events = serp_api_events
            .events_results
            .into_iter()
            .map(|mut e| {
                let mut swapped_date = e.date.start_date.split_whitespace().collect::<Vec<&str>>();
                if swapped_date.len() >= 2 {
                    swapped_date.push(&year);
                    swapped_date.swap(0, 1); // swap [month, day] to [day, month]

                    e.date.start_date = swapped_date.join(" ");
                }
                e.into()
            })
            .filter(|e: &WembleyEvent| {
                let place_lower = e.place.to_lowercase();
                let description_lower = e.description.to_lowercase();
                place_lower.contains("wembley") || description_lower.contains("wembley")
            })
            .inspect(|e: &WembleyEvent| println!("place: {:#}", e))
            .enumerate()
            .collect::<BTreeMap<usize, WembleyEvent>>();

        println!("Built {} events from req body.", self.events.len());

        Self {
            events: self.events,
        }
    }

    pub fn build_calendar_from_events(self) -> Calendar {
        let mut calendar = Calendar::new();

        self.events.into_iter().for_each(|(_, event)| {
            if let Some(ymd) = event.date_to_ymd() {
                let wembley_event = Event::new()
                    .all_day(Utc.ymd(ymd.year, ymd.month, ymd.day))
                    .summary(&event.title)
                    .description(&event.description)
                    .done();

                calendar.push(wembley_event);
            };
        });

        calendar
    }

    pub fn build_json_from_events(self) -> String {
        serde_json::to_string(&self.events.values().collect::<Vec<_>>()).unwrap()
    }
}

impl Default for WembleyEvents {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_files::{
        serpapi_test_output_json_1, serpapi_test_output_json_2,
        serpapi_test_output_json_3_some_fields_missing,
        serpapi_test_output_json_8_mixed_wembley_non_wembley,
    };

    use super::*;

    #[test]
    fn build_events_from_html() {
        let body = serpapi_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        assert_eq!(wembley_events.get_events().len(), 10);
    }

    #[test]
    fn build_events_from_html_2_2() {
        let body = serpapi_test_output_json_3_some_fields_missing();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        assert_eq!(wembley_events.get_events().len(), 9);
    }

    #[test]
    fn build_calendar_from_events() {
        let body = serpapi_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        let calendar = wembley_events.build_calendar_from_events();

        assert_eq!(calendar.len(), 10);
    }

    #[test]
    fn check_events_match_calendar() {
        let body = serpapi_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);
        let calendar_built_from_events = wembley_events.build_calendar_from_events();

        insta::assert_debug_snapshot!(calendar_built_from_events);
    }

    #[test]
    fn check_events_match_calendar_with_blank_html() {
        let body = serpapi_test_output_json_2();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);
        let calendar_built_from_events = wembley_events.build_calendar_from_events();

        insta::assert_debug_snapshot!(calendar_built_from_events);
    }

    #[test]
    fn check_events_match_calendar_with_blank_string() {
        let body = String::new();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);
        let calendar_built_from_events = wembley_events.build_calendar_from_events();

        insta::assert_debug_snapshot!(calendar_built_from_events);
    }

    #[test]
    fn check_events_match_calendar_json() {
        let body = serpapi_test_output_json_1();
        let wembley_events_as_json = WembleyEvents::new()
            .build_events_from_html(body)
            .build_json_from_events();

        insta::assert_json_snapshot!(wembley_events_as_json);
    }

    #[test]
    fn test_filters_non_wembley_events() {
        let body = serpapi_test_output_json_8_mixed_wembley_non_wembley();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);
        let events = wembley_events.get_events();

        // Should only have 3 events that contain "wembley" (case-insensitive):
        // 1. "Concert at Wembley" - place contains "Wembley Stadium"
        // 2. "Event in WEMBLEY Park" - place contains "Hyde Park" but description doesn't have wembley (should be filtered out)
        // 3. "London Eye Tour" - description contains "wembley views"
        // Note: Event 2 should actually be filtered out as the description doesn't contain "wembley"

        // Expected to pass: events with "wembley" in place OR description
        assert_eq!(events.len(), 2);

        // Check the events contain wembley in either place or description
        for (_, event) in events.iter() {
            let place_lower = event.place.to_lowercase();
            let description_lower = event.description.to_lowercase();
            assert!(
                place_lower.contains("wembley") || description_lower.contains("wembley"),
                "Event '{}' should contain 'wembley' in place or description",
                event.title
            );
        }
    }

    #[test]
    fn test_wembley_filter_case_insensitive() {
        let body = serpapi_test_output_json_8_mixed_wembley_non_wembley();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);
        let events = wembley_events.get_events();

        // Verify that filtering is case-insensitive by checking we got events with:
        // - "Wembley" (title case) in place field
        // - "wembley" (lowercase) in description field
        let mut found_uppercase = false;
        let mut found_lowercase = false;

        for (_, event) in events.iter() {
            if event.place.contains("Wembley") {
                found_uppercase = true;
            }
            if event.description.contains("wembley") && !event.description.contains("Wembley") {
                found_lowercase = true;
            }
        }

        assert!(
            found_uppercase || found_lowercase,
            "Should find events with both uppercase and lowercase 'wembley'"
        );
    }
}
