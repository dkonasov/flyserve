pub mod prelude {
    use std::net::TcpListener;
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
                stream.unwrap();
                println!("Connection established!");
            }
            println!("Starting server at {}:{}", self.host, self.port);
        }
    }
}
