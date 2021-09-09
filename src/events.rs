use std::collections::BTreeMap;

use chrono::*;
use icalendar::*;
use select::document::Document;
use select::predicate::{Class, Name};

#[derive(Clone)]
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
            let ymd = event.date_to_ymd();
            let wembley_event = Event::new()
                .all_day(Utc.ymd(ymd.year, ymd.month, ymd.day))
                .summary(&event.title)
                .description(&event.description)
                .done();

            calendar.push(wembley_event);
        }

        calendar
    }
}

impl Default for WembleyEvents {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Ymd {
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Debug, Clone)]
pub struct WembleyEvent {
    date: String,
    time_and_place: String,
    title: String,
    description: String,
}

impl WembleyEvent {
    pub fn new(
        date: String,
        time_and_place: String,
        title: String,
        description: String,
    ) -> WembleyEvent {
        WembleyEvent {
            date,
            time_and_place,
            title,
            description,
        }
    }
    fn date_to_ymd(&self) -> Ymd {
        let date_str = self.date.split_whitespace().collect::<Vec<&str>>();
        let month_str = date_str[1];
        let month = match month_str {
            "January" => 1,
            "February" => 2,
            "March" => 3,
            "April" => 4,
            "May" => 5,
            "June" => 6,
            "July" => 7,
            "August" => 8,
            "September" => 9,
            "October" => 10,
            "November" => 11,
            "December" => 12,
            _ => panic!("Unknown month: {}", month_str),
        };

        Ymd {
            year: date_str[2].parse::<i32>().unwrap(),
            month,
            day: date_str[0].parse::<u32>().unwrap(),
        }
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
}
