use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // IP address of local computer (localhost) is 127.0.0.1
    // ::bind returns a new TcpListener instance
    // connecting to a port to listen to is called "binding to a port"

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
