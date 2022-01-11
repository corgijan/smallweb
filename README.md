# Smallweb development showcase

**smallweb** is a minimal Rust based web framework that I am implementing to learn Rust. 

It implements a subset of HTTP 1.1 and is fairly to easy setup.
It is just dependant on following crates: regex, urlencoding and threadpool.

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
"<h1> Hi {}</h1><p> My name is jan</p>", r.url_params.get("name").unwrap())))
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

