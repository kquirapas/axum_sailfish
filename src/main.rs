use axum::{extract::State, response::Html, routing::get, Router};
use std::sync::Arc;

use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "hello.stpl")]
struct HelloTemplate {
    messages: Vec<String>,
}

// #[tokio::main]
// async fn main() {
//     // build our application with a single route
//     let ctx = HelloTemplate {
//         messages: vec![String::from("foo"), String::from("bar")],
//     };
//
//     let app = Router::new()
//         .route("/", get(handler(State(html_string)).await))
//
//     // run our app with hyper, listening globally on port 3000
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }
#[tokio::main]
async fn main() {
    let ctx = HelloTemplate {
        messages: vec![String::from("foo"), String::from("bar")],
    };

    let html_string = ctx.render_once().unwrap();

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { Html(html_string) }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
