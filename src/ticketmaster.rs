use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TicketmasterPage {
    #[serde(default)]
    pub size: u32,
    #[serde(rename = "totalElements", default)]
    pub total_elements: u32,
    #[serde(rename = "totalPages", default)]
    pub total_pages: u32,
    #[serde(default)]
    pub number: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TicketmasterImage {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TicketmasterStartDate {
    #[serde(rename = "localDate", default)]
    pub local_date: String,
    #[serde(rename = "localTime", default)]
    pub local_time: String,
    #[serde(rename = "dateTime", default)]
    pub date_time: String,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TicketmasterDates {
    #[serde(default)]
    pub start: TicketmasterStartDate,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TicketmasterCity {
    #[serde(default)]
    pub name: String,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TicketmasterVenue {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub city: TicketmasterCity,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TicketmasterEventEmbedded {
    #[serde(default)]
    pub venues: Vec<TicketmasterVenue>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TicketmasterEvent {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub images: Vec<TicketmasterImage>,
    #[serde(default)]
    pub dates: TicketmasterDates,
    #[serde(rename = "_embedded", default)]
    pub embedded: TicketmasterEventEmbedded,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub info: String,
    #[serde(rename = "pleaseNote", default)]
    pub please_note: String,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TicketmasterEventsEmbedded {
    #[serde(default)]
    pub events: Vec<TicketmasterEvent>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TicketmasterResponse {
    #[serde(rename = "_embedded", default)]
    pub embedded: TicketmasterEventsEmbedded,
    #[serde(default)]
    pub page: TicketmasterPage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticketmaster_response_deserialization() {
        let json = r#"{
            "_embedded": {
                "events": [
                    {
                        "name": "Test Event",
                        "id": "test-id",
                        "url": "https://www.ticketmaster.com/test",
                        "images": [
                            {
                                "url": "https://example.com/image.jpg",
                                "width": 1024,
                                "height": 576
                            }
                        ],
                        "dates": {
                            "start": {
                                "localDate": "2024-06-19",
                                "localTime": "19:30:00",
                                "dateTime": "2024-06-19T18:30:00Z"
                            }
                        },
                        "_embedded": {
                            "venues": [
                                {
                                    "name": "Wembley Stadium",
                                    "city": {
                                        "name": "London"
                                    }
                                }
                            ]
                        },
                        "description": "Test event description",
                        "info": "Event info",
                        "pleaseNote": "Please note"
                    }
                ]
            },
            "page": {
                "size": 20,
                "totalElements": 1,
                "totalPages": 1,
                "number": 0
            }
        }"#;

        let response: TicketmasterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.embedded.events.len(), 1);
        assert_eq!(response.embedded.events[0].name, "Test Event");
        assert_eq!(response.page.total_elements, 1);
    }

    #[test]
    fn test_empty_ticketmaster_response() {
        let json = r#"{
            "_embedded": {
                "events": []
            },
            "page": {
                "size": 20,
                "totalElements": 0,
                "totalPages": 0,
                "number": 0
            }
        }"#;

        let response: TicketmasterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.embedded.events.len(), 0);
    }

    #[test]
    fn test_ticketmaster_response_with_defaults() {
        let json = r#"{}"#;
        let response: TicketmasterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.embedded.events.len(), 0);
    }
}
