//! API client for fetching package data from the meta-registry.

// r[impl frontend.api.callback]
// r[impl frontend.api.base-url]

use wasm_meta_registry_client::KnownPackage;
use wstd::http::{Body, Client, Request};

/// Default API base URL when no environment variable is set.
const DEFAULT_API_BASE_URL: &str = "http://localhost:3000";

/// Thin wrapper around `wstd::http::Client` for meta-registry API calls.
#[derive(Debug)]
pub(crate) struct ApiClient {
    base_url: String,
    client: Client,
}

impl ApiClient {
    /// Create a new client with the given base URL.
    #[must_use]
    pub(crate) fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
        }
    }

    /// Create a client using the API base URL.
    ///
    /// The URL is set at compile time via the `API_BASE_URL` environment
    /// variable. Falls back to `http://localhost:3000` when unset.
    #[must_use]
    pub(crate) fn from_env() -> Self {
        let base_url = option_env!("API_BASE_URL").unwrap_or(DEFAULT_API_BASE_URL);
        Self::new(base_url)
    }

    /// Fetch recently updated packages from the meta-registry.
    pub(crate) async fn fetch_recent_packages(&self, limit: u32) -> Vec<KnownPackage> {
        let url = format!("{}/v1/packages?limit={limit}", self.base_url);
        self.fetch_packages_from(&url).await
    }

    /// Search packages by query string.
    pub(crate) async fn search_packages(&self, query: &str) -> Vec<KnownPackage> {
        let url = format!("{}/v1/search?q={query}", self.base_url);
        self.fetch_packages_from(&url).await
    }

    /// Fetch all packages with pagination.
    pub(crate) async fn fetch_all_packages(&self, offset: u32, limit: u32) -> Vec<KnownPackage> {
        let url = format!(
            "{}/v1/packages?offset={offset}&limit={limit}",
            self.base_url
        );
        self.fetch_packages_from(&url).await
    }

    /// Look up a package by its WIT namespace and name.
    ///
    /// Uses the search endpoint and filters client-side, since the
    /// meta-registry does not yet have a dedicated by-WIT-name endpoint.
    pub(crate) async fn fetch_package_by_wit(
        &self,
        namespace: &str,
        name: &str,
    ) -> Option<KnownPackage> {
        let query = format!("{namespace}:{name}");
        let packages = self.search_packages(&query).await;
        packages.into_iter().find(|pkg| {
            pkg.wit_namespace.as_deref() == Some(namespace)
                && pkg.wit_name.as_deref() == Some(name)
        })
    }

    /// Fetch and deserialize a list of packages from the given URL.
    async fn fetch_packages_from(&self, url: &str) -> Vec<KnownPackage> {
        let Ok(req) = Request::get(url).body(Body::empty()) else {
            eprintln!("wasm-frontend: failed to build request for {url}");
            return Vec::new();
        };

        let response = match self.client.send(req).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("wasm-frontend: HTTP request to {url} failed: {e}");
                return Vec::new();
            }
        };

        let mut body = response.into_body();
        match body.json::<Vec<KnownPackage>>().await {
            Ok(packages) => packages,
            Err(e) => {
                eprintln!("wasm-frontend: failed to parse response from {url}: {e}");
                Vec::new()
            }
        }
    }
}
