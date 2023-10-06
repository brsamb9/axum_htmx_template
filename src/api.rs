use axum::{
    extract::State,
    headers::UserAgent,
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Sse,
    },
    routing::get,
    Json, Router, TypedHeader,
};

use futures_util::Stream;

use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use strum::AsRefStr;
use surrealdb::{engine::local::Db, Surreal};
use tokio::sync::broadcast;
use tokio_stream::StreamExt as _;

use crate::db_client::SurrealDbHelperFuncs;

pub fn get_api_router<S>(state: Arc<AppState>) -> Router<S> {
    Router::new()
        .route("/click", get(click_event))
        .route("/sse", get(sse_handler))
        .with_state(state)
}

pub struct AppState {
    pub db: Surreal<Db>,
    pub tx: broadcast::Sender<AppEvents>,
    pub rx: broadcast::Receiver<AppEvents>,
    pub statistics: Mutex<WebScrapingStatistics>,
}

impl AppState {
    pub async fn new() -> Self {
        // Can I improve this by using a MPSC channel (can't seem to get the streams working in a similiar manner)
        let (tx, rx) = broadcast::channel::<AppEvents>(20);

        AppState {
            db: SurrealDbHelperFuncs::surrealdb_init().await.unwrap(),
            tx,
            rx,
            statistics: Mutex::new(WebScrapingStatistics::default()),
        }
    }
}

#[derive(Debug, Default)]
pub struct WebScrapingStatistics {
    websites_scrapped: usize,
}

#[derive(Debug, Clone, AsRefStr)]
pub enum AppEvents {
    WebsiteScraped,
}

// #[axum::debug_handler]
async fn click_event(State(shared_state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut stats = shared_state.statistics.lock().unwrap();
    shared_state.tx.send(AppEvents::WebsiteScraped).unwrap();
    stats.websites_scrapped += 1;
    Json("One more time!").into_response()
}

async fn sse_handler(
    State(shared_state): State<Arc<AppState>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("{} broadcast connected", user_agent.as_str());
    let rx = shared_state.rx.resubscribe();
    let stream = tokio_stream::wrappers::BroadcastStream::new(rx);

    Sse::new(
        stream
            .map(move |msg| {
                let msg = msg.unwrap();
                let data = {
                    let stats = shared_state.statistics.lock().unwrap();
                    match &msg {
                        AppEvents::WebsiteScraped => stats.websites_scrapped.to_string(),
                    }
                };
                // json_data seems cool
                Event::default().event(&msg).data(data)
            })
            .map(Ok),
    )
    .keep_alive(KeepAlive::default())
}
