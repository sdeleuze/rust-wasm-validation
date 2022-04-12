use std::net::SocketAddr;

use axum::{response::Html, routing::get, Json, Router};
use common::Person;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/person", get(person));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> Html<&'static str> {
    Html("<h1>hello, world!</h1>")
}

async fn person() -> Json<Person> {
    Json(Person {
        firstname: String::from("Sébastien"),
        lastname: String::from("Deleuze"),
        age: 40,
    })
}
