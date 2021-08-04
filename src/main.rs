extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: uncomment below for live data
    // let url = "https://www.brent.gov.uk/events-and-whats-on-calendar/?eventCat=Wembley+Stadium+events&startDate=01%2F01%2F2021&endDate=31%2F12%2F2029&count=50";
    // let res = reqwest::get(url).await?;
    // let body = res.text().await?;

    // TODO: move file read to test
    let body: String = fs::read_to_string("test/example1.html")?.parse()?;

    let document = Document::from(body.as_str());
    let event_dates_iter = document.find(Name("h3")).map(|x| x.text());

    let mut events: BTreeMap<usize, WembleyEvent> = BTreeMap::new(); // TODO: use BTreeSet?

    for ((i, node), event_date) in document
        .find(Class("brent_newEvent"))
        .enumerate()
        .zip(event_dates_iter)
    {
        let event_title = node
            .find(Class("card-header"))
            .map(|x| x.text())
            .collect::<String>();
        let event_time_and_place = node
            .find(Class("brent_newEventDetails"))
            .map(|x| x.text())
            .collect::<String>();
        let event_description = node
            .find(Class("col-lg-9"))
            .map(|x| x.children().skip(1).map(|y| y.text()).collect::<String>())
            .collect::<String>();

        let event = WembleyEvent::new(
            event_date,
            event_time_and_place,
            event_title,
            event_description,
        );

        events.insert(i, event);
    }

    println!("{:#?}", events);

    Ok(())
}

#[derive(Debug)]
struct WembleyEvent {
    date: String,
    time_and_place: String,
    title: String,
    description: String,
}

impl WembleyEvent {
    fn new(
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
}
