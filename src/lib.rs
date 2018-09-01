pub mod prelude {
    extern crate flyserve_api;
    use std::collections::HashMap;
    use std::io::prelude::*;
    use std::net::TcpStream;
    use std::net::TcpListener;
    use std::thread;
    use self::flyserve_api::*;
    use std::sync::Arc;
    use std::clone::Clone;
    use std::ops::Deref;
    
    #[derive(Clone)]
    struct Route {
        pattern: String,
        pub handlers: Vec<fn(&mut HttpRequest, &mut HttpResponse)>
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
            let arc_routes = Arc::new(self.routes.clone());
            for stream in listener.incoming() {
                let arc_cloned = Arc::clone(&arc_routes);
                thread::spawn(|| {
                    Server::handle_stream(stream.unwrap(), arc_cloned);
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
        fn handle_stream(mut stream: TcpStream, routes: Arc<Vec<Route>>) {
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap();
            let mut response = HttpResponse::new();
            {
                
            }
            response.set_response_handler(Box::new(|res| {
                println!("Sending a response...");
                stream.write(res.to_string().as_bytes()).unwrap();
                stream.flush().unwrap();
            }));
            match String::from_utf8(buffer) {
                Ok(result) => {
                    match HttpRequest::parse(&result) {
                        Ok(mut req) => {
                            let mut has_some_handlers = false;
                            for route in routes.deref() {
                                let cmp_result = route.compare(&req.path);
                                if cmp_result.is_some() {
                                    if route.handlers.len() > 0 {
                                        has_some_handlers = true;
                                    }
                                    let path_params = cmp_result.unwrap();
                                    for (key, value) in path_params.iter() {
                                        req.params.insert(key.to_string(), value.to_string());
                                    }
                                    for handler in route.handlers.iter() {
                                        handler(&mut req, &mut response);
                                    }
                                }
                            }
                            if !has_some_handlers {
                                response.status_code = 404;
                                response.status_msg = Some("Not found".to_string());
                                response.send();
                            }
                        },
                        Err(error) => {
                            println!("{}", error);
                            response.status_code = 400;
                            response.status_msg = Some("Bad request".to_string());
                            response.send();
                        }
                    }
                },
                Err(_) => {
                    response.status_code = 400;
                    response.status_msg = Some("Bad request".to_string());
                    response.send();
                }
            }
        }
    }
}
