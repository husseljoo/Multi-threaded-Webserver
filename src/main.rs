use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("\n\n\n EVERYTHING IS DONE !!!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (file_name, status_line) = if buffer.starts_with(get) {
        ("home", "HTTP/1.1 200 OK")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("home", "HTTP/1.1 200 OK")
    } else {
        ("error", "HTTP/1.1 404 Not Found")
    };

    let contents = fs::read_to_string(format!("html/{}.html", file_name)).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
