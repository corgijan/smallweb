

#[path = "server.rs"]
pub(crate) mod server;

pub use server::{Request};
use crate::{Response, Router};


pub(crate) fn route(mut req: Request, mut resp: Response, router: &Router) {
    //check if route is dynamic
    match  router.paths.get(&(req.method.clone(), req.path.clone())){
                //check if it ia static reuqest
                Some(f)=>{
                    let response_txt = f(req);
                    let response_with_http = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}", response_txt.len(), response_txt);
                    resp.set_resp(&response_with_http);
                    resp.send();
                },
                None=>{
                            //check if it is in dyn routing
                            match  router.matches_dyn_route(&req.path){
                                Some((route, params))=>{
                                    req.set_url_params(params);
                                    let response_txt = router.paths.get(&(req.method.clone(), route.clone())).unwrap_or(&{ |_r: Request| "404 DYN".to_string() })(req);
                                    let response_with_http = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}", response_txt.len(), response_txt);
                                    resp.set_resp(&response_with_http);
                                    resp.send();
                                },
                                _=>{
                                    //default behaviour
                                    let response_txt ="404";
                                    let response_with_http = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}", response_txt.len(), response_txt);
                                    resp.set_resp(&response_with_http);
                                    resp.send();
                                }
                             }
                }
    }
}


