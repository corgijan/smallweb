use smallweb::*;
use smallweb::HttpResponse::*;
use std::thread;


fn main(){
    let router = Router::new().get("/hello",|r:Request| OK("Hello".to_string()))
        .validator(|r:Request| Some(r))
        .default(OK("NOT FOUND".to_string()))
        .get("/saymyname", hello_params) // responds to /saymyname&name=jan
        .get("/:name", hello_url_param)
        .get("/",|r:Request| OK("<h1>HELLO from the BASE</h1>".to_string()))
        .get("/:name/:aaa",|r:Request| OK(
            format!("<h1>HELLO {} {}</h1>",r.url_params.get("name").unwrap(),r.url_params.get("aaa").unwrap()))
        )
        .thradpool_size(4)
        .okay();

    serve("127.0.0.1:7000",router);
}

fn hello_params(r:Request) -> HttpResponse{
    OK(format!("Hello {} </h1> by request param</p>", r.params.get("name").unwrap()))
}

fn hello_url_param(r:Request) -> HttpResponse {
    OK( format!("Hello {}", r.url_params.get("name").unwrap()))
}


#[test]
fn t(){
   assert_eq!(3,3);
}
