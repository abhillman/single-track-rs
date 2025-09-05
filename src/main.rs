//! Simple process for serving stdin over HTTP without tokio.
//!
//! # Usage
//! ```sh
//! cat /tmp/foo | cargo run -- 127.0.0.1:8080
//! # then in another shell:
//! curl -i http://127.0.0.1:8080/
//! ```
//!
//! Binds to `127.0.0.1:8080` by default if no address is provided.

use std::io::Read;

fn main() -> std::io::Result<()> {
    // Read entire stdin once and keep it for all requests.
    let mut body: Vec<u8> = vec![];
    std::io::stdin()
        .read_to_end(&mut body)
        .expect("could not read from stdin.");

    // Accept optional bind address from argv[1]; default to 127.0.0.1:8080.
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    stdin_http_rs::run(Some(addr), body)
}
