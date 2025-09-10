//! Neutral TS example with Warp
//! See: https://github.com/FranBarInstance/neutralts-docs

use warp::{Filter, Reply, Rejection, http::HeaderMap};
use std::path::Path;
use std::collections::HashMap;
use serde_json::Value;

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
async fn catch_all_get(path: warp::path::Tail, headers: HeaderMap, query: Option<String>) -> Result<Box<dyn Reply>, Rejection> {
    let route = path.as_str().trim_matches('/').to_string();

    // Serve static files directly
    let file_path = Path::new(STATIC_FOLDER).join(&route);
    if file_path.exists() && file_path.is_file() {
        match tokio::fs::read(&file_path).await {
            Ok(content) => {
                // Determine content type based on file extension
                let content_type = if route.ends_with(".css") {
                    "text/css"
                } else if route.ends_with(".js") {
                    "application/javascript"
                } else if route.ends_with(".html") {
                    "text/html"
                } else if route.ends_with(".json") {
                    "application/json"
                } else if route.ends_with(".png") {
                    "image/png"
                } else if route.ends_with(".jpg") || route.ends_with(".jpeg") {
                    "image/jpeg"
                } else if route.ends_with(".gif") {
                    "image/gif"
                } else if route.ends_with(".svg") {
                    "image/svg+xml"
                } else if route.ends_with(".ico") {
                    "image/x-icon"
                } else {
                    "application/octet-stream"
                };

                return Ok(Box::new(warp::reply::with_header(
                    warp::reply::Response::new(content.into()),
                    "content-type",
                    content_type
                )));
            },
            Err(_) => return Err(warp::reject::not_found())
        }
    }

    // Serve dynamic content
    let schema = Schema::new(&headers, &route, query.as_deref());
    let template = Template::new(schema.get().clone());
    match template.render() {
        Ok(response) => Ok(Box::new(response)),
        Err(_) => Err(warp::reject::not_found())
    }
}

/// Display form login (GET)
async fn form_login_get(headers: HeaderMap, query: Option<String>) -> Result<impl Reply, Rejection> {
    let route = "form-login";
    let schema = Schema::new(&headers, route, query.as_deref());

    let template = Template::new(schema.get().clone());
    match template.render() {
        Ok(response) => Ok(response),
        Err(_) => Err(warp::reject::not_found())
    }
}

/// Process login form in POST (Fake login)
async fn form_login_post(headers: HeaderMap, form: HashMap<String, String>, query: Option<String>) -> Result<impl Reply, Rejection> {
    let route = "form-login";
    let mut schema = Schema::new_with_post(&headers, route, form, query.as_deref());
    let current_schema = schema.get_mut();

    current_schema["data"]["send_form_login"] = Value::Number(serde_json::Number::from(1));

    // Fake login, any user, password: 1234
    let passwd = current_schema["data"]["CONTEXT"]["POST"]["passwd"]
        .as_str()
        .unwrap();

    if passwd == "1234" {
        current_schema["data"]["send_form_login_fails"] = Value::Null;
        current_schema["data"]["CONTEXT"]["SESSION"] = Value::String(SIMULATE_SECRET_KEY.to_string());
    } else {
        current_schema["data"]["send_form_login_fails"] = Value::Number(serde_json::Number::from(1));
    }

    let template = Template::new(schema.get().clone());
    match template.render() {
        Ok(response) => Ok(response),
        Err(_) => Err(warp::reject::not_found())
    }
}


// Home GET
async fn home_get(headers: HeaderMap, query: Option<String>) -> Result<impl Reply, Rejection> {
    let route = "home";
    let schema = Schema::new(&headers, route, query.as_deref());
    let template = Template::new(schema.get().clone());
    match template.render() {
        Ok(response) => Ok(response),
        Err(_) => Err(warp::reject::not_found())
    }
}

// Home POST
async fn home_post(headers: HeaderMap, form: HashMap<String, String>, query: Option<String>) -> Result<impl Reply, Rejection> {
    let route = "home";
    let schema = Schema::new_with_post(&headers, route, form, query.as_deref());
    let template = Template::new(schema.get().clone());
    match template.render() {
        Ok(response) => Ok(response),
        Err(_) => Err(warp::reject::not_found())
    }
}

#[tokio::main]
async fn main() {
    println!("Starting Neutral TS PWA Warp server on http://127.0.0.1:9090");

    // Static files route
    let static_files = warp::path("static")
        .and(warp::get())
        .and(warp::fs::dir(STATIC_FOLDER));

    // Home routes
    let home_get_route = warp::path::end()
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .and(warp::query::raw().map(Some).or_else(|_| async { Ok::<_, Rejection>((None,)) }))
        .and_then(home_get);

    let home_post_route = warp::path::end()
        .and(warp::post())
        .and(warp::header::headers_cloned())
        .and(warp::body::form())
        .and(warp::query::raw().map(Some).or_else(|_| async { Ok::<_, Rejection>((None,)) }))
        .and_then(home_post);

    // Form login routes
    let form_login_get_route = warp::path("form-login")
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .and(warp::query::raw().map(Some).or_else(|_| async { Ok::<_, Rejection>((None,)) }))
        .and_then(form_login_get);

    let form_login_post_route = warp::path("form-login")
        .and(warp::post())
        .and(warp::header::headers_cloned())
        .and(warp::body::form())
        .and(warp::query::raw().map(Some).or_else(|_| async { Ok::<_, Rejection>((None,)) }))
        .and_then(form_login_post);


    // Catch all route for dynamic content
    let catch_all_route = warp::path::tail()
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .and(warp::query::raw().map(Some).or_else(|_| async { Ok::<_, Rejection>((None,)) }))
        .and_then(catch_all_get);

    // Combine all routes
    let routes = static_files
        .or(home_get_route)
        .or(home_post_route)
        .or(form_login_get_route)
        .or(form_login_post_route)
        .or(catch_all_route);

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 9090))
        .await;
}
