// below imports let us get traits/types that let us 
// read from and write to stream
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // IP address of local computer (localhost) is 127.0.0.1
    // ::bind returns a new TcpListener instance
    // connecting to a port to listen to is called "binding to a port"

    // .incoming returns a series of *connection ATTEMPTS* - which 
    //  aren't necessarily connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let status_line;
    let contents; 

    if request_line == "GET / HTTP/1.1" {
        status_line = "HTTP/1.1 200 OK\r\n\r\n";
        contents = fs::read_to_string("hello.html").unwrap();
        // print_type_of(&response);
        // * as_bytes - turns string data into bytes
        // write_all seems to be like *send* in JavaScript
    } else {
        // some other request
        status_line = "HTTP/1.1 404 NOTFOUND\r\n\r\n";
        contents = fs::read_to_string("404.html").unwrap();
        /*
           let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        // print_type_of(&response);
        stream.write_all(response.as_bytes()).unwrap();
        // * as_bytes - turns string data into bytes
        // write_all seems to be like *send* in JavaScript
          */
    }
    // TODO - refactor above to use tuple!

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}

/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
*/




