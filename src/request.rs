const BASE_URL: &str = "https://discord.com/api/v9";

#[derive(Debug)]
pub struct Request {
    route: Route,
    params: Vec<(String, String)>,
    header: Vec<(String, String)>,
}

impl Request {
    pub fn get(endpoint: &str) -> Self {
        Self::new(HTTPMethod::Get, endpoint)
    }

    pub fn head(endpoint: &str) -> Self {
        Self::new(HTTPMethod::Head, endpoint)
    }

    pub fn post(endpoint: &str) -> Self {
        Self::new(HTTPMethod::Post, endpoint)
    }

    pub fn put(endpoint: &str) -> Self {
        Self::new(HTTPMethod::Put, endpoint)
    }

    pub fn delete(endpoint: &str) -> Self {
        Self::new(HTTPMethod::Delete, endpoint)
    }

    fn new(method: HTTPMethod, endpoint: &str) -> Self {
        Self {
            route: Route::new(method, endpoint),
            params: Vec::new(),
            header: Vec::new(),
        }
    }

    pub fn authorize(mut self, token: &str) -> Self {
        self.with_header("Authorization", &format!("Bot {}", token))
    }

    pub fn with_param(mut self, key: &str, val: &str) -> Self {
        self.add_param(key, val);
        self
    }

    pub fn add_param(&mut self, key: &str, val: &str) {
        self.params.push((String::from(key), String::from(val)));
    }

    pub fn add_params(&mut self, new_params: Vec<(&str, &str)>) {
        self.params.extend(
            new_params
                .into_iter()
                .map(|(s1, s2)| (String::from(s1), String::from(s2))),
        );
    }

    pub fn with_header(mut self, key: &str, val: &str) -> Self {
        self.add_header(key, val);
        self
    }

    pub fn add_header(&mut self, key: &str, val: &str) {
        self.header.push((String::from(key), String::from(val)));
    }

    pub fn add_headers(&mut self, new_headers: Vec<(&str, &str)>) {
        self.header.extend(
            new_headers
                .into_iter()
                .map(|(s1, s2)| (String::from(s1), String::from(s2))),
        );
    }

    pub fn call(self) -> Result<ureq::Response, ureq::Error> {
        let url = format!("{}{}", BASE_URL, self.route.endpoint);
        let mut request = self.route.method.call(&url);
        for (key, val) in self.params {
            request = request.query(&key, &val);
        }
        for (key, val) in self.header {
            request = request.set(&key, &val);
        }
        request.call()
    }
}

#[derive(Debug)]
struct Route {
    method: HTTPMethod,
    endpoint: String,
}
impl Route {
    fn new(method: HTTPMethod, endpoint: &str) -> Self {
        Self {
            method,
            endpoint: String::from(endpoint),
        }
    }
}

#[derive(Debug)]
enum HTTPMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
}

impl HTTPMethod {
    fn call(&self, path: &str) -> ureq::Request {
        use HTTPMethod::*;
        match *self {
            Get => ureq::get(path),
            Head => ureq::head(path),
            Post => ureq::post(path),
            Put => ureq::put(path),
            Delete => ureq::delete(path),
        }
    }
}
