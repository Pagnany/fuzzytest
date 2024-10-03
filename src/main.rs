use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::prelude::*;
use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

mod article_search;

#[tokio::main]
async fn main() {
    println!("Reading data");
    let start_time = Local::now();

    let art_all: article_search::ArtikelListe = Default::default();
    let shared_state = Arc::new(RwLock::new(art_all));
    let reload_state = Arc::clone(&shared_state);

    let mut first_update = true;
    tokio::spawn(async move {
        loop {
            if first_update {
                first_update = false;
            } else {
                sleep(Duration::from_secs(60 * 10)).await;
            }
            let start_time_reload = Local::now();

            if let Ok(data) = fs::read_to_string("./json/artikel.json") {
                let data: String = data
                    .chars()
                    .filter(|c| (c >= &'!' && c <= &'ÿ') || c == &' ')
                    .collect();

                let artikel_list = article_search::ArtikelListe::from_json(&data);

                let mut write_guard = reload_state.write().await;
                *write_guard = artikel_list;

                let end_time_reload = Local::now();
                let duration = end_time_reload.signed_duration_since(start_time_reload);
                tracing::info!("RELOAD artikel.json took: {:?}", duration);
            } else {
                tracing::error!("RELOAD Failed artikel.json");
            }
        }
    });

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    tracing::info!("Reading Data took: {:?}", duration);

    println!("Starting server");
    tracing_subscriber::fmt::init();
    // Setup webserver
    let app = Router::new()
        .route("/", get(root))
        .route("/search", post(search_article))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Search Service"
}

async fn search_article(
    State(artikel_list): State<Arc<RwLock<article_search::ArtikelListe>>>,
    Json(query): Json<article_search::ArtInput>,
) -> (StatusCode, Json<Vec<article_search::ArtOutput>>) {
    let start_time = Local::now();
    tracing::info!("Searching for: {}", query.search);

    let art_all = artikel_list.read().await;

    let mut art_list = article_search::article_search(&art_all, query.search.as_str());

    art_list.truncate(100);

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    tracing::info!("Search took: {:?}", duration);

    (StatusCode::OK, Json(art_list))
}
