use std::process;

use wembley_events::{CalendarWriter, Config, HttpClient, WembleyEvents};

#[tokio::main]
async fn main() {
    let config = Config::new().load_from_dotenv();

    if let Ok(res) = HttpClient::new(&config.calendar_url)
        .get_text_from_url()
        .await
    {
        let wembley_events = WembleyEvents::new().build_events_from_html(res.body);

        let wembley_events_json = wembley_events.clone().build_json_from_events();

        let calendar_writer = CalendarWriter::new(wembley_events);

        if calendar_writer
            .write_calendar_to_file(&config.calendar_save_path)
            .is_err()
        {
            process::exit(1);
        }

        if let Some(json_path) = &config.calendar_json_save_path {
            if wembley_events::CalendarWriter::write_json_to_file(wembley_events_json, json_path)
                .is_err()
            {
                process::exit(1);
            }
        } else {
            println!("No JSON calendar path found in config. Not creating JSON file.");
        }
    } else {
        process::exit(1);
    }
}
