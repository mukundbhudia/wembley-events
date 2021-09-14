use icalendar::Calendar;
use std::{fs, process};

pub struct CalendarWriter {
    calendar: Calendar,
}

impl CalendarWriter {
    pub fn new(calendar: Calendar) -> Self {
        CalendarWriter { calendar }
    }

    pub fn write_calendar_to_file(self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.calendar.is_empty() {
            eprintln!(
                "Unable to write calendar to file. Calendar has no events. Check if the HTML structure has changed."
            );
            process::exit(1);
        } else {
            fs::write(path, self.calendar.to_string())
                .expect("IO error. Unable to write calendar to file");
            Ok(())
        }
    }
}
