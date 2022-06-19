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

#[cfg(test)]
mod tests {
    use crate::{
        serpapi_test_output_json_1, serpapi_test_output_json_2,
        test_files::{
            serpapi_test_output_json_3, serpapi_test_output_json_4,
            serpapi_test_output_json_5_missing_date, serpapi_test_output_json_6_one_bad_json_event,
        },
        WembleyEvents,
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
        let mock_serpapi_response = serpapi_test_output_json_3();

        let wembley_events = WembleyEvents::new().build_events_from_html(mock_serpapi_response);
        assert_eq!(wembley_events.events.len(), 0);
        insta::assert_debug_snapshot!(wembley_events);
    }

    #[test]
    fn bad_json_in_response_serpapi_events_test() {
        let mock_serpapi_response = serpapi_test_output_json_4();

        let wembley_events = WembleyEvents::new().build_events_from_html(mock_serpapi_response);
        assert_eq!(wembley_events.events.len(), 0);
        insta::assert_debug_snapshot!(wembley_events);
    }

    #[test]
    fn one_good_one_bad_event_serpapi_events_test() {
        let mock_serpapi_response = serpapi_test_output_json_5_missing_date();

        let wembley_events = WembleyEvents::new().build_events_from_html(mock_serpapi_response);
        assert_eq!(wembley_events.events.len(), 0);
        insta::assert_debug_snapshot!(wembley_events);
    }

    #[test]
    fn one_good_one_bad_json_event_serpapi_events_test() {
        let mock_serpapi_response = serpapi_test_output_json_6_one_bad_json_event();

        let wembley_events = WembleyEvents::new().build_events_from_html(mock_serpapi_response);
        assert_eq!(wembley_events.events.len(), 0);
        insta::assert_debug_snapshot!(wembley_events);
    }
}
