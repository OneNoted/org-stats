mod cache;
mod cards;
mod github;
mod theme;

use axum::extract::{Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;

struct AppState {
    client: Client,
    token: String,
    cache: cache::Cache,
}

#[derive(Deserialize)]
struct CardParams {
    org: Option<String>,
    theme: Option<String>,
    hide_border: Option<String>,
    langs_count: Option<usize>,
}

impl CardParams {
    fn hide_border(&self) -> bool {
        self.hide_border
            .as_deref()
            .is_some_and(|v| v == "true" || v == "1")
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "org_stats=info".parse().unwrap()),
        )
        .init();

    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());

    let state = Arc::new(AppState {
        client: Client::new(),
        token,
        cache: cache::Cache::new(),
    });

    let app = Router::new()
        .route("/api/stats", get(stats_handler))
        .route("/api/langs", get(langs_handler))
        .route("/api/activity", get(activity_handler))
        .route("/health", get(health_handler))
        .with_state(state);

    let addr = format!("0.0.0.0:{port}");
    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_org_data(state: &AppState, org: &str) -> Result<github::OrgData, String> {
    let cache_key = org.to_string();

    if let Some(data) = state.cache.get(&cache_key).await {
        tracing::debug!("cache hit for {org}");
        return Ok(data);
    }

    tracing::info!("cache miss for {org}, fetching from GitHub");
    match github::fetch_org_data(&state.client, &state.token, org).await {
        Ok(data) => {
            state.cache.set(cache_key, data.clone()).await;
            Ok(data)
        }
        Err(e) => {
            tracing::warn!("GitHub API error for {org}: {e}");
            if let Some(stale) = state.cache.get_stale(&cache_key).await {
                tracing::info!("serving stale cache for {org}");
                Ok(stale)
            } else {
                Err(e)
            }
        }
    }
}

fn svg_response(svg: String) -> Response {
    (
        [
            (header::CONTENT_TYPE, "image/svg+xml"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        svg,
    )
        .into_response()
}

fn error_svg(msg: &str) -> String {
    let theme = theme::get_theme(None);
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="495" height="120" viewBox="0 0 495 120">
  <style>text {{ font-family: 'Segoe UI', Ubuntu, 'Helvetica Neue', sans-serif; }}</style>
  <rect x="0.5" y="0.5" width="494" height="119" rx="4.5" fill="{}" stroke="{}"/>
  <text x="247" y="45" fill="{}" font-size="14" text-anchor="middle" font-weight="bold">Error</text>
  <text x="247" y="75" fill="{}" font-size="12" text-anchor="middle">{msg}</text>
</svg>"#,
        theme.bg, theme.surface1, theme.red, theme.subtext
    )
}

async fn stats_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CardParams>,
) -> Response {
    let Some(org) = params.org.as_deref() else {
        return svg_response(error_svg("Missing 'org' query parameter"));
    };

    match get_org_data(&state, org).await {
        Ok(data) => {
            let t = theme::get_theme(params.theme.as_deref());
            svg_response(cards::stats::render(&data, org, t, params.hide_border()))
        }
        Err(e) => svg_response(error_svg(&e)),
    }
}

async fn langs_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CardParams>,
) -> Response {
    let Some(org) = params.org.as_deref() else {
        return svg_response(error_svg("Missing 'org' query parameter"));
    };

    match get_org_data(&state, org).await {
        Ok(data) => {
            let t = theme::get_theme(params.theme.as_deref());
            let count = params.langs_count.unwrap_or(8);
            svg_response(cards::langs::render(&data, org, t, params.hide_border(), count))
        }
        Err(e) => svg_response(error_svg(&e)),
    }
}

async fn activity_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CardParams>,
) -> Response {
    let Some(org) = params.org.as_deref() else {
        return svg_response(error_svg("Missing 'org' query parameter"));
    };

    match get_org_data(&state, org).await {
        Ok(data) => {
            let t = theme::get_theme(params.theme.as_deref());
            svg_response(cards::activity::render(&data, org, t, params.hide_border()))
        }
        Err(e) => svg_response(error_svg(&e)),
    }
}

async fn health_handler() -> &'static str {
    "ok"
}
