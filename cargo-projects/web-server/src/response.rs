use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Response {
    method: String,
    status: String,
    version: String,
    headers: HashMap<String, String>,
    header: String,
    body: String
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::new()
    }

    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn headers(&self) -> &String {
        &self.header
    }

    pub fn body(&self) -> &String {
        &self.body
    }
}

#[derive(Default)]
pub struct ResponseBuilder {
    response: Response
}

impl ResponseBuilder {
    fn new() -> ResponseBuilder {
        ResponseBuilder::default()
    }

    pub fn method(mut self, method: &String) -> ResponseBuilder {
        self.response.method = method.to_owned();
        self
    }

    pub fn status(mut self, status: &String) -> ResponseBuilder {
        self.response.status = status.to_owned();
        self
    }

    pub fn header(mut self, header: (String, String)) -> ResponseBuilder {
        self.response.headers.insert(header.0, header.1);
        self
    }

    pub fn version(mut self, version: &String) -> ResponseBuilder {
        self.response.version = version.to_owned();
        self
    }

    pub fn body(mut self, body: String) -> Response {
        // concat headers
        let map = self.response.headers.clone();
        let mut header: String = "".into();

        for (k, v) in map.iter() {
            header = format!("{}{}:{}\r\n", header, k, v);
        }

        self.response.header = header;

        // instantiate Response
        Response {
            method: self.response.method,
            status: self.response.status,
            version: self.response.version,
            headers: self.response.headers,
            body,
            header: self.response.header
        }
    }
}
