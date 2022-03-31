use async_std::net::TcpStream;
use std::str;
use regex::Regex;
use std::collections::HashMap;
use crate::HTTP_RESPONSE::*;
use urlencoding::{decode_binary, encode};
use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;
use futures::AsyncWriteExt;

type HTTPMETHOD = String;
type Validator = fn(Request) -> Option<Request>;

#[derive(Debug)]
#[derive(Clone)]
pub enum HTTP_RESPONSE {
    _OK(String),
    _REDIRECT(String),
    _UNAUTHORIZED,
    _NOT_FOUND,
    _OK_with_header(String, HashMap<String, String>),
}

/// Routes the Request  to a static or dynamic route
/// # Arguments
///
/// * `request `- A request object with all releveant information like th path and method
///* `response`- An Object of Response type
/// * `Router`- A Router that has all routes declared and will be used to route the `request `
pub async fn route(mut request: Request, mut response: Response, router: &Router) {
    //check if route is dynamic
    match router.validate(request) {
        Some(mut request) => {
            match router.paths.get(&(request.method.clone(), request.path.clone())) {
                //check if it ia static reuqest
                Some(f) => {
                    let response_txt = f(request);
                    response.send(response_txt).await;
                }
                None => {
                    //check if it is in dyn routing
                    match router.matches_dyn_route(&request.path) {
                        Some((route, params)) => {
                            request.set_url_params(params);
                            let response_txt = router.paths.get(&(request.method.clone(), route.clone())).unwrap_or(&{ |_r: Request| _OK("404 DYN".to_string()) })(request);
                            response.send(response_txt).await;
                        }
                        _ => {
                            //default behaviour
                            response.send(router.default_response.clone()).await;
                        }
                    }
                }
            }
        }
        None => response.send(HTTP_RESPONSE::_UNAUTHORIZED).await
    }
}

#[derive(Debug)]
/// represents a router with all paths and dynmaic paths
#[derive(Clone)]
pub struct Router {
    pub paths: HashMap<(HTTPMETHOD, String), fn(Request) -> HTTP_RESPONSE>,
    pub dyn_paths: Vec<(String, Regex, Vec<String>)>,
    default_response: HTTP_RESPONSE,
    validator: Validator,
}

impl Router {
    /// Returns a Router with an empty paths and dyn_paths directory
    pub fn new() -> Router {
        Router { paths: HashMap::new(), dyn_paths: Vec::new(), default_response: HTTP_RESPONSE::_NOT_FOUND, validator: |r: Request| Some(r) }
    }
    /// Registers a GET path for the given `path ` and returns the router for nicer instanciating
    ///
    /// # Arguments
    ///
    /// * `path `- A string slice that holds the name of the path
    /// * `f`- A function that has a recives a request and Returns a String
    ///
    /// # Examples
    ///
    /// ```
    /// // Here you can see how the Ruter is used
    /// Router::new().get("/:name",hello).post("/",|a:Request|"<h1>CLOJURE</h1>".to_string());
    ///
    /// fn hello(r:Request)->String{
    ///     hello.to_string()
    /// }
    ///
    /// ```
    pub fn get(mut self, path: &str, f: fn(Request) -> HTTP_RESPONSE) -> Router {
        self.add_route(path, f, "GET".to_string());
        self
    }

    pub fn post(mut self, path: &str, f: fn(Request) -> HTTP_RESPONSE) -> Router {
        self.add_route(path, f, "POST".to_string());
        self
    }

    pub fn put(mut self, path: &str, f: fn(Request) -> HTTP_RESPONSE) -> Router {
        self.add_route(path, f, "PUT".to_string());
        self
    }

    pub fn delete(mut self, path: &str, f: fn(Request) -> HTTP_RESPONSE) -> Router {
        self.add_route(path, f, "DELETE".to_string());
        self
    }
    pub fn default(mut self, default_resp: HTTP_RESPONSE) -> Router {
        self.default_response = default_resp;
        self
    }
    pub fn add_route(&mut self, path: &str, f: fn(Request) -> HTTP_RESPONSE, method: HTTPMETHOD) {
        let params = Regex::new(r":\w+").unwrap().find_iter(path).map(|x| x.as_str().to_owned()).collect::<Vec<String>>();
        if params.len() > 0 {
            let mut regex_withparams = path.to_string();
            for n in &params {
                regex_withparams = regex_withparams.replace(n, &format!(r#"(?P<{name}>\w+)/?"#, name = n.replace(":", "")));
            }
            self.dyn_paths.push((path.to_string(), Regex::new(&regex_withparams).unwrap(), params));
            self.paths.insert((method, path.to_string()), f);
        } else {
            self.paths.insert((method, path.to_string()), f);
        }
    }
    pub fn okay(self) -> &'static mut Self {
        return Box::leak(Box::new(self.clone()));
    }
    fn validate(&self, r: Request) -> Option<Request> {
        (self.validator)(r)
    }
    pub fn validator(mut self, f: Validator) -> Router {
        self.validator = f;
        self
    }

    fn matches_dyn_route(&self, requested_path: &str) -> Option<(&String, HashMap<String, String>)> {
        for (key, regex, params) in &self.dyn_paths {
            let match_group_path = regex.find_iter(requested_path).map(|x| x.as_str().to_owned()).collect::<Vec<String>>();
            if match_group_path.get(0)?.len() == requested_path.len() {
                let mut map = HashMap::new();
                regex.captures(requested_path).and_then(|cap| {
                    for item in params {
                        let key = &*item.replace(":", "");
                        map.insert(key.to_string(), cap.name(key).map(|login| login.as_str().to_owned()).unwrap());
                    }
                    Some(1)//needs return?
                });
                return Some((&key, map));
            }
        }
        None
    }
}


#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: HTTPMETHOD,
    pub params: HashMap<String, String>,
    pub url_params: HashMap<String, String>,
    pub header: HashMap<String, String>,
    pub complete_req: String,
}

