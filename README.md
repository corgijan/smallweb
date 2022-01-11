# Smallweb development showcase

**smallweb** is a small rust based web framework that i am building to learn Rust. 

It implements a subset of HTTP 1.1 and is fairly easy setup. 

```
    serve("127.0.0.1:7000",
          Router::new().get("/h",|r:Request|_OK("Hello".to_string()))
              .validator(|r:Request|Some(r))
              .default(_OK("NOT FOUND".to_string()))
              //here hello is a function with type fn(Request)->HTTP_METHOD
              .get("/:name",hello)
              .get("/",|a:Request| _OK("<h1>HELLOW</h1>".to_string()))
              .thradpool_size(16)
    );

```
Documentation will follow as soon as I am happy with the current status.
