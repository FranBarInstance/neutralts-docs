//! Neutral TS example with Actix-web
//! See: https://github.com/FranBarInstance/neutralts-docs

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_files::NamedFile;
use serde_json::Value;
use std::path::Path;
use std::collections::HashMap;

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
async fn catch_all_get(req: HttpRequest, path: web::Path<String>) -> Result<HttpResponse> {
    let route = path.into_inner();

    // Serve static files directly
    let file_path = Path::new(STATIC_FOLDER).join(&route);
    if file_path.exists() && file_path.is_file() {
        return Ok(NamedFile::open(file_path)?.into_response(&req));
    }

    // Serve dynamic content
    let schema = Schema::new(&req, &route);
    let template = Template::new(schema.get().clone());
    template.render().map_err(|e| actix_web::error::ErrorInternalServerError(e))
}

/// Display form login (GET)
async fn form_login_get(req: HttpRequest) -> Result<HttpResponse> {
    let route = "form-login";
    let schema = Schema::new(&req, route);

    let template = Template::new(schema.get().clone());
    template.render().map_err(|e| actix_web::error::ErrorInternalServerError(e))
}

/// Process login form in POST (Fake login)
async fn form_login_post(req: HttpRequest, form: web::Form<HashMap<String, String>>) -> Result<HttpResponse> {
    let route = "form-login";
    let mut schema = Schema::new_with_post(&req, route, form);
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
    template.render().map_err(|e| actix_web::error::ErrorInternalServerError(e))
}

// Home GET
async fn home_get(req: HttpRequest) -> Result<HttpResponse> {
    let route = "home";
    let schema = Schema::new(&req, route);
    let template = Template::new(schema.get().clone());
    template.render().map_err(|e| actix_web::error::ErrorInternalServerError(e))
}

// Home POST
async fn home_post(req: HttpRequest, form: web::Form<HashMap<String, String>>) -> Result<HttpResponse> {
    let route = "home";
    let schema = Schema::new_with_post(&req, route, form);
    let template = Template::new(schema.get().clone());
    template.render().map_err(|e| actix_web::error::ErrorInternalServerError(e))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Neutral TS PWA Actix server on http://127.0.0.1:9090");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home_get))
            .route("/", web::post().to(home_post))
            .route("/form-login", web::get().to(form_login_get))
            .route("/form-login", web::post().to(form_login_post))
            .route("/{path:.*}", web::get().to(catch_all_get))
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
