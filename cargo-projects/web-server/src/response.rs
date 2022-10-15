use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct Response {
    method: String,
    status: String,
    version: String,
    headers: HashMap<String, String>,
    body: String
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::new()
    }

    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn headers(&self) -> String {
        let map = self.headers.clone();
        let mut header: String = String::from("");

        for (k, v) in map.iter() {
            header = format!("{}{}:{}\r\n", header, k, v);
        }

        header
    }

    pub fn body(&self) -> &String {
        &self.body
    }
}

#[derive(Default, Debug)]
pub struct ResponseBuilder {
    response: Response
}

impl ResponseBuilder {
    fn new() -> ResponseBuilder {
        ResponseBuilder::default()
    }

    pub fn method(&mut self, method: &String) -> &mut ResponseBuilder {
        self.response.method = method.to_owned();
        self
    }

    pub fn status(&mut self, status: &String) -> &mut ResponseBuilder {
        self.response.status = status.to_owned();
        self
    }

    pub fn header(&mut self, header: (String, String)) -> &mut ResponseBuilder {
        self.response.headers.insert(header.0, header.1);
        self
    }

    pub fn version(&mut self, version: &String) -> &mut ResponseBuilder {
        self.response.version = version.to_owned();
        self
    }

    pub fn body(mut self, body: String) -> Response {
        // instantiate Response
        Response {
            method: self.response.method,
            status: self.response.status,
            version: self.response.version,
            headers: self.response.headers,
            body,
        }
    }
}
