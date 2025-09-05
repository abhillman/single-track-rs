use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Serves `body` over HTTP to every incoming TCP connection on the given
/// address; defaults to `127.0.0.1:8080` if `None` is given.
///
/// The program is intentionally minimal:
/// - Spawns a new OS thread per connection.
/// - Ignores the HTTP request bytes and always responds `200 OK`.
/// - Sends `Content-Type: text/plain; charset=utf-8`.
///
/// # Errors
/// Returns any I/O errors from binding the listener, or
/// accepting connections.
pub fn run(addr: Option<String>, body: Vec<u8>) -> std::io::Result<()> {
    let addr = addr.unwrap_or("127.0.0.1:8080".to_string());
    let listener = TcpListener::bind(&addr)?;
    println!("Listening on http://{}", addr);

    let body = std::sync::Arc::new(body);
    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                let body = body.clone();
                thread::spawn(move || {
                    let _ = handle_client(tcp_stream, body);
                });
            }
            Err(err) => eprintln!("accept error: {err}"),
        }
    }
    Ok(())
}

/// Handles a single client connection by discarding the request bytes and
/// replying with a fixed `200 OK` response containing `body`.
///
/// # Parameters
/// - `stream`: The accepted `TcpStream` for the client.
/// - `body`: The UTF-8 response payload to send.
///
/// # Behavior
/// - Reads and ignores up to 2 KiB of the request.
/// - Writes minimal HTTP/1.1 headers with the correct `Content-Length`.
/// - Closes the connection (`Connection: close`).
///
/// # Errors
/// Propagates I/O errors from reading or writing on the stream.
pub(crate) fn handle_client(
    mut stream: TcpStream,
    body: std::sync::Arc<Vec<u8>>,
) -> std::io::Result<()> {
    // Ignore the request
    let mut _buf = [0u8; 2048];
    let _ = stream.read(&mut _buf);

    // Same response regardless
    let headers = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Length: {}\r\n\
         Content-Type: text/plain; charset=utf-8\r\n\
         Connection: close\r\n\
         \r\n",
        body.len()
    );

    stream.write_all(headers.as_bytes())?;
    stream.write_all(&body)?;
    stream.flush()?;

    Ok(())
}
