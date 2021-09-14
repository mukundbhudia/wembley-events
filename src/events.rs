#[derive(Debug)]
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
    pub fn date_to_ymd(&self) -> Ymd {
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
