//! REST API for the SAMP Query library.

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use samp_query::Client;
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

mod error;
use error::ApiError;

#[derive(Clone)]
struct AppState {
    //add any shared state here
}

#[derive(Deserialize)]
struct ServerAddress {
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let state = Arc::new(AppState {});

    let app = Router::new()
        .route("/", get(root))
        .route("/api/v1/servers/:address/info", get(get_server_info))
        .route("/api/v1/servers/:address/rules", get(get_server_rules))
        .route("/api/v1/servers/:address/players", get(get_player_list))
        .route(
            "/api/v1/servers/:address/players/detailed",
            get(get_detailed_player_list),
        )
        .route("/api/v1/servers/:address/ping", get(get_server_ping))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "SAMP Query API"
}

async fn get_server_info(
    Path(ServerAddress { address }): Path<ServerAddress>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<samp_query::ServerInfo>, ApiError> {
    let addr: SocketAddr = address.parse()?;
    let client = Client::connect(addr).await?;
    let info = client.query_info().await?;
    Ok(Json(info))
}

async fn get_server_rules(
    Path(ServerAddress { address }): Path<ServerAddress>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<samp_query::ServerRules>, ApiError> {
    let addr: SocketAddr = address.parse()?;
    let client = Client::connect(addr).await?;
    let rules = client.query_rules().await?;
    Ok(Json(rules))
}

async fn get_player_list(
    Path(ServerAddress { address }): Path<ServerAddress>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<samp_query::PlayerList>, ApiError> {
    let addr: SocketAddr = address.parse()?;
    let client = Client::connect(addr).await?;
    let players = client.query_client_list().await?;
    Ok(Json(players))
}

async fn get_detailed_player_list(
    Path(ServerAddress { address }): Path<ServerAddress>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<samp_query::DetailedPlayerList>, ApiError> {
    let addr: SocketAddr = address.parse()?;
    let client = Client::connect(addr).await?;
    let players = client.query_detailed_player_info().await?;
    Ok(Json(players))
}

async fn get_server_ping(
    Path(ServerAddress { address }): Path<ServerAddress>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<samp_query::PingInfo>, ApiError> {
    let addr: SocketAddr = address.parse()?;
    let client = Client::connect(addr).await?;
    let ping = client.query_ping().await?;
    Ok(Json(ping))
}
