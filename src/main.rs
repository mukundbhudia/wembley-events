use std::process;


use wembley_events::{CalendarWriter, Config, HttpClient, WembleyEvents, serpapi_test_output_json_1, SerpapiEvents};

#[tokio::main]
async fn main() {
    let config = Config::new().load_from_dotenv();
    // let full_url = format!("{}{}", &config.calendar_url, &config.serpapi_api_key);
    let full_url = "https://httpbin.org/get".to_string();
    println!("{}", full_url);
    let test_body = serpapi_test_output_json_1();
    // println!("test_body: {:?}", test_body);

    if let Ok(res) = HttpClient::new(&full_url)
        .get_text_from_url()
        .await
    {
        println!("res.body: {:?}", res.body);
        println!("serde: {:?}", serde_json::from_str::<SerpapiEvents>(&test_body));
        // assert_eq!(test_body, res.body);
        let wembley_events = WembleyEvents::new().build_events_from_html(res.body);
        let calendar_writer = CalendarWriter::new(wembley_events);

        if calendar_writer
            .write_calendar_to_file(&config.calendar_save_path, &config.calendar_json_save_path)
            .is_err()
        {
            process::exit(1);
        }
    } else {
        process::exit(1);
    }
}
