use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct SerpapiDate {
    start_date: String,
    when: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct SerpapiEvent {
    title: String,
    description: String,
    date: SerpapiDate,
    link: String,
    thumbnail: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SerpapiEvents {
    events_results: Vec<SerpapiEvent>,
}