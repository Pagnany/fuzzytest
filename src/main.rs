use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::prelude::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

mod article_search;

#[tokio::main]
async fn main() {
    let art_all: article_search::ArtikelListe = Default::default();
    let shared_state = Arc::new(RwLock::new(art_all));
    let reload_state = Arc::clone(&shared_state);

    tokio::spawn(async move {
        let mut first_update = true;
        loop {
            if first_update {
                first_update = false;
            } else {
                sleep(Duration::from_secs(60 * 1)).await;
            }
            let start_time_reload = Local::now();

            if let Ok(data) = fs::read_to_string("./json/artikel.json") {
                let data: String = data
                    .chars()
                    .filter(|c| (c >= &'!' && c <= &'ÿ') || c == &' ')
                    .collect();

                if let Ok(artikel_list) = article_search::ArtikelListe::from_json(&data) {
                    let mut write_guard = reload_state.write().await;
                    *write_guard = artikel_list;

                    let end_time_reload = Local::now();
                    let duration = end_time_reload.signed_duration_since(start_time_reload);
                    tracing::info!(
                        "RELOAD artikel.json took: {}ms",
                        duration.num_milliseconds()
                    );
                } else {
                    tracing::error!("RELOAD Failed artikel.json parse");
                }
            } else {
                tracing::error!("RELOAD Failed artikel.json file");
            }
        }
    });

    println!("Starting server");
    tracing_subscriber::fmt::init();
    // Setup webserver
    let app = Router::new()
        .route("/", get(root))
        .route("/search", post(search_article))
        .route("/search/merkmal", post(search_merkmal))
        .route("/search/matchcode", post(search_matchcode))
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

    let search_string = query.search.to_lowercase();
    let mut art_out: Vec<article_search::ArtOutput> = Vec::new();
    let matcher = SkimMatcherV2::default();

    {
        let art_all = artikel_list.read().await;
        for a in &art_all.artikel_liste {
            let art_str = a.get_string();
            match matcher.fuzzy_match(&art_str, &search_string) {
                Some(score) => {
                    art_out.push(article_search::ArtOutput::new(
                        a.satz_id.clone(),
                        a.bezeich01.clone(),
                        score,
                    ));
                }
                None => {}
            }
        }
    }

    art_out.sort_by(|a, b| b.score.cmp(&a.score));
    art_out.truncate(100);

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    tracing::info!("Search took: {}ms", duration.num_milliseconds());

    (StatusCode::OK, Json(art_out))
}

async fn search_merkmal(
    State(artikel_list): State<Arc<RwLock<article_search::ArtikelListe>>>,
    Json(query): Json<article_search::ArtInput>,
) -> (StatusCode, Json<Vec<article_search::ArtOutput>>) {
    let start_time = Local::now();
    tracing::info!("Searching for Merkmal: {}", query.search);

    let search_string = query.search.to_lowercase();
    let mut art_out: Vec<article_search::ArtOutput> = Vec::new();
    let matcher = SkimMatcherV2::default();

    {
        let art_all = artikel_list.read().await;
        for a in &art_all.artikel_liste {
            let art_str = a.get_string_merkmale();
            match matcher.fuzzy_match(&art_str, &search_string) {
                Some(score) => {
                    art_out.push(article_search::ArtOutput::new(
                        a.satz_id.clone(),
                        a.bezeich01.clone(),
                        score,
                    ));
                }
                None => {}
            }
        }
    }

    art_out.sort_by(|a, b| b.score.cmp(&a.score));
    art_out.truncate(100);

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    tracing::info!("Search for Merkmal took: {}ms", duration.num_milliseconds());

    (StatusCode::OK, Json(art_out))
}

async fn search_matchcode(
    State(artikel_list): State<Arc<RwLock<article_search::ArtikelListe>>>,
    Json(query): Json<article_search::ArtInput>,
) -> (StatusCode, Json<Vec<article_search::ArtOutput>>) {
    let start_time = Local::now();
    tracing::info!("Searching for Matchcode: {}", query.search);

    let search_string = query.search.to_lowercase();
    let mut art_out: Vec<article_search::ArtOutput> = Vec::new();
    let matcher = SkimMatcherV2::default();

    {
        let art_all = artikel_list.read().await;
        for a in &art_all.artikel_liste {
            let art_str = a.get_string_matchcode();
            if art_str.trim().is_empty() {
                continue;
            }
            match matcher.fuzzy_match(&art_str, &search_string) {
                Some(score) => {
                    art_out.push(article_search::ArtOutput::new(
                        a.satz_id.clone(),
                        a.bezeich01.clone(),
                        score,
                    ));
                }
                None => {}
            }
        }
    }

    art_out.sort_by(|a, b| b.score.cmp(&a.score));
    art_out.truncate(100);

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    tracing::info!(
        "Search for Matchcode took: {}ms",
        duration.num_milliseconds()
    );

    (StatusCode::OK, Json(art_out))
}
