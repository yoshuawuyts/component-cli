//! HTTP server with JSON API endpoints for package discovery.
//!
//! Provides search and listing endpoints backed by the `component-package-manager`
//! known packages database.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router, routing::get, routing::post};
use component_package_manager::manager::Manager;
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Shared application state wrapping a `Manager` in a `std::sync::Mutex`.
///
/// This is safe because all handler methods on `Manager` are synchronous
/// (no `.await` while holding the lock).
///
/// # Example
///
/// ```no_run
/// use component_meta_registry::server::AppState;
/// use component_package_manager::manager::Manager;
/// use std::sync::Arc;
///
/// # async fn example() -> anyhow::Result<()> {
/// let manager = Manager::open().await?;
/// let state: AppState = Arc::new(Mutex::new(manager));
/// # Ok(())
/// # }
/// ```
pub type AppState = Arc<tokio::sync::Mutex<Manager>>;

/// Query parameters for search.
///
/// # Example
///
/// ```
/// use component_meta_registry::server::SearchParams;
///
/// let params = SearchParams {
///     q: "wasi".to_string(),
///     offset: 0,
///     limit: 20,
/// };
///
/// assert_eq!(params.q, "wasi");
/// ```
#[derive(Debug, Deserialize)]
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
/// use component_meta_registry::server::ListParams;
///
/// let params = ListParams {
///     offset: 0,
///     limit: 50,
/// };
///
/// assert_eq!(params.limit, 50);
/// ```
#[derive(Debug, Deserialize)]
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

