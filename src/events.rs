#[derive(Debug, PartialEq, Eq)]
pub struct Ymd {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Clone)]
pub struct WembleyEvent {
    date: String,
    time_and_place: String,
    pub title: String,
    pub description: String,
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
                _ => 0,
            };

            let year = date_str[2].parse::<i32>().unwrap();
            let day = date_str[0].parse::<u32>().unwrap();

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
