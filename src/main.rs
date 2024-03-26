//!
//! Shorten an url:
//!
//! curl --header "Content-Type: application/json" \
//!  --request POST \
//!  --data '{"url":"http://whalar.com"}' \
//!  http://localhost:3000/api/v1/tiny
//!
//! Retrieve the long url:
//!
//! curl -v --request GET http://localhost:3000/api/v1/tiny\?url\="0fe5e13014"
//!
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sha256::digest;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type Db = Arc<RwLock<HashMap<String, String>>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ShortenUrl {
    url: String,
}

#[derive(Debug, Deserialize)]
struct RetrieveParams {
    url: String,
}

fn hash_sha256(url: String) -> String {
    let mut val = digest(url);
    val.truncate(10);
    val
}

async fn shorten(State(db): State<Db>, Json(input): Json<ShortenUrl>) -> impl IntoResponse {
    // Compute teh hash for the incoming url
    let hash = hash_sha256(input.url.clone());

    // Save in the database the pair (hash, long_url)
    db.write().unwrap().insert(hash.clone(), input.url);

    // Returns the created status code.
    (StatusCode::CREATED, format!("{hash}"))
}

enum RetrieveResponse {
    RedirectT(String),
    SomethingWentWrong,
}

impl IntoResponse for RetrieveResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            RetrieveResponse::RedirectT(uri) => {
                let res = Redirect::temporary(uri.as_str());
                res.into_response()
            }
            RetrieveResponse::SomethingWentWrong => {
                let res = (StatusCode::OK, "".to_string());
                res.into_response()
            }
        }
    }
}

async fn retrieve(Query(params): Query<RetrieveParams>, State(db): State<Db>) -> impl IntoResponse {
    let urls = db.read().unwrap();

    // Retrieve the long url based on the received key
    let res = urls.get_key_value(&params.url);

    if let Some((_key, long_url)) = res {
        RetrieveResponse::RedirectT(long_url.clone())
    } else {
        RetrieveResponse::SomethingWentWrong
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aabel_tiny=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Db::default();

    // Compose the routes
    let app = Router::new()
        // Add the routes
        .route("/api/v1/tiny", get(retrieve).post(shorten))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
