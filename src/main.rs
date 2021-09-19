use std::process;

use wembley_events::{CalendarWriter, HttpClient, WembleyEvents};

#[tokio::main]
async fn main() {
    let calendar_url = "https://www.brent.gov.uk/events-and-whats-on-calendar/?eventCat=Wembley+Stadium+events&startDate=01%2F01%2F2021&endDate=31%2F12%2F2029&count=50";

    if let Ok(res) = HttpClient::new(calendar_url).get_text_from_url().await {
        let wembley_events_calendar = WembleyEvents::new()
            .build_events_from_html(res.body)
            .build_calendar_from_events();

        if CalendarWriter::new(wembley_events_calendar)
            .write_calendar_to_file("output/wembley-events.ics")
            .is_err()
        {
            process::exit(1);
        }
    } else {
        process::exit(1);
    }
}
