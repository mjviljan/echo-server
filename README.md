# A Simple Echo Server

A simple server that just returns the payload that was sent to it. Currently only JSON requests are supported.

The server is written in [Rust](https://www.rust-lang.org/). 

## Prerequisites

You need to have Rust and its package manager Cargo [installed](https://rustup.rs/). 
## Development

To install required dependencies, run
```
cargo update
```

Then you start the server by running
```
cargo run
```

## Testing

To test the server, post a JSON payload to the route defined in the code. You can use the provided test JSON file with e.g. Curl:
```
curl -X POST -H "Content-Type: application/json" -d "@test.json"  http://localhost:8000/echo/json
```