use reqwest::{header::HeaderMap, Client, Error, Response};
use tokio::net::TcpStream;

const BASE_URL: &str = "https://discord.com/api/v9";

#[derive(Debug)]
pub struct Request {
    route: Route,
    params: Vec<(String, String)>,
    header: HeaderMap,
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
            header: HeaderMap::new(),
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

    pub fn with_header(mut self, key: &'static str, val: &str) -> Self {
        self.add_header(key, val);
        self
    }

    pub fn add_header(&mut self, key: &'static str, val: &str) {
        self.header.insert(key, val.try_into().unwrap());
    }

    pub fn add_headers(&mut self, new_headers: Vec<(&'static str, &str)>) {
        for (key, val) in new_headers {
            self.header.insert(key, val.try_into().unwrap());
        }
    }

    pub async fn call(self) -> Result<Response, Error> {
        let url = format!("{}{}", BASE_URL, self.route.endpoint);
        let client = reqwest::Client::new();
        let request = self.route.method.call(client, &url);
        request
            .query(&self.params)
            .headers(self.header)
            .send()
            .await
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
    fn call(&self, client: Client, path: &str) -> reqwest::RequestBuilder {
        use HTTPMethod::*;
        match *self {
            Get => client.get(path),
            Head => client.head(path),
            Post => client.post(path),
            Put => client.put(path),
            Delete => client.delete(path),
        }
    }
}
