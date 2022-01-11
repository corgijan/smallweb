use std::collections::HashMap;
use smallweb::*;
use smallweb::HTTP_RESPONSE::*;
use std::thread;

fn main(){
    serve("127.0.0.1:7000",
          Router::new().get("/hello",|r:Request|_OK("Hello".to_string()))
              .validator(|r:Request|Some(r))
              .default(_OK("NOT FOUND".to_string()))
              .get("/:name",helloName)
              .get("/",|a:Request| _OK("<h1>HELLOW</h1>".to_string()))
              .thradpool_size(16)
    );
}

fn hello(r:Request)->HTTP_RESPONSE{
    _OK((format!(
"<h1>{}</h1><p> My name is jan</p><details>
<summary>These are details</summary>
<p>Im A detail</p>
</details>
", r.url_params.get("name").unwrap())))
}


fn helloName(r:Request)->HTTP_RESPONSE{
    _OK( "smallweb!".to_owned())
}


#[test]
fn t(){
   assert_eq!(3,3);
}
