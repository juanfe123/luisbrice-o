use std::{fs, path::PathBuf};

use axum::response::IntoResponse;
use axum::{Router, routing::get, response::Html, extract::Path, http::{HeaderMap, HeaderValue, StatusCode},};
use askama::Template;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IT {
    content: String,
    content0: String,
}

async fn render() -> Html<String> {
    let html_cont = fs::read_to_string("templates/abilidad1.html").expect("E");
    let html_cont0 = fs::read_to_string("templates/vida.html").expect("E");
    let template = IT{content: html_cont, content0: html_cont0};
    Html(template.render().unwrap())
}

async fn porta_f(Path(filename): Path<String>) -> impl IntoResponse {
    let archivo = PathBuf::from("cargar").join(&filename);

    match tokio::fs::read(&archivo).await {
        Ok(data_file) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                "Content-Disposition",
                HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename)).unwrap(),
            );
            (StatusCode::OK, headers, data_file).into_response()
        }
        Err(_) => {(StatusCode::NOT_FOUND, HeaderMap::new(), Vec::new()).into_response()}
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let app = Router::new()
      .nest_service("/img", ServeDir::new("./img"))
      .route("/", get(render))
      .route("/cargar/:filename", get(porta_f))
      .layer(CompressionLayer::new());
    let escucha = tokio::net::TcpListener::bind("127.0.0.1:3050").await.unwrap();
    axum::serve(escucha, app).await.unwrap();
}