/// Build the axum router with all API routes.
///
/// # Example
///
/// ```no_run
/// use component_meta_registry::router;
/// use component_package_manager::manager::Manager;
/// use std::sync::Arc;
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
    // Routes with explicit suffixes must be registered before the catch-all
    // wildcard `{*repository}` to avoid conflicts.  We achieve this by
    // nesting the version/detail routes under a separate "prefix" router
    // that axum matches first.
    let package_detail_routes =
        Router::new().route("/{registry}/{*repository}", get(get_package_detail_nested));

    let package_versions_routes = Router::new().route(
        "/{registry}/{*repository}",
        get(get_package_versions_nested),
    );

    Router::new()
        .route("/v1/health", get(health))
        .route("/v1/search", get(search))
        .route("/v1/search/by-import", get(search_by_import))
        .route("/v1/search/by-export", get(search_by_export))
        .route("/v1/packages", get(list_packages))
        .route("/v1/packages/recent", get(list_recent_packages))
        .nest("/v1/packages/detail", package_detail_routes)
        .nest("/v1/packages/versions", package_versions_routes)
        .route(
            "/v1/packages/version/{registry}/{version}/{*repository}",
            get(get_package_version_reordered),
        )
        .route("/v1/packages/{registry}/{*repository}", get(get_package))
        .route("/v1/queue", get(get_queue_status))
        .route(
            "/v1/packages/notify/{registry}/{*repository}",
            post(notify_new_version),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Health check endpoint.
async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

/// Fetch queue status.
async fn get_queue_status(State(manager): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let manager = manager.lock().await;
    let status = manager.get_queue_status().await?;
    Ok(Json(status))
}

/// Search packages by query string.
async fn search(
    State(manager): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<impl IntoResponse, AppError> {
    let manager = manager.lock().await;
    let packages = manager.search_packages(&params.q, params.offset, params.limit).await?;
    Ok(Json(packages))
}

/// List all known packages.
async fn list_packages(
    State(manager): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<impl IntoResponse, AppError> {
    let manager = manager.lock().await;
    let packages = manager.list_known_packages(params.offset, params.limit).await?;
    Ok(Json(packages))
}

/// List recently updated known packages.
async fn list_recent_packages(
    State(manager): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<impl IntoResponse, AppError> {
    let manager = manager.lock().await;
    let packages = manager.list_recent_known_packages(params.offset, params.limit).await?;
    Ok(Json(packages))
}

/// Get a specific package by registry and repository.
async fn get_package(
    State(manager): State<AppState>,
    Path((registry, repository)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    // Wildcard captures include a leading `/`; strip it.
    let repository = repository.trim_start_matches('/');
    let manager = manager.lock().await;
    match manager.get_known_package(&registry, repository).await? {
        Some(package) => Ok(Json(package).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

/// Query parameters for interface-based search.
#[derive(Debug, Deserialize)]
pub struct InterfaceSearchParams {
    /// The interface to search for (e.g. `"wasi:io/streams"`).
    pub interface: String,
    /// Pagination offset (default: 0).
    #[serde(default)]
    pub offset: u32,
    /// Pagination limit (default: 20).
    #[serde(default = "default_limit")]
    pub limit: u32,
}

/// Search packages by imported interface.
// r[verify server.search.by-import]
async fn search_by_import(
    State(manager): State<AppState>,
    Query(params): Query<InterfaceSearchParams>,
) -> Result<impl IntoResponse, AppError> {
    let manager = manager.lock().await;
    let packages =
        manager.search_packages_by_import(&params.interface, params.offset, params.limit).await?;
    Ok(Json(packages))
}

/// Search packages by exported interface.
// r[verify server.search.by-export]
async fn search_by_export(
    State(manager): State<AppState>,
    Query(params): Query<InterfaceSearchParams>,
) -> Result<impl IntoResponse, AppError> {
    let manager = manager.lock().await;
    let packages =
        manager.search_packages_by_export(&params.interface, params.offset, params.limit).await?;
    Ok(Json(packages))
}

/// Get full package detail including all versions and metadata.
// r[verify server.detail]
async fn get_package_detail_nested(
    State(manager): State<AppState>,
    Path((registry, repository)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let repository = repository.trim_start_matches('/');
    let manager = manager.lock().await;
    match manager.get_package_detail(&registry, repository).await? {
        Some(detail) => Ok(Json(detail).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

/// List all versions of a package.
// r[verify server.versions.list]
async fn get_package_versions_nested(
    State(manager): State<AppState>,
    Path((registry, repository)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let repository = repository.trim_start_matches('/');
    let manager = manager.lock().await;
    match manager.get_package_detail(&registry, repository).await? {
        Some(detail) => Ok(Json(detail.versions).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

/// Get a specific version of a package by tag.
// r[verify server.versions.get]
async fn get_package_version_reordered(
    State(manager): State<AppState>,
    Path((registry, version, repository)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let repository = repository.trim_start_matches('/');
    let manager = manager.lock().await;
    match manager.get_package_version(&registry, repository, &version).await? {
        Some(ver) => Ok(Json(ver).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

/// Query parameters for `POST /v1/packages/notify/...`.
#[derive(Debug, Deserialize)]
pub struct NotifyParams {
    /// Tag of the newly-published version (e.g. `"1.1.0"`).
    pub tag: String,
}

/// Notify the registry that a new version was just published, requesting it
/// be pulled as soon as possible.
///
/// Returns `202 Accepted` with a JSON `NotifyOutcome` body in both the
/// "enqueued" and "skipped" cases — the request itself was accepted; the
/// outcome describes what the registry decided to do with it.
///
/// To prevent abuse, the endpoint:
///
/// * Rejects empty tags with `400 Bad Request`.
/// * Only accepts notifications for packages already known to this
///   registry (i.e. previously indexed). Notifications for unknown
///   packages return `404 Not Found` so the queue can't be flooded with
///   arbitrary `(registry, repository, tag)` triples.
/// * Enforces a freshness window (the same 1-hour cooldown used by the
///   periodic indexer). Repeated notifications for a tag that was just
///   pulled are returned as `{"status":"skipped"}`.
async fn notify_new_version(
    State(manager): State<AppState>,
    Path((registry, repository)): Path<(String, String)>,
    Query(params): Query<NotifyParams>,
) -> Result<axum::response::Response, AppError> {
    let repository = repository.trim_start_matches('/');
    let tag = params.tag.trim();
    if tag.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "tag must not be empty" })),
        )
            .into_response());
    }

    let manager = manager.lock().await;

    // Only allow notifications for packages we already know about. This
    // prevents arbitrary clients from filling the fetch queue with
    // unknown `(registry, repository, tag)` triples.
    if manager.get_known_package(&registry, repository).await?.is_none() {
        return Ok((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "unknown package; only previously-indexed packages may be notified"
            })),
        )
            .into_response());
    }

    let outcome = manager.notify_new_version(&registry, repository, tag).await?;
    Ok((StatusCode::ACCEPTED, Json(outcome)).into_response())
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
        let state = Arc::new(tokio::sync::Mutex::new(manager));
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

    /// Verify the notify endpoint enqueues a pull task and is idempotent.
    /// A second notify for the same tag while the task is still pending
    /// should also return `enqueued` (the queue dedupes internally).
    #[tokio::test]
    async fn notify_endpoint_enqueues_pull_task() {
        use component_meta_registry_types::NotifyOutcome;

        let manager = Manager::open().await.expect("failed to open manager");

        // Register the target as a known package up-front: the notify
        // endpoint rejects unknown packages with `404` to prevent
        // arbitrary clients from flooding the fetch queue.
        let registry = "example.test";
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let repository = format!("notify-test-{unique}");
        manager
            .add_known_package(registry, &repository, None, None)
            .await
            .expect("failed to register known package");

        let state = Arc::new(tokio::sync::Mutex::new(manager));
        let app = router(state);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("failed to bind listener");
        let addr = listener.local_addr().expect("failed to get local addr");

        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.expect("server error");
        });

        let url =
            format!("http://{addr}/v1/packages/notify/{registry}/{repository}?tag=0.0.1-test");
        let client = reqwest::Client::new();
        let resp = client.post(&url).send().await.expect("request failed");
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
        let outcome: NotifyOutcome = resp.json().await.expect("invalid json");
        assert_eq!(outcome, NotifyOutcome::Enqueued);

        // A second notify for the same tag while the task is still
        // pending must also return `enqueued` — the underlying queue
        // dedupes by `(registry, repository, tag)` so the request is
        // accepted and reported as enqueued without creating a duplicate
        // row.
        let resp = client
            .post(&url)
            .send()
            .await
            .expect("second request failed");
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
        let outcome: NotifyOutcome = resp.json().await.expect("invalid json");
        assert_eq!(outcome, NotifyOutcome::Enqueued);

        server.abort();
    }
}
