# wembley-events

[![Release](https://github.com/mukundbhudia/wembley-events/actions/workflows/release.yml/badge.svg)](https://github.com/mukundbhudia/wembley-events/actions/workflows/release.yml)
[![Rust](https://github.com/mukundbhudia/wembley-events/actions/workflows/rust.yml/badge.svg)](https://github.com/mukundbhudia/wembley-events/actions/workflows/rust.yml)
[![Trigger calendar generation on a CRON Schedule](https://github.com/mukundbhudia/wembley-events/actions/workflows/scheduled-builds.yml/badge.svg)](https://github.com/mukundbhudia/wembley-events/actions/workflows/scheduled-builds.yml)

Generates a calendar file on all the upcoming events at Wembley Stadium.
The file output conforms to the [iCalendar](https://tools.ietf.org/html/rfc5545) specification.

Calendar available here: https://mukundbhudia.github.io/wembley-events/wembley-events.ics.

## Pre-requisites

### Rust

- [Install Rust here](https://www.rust-lang.org/tools/install).
- Minimum Supported Rust Version (MSRV) is any 2018 edition.

## Development

- Within the repo directory run `cargo r`.
- To generate a calendar to file run `cargo r > output/test.cal` for example.

## Production

- To make a production build, within the repo directory run `cargo b --release`.
- To run the release build, within the project directory run `target/release/wembley-events` with the same arguments as in development.

## Testing

- To run **all** your tests, within the repo directory run `cargo t`. This will run all tests in the `/tests` directory.
- To run test a **specific** test such as `/tests/my_test.rs`, within the repo directory run `cargo t --test my_test`.

## Resources & Thanks

- [Brent Council events](https://www.brent.gov.uk/events-and-whats-on-calendar/).
