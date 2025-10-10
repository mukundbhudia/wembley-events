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

#[cfg(test)]
mod tests {
    use crate::{
        WembleyEvents,
        test_files::{
            serpapi_test_output_json_1, serpapi_test_output_json_2, serpapi_test_output_json_4,
            serpapi_test_output_json_5, serpapi_test_output_json_6_missing_date,
            serpapi_test_output_json_7_one_bad_json_event,
        },
    };

    #[test]
    fn basic_serpapi_events_test() {
        let mock_serpapi_response = serpapi_test_output_json_1();

        let wembley_events = WembleyEvents::new().build_events_from_html(mock_serpapi_response);

        assert_eq!(wembley_events.events.len(), 10);
        insta::assert_debug_snapshot!(wembley_events);
    }

    #[test]
    fn empty_serpapi_events_test() {
        let mock_serpapi_response = serpapi_test_output_json_2();

        let wembley_events = WembleyEvents::new().build_events_from_html(mock_serpapi_response);

        assert_eq!(wembley_events.events.len(), 0);
        insta::assert_debug_snapshot!(wembley_events);
    }

    #[test]
    fn no_json_response_serpapi_events_test() {
        let mock_serpapi_response = String::new();

        let wembley_events = WembleyEvents::new().build_events_from_html(mock_serpapi_response);
        assert_eq!(wembley_events.events.len(), 0);
        insta::assert_debug_snapshot!(wembley_events);
    }

    #[test]
    fn no_events_in_response_serpapi_events_test() {
        let mock_serpapi_response = serpapi_test_output_json_4();

        let wembley_events = WembleyEvents::new().build_events_from_html(mock_serpapi_response);
        assert_eq!(wembley_events.events.len(), 0);
        insta::assert_debug_snapshot!(wembley_events);
    }

    #[test]
    fn bad_json_in_response_serpapi_events_test() {
        let mock_serpapi_response = serpapi_test_output_json_5();

        let wembley_events = WembleyEvents::new().build_events_from_html(mock_serpapi_response);
        assert_eq!(wembley_events.events.len(), 0);
        insta::assert_debug_snapshot!(wembley_events);
    }

    #[test]
    fn one_good_one_bad_event_serpapi_events_test() {
        let mock_serpapi_response = serpapi_test_output_json_6_missing_date();

        let wembley_events = WembleyEvents::new().build_events_from_html(mock_serpapi_response);
        assert_eq!(wembley_events.events.len(), 2);
        insta::assert_debug_snapshot!(wembley_events);
    }

    #[test]
    fn one_good_one_bad_json_event_serpapi_events_test() {
        let mock_serpapi_response = serpapi_test_output_json_7_one_bad_json_event();

        let wembley_events = WembleyEvents::new().build_events_from_html(mock_serpapi_response);
        assert_eq!(wembley_events.events.len(), 0);
        insta::assert_debug_snapshot!(wembley_events);
    }
}
