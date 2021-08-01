extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name};
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
    document
        .find(Name("h3"))
        .for_each(|x| println!("{:?}", x.text()));
    for node in document.find(Class("brent_newEvent")) {
        let _event_header = node
            .find(Class("card-header"))
            .for_each(|x| println!("{:?}", x.text()));
        let _event_card_details = node
            .find(Class("brent_newEventDetails"))
            .for_each(|x| println!("{:?}", x.text()));
        let _new_event_mini_details = node.find(Class("col-lg-9")).for_each(|x| {
            println!(
                "{:?}",
                x.children().for_each(|y| println!("{:?}", y.text()))
            )
        });
    }
    Ok(())
}
