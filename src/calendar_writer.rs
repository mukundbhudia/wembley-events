use icalendar::Calendar;
use std::fs;

use crate::WembleyEvents;

pub struct CalendarWriter {
    calendar: Calendar,
    events: WembleyEvents,
}

impl CalendarWriter {
    pub fn new(events: WembleyEvents) -> Self {
        CalendarWriter {
            calendar: events.clone().build_calendar_from_events(),
            events,
        }
    }

    pub fn write_calendar_to_file(
        self,
        calendar_path: &str,
        json_calendar_path: &Option<String>,
    ) -> Result<(), CalendarWriterError> {
        use super::*;

        if self.calendar.is_empty() {
            eprintln!(
                "Unable to write calendar to file. Calendar has no events. Check if the HTML structure has changed."
            );
            Err(CalendarWriterError::EmptyCalendar)
        } else {
            CalendarWriter::write_string_to_file(self.calendar.to_string(), calendar_path)?;
            if let Some(json_path) = json_calendar_path {
                CalendarWriter::write_string_to_file(
                    self.events.build_json_from_events(),
                    json_path,
                )
            } else {
                println!("No JSON calendar path found in config. Skipping creating JSON file.");
                Ok(())
            }
        }
    }

    pub fn write_string_to_file(
        file_as_string: String,
        path: &str,
    ) -> Result<(), CalendarWriterError> {
        let path = std::path::Path::new(path);

        if let Some(prefix) = path.parent() {
            if file_as_string.is_empty() {
                eprintln!(
                    "Unable to write JSON to file. JSON has no events. Check if the HTML structure has changed."
                );
                Err(CalendarWriterError::EmptyCalendar)
            } else if let Err(e) = fs::create_dir_all(prefix) {
                eprintln!("Unable to directory for calendar. {:#?}", e);
                Err(CalendarWriterError::CannotCreateDirectory)
            } else if let Err(e) = fs::write(path, file_as_string) {
                eprintln!("Unable to write JSON to file. {:#?}", e);
                Err(CalendarWriterError::CannotWriteToFile)
            } else {
                Ok(())
            }
        } else {
            eprintln!("Unable to write JSON to file. Bad path given.");
            Err(CalendarWriterError::BadFilePath)
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CalendarWriterError {
    EmptyCalendar,
    CannotWriteToFile,
    CannotCreateDirectory,
    BadFilePath,
}

#[cfg(test)]
mod tests {
    use crate::{
        WembleyEvents,
        test_files::{serpapi_test_output_json_1, serpapi_test_output_json_2},
    };

    use super::*;

    #[test]
    fn test_calendar_writer_empty_calendar_to_write() {
        let events = WembleyEvents::new();

        let writer =
            CalendarWriter::new(events).write_calendar_to_file("output/test_fail.ics", &None);

        assert_eq!(writer, Err(CalendarWriterError::EmptyCalendar));
    }

    #[test]
    fn test_calendar_writer_bad_path_to_write() {
        let body = serpapi_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        let writer = CalendarWriter::new(wembley_events)
            .write_calendar_to_file("/output/test_fail.ics", &None);

        assert_eq!(writer, Err(CalendarWriterError::CannotCreateDirectory));
    }

    #[test]
    fn test_calendar_writer_empty_html_calendar_to_write() {
        let body = serpapi_test_output_json_2();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        let writer = CalendarWriter::new(wembley_events)
            .write_calendar_to_file("output/test_fail.ics", &None);

        assert_eq!(writer, Err(CalendarWriterError::EmptyCalendar));
    }

    #[test]
    fn test_calendar_writer() {
        let body = serpapi_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        let writer = CalendarWriter::new(wembley_events)
            .write_calendar_to_file("output/test_run.ics", &None);

        assert_eq!(writer, Ok(()));
    }

    #[test]
    fn test_calendar_writer_nested_directory() {
        let body = serpapi_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        let writer = CalendarWriter::new(wembley_events)
            .write_calendar_to_file("output/test1/test2/test_run.ics", &None);

        assert_eq!(writer, Ok(()));
    }

    #[test]
    fn test_calendar_writer_with_json() {
        let body = serpapi_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        let writer = CalendarWriter::new(wembley_events)
            .write_calendar_to_file("output/test_run.ics", &Some("output/test_run.json".into()));

        assert_eq!(writer, Ok(()));
    }

    #[test]
    fn test_calendar_writer_with_json_bad_json_path() {
        let body = serpapi_test_output_json_1();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        let writer = CalendarWriter::new(wembley_events)
            .write_calendar_to_file("output/test_run.ics", &Some("/output/test_run.json".into()));

        assert_eq!(writer, Err(CalendarWriterError::CannotCreateDirectory));
    }

    #[test]
    fn test_calendar_writer_empty_html_and_json_calendar_to_write() {
        let body = serpapi_test_output_json_2();
        let wembley_events = WembleyEvents::new().build_events_from_html(body);

        let writer = CalendarWriter::new(wembley_events).write_calendar_to_file(
            "output/test_fail.ics",
            &Some("output/test_fail.json".into()),
        );

        assert_eq!(writer, Err(CalendarWriterError::EmptyCalendar));
    }
}
