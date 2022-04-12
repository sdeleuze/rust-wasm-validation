use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::Redirect,
    routing::{get, get_service},
    Json, Router,
};
use common::Person;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(redirect))
        .route("/person", get(person))
        .nest(
            "/static",
            get_service(ServeDir::new("./client")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn redirect() -> Redirect {
    Redirect::permanent("/static")
}

async fn person() -> Json<Person> {
    let person = Person {
        firstname: String::from("Sébastien"),
        lastname: String::from("Deleuze"),
        age: 40,
    };
    person.validate().unwrap();
    Json(person)
}
