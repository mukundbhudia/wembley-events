use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SerpapiSearchMetaData {
    #[serde(default)]
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SerpapiDate {
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub when: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SerpapiVenue {
    #[serde(default)]
    pub name: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SerpapiEvent {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub date: SerpapiDate,
    #[serde(default)]
    pub link: String,
    #[serde(default)]
    pub venue: SerpapiVenue,
    #[serde(default)]
    pub thumbnail: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SerpapiEvents {
    pub search_metadata: SerpapiSearchMetaData,
    pub events_results: Vec<SerpapiEvent>,
}

// Note: These tests are now obsolete as the project has migrated from SerpAPI to Ticketmaster Discovery API.
// The SerpAPI data structures are kept for reference but are no longer used in the codebase.
// See ticketmaster.rs for the new API structures and event_store.rs for updated tests.
