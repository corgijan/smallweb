use smallweb::*;
use smallweb::HTTP_RESPONSE::*;
use std::thread;

#[async_std::main]
async fn main(){
    let router = Router::new().get("/hello",|r:Request|_OK("Hello".to_string()))
        .validator(|r:Request| Some(r))
        .default(_OK("NOT FOUND".to_string()))
        .get("/saymyname", hello_params) // responds to /saymyname&name=jan
        .get("/:name", hello_url_param)
        .get("/",|r:Request| _OK("<h1>HELLO from the BASE</h1>".to_string()))
        .okay();

    serve("127.0.0.1:7000",router).await;
}

fn hello_params(r:Request) -> HTTP_RESPONSE{
    return match r.params.get("name"){
        Some(s) => _OK( format!("Hello {s} </h1> by request param</p>")),
        _=> {_NOT_FOUND}
    }
}

fn hello_url_param(r:Request) -> HTTP_RESPONSE {
     return match r.url_params.get("name"){
        Some(s) => _OK( format!("Hello {} by url", s)),
        _=> {_NOT_FOUND}
    }
}



#[test]
fn t(){
   assert_eq!(3,3);
}
