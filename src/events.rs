use std::collections::BTreeMap;

use select::document::Document;
use select::predicate::{Class, Name};

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
    pub fn build_calendar_from_html(mut self, html: String) -> WembleyEvents {
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
}

impl Default for WembleyEvents {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_calendar_from_file() {
        use std::fs;

        let body: String = fs::read_to_string("test/example1.html")
            .expect("unable to read file")
            .parse()
            .expect("enable to parse file as string");
        let wembley_events = WembleyEvents::new().build_calendar_from_html(body);

        assert_eq!(wembley_events.get_events().len(), 7);
    }
}
