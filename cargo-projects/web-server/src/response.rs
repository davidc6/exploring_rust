use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Response<'a> {
    method: &'a str,
    status: String,
    version: &'a str,
    headers: HashMap<String, String>,
    body: String
}

impl<'a> Response<'a> {
    pub fn builder() -> ResponseBuilder<'a> {
        ResponseBuilder::new()
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn headers(self) -> String {
        let map = self.headers.clone();
        let mut header: String = "".into();

        for (k, v) in map.iter() {
            header = format!("{}{}:{}\r\n", header, k, v);
        }

        header
    }

    pub fn body(self) -> String {
        self.body
    }
}

#[derive(Default)]
pub struct ResponseBuilder<'a> {
    response: Response<'a>
}

impl<'a> ResponseBuilder<'a> {
    fn new() -> ResponseBuilder<'a> {
        ResponseBuilder::default()
    }

    pub fn method(mut self, method: &'a str) -> ResponseBuilder {
        self.response.method = method;
        self
    }

    pub fn status(mut self, status: &String) -> ResponseBuilder<'a> {
        self.response.status = status.to_owned();
        self
    }

    pub fn version(mut self, version: &'a str) -> ResponseBuilder<'a> {
        self.response.version = version;
        self
    }

    pub fn body(self, body: String) -> Response<'a> {
        Response {
            method: self.response.method,
            status: self.response.status,
            version: self.response.version,
            headers: self.response.headers,
            body
        }
    }
}
