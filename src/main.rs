use salvo::{prelude::*, extra::compression, extra::serve_static::{FileHandler, DirHandler}};

#[fn_handler]
async fn hello_world() -> &'static str{
    "Hello, World"
}

#[tokio::main]
async fn main() {
    //let router = Router::new().get(hello_world);
    let router = Router::new().push(
        Router::new()
            .path("/<*path>")
            .get(DirHandler::new("spa/"))
    ).push(
        Router::new()
            .path("public/<*path>")
            .get(DirHandler::new("public/"))
    ).push(
        Router::new()
            .path("hello")
            .get(hello_world)
    );

    Server::new(TcpListener::bind("127.0.0.1:3000")).serve(router).await;
}
