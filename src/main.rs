use std::error::Error;

use wembley_events::{HttpClient, WembleyEvents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let body = HttpClient::new("https://www.brent.gov.uk/events-and-whats-on-calendar/?eventCat=Wembley+Stadium+events&startDate=01%2F01%2F2021&endDate=31%2F12%2F2029&count=50")
        .get_text_from_url()
        .await?;
    let wembley_events = WembleyEvents::new().build_calendar_from_html(body);

    println!("{:#?}", &wembley_events.get_events());

    Ok(())
}
