use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rust_tutorial_webserver::ThreadPool;

const STATUS_LINE_200: &str = "HTTP/1.1 200 OK";
const STATUS_LINE_404: &str = "HTTP/1.1 404 NOT FOUND";

const MAIN_PAGE: &str = "welcome.html";
const PAGE_404: &str = "404.html";

const REQUEST_LINE_MAIN: &str = "GET / HTTP/1.1";
const REQUEST_LINE_SLEEP: &str = "GET /sleep HTTP/1.1";

const THREAD_POOL_SIZE: usize = 4;

fn main() {
    // Listen at local address '127.0.0.1:7878' for incoming
    // TCP streams

    // Bind to ports. unwrap() stops program if errors happen.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Thread pool: Group of spawned threads that are waiting
    // and ready to handle a task.

    // Must limit pool size to avoid DoS attacks

    // Create a new thread pool with THREAD_POOL_SIZE threads
    let t_pool = ThreadPool::new(THREAD_POOL_SIZE);

    // incoming() returns iterator that gives sequence of
    // streams.
    // stream = open connection between client & server.

    // Process each connection & produce a series of streams
    // to handle
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // pool.execute takes a closure and gives it to a thread
        // in the pool to run
        t_pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Create new BufReader instance that wraps a mutable
    // reference to the stream. BufReader adds buffering by
    // managing calls to the std::io::Read trait methods
    let buf_reader = BufReader::new(&mut stream);

    // Read first line of HTTP request
    // Call next() to get first item from iterator
    // First unwrap handles Option, stops if no items
    // Second unwrap handles Result, stops if invalid request
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        REQUEST_LINE_MAIN => (STATUS_LINE_200, MAIN_PAGE),
        // Simulated slow response
        REQUEST_LINE_SLEEP => {
            thread::sleep(Duration::from_secs(5));
            (STATUS_LINE_200, MAIN_PAGE)
        }
        _ => (STATUS_LINE_404, PAGE_404),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // write_all() takes &[u8] & sends those bytes directly down
    // the connection
    // write_all() can fail, so using unwrap() for simplicity.
    stream.write_all(response.as_bytes()).unwrap();
}
