# Rust AXUM Full Course
from: https://github.com/jeremychone-channel/rust-axum-course/tree/main

CODE UPDATED TO AXUM 0.7 tag: axum-06-to-07

This repository contains the source code for the Rust Axum Full Course, which is available on YouTube. The code is available under either the MIT or Apache license and is free to use.

Full Rust Web App Production Blueprint Course and Repository at https://rust10x.com/web-app (YouTube video: Episode 01 - Rust Web App - Course to Production Coding)

Important: Make sure to refresh this repo. I implemented a fix on cookie.add for AUTH_TOKEN (see details below in the notes section).

Here is a Per Chapter Fork by @FloWi. Big thanks!

## Development REPL
### Terminal 1 - To run the server.
cargo watch -q -c -w src/ -x "run"

### Terminal 2 - To run the tests.
Run as code from `examples/` 
cargo watch -q -c -w examples/ -x "run --example quick_dev"

Run as a test from `tests/`
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

## Development
# Terminal 1 - To run the server.
cargo run

# Terminal 2 - To run the tests.
cargo test quick_dev -- --nocapture

## Notes

- ***IMPORTANT *- for AUTH_TOKEN cookie, make sure to call cookie.set_path("/");. See commit - Fix AUTH_TOKEN cookie issue
- The tests/quick_dev.rs file has been moved to examples/quick_dev.rs (with the same code) as it is more appropriate and seems to resolve a Windows limitation when running test and run simultaneously.
- Use the --poll flag for cargo watch (latest 8.4.0) on my Fedora Linux environment - both server and test. Things were not updating without it. see #1
- Make sure to use axum version 0.6.16 or above, as version 0.6.15 had a bug in the static routing.
- Here is a Per Chapter Fork by @FloWi. Big thanks!

