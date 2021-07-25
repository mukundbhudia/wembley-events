extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://www.brent.gov.uk/events-and-whats-on-calendar/?eventCat=Wembley+Stadium+events&startDate=01%2F01%2F2021&endDate=31%2F12%2F2029&count=50";

    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    Document::from(body.as_str())
        .find(Name("h3"))
        .for_each(|x| println!("{:?}", x.text()));
    Ok(())
}
