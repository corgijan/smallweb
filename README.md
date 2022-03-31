# Smallweb development showcase

**smallweb** is a minimal Rust based web framework that I am implementing to learn Rust. 

It implements a subset of HTTP 1.1 and is fairly easy to set up.
It is dependent on following crates: regex, urlencoding and threadpool.

```rust
fn main(){
    serve("127.0.0.1:7000",
          Router::new()
              .get("/",|r:Request|_OK("Hello".to_string()))
              .get("/saymyname", hello_params) // responds to /saymyname&name=jan
              .get("/:name", hello_url_param) // responds to /<name> except /saymyname
              //.validator(|r:Request| Some(r))
              //.default(_OK("NOT FOUND".to_string()))
              //.thradpool_size(16)
    );
}

fn hello_params(r:Request) -> HTTP_RESPONSE{
    _OK((format!("Hello {} </h1> by request param</p>", r.params.get("name").unwrap())))
}

fn hello_url_param(r:Request) -> HTTP_RESPONSE {
    _OK( format!("Hello {} by url", r.url_params.get("name").unwrap()))
}
```
Proper documentation will follow as soon as I am happy with the current status.

```

#[derive(Debug)]
#[derive(Clone)]
pub enum HTTP_RESPONSE{
    _OK(String),
    _REDIRECT(String),
    _UNAUTHORIZED,
    _NOT_FOUND,
    _OK_with_header(String, HashMap<String,String>) //subject to change
}
```

