use axum::{
    Extension, Router,
    extract::{Request, State},
    http::{HeaderMap, StatusCode, request},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, migrate::MigrateDatabase, pool};
use tower_http::services::{ServeDir, ServeFile};

async fn testres(headers: HeaderMap, req: Request) -> Html<String> {
    let mut s = String::new();

    for (k, v) in headers.iter() {
        s.push_str(&format!("{}: {} <br />", k, v.to_str().unwrap()));
    }

    Html(format!("<h1>ip should be <br /> {}</h1>", s))
}

fn init_endpoints(pool: Pool<Sqlite>) -> Router {
    let static_files = ServeDir::new("./static");
    Router::new()
        .route_service("/", ServeFile::new("./static/index.html"))
        .route("/testres", get(testres))
        .nest_service("/static", static_files)
        .route("/new", post(add_question))
        .with_state(pool)
}

// db
#[derive(sqlx::FromRow, Serialize)]
struct Question {
    id: i32,
    eqimg: Vec<u8>,
    msimg: Vec<u8>,
    difficulty: u8,
    tags: String,
}

async fn db() -> Pool<Sqlite> {
    let opt = sqlx::sqlite::SqliteConnectOptions::new()
        .filename("db.db")
        .create_if_missing(true);

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(opt)
        .await
        .unwrap();

    let x = sqlx::query(
        "CREATE TABLE IF NOT EXISTS questions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            eqimg BLOB NOT NULL,
            msimg BLOB NOT NULL,
            difficulty INTEGER NOT NULL,
            tags TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

async fn add_question(State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    println!("add_question() called");

    let dummyblob: Vec<u8> = (0..255).map(|i| i as u8).collect();
    let dummyblob2: Vec<u8> = (0..255).map(|i| i as u8).collect();

    sqlx::query(
        "INSERT INTO questions (eqimg, msimg, difficulty, tags)
        VALUES (?, ?, ?, ?);",
    )
    .bind(dummyblob)
    .bind(dummyblob2)
    .bind(4)
    .bind("test,asdfasdf,yuhhh")
    .execute(&pool)
    .await
    .unwrap();

    (StatusCode::OK, "ja").into_response();
}

// main
#[tokio::main]
async fn main() {
    println!("initing...");

    let pool = db().await;

    let app = init_endpoints(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("server up");

    axum::serve(listener, app).await.unwrap();
}
