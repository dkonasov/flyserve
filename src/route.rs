extern crate flyserve_api;

use std::collections::HashMap;

#[derive(Clone)]
pub struct Route {
    pub pattern: String,
    pub handlers: Vec<fn(&mut flyserve_api::HttpRequest, &mut flyserve_api::HttpResponse)>
}

impl Route {
    pub fn compare(&self, path: &flyserve_api::Path) -> Option<HashMap<String, String>> {
        return path.compare(&self.pattern);
    }
    pub fn add_handler(&mut self, handler: fn(&mut flyserve_api::HttpRequest, &mut flyserve_api::HttpResponse)) {
        self.handlers.push(handler);
    }
}