//! Self-hosted Iosevka webfont serving.
//!
//! Embeds woff2 font files at compile time and serves them via
//! dedicated routes with immutable caching.

use axum::http::{HeaderValue, header};
use axum::response::{IntoResponse, Response};

/// Iosevka Regular (400) woff2.
static IOSEVKA_REGULAR: &[u8] = include_bytes!("../fonts/Iosevka-Regular.woff2");

/// Iosevka Medium (500) woff2.
static IOSEVKA_MEDIUM: &[u8] = include_bytes!("../fonts/Iosevka-Medium.woff2");

/// Iosevka SemiBold (600) woff2.
static IOSEVKA_SEMIBOLD: &[u8] = include_bytes!("../fonts/Iosevka-SemiBold.woff2");

/// Iosevka Bold (700) woff2.
static IOSEVKA_BOLD: &[u8] = include_bytes!("../fonts/Iosevka-Bold.woff2");

/// Serve a woff2 font file with appropriate headers.
fn font_response(data: &'static [u8]) -> Response {
    let mut response = data.into_response();
    let headers = response.headers_mut();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("font/woff2"));
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=31536000, immutable"),
    );
    response
}

/// Handler for `/fonts/iosevka-regular.woff2`.
pub(crate) async fn regular() -> Response {
    font_response(IOSEVKA_REGULAR)
}

/// Handler for `/fonts/iosevka-medium.woff2`.
pub(crate) async fn medium() -> Response {
    font_response(IOSEVKA_MEDIUM)
}

/// Handler for `/fonts/iosevka-semibold.woff2`.
pub(crate) async fn semibold() -> Response {
    font_response(IOSEVKA_SEMIBOLD)
}

/// Handler for `/fonts/iosevka-bold.woff2`.
pub(crate) async fn bold() -> Response {
    font_response(IOSEVKA_BOLD)
}
