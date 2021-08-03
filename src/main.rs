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
    let _event_dates = document
        .find(Name("h3"))
        .map(|x| x.text())
        .collect::<Vec<String>>();

    println!("evt dates {:?}", _event_dates);

    for (_i, node) in document.find(Class("brent_newEvent")).enumerate() {
        let _event_header = node
            .find(Class("card-header"))
            .map(|x| x.text())
            .collect::<String>();
        let _event_card_details = node
            .find(Class("brent_newEventDetails"))
            .map(|x| x.text())
            .collect::<String>();
        let _new_event_mini_details = node
            .find(Class("col-lg-9"))
            .map(|x| x.children().skip(1).map(|y| y.text()).collect::<String>())
            .collect::<String>();

        println!("evt {} card header: {}", _i, _event_header);
        println!("evt {} card details: {}", _i, _event_card_details);
        println!("evt {} mini details: {}", _i, _new_event_mini_details);
    }

    Ok(())
}
