use std::collections::HashMap;

// #[derive(Default)]
// pub struct Request<'a> {
//     method: &'a str,
//     uri: &'a str,
//     pub http_version: &'a str,
//     pub headers: HashMap<String, String>
// }

// impl<'a> Request<'a> {
//     pub fn new() -> Request<'a> {
//         Request::default()
//     }

//     pub fn method(&mut self, val: &'a str) -> &mut Request<'a> {
//         self.method = val;
//         self
//     }

//     pub fn uri(&mut self, val: &'a str) -> &mut Request<'a> {
//         self.uri = val;
//         self
//     }

//     pub fn http_version(&mut self, val: &'a str) -> &mut Request<'a> {
//         self.http_version = val;
//         self
//     }

//     pub fn build(&mut self) -> &mut Request<'a> {
//         self
//     }
// }

#[derive(Default)]
pub struct Request {
    method: String,
    uri: String,
    pub http_version: String,
    pub headers: HashMap<String, String>
}

impl Request {
    pub fn new() -> Request {
        Request::default()
    }

    pub fn method(&mut self, val: String) -> &mut Request {
        self.method = val;
        self
    }

    pub fn uri(&mut self, val: String) -> &mut Request {
        self.uri = val;
        self
    }

    pub fn http_version(&mut self, val: String) -> &mut Request {
        self.http_version = val;
        self
    }

    pub fn header(&mut self, k: String, v: String) -> &mut Request {
        self.headers.insert(k, v);
        self
    }

    pub fn build(&self) -> &Request {
        self
    }
}