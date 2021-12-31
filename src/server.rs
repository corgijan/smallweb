use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use regex::Regex;
use std::collections::HashMap;
use crate::route;
type HTTPMETHOD =String;


#[derive(Debug)]
pub struct Router{
    pub paths: HashMap<(HTTPMETHOD,String),fn(Request)->String>,
    pub dyn_paths:Vec<(String,Regex,Vec<String>)>
}
impl Router{
    pub fn new()->Router{
        Router{paths:HashMap::new(),dyn_paths:Vec::new()}
    }
    pub fn get(&mut self,path:&str,f:fn(Request)->String)->&mut Router{
        self.add_route(path,f,"GET".to_string());
        self
    }
    pub fn post(&mut self,path:&str,f:fn(Request)->String)->&mut Router{
        self.add_route(path,f,"POST".to_string());
        self
    }
    pub fn add_route(&mut self,path:&str,f:fn(Request)->String,method:HTTPMETHOD){
        let params =Regex::new(r":\w+").unwrap().find_iter(path).map(|x| x.as_str().to_owned()).collect::<Vec<String>>();
        if params.len()>0{
            let mut regex_withparams = path.to_string();
            for n in &params {
                regex_withparams = regex_withparams.replace(n, &format!(r#"(?P<{name}>\w+)/?"#, name=n.replace(":", "")));
            }
            self.dyn_paths.push((path.to_string(),Regex::new(&regex_withparams).unwrap(),params));
            self.paths.insert((method,path.to_string()),f);
        }else{
            self.paths.insert((method,path.to_string()),f);
        }
    }

    pub fn matches_dyn_route(&self, requested_path:&str) -> Option<(&String, HashMap<String,String>)> {
        for (key, regex,params) in &self.dyn_paths {
            let match_group_path =regex.find_iter(requested_path).map(|x| x.as_str().to_owned()).collect::<Vec<String>>();
            if match_group_path.get(0)?.len()==requested_path.len() {
                let mut map =HashMap::new();
                regex.captures(requested_path).and_then(|cap| {
                    for item in params{
                        let key =&*item.replace(":", "");
                        map.insert(key.to_string(), cap.name(key).map(|login| login.as_str().to_owned()).unwrap());
                    }
                    Some(1)//needs return?
                });
                return Some((&key,map));
            }
        }
        None
        }
    }



#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method:HTTPMETHOD,
    pub values: HashMap<String,String>,
    pub url_params: HashMap<String,String>,
    pub complete:String
}
impl Request {
    pub fn new(path:&str,method:&str,complete:&str)-> Request {
        Request {path: path.to_owned(),method: method.to_owned(),values: HashMap::new(),complete:complete.to_string(),url_params:HashMap::new()}
    }
    pub fn add(&mut self,key:&str,value:&str){
        self.values.insert(key.to_string(),value.to_string());
    }

    pub  fn set_url_params(&mut self, map:HashMap<String,String>)  {
        self.url_params= map;
    }
}

#[derive(Debug)]
pub struct Response {
    pub response_text: String,
    pub stream:TcpStream
}

impl Response {
    pub fn new(stream:TcpStream)-> Response {
        Response { response_text:"".to_string(), stream:stream }
    }
    pub fn set_resp(&mut self, msg:&str){
        self.response_text = msg.to_string()
    }
    pub fn send(mut self){
        self.stream.write(self.response_text.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}

pub fn serve(address:&str, router:&mut Router){
    let listener = TcpListener::bind(address).unwrap();
    for stream_in in listener.incoming() {
        let mut stream = stream_in.unwrap();
        let mut buffer = [0; 1024];
        let am = stream.read(&mut buffer).unwrap();
        let  txt=  str::from_utf8(&buffer[..am]).unwrap();
        let req = parse_req(txt);
        route(dbg!( req),Response::new(stream),&router);
    }
}

pub fn parse_req(req_as_str:&str)->Request{
    let mut path_with_params: String = Regex::new(r"(GET|POST|PUT|DELETE) /[\w|/]+").unwrap().find_iter(req_as_str).map(|x| x.as_str().replace("GET ", "").replace("DELETE", "").replace("POST ", "").replace("PUT ", "").replace("&", "")).collect();
    let method: String = Regex::new(r"(GET|POST|PUT|DELETE)").unwrap().find_iter(req_as_str).map(|x| x.as_str().to_owned()).collect();

    let params: String = Regex::new(r"(\?|&|\r\n)\w+=\w+").unwrap().find_iter(req_as_str).map(|x| x.as_str().to_owned()).collect();
    if path_with_params==""{
        path_with_params= "/".to_string()
    }
    let mut req = Request::new(&path_with_params, &method, req_as_str);

    let s: Vec<String> = Regex::new(r"\w+=\w+").unwrap().find_iter(&params).map(|x| x.as_str().replace("&", "").replace("?", "").replace("\r\n", "")).collect();
    for cap in s {
        let splits: Vec<&str> = cap.split("=").collect();
        req.add(splits[0], splits[1]);

    }
    //println!("{}", req.get("h").unwrap());
    return req;
}