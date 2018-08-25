pub mod prelude {
    use std::io::prelude::*;
    use std::net::TcpStream;
    use std::net::TcpListener;
    use std::thread;

    pub struct Server {
        host: String,
        port: i32,
    }
    impl Server {
        pub fn new (host: &str, port: &i32) -> Server {
            Server{
                host: host.to_string(),
                port: *port
            }
        }
        pub fn start(&self) {
            let addr = format!("{}:{}", self.host, self.port);
            let listener = TcpListener::bind(addr).unwrap();
            for stream in listener.incoming() {
                thread::spawn(|| {
                    Server::handle_stream(stream.unwrap());
                });
            }
        }
        fn handle_stream(mut stream: TcpStream) {
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap();
            let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
