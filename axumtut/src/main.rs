use axum::{
    Router, ServiceExt,
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, get_service},
};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(all_routes());
    // .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn all_routes() -> Router {
    Router::new().route("/hello", get(hello))
    // .route("/helloparam/{name}", get(hello_pathparam))
}

// fn routes_static() -> Router {
//     Router::new().nest_service("/", get_service(ServeDir::new("./").into_make_service()))
// }

// fn routes_static() -> Router {
//     Router::new().nest_service(
//         "/",
//         get_service(ServeDir::new("./")).handle_error(|error| async move {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("Static file error: {}", error),
//             )
//         }),
//     )
// }

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("Hello endpoint");
    let name = params.name.unwrap_or(String::from("World"));
    Html(format!("<h1>Hello {name}</h1>"))
}

async fn hello_pathparam(Path(name): Path<String>) -> impl IntoResponse {
    println!("Api: hellopathparam");

    Html(format!("<h1>Hello {name}</h1>"))
}
