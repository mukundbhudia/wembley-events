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
    use crate::test_file_1;

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
        let wembley_events_to_test = wembley_events.clone();
        let calendar_built_from_events = wembley_events.build_calendar_from_events();

        let expected_events = vec![
            WembleyEvent {
                date: "26 June 2021".to_string(),
                time_and_place: "26 June 2021, 8pm to 12am, Wembley Stadium, Wembley, London HA9 0WS".to_string(),
                title: "Wembley Stadium event - Round of 16: Italy v Austria".to_string(),
                description: "Round of 16: 1A v 2C takes place on Saturday 26 June 2021. Kick off is at 8pm and parking restrictions will be in place until midnight.".to_string(),
            },
            WembleyEvent {
                date: "29 June 2021".to_string(),
                time_and_place: "29 June 2021, Kick off: 5pm, Wembley Stadium, Wembley, London HA9 0WS".to_string(),
                title: "Wembley Stadium event - Round of 16: England v Germany".to_string(),
                description: "Round of 16: 1D v 2F takes place on Tuesday 29 June 2021. Kick off is at 5pm and parking restrictions will be in place until midnight.".to_string(),
            },
            WembleyEvent {
                date: "06 July 2021".to_string(),
                time_and_place: "6 July 2021, 8pm to 12am, Wembley Stadium, Wembley, London HA9 0WS".to_string(),
                title: "Wembley Stadium event - UEFA Euro 2020: Italy v Spain".to_string(),
                description: "The first semi-final between Italy and Spain takes place on Tuesday 6 July 2021. Kick off is 8pm and parking restrictions will be in place until midnight.".to_string(),
            },
            WembleyEvent {
                date: "07 July 2021".to_string(),
                time_and_place: "7 July 2021, 8pm to 12am, Wembley Stadium, Wembley, London HA9 0WS".to_string(),
                title: "Wembley Stadium event - UEFA Euro 2020: England v Denmark".to_string(),
                description: "The second semi-final between England and Denmark takes place on Wednesday 7 July 2021. Kick off is 8pm and parking restrictions will be in place until midnight.".to_string(),
            },
            WembleyEvent {
                date: "17 July 2021".to_string(),
                time_and_place: "17 July 2021, Kick off: 12pm, Wembley Stadium, Wembley, London HA9 0WS".to_string(),
                title: "Wembley Stadium event - The Betfred Challenge Cup Final between Castleford Tigers and St Helens".to_string(),
                description: "The Betfred Challenge Cup Final between Castleford Tigers and St Helens will take place on Saturday 17 July 2021 at Wembley Stadium. Event day parking restrictions will be in place until midnight.".to_string(),
            },
            WembleyEvent {
                date: "07 August 2021".to_string(),
                time_and_place: "7 August 2021, 5pm to 12am".to_string(),
                title: "The FA community shield supported by McDonald's".to_string(),
                description: "The FA community shield supported by McDonald's: Manchester City v Leicester City".to_string(),
            },
            WembleyEvent {
                date: "05 December 2021".to_string(),
                time_and_place: "5 December 2021, TBC, Wembley Stadium, Wembley, London HA9 0WS".to_string(),
                title: "Wembley Stadium event - The women's FA cup final".to_string(),
                description: "The women's FA cup final takes place on Sunday 5 December 2021. Event day parking restrictions will be in place until midnight.".to_string(),
            },
        ];

        for (i, wembley_event) in wembley_events_to_test.events.into_iter() {
            assert_eq!(wembley_event.date, expected_events[i].date);
            assert_eq!(
                wembley_event.time_and_place,
                expected_events[i].time_and_place
            );
            assert_eq!(wembley_event.title, expected_events[i].title);
            assert_eq!(wembley_event.description, expected_events[i].description);
        }

        assert_eq!(calendar_built_from_events.len(), expected_events.len());
    }
}