impl Request {
    fn new(path: &str, method: &str, complete: &str) -> Request {
        Request { path: path.to_owned(), method: method.to_owned(), params: HashMap::new(), complete_req: complete.to_string(), url_params: HashMap::new(), header: HashMap::new() }
    }
    fn add_param(&mut self, key: &str, value: &str) {
        self.params.insert(key.to_string(), value.to_string());
    }
    fn add_header(&mut self, key: &str, value: &str) {
        self.header.insert(key.to_string(), value.to_string());
    }

    fn set_url_params(&mut self, map: HashMap<String, String>) {
        self.url_params = map;
    }
}

#[derive(Debug)]
pub struct Response {
    pub response_text: String,
    pub stream: TcpStream,
    pub header: HashMap<String, String>,
}

impl Response {
    fn new(stream: TcpStream) -> Response {
        Response { response_text: "".to_string(), stream: stream, header: HashMap::new() }
    }
    pub async fn send(mut self, response: HTTP_RESPONSE) {
        let msg = match response {
            _OK(msg) => { format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n{}Content-Length:{}\r\n\r\n{}", header_to_string(self.header), msg.len(), &msg) }
            _OK_with_header(msg, header) => {
                dbg!(format!("HTTP/1.1 200 OK\r\n{}Content-Length:{}\r\n\r\n{}", header_to_string(header), msg.len(), &msg))
            }
            _REDIRECT(location) => { format!("HTTP/1.1 308 Permanent Redirect\r\nLocation:{}\r\n\r\n", location) }
            _UNAUTHORIZED => { format!("HTTP/1.1 401 Unauthorized\r\n\r\n") }
            _NOT_FOUND => { format!("HTTP/1.1 404 Not Found\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length:{}\r\n\r\n{}", "404 NOT FOUND".len(), "404 NOT FOUND") }
        };
        AsyncWriteExt::write(&mut self.stream, msg.as_bytes()).await.unwrap();
        AsyncWriteExt::flush(&mut self.stream).await.unwrap();
    }
}

fn header_to_string(header: HashMap<String, String>) -> String {
    let mut acc = String::new();
    for (k, v) in header {
        acc = acc + &k + ":" + &v + "\r\n";
    }
    acc
}

use async_std::task::spawn;
use futures::stream::StreamExt;

pub async fn serve(address: &str, router: &'static Router) {
    let listener = TcpListener::bind(address).await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |tcpstream| async move {
            //let router=router.clone();
            let mut s = tcpstream.unwrap();

            let mut buffer = [0; 1024 * 16];
            let am = s.read(&mut buffer).await.unwrap();
            let decoded = decode_binary(&buffer[..am]).to_owned();

            let decoded = String::from_utf8_lossy(&decoded);

            match parse_req(&decoded) {
                Some(req) => {
                    route(req, Response::new(s), &router).await;
                }
                _ => {
                    Response::new(s).send(HTTP_RESPONSE::_NOT_FOUND).await;
                }
            }
        })
        .await;
}

fn parse_req(req_as_str: &str) -> Option<Request> {
    if &req_as_str == &"" { return None; }
    let mut path_with_params: String = Regex::new(r" /[\w|/]+").unwrap().find_iter(req_as_str).map(|x| x.as_str().trim().replace("&", "")).collect();
    let method = Regex::new(r"(GET|POST|PUT|DELETE)").unwrap().captures(req_as_str).unwrap().get(1).map_or("", |m| m.as_str()).to_owned();

    let params: String = Regex::new(r"(\?|&|\r\n)\w+=\w+").unwrap().find_iter(req_as_str).map(|x| x.as_str().to_owned()).collect();
    if path_with_params == "" {
        path_with_params = "/".to_string()
    }
    let mut req = Request::new(&path_with_params, &method, req_as_str);
    //headers
    let header = req_as_str.split("\r\n").collect::<Vec<&str>>();

    for s in header {
        let mut splitter = s.splitn(2, ':');
        let first = splitter.next();
        let second = splitter.next();
        match (first, second) {
            (Some(first), Some(second)) => { req.add_header(first.trim(), second.trim()) }
            _ => {}
        }
    }


    let s: Vec<String> = Regex::new(r"\w+=\w+").unwrap().find_iter(&params).map(|x| x.as_str().replace("&", "").replace("?", "").replace("\r\n", "")).collect();
    for cap in s {
        let splits: Vec<&str> = cap.split("=").collect();
        req.add_param(splits[0], splits[1]);
    }
    return Some(req);
}