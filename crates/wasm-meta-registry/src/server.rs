//! HTTP server with JSON API endpoints for package discovery.
//!
//! Provides search and listing endpoints backed by the `wasm-package-manager`
//! known packages database.

// The `OpenApi` derive macro (from utoipa) generates code that triggers this
// lint.  We suppress it at module level because item-level `#[allow]` does
// not propagate into derive-generated code.
#![allow(clippy::needless_for_each)]

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use wasm_meta_registry_client::KnownPackage;
use wasm_package_manager::manager::Manager;

/// Shared application state wrapping a `Manager` in a `std::sync::Mutex`.
///
/// This is safe because all handler methods on `Manager` are synchronous
/// (no `.await` while holding the lock).
///
/// # Example
///
/// ```no_run
/// use wasm_meta_registry::server::AppState;
/// use wasm_package_manager::manager::Manager;
/// use std::sync::{Arc, Mutex};
///
/// # async fn example() -> anyhow::Result<()> {
/// let manager = Manager::open().await?;
/// let state: AppState = Arc::new(Mutex::new(manager));
/// # Ok(())
/// # }
/// ```
pub type AppState = Arc<std::sync::Mutex<Manager>>;

/// Query parameters for search.
///
/// # Example
///
/// ```
/// use wasm_meta_registry::server::SearchParams;
///
/// let params = SearchParams {
///     q: "wasi".to_string(),
///     offset: 0,
///     limit: 20,
/// };
///
/// assert_eq!(params.q, "wasi");
/// ```
#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct SearchParams {
    /// Search query string.
    pub q: String,
    /// Pagination offset (default: 0).
    #[serde(default)]
    pub offset: u32,
    /// Pagination limit (default: 20).
    #[serde(default = "default_limit")]
    pub limit: u32,
}

/// Query parameters for listing packages.
///
/// # Example
///
/// ```
/// use wasm_meta_registry::server::ListParams;
///
/// let params = ListParams {
///     offset: 0,
///     limit: 50,
/// };
///
/// assert_eq!(params.limit, 50);
/// ```
#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct ListParams {
    /// Pagination offset (default: 0).
    #[serde(default)]
    pub offset: u32,
    /// Pagination limit (default: 20).
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    20
}

/// JSON body returned by the health endpoint.
#[derive(Debug, Serialize, utoipa::ToSchema)]
struct HealthResponse {
    /// Service status, always `"ok"`.
    status: String,
}

/// JSON body returned when a request fails.
#[derive(Debug, Serialize, utoipa::ToSchema)]
struct ErrorResponse {
    /// Human-readable error description.
    error: String,
}

/// OpenAPI description for the wasm-meta-registry API.
#[derive(Debug, OpenApi)]
#[openapi(
    info(
        title = "wasm-meta-registry",
        description = "HTTP API for WebAssembly package discovery",
        version = "1.0.0",
    ),
    paths(health, search, list_packages, get_package, openapi_spec),
    components(schemas(
        HealthResponse,
        ErrorResponse,
        KnownPackage,
        wasm_meta_registry_client::PackageDependencyRef,
    ))
)]
pub struct ApiDoc;

/// Build the axum router with all API routes.
///
/// # Example
///
/// ```no_run
/// use wasm_meta_registry::router;
/// use wasm_package_manager::manager::Manager;
/// use std::sync::{Arc, Mutex};
///
/// # async fn example() -> anyhow::Result<()> {
/// let manager = Manager::open().await?;
/// let state = Arc::new(Mutex::new(manager));
/// let app = router(state);
///
/// let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
/// axum::serve(listener, app).await?;
/// # Ok(())
/// # }
/// ```
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/v1/health", get(health))
        .route("/v1/search", get(search))
        .route("/v1/packages", get(list_packages))
        .route("/v1/packages/recent", get(list_recent_packages))
        .route("/v1/packages/{registry}/{*repository}", get(get_package))
        .route("/v1/openapi.json", get(openapi_spec))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Health check endpoint.
#[utoipa::path(
    get,
    path = "/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

