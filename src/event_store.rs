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
                swapped_date.push(&year);
                swapped_date.swap(0, 1); // swap [month, day] to [day, month]

                e.date.start_date = swapped_date.join(" ");
                e.into()
            })
            .enumerate()
            .collect::<BTreeMap<usize, WembleyEvent>>();

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
    use crate::{serpapi_test_output_json_1, serpapi_test_output_json_2};

    use super::*;

    #[test]
    fn build_events_from_html() {
        let body = serpapi_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        assert_eq!(wembley_events.get_events().len(), 10);
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
}
