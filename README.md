# Smallweb development showcase

**smallweb** is a small rust based web framework that i am building to learn Rust. 

It implements a subset of HTTP 1.1 and is fairly easy setup. 

```rust
fn main(){
	serve("127.0.0.1:7000",
          Router::new().get("/h",|r:Request|_OK("Hello".to_string()))
              .validator(|r:Request|Some(r))
              .default(_OK("NOT FOUND".to_string()))
              //here hello is a function with type fn(Request)->HTTP_RESPONSE
              .get("/:name",hello)
              .get("/",|a:Request| _OK("<h1>HELLOW</h1>".to_string()))
			  //get will be reworked to use url-params as closure params
              .thradpool_size(16)
    );

}
fn hello(r:Request)->HTTP_RESPONSE{
    _OK((format!(
"<h1> Hi {}</h1><p> My name is jan</p><details>
", r.url_params.get("name").unwrap())))
}
```
Documentation will follow as soon as I am happy with the current status.

```

#[derive(Debug)]
#[derive(Clone)]
pub enum HTTP_RESPONSE{
    _OK(String),
    _REDIRECT(String),
    _UNAUTHORIZED,
    _NOT_FOUND,
    _OK_with_header(String, HashMap<String,String>)
}
```

