use std::{collections::HashMap};

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

#[derive(Default)]
struct Attrs {
    method: String,
    version: String,
    uri: String,
    headers: HashMap<String, String>
}

#[derive(Default)]
pub struct RequestBuilder {
    request: Attrs
}

impl RequestBuilder {
    pub fn new() -> RequestBuilder {
        RequestBuilder::default()
    }

    // &mut self -> mutable ref to self / RequestBuilder instance
    // version() mutates self.request.verion (the value it borrows)
    pub fn version(&mut self, version: String) -> &mut RequestBuilder {
        self.request.version = version;
        self
    }

    pub fn method(&mut self, version: String) -> &mut RequestBuilder {
        self.request.version = version;
        self
    }

    pub fn uri(&mut self, version: String) -> &mut RequestBuilder {
        self.request.version = version;
        self
    }

    pub fn header(&mut self, header: (String, String)) -> &mut RequestBuilder {
        self.request.headers.insert(header.0, header.1);
        self
    }

    pub fn body(self) -> Request {
        Request {
            method: self.request.method,
            http_version: self.request.version,
            uri: self.request.uri,
            headers: self.request.headers
        }
    }
}

// #[derive(Copy, Clone)]
// enum Http {
//     Http11
// }

// #[derive(Copy, Clone)]
// struct Version(Http);

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }

    pub fn method(self) -> String {
        self.method
    }

    pub fn uri(self) -> String {
        self.uri
    }

    pub fn http_version(self) -> String {
        self.http_version
    }

    pub fn headers(self) -> HashMap<String, String> {
        self.headers
    }

    // pub fn build(self) -> Request {
    //     let mut map2: HashMap<String, String> = HashMap::new();
    //     map2.extend(self.headers.into_iter());

    //     Request {
    //         method: self.method.clone(),
    //         uri: self.uri.clone(),
    //         http_version: self.http_version.clone(),
    //         headers: map2
    //     }
    // }
}