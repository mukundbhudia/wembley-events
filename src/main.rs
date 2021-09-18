use std::{error::Error, process};

use wembley_events::{CalendarWriter, HttpClient, WembleyEvents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let body = HttpClient::new("https://www.brent.gov.uk/")
        .get_text_from_url()
        .await?
        .body;

    let wembley_events_calendar = WembleyEvents::new()
        .build_events_from_html(body)
        .build_calendar_from_events();

    if CalendarWriter::new(wembley_events_calendar)
        .write_calendar_to_file("output/wembley-events.ics")
        .is_err()
    {
        process::exit(1);
    }

    Ok(())
}
