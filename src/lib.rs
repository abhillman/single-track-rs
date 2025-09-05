/// Serves `body` over HTTP to every incoming TCP connection.
/// # Arguments
///
/// * `addr` - An optional string representing the address (e.g., a URL or socket address)
///            to which the data is sent or from which data is processed. If `None`,
///            `127.0.0.1:3030` is used.
/// * `body` - A `Vec<u8>` representing the raw binary content or payload.
/// * `content_type` - An optional string representing the MIME type of the body (e.g.,
///                    `"application/json"` or `"text/plain"`). If `None` is specified,
///                    `text/html; charset=utf-8` is used.
///
/// # Notes
/// - Spawns a new OS thread per connection.
/// - Ignores the HTTP request bytes and always responds `200 OK`.
/// - Sends `Content-Type: text/plain; charset=utf-8`.
///
/// # Errors
/// Returns any I/O errors from binding the listener, or accepting connections.
pub fn run(
    addr: Option<String>,
    body: Vec<u8>,
    content_type: Option<String>,
) -> std::io::Result<()> {
    let addr = addr.unwrap_or("127.0.0.1:3030".to_string());
    let listener = std::net::TcpListener::bind(&addr)?;
    println!("Listening on http://{}", addr);

    let body = std::sync::Arc::new(body);
    let content_type =
        std::sync::Arc::new(content_type.unwrap_or("text/html; charset=utf-8".into()));
    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                let body = body.clone();
                let content_type = content_type.clone();

                std::thread::spawn(move || {
                    let _ = handle_client(tcp_stream, body, content_type);
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
    mut stream: std::net::TcpStream,
    body: std::sync::Arc<Vec<u8>>,
    content_type: std::sync::Arc<String>,
) -> std::io::Result<()> {
    // Ignore the request
    let mut _buf = [0u8; 2048];
    let _ = std::io::Read::read(&mut stream, &mut _buf);

    // Same response regardless
    let headers = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Length: {}\r\n\
         Content-Type: {}\r\n\
         Connection: close\r\n\
         \r\n",
        body.len(),
        content_type
    );

    std::io::Write::write_all(&mut stream, headers.as_bytes())?;
    std::io::Write::write_all(&mut stream, &body)?;
    std::io::Write::flush(&mut stream)?;

    Ok(())
}
