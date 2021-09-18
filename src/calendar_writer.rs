use icalendar::Calendar;
use std::fs;

pub struct CalendarWriter {
    calendar: Calendar,
}

impl CalendarWriter {
    pub fn new(calendar: Calendar) -> Self {
        CalendarWriter { calendar }
    }

    pub fn write_calendar_to_file(self, path: &str) -> Result<(), CalendarWriterError> {
        if self.calendar.is_empty() {
            eprintln!("Unable to write calendar to file. Calendar has no events. Check if the HTML structure has changed.");
            Err(CalendarWriterError::EmptyCalendar)
        } else {
            match fs::write(path, self.calendar.to_string()) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Unable to write calendar to file. {:#?}", e);
                    Err(CalendarWriterError::CannotWriteToFile)
                }
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CalendarWriterError {
    EmptyCalendar,
    CannotWriteToFile,
}

#[cfg(test)]
mod tests {
    use crate::{test_file_1, test_file_2, WembleyEvents};

    use super::*;

    #[test]
    fn test_calendar_writer_empty_calendar_to_write() {
        let calendar = Calendar::new();
        let writer = CalendarWriter::new(calendar).write_calendar_to_file("output/test_fail.ics");

        assert_eq!(writer, Err(CalendarWriterError::EmptyCalendar));
    }

    #[test]
    fn test_calendar_writer_bad_path_to_write() {
        let body = test_file_1();
        let wembley_events = WembleyEvents::new()
            .build_events_from_html(body)
            .build_calendar_from_events();

        let writer =
            CalendarWriter::new(wembley_events).write_calendar_to_file("/output/test_fail.ics");

        assert_eq!(writer, Err(CalendarWriterError::CannotWriteToFile));
    }

    #[test]
    fn test_calendar_writer_empty_html_calendar_to_write() {
        let body = test_file_2();
        let wembley_events = WembleyEvents::new()
            .build_events_from_html(body)
            .build_calendar_from_events();

        let writer =
            CalendarWriter::new(wembley_events).write_calendar_to_file("/output/test_fail.ics");

        assert_eq!(writer, Err(CalendarWriterError::EmptyCalendar));
    }

    #[test]
    fn test_calendar_writer() {
        let body = test_file_1();
        let wembley_events = WembleyEvents::new()
            .build_events_from_html(body)
            .build_calendar_from_events();

        let writer =
            CalendarWriter::new(wembley_events).write_calendar_to_file("output/test_run.ics");

        assert_eq!(writer, Ok(()));
    }
}
