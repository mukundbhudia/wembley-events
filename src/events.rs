use serde::{Deserialize, Serialize};

use crate::SerpapiEvent;

#[derive(Debug, PartialEq, Eq)]
pub struct Ymd {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WembleyEvent {
    pub date: String,
    pub time_and_place: String,
    pub title: String,
    pub description: String,
}

impl From<SerpapiEvent> for WembleyEvent {
    fn from(serp_event: SerpapiEvent) -> Self {
        Self {
            date: serp_event.date.start_date,
            time_and_place: "TODO".to_string(), // TODO: need to update place
            title: serp_event.title,
            description: serp_event.description,
        }
    }
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

    pub fn date_to_ymd(&self) -> Option<Ymd> {
        let date_str = self.date.split_whitespace().collect::<Vec<&str>>();
        if date_str.len() == 3 {
            let month_str = date_str[0];
            let month = match month_str {
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
                _ => 0,
            };

            let year = date_str[2].parse::<i32>().unwrap();
            let day = date_str[1].parse::<u32>().unwrap();

            if month != 0 && (day > 0 && day <= 31) && (year > 1900 && year < 3000) {
                Some(Ymd { year, month, day })
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_event() {
        let wembley_event = WembleyEvent::new(
            "5 June 2021".to_string(),
            "Somewhere".to_string(),
            "Title".to_string(),
            "description".to_string(),
        );
        assert_eq!(
            wembley_event.date_to_ymd(),
            Some(Ymd {
                year: 2021,
                month: 6,
                day: 5
            })
        );
    }

    #[test]
    fn test_make_event_bad_month() {
        let wembley_event = WembleyEvent::new(
            "5 Test 2021".to_string(),
            "Somewhere".to_string(),
            "Title".to_string(),
            "description".to_string(),
        );
        assert_eq!(wembley_event.date_to_ymd(), None);
    }

    #[test]
    fn test_make_event_bad_date_string() {
        let wembley_event = WembleyEvent::new(
            "sdfsdfgf".to_string(),
            "Somewhere".to_string(),
            "Title".to_string(),
            "description".to_string(),
        );
        assert_eq!(wembley_event.date_to_ymd(), None);
    }

    #[test]
    fn test_make_event_bad_date_extra_item() {
        let wembley_event = WembleyEvent::new(
            "5 June 2021 5 June 2021".to_string(),
            "Somewhere".to_string(),
            "Title".to_string(),
            "description".to_string(),
        );
        assert_eq!(wembley_event.date_to_ymd(), None);
    }

    #[test]
    fn test_make_event_bad_date_reversed() {
        let wembley_event = WembleyEvent::new(
            "2021 June 5".to_string(),
            "Somewhere".to_string(),
            "Title".to_string(),
            "description".to_string(),
        );
        assert_eq!(wembley_event.date_to_ymd(), None);
    }
}
