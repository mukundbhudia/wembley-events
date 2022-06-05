use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SerpapiDate {
    pub start_date: String,
    pub when: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SerpapiEvent {
    pub title: String,
    pub description: String,
    pub date: SerpapiDate,
    pub link: String,
    pub thumbnail: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SerpapiEvents {
    pub events_results: Vec<SerpapiEvent>,
}
