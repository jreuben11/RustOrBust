use axum::routing::get;

#[tokio::main]
pub async fn main() {
     // Build our application by creating our router.
    let app = axum::Router::new()
        .fallback(fallback)
        .route("/",  axum::routing::get(|| async { "default!" }))
        .route("/hello", get(hello))
        .route("/demo.html", get(get_demo_html))
        .route("/demo-status", get(demo_status))
        .route("/demo-uri", get(demo_uri))
        .route("/demo.png", get(get_demo_png))
        .route("/foo",
            get(get_foo)
            .put(put_foo)
            .patch(patch_foo)
            .post(post_foo)
            .delete(delete_foo),
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

/// axum handler for "GET /" which returns a string and causes axum to
/// immediately respond with status code `200 OK` and with the string.
pub async fn hello() -> String {
    "Hello, World!".to_string()
}

/// axum handler that responds with typical HTML coming from a file.
/// This uses the Rust macro `std::include_str` to include a UTF-8 file
/// path, relative to `main.rs`, as a `&'static str` at compile time.
pub async fn get_demo_html() -> axum::response::Html<&'static str> {
    include_str!("hello.html").into()
}

/// axum handler for "GET /demo-status" which returns a HTTP status
/// code, such as OK (200), and a custom user-visible string message.
pub async fn demo_status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is OK".to_string())
}

/// axum handler for "GET /demo-uri" which shows the request's own URI.
/// This shows how to write a handler that receives the URI.
pub async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}

/// axum handler for "GET /demo.png" which responds with an image PNG.
/// This sets a header "image/png" then sends the decoded image data.
async fn get_demo_png() -> impl axum::response::IntoResponse {
    use base64::Engine;
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        axum::response::AppendHeaders([
            (axum::http::header::CONTENT_TYPE, "image/png"),
        ]),
        base64::engine::general_purpose::STANDARD.decode(png).unwrap(),
    )
}

/// axum handler for "GET /foo" which returns a string message.
/// This shows our naming convention for HTTP GET handlers.
pub async fn get_foo() -> String {
    "GET foo".to_string()
 }
 
 /// axum handler for "PUT /foo" which returns a string message.
 /// This shows our naming convention for HTTP PUT handlers.
 pub async fn put_foo() -> String {
    "PUT foo".to_string()
 }
 
 /// axum handler for "PATCH /foo" which returns a string message.
 /// This shows our naming convention for HTTP PATCH handlers.
 pub async fn patch_foo() -> String {
    "PATCH foo".to_string()
 }
 
 /// axum handler for "POST /foo" which returns a string message.
 /// This shows our naming convention for HTTP POST handlers.
 pub async fn post_foo() -> String {
    "POST foo".to_string()
 }
 
 /// axum handler for "DELETE /foo" which returns a string message.
 /// This shows our naming convention for HTTP DELETE handlers.
 pub async fn delete_foo() -> String {
    "DELETE foo".to_string()
 }