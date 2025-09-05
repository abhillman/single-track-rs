fn main() -> std::io::Result<()> {
    stdin_http_rs::run(None, include_bytes!("../Cargo.toml").to_vec(), None)
}