/// Search packages by query string.
#[utoipa::path(
    get,
    path = "/v1/search",
    params(SearchParams),
    responses(
        (status = 200, description = "Matching packages", body = Vec<KnownPackage>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
async fn search(
    State(manager): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<impl IntoResponse, AppError> {
    let manager = manager
        .lock()
        .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?;
    let packages = manager.search_packages(&params.q, params.offset, params.limit)?;
    Ok(Json(packages))
}

/// List all known packages.
#[utoipa::path(
    get,
    path = "/v1/packages",
    params(ListParams),
    responses(
        (status = 200, description = "Paginated list of all known packages", body = Vec<KnownPackage>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
async fn list_packages(
    State(manager): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<impl IntoResponse, AppError> {
    let manager = manager
        .lock()
        .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?;
    let packages = manager.list_known_packages(params.offset, params.limit)?;
    Ok(Json(packages))
}

/// List recently updated known packages.
async fn list_recent_packages(
    State(manager): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<impl IntoResponse, AppError> {
    let manager = manager
        .lock()
        .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?;
    let packages = manager.list_recent_known_packages(params.offset, params.limit)?;
    Ok(Json(packages))
}

/// Get a specific package by registry and repository.
#[utoipa::path(
    get,
    path = "/v1/packages/{registry}/{repository}",
    params(
        ("registry" = String, Path, description = "OCI registry hostname (e.g. ghcr.io)"),
        ("repository" = String, Path, description = "Repository path (e.g. user/repo)")
    ),
    responses(
        (status = 200, description = "Package found", body = KnownPackage),
        (status = 404, description = "Package not found"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
async fn get_package(
    State(manager): State<AppState>,
    Path((registry, repository)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    // Wildcard captures include a leading `/`; strip it.
    let repository = repository.trim_start_matches('/');
    let manager = manager
        .lock()
        .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?;
    match manager.get_known_package(&registry, repository)? {
        Some(package) => Ok(Json(package).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

/// Serve the OpenAPI specification as JSON.
#[utoipa::path(
    get,
    path = "/v1/openapi.json",
    responses(
        (status = 200, description = "OpenAPI specification", content_type = "application/json")
    )
)]
async fn openapi_spec() -> impl IntoResponse {
    Json(ApiDoc::openapi())
}

/// Application error type that converts to HTTP responses.
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": self.0.to_string() })),
        )
            .into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // r[verify server.health]
    /// Verify the server starts, binds to a port, and responds to `/v1/health`.
    #[tokio::test]
    async fn server_starts_and_listens() {
        let manager = Manager::open().await.expect("failed to open manager");
        let state = Arc::new(std::sync::Mutex::new(manager));
        let app = router(state);

        // Bind to port 0 so the OS assigns a random available port.
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("failed to bind listener");
        let addr = listener.local_addr().expect("failed to get local addr");

        // Spawn the server in a background task.
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.expect("server error");
        });

        // Hit the health endpoint.
        let url = format!("http://{addr}/v1/health");
        let resp = reqwest::get(&url).await.expect("request failed");
        assert_eq!(resp.status(), 200);

        let body: serde_json::Value = resp.json().await.expect("invalid json");
        assert_eq!(body, serde_json::json!({ "status": "ok" }));

        // Clean up.
        server.abort();
    }

    // r[verify server.openapi]
    /// Verify the OpenAPI spec endpoint returns valid JSON with expected metadata.
    #[tokio::test]
    async fn openapi_spec_endpoint() {
        let manager = Manager::open().await.expect("failed to open manager");
        let state = Arc::new(std::sync::Mutex::new(manager));
        let app = router(state);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("failed to bind listener");
        let addr = listener.local_addr().expect("failed to get local addr");

        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.expect("server error");
        });

        let url = format!("http://{addr}/v1/openapi.json");
        let resp = reqwest::get(&url).await.expect("request failed");
        assert_eq!(resp.status(), 200);

        let body: serde_json::Value = resp.json().await.expect("invalid json");
        assert_eq!(body["info"]["title"], "wasm-meta-registry");
        assert!(body["paths"]["/v1/health"].is_object());
        assert!(body["paths"]["/v1/search"].is_object());
        assert!(body["paths"]["/v1/packages"].is_object());

        server.abort();
    }
}
