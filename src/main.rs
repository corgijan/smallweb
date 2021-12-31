

#[path = "routes.rs"] mod routes;
use routes::*;
use routes::server::*;


fn main(){
    serve("127.0.0.1:7000",
          Router::new().get("/:name/:alter/alter",hello)
              .get("/",hello2)
              .post("/",hello3)
    );
}

fn hello(r:Request)->String{
    format!("NAME {} ALTER {}",r.url_params.get("name").unwrap(),r.url_params.get("alter").unwrap())
}
fn hello2(r:Request)->String{
  "GET /".to_owned()
}

fn hello3(r:Request)->String{
    "post".to_owned()
}

#[test]
fn t(){
   assert_eq!(3,3);
}
