//! Neutral TS example with Axum
//! See: https://github.com/FranBarInstance/neutralts-docs

use axum::{Router, routing::get, extract::{Path, Form}, http::{StatusCode, HeaderMap, Uri}, response::IntoResponse};
use serde_json::Value;
use std::path::Path as StdPath;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::fs;
use mime_guess::from_path;

mod constants;
mod schema;
mod template;

use constants::{STATIC_FOLDER, SIMULATE_SECRET_KEY};
use schema::Schema;
use template::Template;

/// Catch all route handler - serves static files and dynamic content (GET only)
///
/// To prevent arbitrary routes contents-[route]-snippets.ntpl must exist,
/// to create a simple view/route you wouldn't need a function or view, just create
/// the contents-[route]-snippets.ntpl template file.
///
/// Following routes do not have a handler and are dispatched here:
/// /simulate-...
/// /help
/// /login
///
/// /login is a container for /form-login (/form-login is loaded via ajax)
async fn catch_all_get(Path(path): Path<String>, headers: HeaderMap, uri: Uri) -> impl IntoResponse {
    // normalize path: empty -> "index.html" behavior handled below
    let route = path.trim_start_matches('/').to_string();

    // Try to serve static file
    let mut file_path = StdPath::new(STATIC_FOLDER).join(&route);

    if route.is_empty() {
        file_path = StdPath::new(STATIC_FOLDER).join("index.html");
    }

    if file_path.exists() && file_path.is_file() {
        match fs::read(&file_path) {
            Ok(bytes) => {
                let mime = from_path(&file_path).first_or_octet_stream();
                return (StatusCode::OK, [("content-type", mime.essence_str())], bytes).into_response();
            }
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("IO error: {}", e)).into_response(),
        }
    }

    // Dynamic content - pass headers and uri to Schema
    let schema = Schema::new(headers.clone(), uri.clone(), &route);
    let template = Template::new(schema.get().clone());
    match template.render() {
        Ok(resp) => resp,
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)).into_response(),
    }
}

/// Display form login (GET)
async fn form_login_get(headers: HeaderMap, uri: Uri) -> impl IntoResponse {
    let route = "form-login";
    let schema = Schema::new(headers.clone(), uri.clone(), route);
    let template = Template::new(schema.get().clone());

    match template.render() {
        Ok(resp) => resp,
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)).into_response(),
    }
}

/// Process login form in POST (Fake login)
async fn form_login_post(headers: HeaderMap, uri: Uri, Form(form): Form<HashMap<String, String>>) -> impl IntoResponse {
    let route = "form-login";
    let mut schema = Schema::new_with_post(headers.clone(), uri.clone(), route, form);
    let current_schema = schema.get_mut();

    current_schema["data"]["send_form_login"] = Value::Number(serde_json::Number::from(1));

    let passwd = current_schema["data"]["CONTEXT"]["POST"]["passwd"].as_str().unwrap_or("");

    if passwd == "1234" {
        current_schema["data"]["send_form_login_fails"] = Value::Null;
        current_schema["data"]["CONTEXT"]["SESSION"] = Value::String(SIMULATE_SECRET_KEY.to_string());
    } else {
        current_schema["data"]["send_form_login_fails"] = Value::Number(serde_json::Number::from(1));
    }

    let template = Template::new(schema.get().clone());
    match template.render() {
        Ok(resp) => resp,
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)).into_response(),
    }
}

/// Home GET
async fn home_get(headers: HeaderMap, uri: Uri) -> impl IntoResponse {
    let route = "home";
    let schema = Schema::new(headers.clone(), uri.clone(), route);
    let template = Template::new(schema.get().clone());
    match template.render() {
        Ok(resp) => resp,
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)).into_response(),
    }
}

/// Home POST
async fn home_post(headers: HeaderMap, uri: Uri, Form(form): Form<HashMap<String, String>>) -> impl IntoResponse {
    let route = "home";
    let schema = Schema::new_with_post(headers.clone(), uri.clone(), route, form);
    let template = Template::new(schema.get().clone());
    match template.render() {
        Ok(resp) => resp,
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)).into_response(),
    }
}

#[tokio::main]
async fn main() {
    println!("Starting Neutral TS PWA Axum server on http://127.0.0.1:9090");

    let app = Router::new()
        .route("/", get(home_get).post(home_post))
        .route("/form-login", get(form_login_get).post(form_login_post))
        .route("/*path", get(catch_all_get));

    let addr = SocketAddr::from(([127, 0, 0, 1], 9090));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
