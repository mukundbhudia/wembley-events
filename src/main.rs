extern crate reqwest;

use std::error::Error;

use wembley_events::WembleyEvents;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: uncomment below for live data
    let url = "https://www.brent.gov.uk/events-and-whats-on-calendar/?eventCat=Wembley+Stadium+events&startDate=01%2F01%2F2021&endDate=31%2F12%2F2029&count=50";
    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    let wembley_events = WembleyEvents::new().build_calendar_from_html(body);

    println!("{:#?}", &wembley_events.get_events());

    Ok(())
}
