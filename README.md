# wembley-events

![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/mukundbhudia/wembley-events)
[![Release](https://github.com/mukundbhudia/wembley-events/actions/workflows/release.yml/badge.svg)](https://github.com/mukundbhudia/wembley-events/actions/workflows/release.yml)
[![Rust](https://github.com/mukundbhudia/wembley-events/actions/workflows/rust.yml/badge.svg)](https://github.com/mukundbhudia/wembley-events/actions/workflows/rust.yml)
[![Trigger calendar generation on a CRON Schedule](https://github.com/mukundbhudia/wembley-events/actions/workflows/scheduled-builds.yml/badge.svg)](https://github.com/mukundbhudia/wembley-events/actions/workflows/scheduled-builds.yml)

Know when Wembley Stadium events are happening ahead of time.
This project generates a calendar file on all the upcoming events at Wembley Stadium.

- Don't get caught out in the traffic.
- Don't struggle with parking.
- Don't waste precious time.

The calendar is updated regularly and is always available to subscribe to/download here: **<https://mukundbhudia.github.io/wembley-events/wembley-events.ics>**.

In addition to the calendar, this project can also generate a JSON file containing the events available to download here: <https://mukundbhudia.github.io/wembley-events/wembley-events.json>.

The [wembley-events-web](https://github.com/mukundbhudia/wembley-events-web) project uses the events generated using this project to provide useful a frontend for the calendar.

GitHub Actions runs this project and uploads the calendar file to GitHub pages on a twice daily basis.

Feel free to subscribe to the calendar using your favourite calendar app. The file output conforms to the [RFC 5545 iCalendar](https://tools.ietf.org/html/rfc5545) specification.

## Pre-requisites

- [Install Rust here](https://www.rust-lang.org/tools/install).
- Minimum Supported Rust Version (MSRV) is any 2018 edition.

## Development

- Copy the `.env.example` file to `.env` and edit the values to suit.
- Within the repo directory run `cargo r`.
  - This will generate a calendar file to the path defined in `CALENDAR_SAVE_PATH`.
  - Setting a path for `CALENDAR_JSON_SAVE_PATH` will create a JSON file of events with the same name.
  - Optionally set `GH_PAGES_JSON_FILE_PATH` to enable historical event persistence from a local file (see below).
  - Optionally set `GH_PAGES_JSON_URL` to enable historical event persistence (see below).

## Production

- Copy the `.env.example` file to `.env` and edit the values to suit.
- To make a production build, within the repo directory run `cargo b --release`.
- To run the release build, within the project directory run `target/release/wembley-events` just as in development.

## Testing

- To run **all** your tests, within the repo directory run `cargo t`. This will run all tests in the `/tests` directory.
- To run **ignored** tests, within the repo directory run `cargo t -- --ignored`.

## Historical Event Persistence

This project now supports persisting older events that are no longer returned by the API. When enabled, the system will:

1. Fetch existing events from the previously published JSON file (gh-pages)
2. Filter to keep only future events (events on or after today's date)
3. Merge with newly fetched events from the API
4. Deduplicate based on title, date, and place (case-insensitive)
5. Sort by date and write the combined result

This ensures that events remain visible in the calendar even after they stop appearing in API responses, as long as they haven't occurred yet.

### Configuration

You can enable historical event persistence in two ways:

#### Option 1: Local File Path (Preferred)

Set the `GH_PAGES_JSON_FILE_PATH` environment variable to load from a local file:

```bash
export GH_PAGES_JSON_FILE_PATH="gh-pages-data/wembley-events.json"
```

Or add it to your `.env` file:

```bash
GH_PAGES_JSON_FILE_PATH=gh-pages-data/wembley-events.json
```

This is the preferred method for GitHub Actions, as it avoids network dependencies and fetches directly from the checked-out gh-pages branch.

#### Option 2: URL (Fallback)

Set the `GH_PAGES_JSON_URL` environment variable to fetch from a URL:

```bash
export GH_PAGES_JSON_URL="https://mukundbhudia.github.io/wembley-events/wembley-events.json"
```

Or add it to your `.env` file:

```bash
GH_PAGES_JSON_URL=https://mukundbhudia.github.io/wembley-events/wembley-events.json
```

**Priority:** If both are set, the local file path is tried first, with the URL as a fallback.

If neither variable is set, the system will behave as before, generating a fresh calendar from API results only.

## Resources & Thanks

- [SerpAPI](https://serpapi.com/).
