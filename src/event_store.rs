use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use chrono::NaiveDate;
use icalendar::*;

use crate::{TicketmasterResponse, WembleyEvent};

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
        let ticketmaster_response =
            serde_json::from_str::<TicketmasterResponse>(&html).unwrap_or_default();

        self.events = ticketmaster_response
            .embedded
            .events
            .into_iter()
            .filter_map(|event| {
                // Parse the ISO date format (2024-06-19) to "19 Jun 2024" format
                let date_formatted = if !event.dates.start.local_date.is_empty() {
                    Self::format_date_from_iso(&event.dates.start.local_date)
                } else {
                    return None; // Skip events without a date
                };

                // Format time if available
                let time = if !event.dates.start.local_time.is_empty() {
                    event.dates.start.local_time.clone()
                } else {
                    String::from("TBD")
                };

                // Get venue name
                let place = event
                    .embedded
                    .venues
                    .first()
                    .map(|v| v.name.clone())
                    .unwrap_or_default();

                // Use description field, with fallback to info and please_note
                let description = if !event.description.is_empty() {
                    event.description.clone()
                } else if !event.info.is_empty() && !event.please_note.is_empty() {
                    format!("{}. {}", event.info, event.please_note)
                } else if !event.info.is_empty() {
                    event.info.clone()
                } else if !event.please_note.is_empty() {
                    event.please_note.clone()
                } else {
                    String::new()
                };

                Some(WembleyEvent::new(
                    date_formatted,
                    time,
                    place,
                    event.name,
                    description,
                    event.url,
                ))
            })
            .filter(|e: &WembleyEvent| {
                let place_lower = e.place.to_lowercase();
                let description_lower = e.description.to_lowercase();
                let title_lower = e.title.to_lowercase();
                place_lower.contains("wembley")
                    || description_lower.contains("wembley")
                    || title_lower.contains("wembley")
            })
            // .inspect(|e: &WembleyEvent| println!("Event found...\n{:#}", e))
            .enumerate()
            .collect::<BTreeMap<usize, WembleyEvent>>();

        println!("Built {} events from req body.", self.events.len());

        Self {
            events: self.events,
        }
    }

    /// Converts ISO date format (2024-06-19) to "19 Jun 2024" format
    fn format_date_from_iso(iso_date: &str) -> String {
        let parts: Vec<&str> = iso_date.split('-').collect();
        if parts.len() != 3 {
            return iso_date.to_string();
        }

        let year = parts[0];
        let month = match parts[1] {
            "01" => "Jan",
            "02" => "Feb",
            "03" => "Mar",
            "04" => "Apr",
            "05" => "May",
            "06" => "Jun",
            "07" => "Jul",
            "08" => "Aug",
            "09" => "Sep",
            "10" => "Oct",
            "11" => "Nov",
            "12" => "Dec",
            _ => return iso_date.to_string(),
        };
        let day = parts[2].trim_start_matches('0');

        format!("{} {} {}", day, month, year)
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
    use crate::test_files::{ticketmaster_test_output_json_1, ticketmaster_test_output_json_empty};

    use super::*;

    #[test]
    fn build_events_from_html() {
        let body = ticketmaster_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        assert_eq!(wembley_events.get_events().len(), 2);
    }

    #[test]
    fn build_calendar_from_events() {
        let body = ticketmaster_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        let calendar = wembley_events.build_calendar_from_events();

        assert_eq!(calendar.len(), 2);
    }

    #[test]
    fn check_events_match_calendar() {
        let body = ticketmaster_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);
        let calendar_built_from_events = wembley_events.build_calendar_from_events();

        insta::assert_debug_snapshot!(calendar_built_from_events);
    }

    #[test]
    fn check_events_match_calendar_with_empty_response() {
        let body = ticketmaster_test_output_json_empty();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);
        let calendar_built_from_events = wembley_events.build_calendar_from_events();

        assert_eq!(calendar_built_from_events.len(), 0);
    }

    #[test]
    fn check_events_match_calendar_with_blank_string() {
        let body = String::new();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);
        let calendar_built_from_events = wembley_events.build_calendar_from_events();

        assert_eq!(calendar_built_from_events.len(), 0);
    }

    #[test]
    fn check_events_match_calendar_json() {
        let body = ticketmaster_test_output_json_1();
        let wembley_events_as_json = WembleyEvents::new()
            .build_events_from_html(body)
            .build_json_from_events();

        insta::assert_json_snapshot!(wembley_events_as_json);
    }

    #[test]
    fn test_format_date_from_iso() {
        assert_eq!(
            WembleyEvents::format_date_from_iso("2024-06-19"),
            "19 Jun 2024"
        );
        assert_eq!(
            WembleyEvents::format_date_from_iso("2024-01-05"),
            "5 Jan 2024"
        );
        assert_eq!(
            WembleyEvents::format_date_from_iso("2024-12-25"),
            "25 Dec 2024"
        );
    }

    #[test]
    fn test_format_date_from_iso_invalid() {
        // Invalid format should return the original string
        assert_eq!(WembleyEvents::format_date_from_iso("invalid"), "invalid");
        assert_eq!(
            WembleyEvents::format_date_from_iso("2024-13-01"),
            "2024-13-01"
        );
    }
}
