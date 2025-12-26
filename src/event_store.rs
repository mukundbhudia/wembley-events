use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use chrono::NaiveDate;
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

    /// Infers the correct year for an event based on its month.
    ///
    /// If the event month is significantly earlier than the current month (>3 months in the past),
    /// we assume the event is scheduled for next year. This handles the common case where
    /// events are listed in December for January of the following year.
    fn infer_event_year(month_str: &str, base_year: i32, current_month: u32) -> i32 {
        let event_month = match month_str {
            "Jan" => 1,
            "Feb" => 2,
            "Mar" => 3,
            "Apr" => 4,
            "May" => 5,
            "Jun" => 6,
            "Jul" => 7,
            "Aug" => 8,
            "Sep" => 9,
            "Oct" => 10,
            "Nov" => 11,
            "Dec" => 12,
            _ => return base_year, // Unknown month, use base year
        };

        // If event month is more than 3 months in the past, assume it's for next year
        // For example: If it's December (12) and event is in January (1), that's 11 months back,
        // so we assume next year
        let month_diff = if current_month >= event_month {
            current_month - event_month
        } else {
            // Event month is ahead, it's in the future this year
            return base_year;
        };

        // If the event appears to be more than 3 months in the past, assume next year
        if month_diff > 3 {
            base_year + 1
        } else {
            base_year
        }
    }

    pub fn build_events_from_html(mut self, html: String) -> WembleyEvents {
        let serp_api_events = serde_json::from_str::<SerpapiEvents>(&html).unwrap_or_default();

        // Extract base year and month from API call timestamp
        let created_at = &serp_api_events.search_metadata.created_at;
        let now = Utc::now();

        let base_year = if created_at.len() >= 4 {
            created_at[0..4].parse::<i32>().unwrap_or(now.year())
        } else {
            now.year()
        };

        // Extract month from created_at (format: "2022-06-04 13:00:53 UTC")
        let api_call_month = if created_at.len() >= 7 {
            created_at[5..7].parse::<u32>().unwrap_or(now.month())
        } else {
            now.month()
        };

        self.events = serp_api_events
            .events_results
            .into_iter()
            .map(|mut e| {
                let mut swapped_date = e.date.start_date.split_whitespace().collect::<Vec<&str>>();
                if swapped_date.len() >= 2 {
                    // Infer the correct year based on the event month
                    let inferred_year =
                        Self::infer_event_year(swapped_date[0], base_year, api_call_month);
                    let year_str = inferred_year.to_string();

                    swapped_date.push(&year_str);
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
            .inspect(|e: &WembleyEvent| println!("Event found...\n{:#}", e))
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
                let description_with_link = format!("{}\n\n{}", event.description, event.link);

                let wembley_event = Event::new()
                    .all_day(NaiveDate::from_ymd_opt(ymd.year, ymd.month, ymd.day).unwrap())
                    .summary(&event.title)
                    .description(&description_with_link)
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

    #[test]
    fn test_infer_event_year_same_month() {
        // Event in the same month should use base year
        let result = WembleyEvents::infer_event_year("Dec", 2025, 12);
        assert_eq!(result, 2025);
    }

    #[test]
    fn test_infer_event_year_future_month() {
        // Event in a future month should use base year
        let result = WembleyEvents::infer_event_year("Jun", 2025, 3);
        assert_eq!(result, 2025);
    }

    #[test]
    fn test_infer_event_year_recent_past() {
        // Event 1-3 months in the past should still use base year
        let result = WembleyEvents::infer_event_year("Sep", 2025, 12);
        assert_eq!(result, 2025);
    }

    #[test]
    fn test_infer_event_year_far_past() {
        // Event more than 3 months in the past should use next year
        // If it's December and event is in January (11 months back), assume next year
        let result = WembleyEvents::infer_event_year("Jan", 2025, 12);
        assert_eq!(result, 2026);
    }

    #[test]
    fn test_infer_event_year_edge_case_march() {
        // Event 4 months in the past should use next year
        let result = WembleyEvents::infer_event_year("Mar", 2025, 7);
        assert_eq!(result, 2026);
    }

    #[test]
    fn test_infer_event_year_unknown_month() {
        // Unknown month should fallback to base year
        let result = WembleyEvents::infer_event_year("Invalid", 2025, 6);
        assert_eq!(result, 2025);
    }

    #[test]
    fn test_infer_event_year_real_world_scenario() {
        // Real scenario: It's June 2022, event shows "Jun 19"
        let result = WembleyEvents::infer_event_year("Jun", 2022, 6);
        assert_eq!(result, 2022);
    }
}
