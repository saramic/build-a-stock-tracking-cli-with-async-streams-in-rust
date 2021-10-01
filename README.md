# build-a-stock-tracking-cli-with-async-streams-in-rust

My learning as part of https://www.manning.com/liveprojectseries/async-streams-in-rust-ser

in 2 parts

## Project 1 Data Streaming with Async Rust

https://www.manning.com/liveproject/data-streaming-with-async-rust

```
# jump into project
cd project-1-data-streaming-with-async-rust

# install dependendencies and build it
cargo build 

# run the tests which currently fail
cargo test

# run the code to get stock values
cargo run -- --from 2021-09-30T12:00:00Z --symbols LYFT,MSFT,AAPL,UBER,LYFT,FB,AMD,GOOG
cargo run -- --from 2021-09-30T12:00:00Z
```

## Project 2 Advanced Data Streaming with Async Rust

https://www.manning.com/liveproject/advanced-data-streaming-with-async-rust

...TODO

## Other

### Rust web development

as part of the Live Project do some exercises as part of the included Live Book
https://www.manning.com/books/rust-web-development

- rust-web-development/eg-2_2-using-match-to-work-with-options
- rust-web-development/eg-2_3-create-and-print-question

### async-std play

getting my head around async-std by following examples from https://docs.rs/async-std/1.10.0/async_std/#examples

```
cd async-std-play
cargo build
cargo run

# to test UDP server
# run netcat in
#   -v verbose
#   -u UDP mode
# on IP and port, any typed content will be echoed back
nc -v -u 127.0.0.1 8080
```

