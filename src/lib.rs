mod flyserve {
    struct Server {
        host: String,
        port: i32,
    }
    impl Server {
        fn new (host: &str, port: &i32) -> Server {
            Server{
                host: host.to_string(),
                port: *port
            }
        }
        fn start(&self) {
            println!("Starting server at {}:{}", self.host, self.port);
        }
    }
}
