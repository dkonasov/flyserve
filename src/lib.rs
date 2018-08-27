pub mod prelude {
    extern crate flyserve_api;
    use std::collections::HashMap;
    use std::io::prelude::*;
    use std::net::TcpStream;
    use std::net::TcpListener;
    use std::thread;
    use self::flyserve_api::*;

    struct Route<'a> {
        pattern: String,
        handlers: Vec<&'a Fn(&mut HttpRequest, &mut HttpResponse)>
    }

    impl<'a> Route<'a> {
        pub fn compare(&self, path: &Path) -> Option<HashMap<String, String>> {
            return path.compare(&self.pattern);
        }
    }

    pub struct Server<'a> {
        host: String,
        port: i32,
        routes: Vec<Route<'a>>
    }
    impl<'a> Server<'a> {
        pub fn new (host: &str, port: &i32) -> Server<'a> {
            Server {
                host: host.to_string(),
                port: *port,
                routes: Vec::new()
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
