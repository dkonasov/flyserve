pub mod prelude {
    extern crate flyserve_api;
    use std::collections::HashMap;
    use std::io::prelude::*;
    use std::net::TcpStream;
    use std::net::TcpListener;
    use std::thread;
    use self::flyserve_api::*;
    
    struct Route {
        pattern: String,
        handlers: Vec<fn(&mut HttpRequest, &mut HttpResponse)>
    }

    impl Route {
        pub fn compare(&self, path: &Path) -> Option<HashMap<String, String>> {
            return path.compare(&self.pattern);
        }
        pub fn add_handler(&mut self, handler: fn(&mut HttpRequest, &mut HttpResponse)) {
            self.handlers.push(handler);
        }
    }
    
    pub struct Server {
        host: String,
        port: i32,
        routes: Vec<Route>
    }
    impl Server {
        pub fn new (host: &str, port: &i32) -> Server {
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
        pub fn add_route(&mut self, pattern: &str, handler: fn(&mut HttpRequest, &mut HttpResponse)) {
            let pattern = pattern.to_owned();
            if !self.routes.iter().any(|el| { el.pattern == pattern}) {
                self.routes.push(Route {
                    pattern: pattern.clone(),
                    handlers: Vec::new()
                });
            }
            self.routes.iter_mut().find(|el| { el.pattern == pattern}).unwrap().add_handler(handler);
        }
        fn handle_stream(mut stream: TcpStream) {
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap();
            let mut response = HttpResponse::new();
            {
                
            }
            response.set_response_handler(Box::new(|res| {
                stream.write(res.to_string().as_bytes()).unwrap();
                stream.flush().unwrap();
            }));
        }
    }
}
