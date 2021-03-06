use std::fs;
use std::io::prelude::*; // get access to certain traits that let us read from and write to the stream.
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use webserver::ThreadPool;

// A single stream represents an open connection between the client and the server.
// A connection is the name for the full request and response process in which a client connects to the server
// TcpStream will read from itself to see what the client sent and then allow us to write our response to the stream.
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    // listener.incomming returns an iterator
    // Take only two requests from the iterator and after that the thread pool will be dropped
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down...");

    // At this point the thread pooll will go out5 of scope and the drop() method from its Drop trait implementation will be invoked
    // which will wait untill all threads have finished and terminated(join() is called on the thread handles).
}

// We need mutable acces to TcpStream even for reading since the TcpStream instance keeps track
// of what data it returnsthe TcpStream instance keeps track of what data it returns and therefore
// needs to be mut because its internal state might change.

// An Response has a format like:
// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body
fn handle_connection(mut stream: TcpStream) {
    // Create a buffer 1024 bytes in size
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // we hardcode the data corresponding to a GET request to /
    // Because we’re reading raw bytes into the buffer, we transform get into a byte string by
    // adding the b"" byte string syntax at the start of the content data
    let get = b"GET / HTTP/1.1\r\n";

    // /sleep path for simulating a lengthy request
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // We check that the request data read in the buffer starts with the bytes
    // defined in the `get` varaible we hardcoded before.
    // Here we either return a success response responce with the html string read froma file
    // or a 404.html reponse
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5)); // Simulate lengthy request by sleeping 5 sec.
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    // We conver the response string to bytes and then the write method on
    // stream takes a array of bytes that is &[u8] and sends those bytes directly down the connection.
    stream.write(response.as_bytes()).unwrap();

    // flush will wait and prevent the program from continuing until all the bytes are written to the connection
    stream.flush().unwrap();
}
