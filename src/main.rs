use axum::{
    Router,
    extract::Request,
    http::{HeaderMap, request},
    response::Html,
    routing::get,
};
use tower_http::services::{ServeDir, ServeFile};

async fn testres(headers: HeaderMap, req: Request) -> Html<String> {
    let mut s = String::new();

    for (k, v) in headers.iter() {
        s.push_str(&format!("{}: {} <br />", k, v.to_str().unwrap()));
    }

    Html(format!("<h1>ip should be <br /> {}</h1>", s))
}

fn init_endpoints() -> Router {
    let static_files = ServeDir::new("./static");
    Router::new()
        .route_service("/", ServeFile::new("./static/index.html"))
        .route("/testres", get(testres))
        .nest_service("/static", static_files)
}

#[tokio::main]
async fn main() {
    println!("initing...");

    let app = init_endpoints();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("server up");

    axum::serve(listener, app).await.unwrap();
}
