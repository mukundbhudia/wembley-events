use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SerpapiSearchMetaData {
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SerpapiDate {
    pub start_date: String,
    pub when: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SerpapiVenue {
    pub name: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SerpapiEvent {
    pub title: String,
    pub description: String,
    pub date: SerpapiDate,
    pub link: String,
    pub venue: SerpapiVenue,
    pub thumbnail: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SerpapiEvents {
    pub search_metadata: SerpapiSearchMetaData,
    pub events_results: Vec<SerpapiEvent>,
}
