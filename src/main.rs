use std::process;

use wembley_events::{CalendarWriter, Config, HttpClient, WembleyEvents};

#[tokio::main]
async fn main() {
    let config = Config::new().load_from_dotenv();

    if let Ok(res) = HttpClient::new(&config.calendar_url)
        .get_text_from_url()
        .await
    {
        let wembley_events_calendar = WembleyEvents::new()
            .build_events_from_html(res.body)
            .build_calendar_from_events();

        if CalendarWriter::new(wembley_events_calendar)
            .write_calendar_to_file(&config.calendar_save_path)
            .is_err()
        {
            process::exit(1);
        }
    } else {
        process::exit(1);
    }
}
