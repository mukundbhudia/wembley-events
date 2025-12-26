use std::process;

use wembley_events::{CalendarWriter, Config, EventOrchestrator};

#[tokio::main]
async fn main() {
    let config = Config::new().load_from_dotenv();

    // Fetch and merge events
    let orchestrator = EventOrchestrator::new(config.clone());
    let events = orchestrator.fetch_and_merge_events().await;

    // Write calendar files
    let calendar_writer = CalendarWriter::new(events);
    if let Err(e) = calendar_writer
        .write_calendar_to_file(&config.calendar_save_path, &config.calendar_json_save_path)
    {
        eprintln!("Failed to write calendar files: {:#?}.", e);
        process::exit(1);
    }

    println!("Executed successfully.");
}
