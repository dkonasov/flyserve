extern crate flyserve_api;

use route::Route;
use std::io::Read;
use std::io::Write;
use std::net::{TcpStream, TcpListener};
use std::sync::Arc;
use std::ops::Deref;
use std::thread;
use std::cell::RefCell;

fn handle_stream(mut stream: TcpStream, routes: Arc<Vec<Route>>) {
        let mut buffer = Vec::new();
        let response_sent = RefCell::new(false);
        while buffer.len() < 4 || &buffer[buffer.len() - 4..] != [13, 10, 13, 10] {
            let mut chunk_buff: [u8; 512] = [0; 512];
            let bytes_count = stream.read(&mut chunk_buff).unwrap();
            for ind in 0..bytes_count {
                buffer.push(chunk_buff[ind]);
            }
        }
        let mut response = flyserve_api::HttpResponse::new();
        response.set_response_handler(Box::new(|res| {
            if !response_sent.borrow().deref() {
                stream.write(res.to_string().as_bytes()).unwrap();
                stream.flush().unwrap();
                response_sent.replace(true);
            }
        }));
        match String::from_utf8(buffer) {
            Ok(result) => {
                match flyserve_api::HttpRequest::parse(&result) {
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
                                    if response_sent.borrow().deref().to_owned() {
                                        break;
                                    }
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
                handle_stream(stream.unwrap(), arc_cloned);
            });
        }
    }
    pub fn add_route(&mut self, pattern: &str, handler: fn(&mut flyserve_api::HttpRequest, &mut flyserve_api::HttpResponse)) {
        let pattern = pattern.to_owned();
        if !self.routes.iter().any(|el| { el.pattern == pattern}) {
            self.routes.push(Route {
                pattern: pattern.clone(),
                handlers: Vec::new()
            });
        }
        self.routes.iter_mut().find(|el| { el.pattern == pattern}).unwrap().add_handler(handler);
    }
}