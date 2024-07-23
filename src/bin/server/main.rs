use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

fn handle_request(req: &str) -> String {
    println!("Received request: {}", req);

    if req.starts_with("GET /hello HTTP/1.1\r\n") {
        return String::from("HTTP/1.1 200 OK\r\n\r\nHello World!");
    } else if req.starts_with("GET /version HTTP/1.1\n\n") {
        return String::from("HTTP/1.1 200 OK\r\n\r\n1.0.0");
    } else {
        String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n")
    }
}

fn handle_client(mut stream: UnixStream) {
    // Currently we are only supporting two requests:
    // GET /hello and GET /version
    let mut buffer = [0; 128]; // Big enough
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);
            let response = handle_request(&request);

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/ms_socket";

    // Remove the socket file if it exists
    if fs::metadata(socket_path).is_ok() {
        fs::remove_file(socket_path)?;
    }

    let listener = UnixListener::bind(socket_path)?;

    println!("Server listening on {}", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(err) => {
                eprintln!("Connection failed: {}", err);
            }
        }
    }

    Ok(())
}
