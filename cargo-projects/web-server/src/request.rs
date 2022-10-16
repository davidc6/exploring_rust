use std::{collections::HashMap};

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

// This builder enables to construct a response object
// which can be quite complicated
impl RequestBuilder {
    // Returns default values for RequestBuilder type
    pub fn new() -> RequestBuilder {
        RequestBuilder::default()
    }

    // &mut self -> mutable ref to self / RequestBuilder instance
    // version() mutates self.request.version (the value it borrows)
    // returns instance after it has been mutated
    pub fn version(&mut self, version: String) -> &mut RequestBuilder {
        self.request.version = version;
        self
    }

    pub fn method(&mut self, method: String) -> &mut RequestBuilder {
        self.request.method = method;
        self
    }

    pub fn uri(&mut self, uri: String) -> &mut RequestBuilder {
        self.request.uri = uri;
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

    pub fn method(&self) -> &String {
        &self.method
    }

    pub fn uri(self) -> String {
        self.uri
    }

    pub fn http_version(&self) -> &String {
        &self.http_version
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