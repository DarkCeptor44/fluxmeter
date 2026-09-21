// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod types;

use crate::{
    VERSION,
    server::{
        api::v1::types::{DownloadParams, HealthResponse},
        utils::Service,
    },
};
use async_stream::stream;
use axum::{
    Json, Router,
    body::Body,
    extract::Query,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use bytes::Bytes;
use chrono::Local;
use futures::StreamExt;
use log::debug;
use std::sync::Arc;
use utoipa::OpenApi;

const CHUNK_SIZE: usize = 64 * 1024; // 64KB
static ZERO_CHUNK: [u8; CHUNK_SIZE] = [0u8; CHUNK_SIZE];

#[derive(OpenApi)]
#[openapi(
    info(title = "Fluxmeter API", version = "1.0.0"),
    paths(download, health, ping, upload),
    components(schemas(DownloadParams, HealthResponse)),
    tags(
        (name = "Speedtest", description = "Speed test endpoints")
    )
)]
pub struct ApiDocV1;

pub fn routes() -> Router<Arc<Service>> {
    Router::new()
        .route("/download", get(download))
        .route("/health", get(health))
        .route("/ping", get(ping))
        .route("/upload", post(upload))
}

/// Download endpoint
#[utoipa::path(
    get,
    path = "/api/v1/download",
    params(DownloadParams),
    responses(
        (status = 200, description = "Download successful", content_type = "application/octet-stream")
    ),
    tag = "Speedtest"
)]
async fn download(Query(params): Query<DownloadParams>) -> impl IntoResponse {
    debug!("download requested: {params:?}");

    let total_bytes = params.bytes.unwrap_or(10_000_000); // 10MB default
    let static_chunk = Bytes::from_static(&ZERO_CHUNK);

    let download_stream = stream! {
        let mut remaining = total_bytes;
        while remaining > 0 {
            let to_send = remaining.min(CHUNK_SIZE);
            remaining -= to_send;

            if to_send == CHUNK_SIZE {
                yield Ok::<_, std::io::Error>(static_chunk.clone());
            } else {
                yield Ok::<_, std::io::Error>(Bytes::copy_from_slice(&ZERO_CHUNK[..to_send]));
            }
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, no-cache, must-revalidate"),
    );
    headers.insert(header::CONTENT_LENGTH, HeaderValue::from(total_bytes));

    (StatusCode::OK, headers, Body::from_stream(download_stream)).into_response()
}

/// Health check
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse, example = json!({"status":"ok","version":"0.1.0","server_time":"2026-01-01T00:00:00+00:00"}))
    )
)]
async fn health() -> impl IntoResponse {
    let now = Local::now().to_rfc3339();
    debug!("health check at {now}");

    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok",
            version: VERSION,
            server_time: &now,
        }),
    )
        .into_response()
}

/// Ping endpoint
#[utoipa::path(
    get,
    path = "/api/v1/ping",
    responses(
        (status = 200, description = "Ping successful", body = String, example = "pong")
    ),
    tag = "Speedtest"
)]
async fn ping() -> impl IntoResponse {
    debug!("ping requested");
    (StatusCode::OK, "pong").into_response()
}

/// Upload endpoint
#[utoipa::path(
    post,
    path = "/api/v1/upload",
    request_body(
        content = Vec<u8>,
        description = "Raw dummy binary payload for upload speed testing",
        content_type = "application/octet-stream"
    ),
    responses(
        (status = 400, description = "Failed to stream request body", body = String, example = "Upload failed"),
        (status = 200, description = "Upload complete", body = String, example = "ok")
    ),
    tag = "Speedtest"
)]
async fn upload(body: Body) -> impl IntoResponse {
    debug!("upload requested: {body:?}");

    let mut stream = body.into_data_stream();
    while let Some(chunk) = stream.next().await {
        if chunk.is_err() {
            return (StatusCode::BAD_REQUEST, "Upload failed").into_response();
        }
    }

    (StatusCode::OK, "ok").into_response()
}
