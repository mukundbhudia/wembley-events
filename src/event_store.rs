use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use chrono::*;
use icalendar::*;
use select::document::Document;
use select::predicate::{Class, Name};

use crate::WembleyEvent;

#[derive(Clone, Serialize, Deserialize)]
pub struct WembleyEvents {
    events: BTreeMap<usize, WembleyEvent>,
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
        let document = Document::from(html.as_str());
        let event_dates_iter = document.find(Name("h3")).map(|x| x.text());

        for ((i, node), event_date) in document
            .find(Class("brent_newEvent"))
            .enumerate()
            .zip(event_dates_iter)
        {
            let event_title = node
                .find(Class("card-header"))
                .map(|x| x.text())
                .collect::<String>();
            let event_time_and_place = node
                .find(Class("brent_newEventDetails"))
                .map(|x| x.text())
                .collect::<String>();
            let event_description = node
                .find(Class("col-lg-9"))
                .map(|x| x.children().skip(1).map(|y| y.text()).collect::<String>())
                .collect::<String>();

            let event = WembleyEvent::new(
                event_date,
                event_time_and_place,
                event_title,
                event_description,
            );

            self.events.insert(i, event);
        }

        Self {
            events: self.events,
        }
    }

    pub fn build_calendar_from_events(self) -> Calendar {
        let mut calendar = Calendar::new();

        for (_, event) in self.events {
            if let Some(ymd) = event.date_to_ymd() {
                let wembley_event = Event::new()
                    .all_day(Utc.ymd(ymd.year, ymd.month, ymd.day))
                    .summary(&event.title)
                    .description(&event.description)
                    .done();

                calendar.push(wembley_event);
            };
        }

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
    use crate::{test_file_1, test_file_2};

    use super::*;

    #[test]
    fn build_events_from_html() {
        let body = test_file_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        assert_eq!(wembley_events.get_events().len(), 7);
    }

    #[test]
    fn build_calendar_from_events() {
        let body = test_file_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        let calendar = wembley_events.build_calendar_from_events();

        assert_eq!(calendar.len(), 7);
    }

    #[test]
    fn check_events_match_calendar() {
        let body = test_file_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);
        let calendar_built_from_events = wembley_events.build_calendar_from_events();

        insta::assert_debug_snapshot!(calendar_built_from_events);
    }

    #[test]
    fn check_events_match_calendar_with_blank_html() {
        let body = test_file_2();
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
        let body = test_file_1();
        let wembley_events_as_json = WembleyEvents::new()
            .build_events_from_html(body)
            .build_json_from_events();

        println!("{}", wembley_events_as_json);

        insta::assert_json_snapshot!(wembley_events_as_json);
    }
}
