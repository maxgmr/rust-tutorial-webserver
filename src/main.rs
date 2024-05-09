use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Listen at local address '127.0.0.1:7878' for incoming
    // TCP streams

    // Bind to ports. unwrap() stops program if errors happen.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming() returns iterator that gives sequence of
    // streams.
    // stream = open connection between client & server.

    // Process each connection & produce a series of streams
    // to handle
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Pass stream to handle_connection
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Create new BufReader instance that wraps a mutable
    // reference to the stream. BufReader adds buffering by
    // managing calls to the std::io::Read trait methods
    let buf_reader = BufReader::new(&mut stream);

    // Collect lines of request the browser sends to the server
    let http_request: Vec<_> = buf_reader
        .lines() // returns iterator of Result<String, std::io::Error>
        .map(|result| result.unwrap()) // for simplicity, stops if error
        .take_while(|line| !line.is_empty()) // ends HTTP request with two newlines in a row
        .collect();

    println!("Request: {:#?}", http_request);
}
