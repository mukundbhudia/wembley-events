# wembley-events

[![Release](https://github.com/mukundbhudia/wembley-events/actions/workflows/release.yml/badge.svg)](https://github.com/mukundbhudia/wembley-events/actions/workflows/release.yml)
[![Rust](https://github.com/mukundbhudia/wembley-events/actions/workflows/rust.yml/badge.svg)](https://github.com/mukundbhudia/wembley-events/actions/workflows/rust.yml)
[![Trigger calendar generation on a CRON Schedule](https://github.com/mukundbhudia/wembley-events/actions/workflows/scheduled-builds.yml/badge.svg)](https://github.com/mukundbhudia/wembley-events/actions/workflows/scheduled-builds.yml)

Know when Wembley Stadium events are happening ahead of time. 
This project generates a calendar file on all the upcoming events at Wembley Stadium.
 - Don't get caught out in the traffic.
 - Don't struggle with parking.
 - Don't waste precious time.

Calendar always available here: **https://mukundbhudia.github.io/wembley-events/wembley-events.ics**.

Feel free to subscribe to the calendar using your favourite calendar app. The file output conforms to the [iCalendar](https://tools.ietf.org/html/rfc5545) specification.

## Pre-requisites

### Rust

- [Install Rust here](https://www.rust-lang.org/tools/install).
- Minimum Supported Rust Version (MSRV) is any 2018 edition.

## Development

- Copy the `.env.example` file to `.env` and edit the values to suit.
- Within the repo directory run `cargo r`.
    - This will generate a calendar file to the path defined in `CALENDAR_SAVE_PATH`.

## Production

- Copy the `.env.example` file to `.env` and edit the values to suit.
- To make a production build, within the repo directory run `cargo b --release`.
- To run the release build, within the project directory run `target/release/wembley-events` with the same arguments as in development.

## Testing

- To run **all** your tests, within the repo directory run `cargo t`. This will run all tests in the `/tests` directory.
- To run test a **specific** test such as `/tests/my_test.rs`, within the repo directory run `cargo t --test my_test`.

## Resources & Thanks

- [Brent Council events](https://www.brent.gov.uk/events-and-whats-on-calendar/).