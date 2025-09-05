# `stdin-http-rs`

A simple Rust program and library that serves a single string over HTTP.

## Usage as a library

Add to your `Cargo.toml`:

```toml
[dependencies]
stdin-http-rs = { git = "https://github.com/abhillman/stdin-http-rs.git", tag = "0.0.1" }
```

Example:

```rust
fn main() -> std::io::Result<()> {
    stdin_http_rs::run(None, include_bytes!("../Cargo.toml").to_vec())
}
```

Test it:

```bash
# Start the server in the background
cargo run --example micro-server > /dev/null &

# Fetch the served content
curl http://localhost:8080

# Stop the server
pkill -9 micro-server
```

Output will look like:

```toml
[package]
name = "stdin-http-rs"
version = "0.1.0"
edition = "2024"

[dependencies]
```

## Usage as a binary

```bash
# Run (defaults to 127.0.0.1:8080)
cat /tmp/foo | cargo run

# Custom bind address
cat /tmp/foo | cargo run -- 0.0.0.0:9000

# Verify output
curl -i http://127.0.0.1:8080/
```
