use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/ms_socket";
    let mut stream = UnixStream::connect(socket_path)?;

    let request = "GET /hello HTTP/1.1\r\nHost: localhost\r\n\r\n";
    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let response = String::from_utf8_lossy(&buffer[..]);
            println!("Received response: {}", response);
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }

    Ok(())
}
