use std::error::Error;

use wembley_events::{HttpClient, WembleyEvents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let body = HttpClient::new("https://www.brent.gov.uk/events-and-whats-on-calendar/?eventCat=Wembley+Stadium+events&startDate=01%2F01%2F2021&endDate=31%2F12%2F2029&count=50")
        .get_text_from_url()
        .await?.body;

    let wembley_events_calendar = WembleyEvents::new()
        .build_events_from_html(body)
        .build_calendar_from_events();
    // TODO: this would be better to output as a string for more options of output
    wembley_events_calendar
        .print()
        .expect("Could not print calendar to stdout");

    Ok(())
}
