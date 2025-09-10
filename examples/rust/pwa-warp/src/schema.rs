//! Schema
//! See: https://github.com/FranBarInstance/neutralts-docs

use warp::http::HeaderMap;
use serde_json::{json, Value};
use std::fs;
use std::collections::HashMap;
use crate::constants::{DEFAULT_SCHEMA, LANG_KEY, THEME_KEY};

// It is important to distinguish between data coming from the user and data
// coming from the application. "CONTEXT" has some security measures such as escaping.
pub struct Schema {
    headers: HeaderMap,
    route: String,
    schema: Value,
}

impl Schema {
    pub fn new(headers: &HeaderMap, route: &str, query: Option<&str>) -> Self {
        let mut schema = Self {
            headers: headers.clone(),
            route: route.trim_matches('/').to_string(),
            schema: Value::Null,
        };
        schema.default();
        schema.populate_context(None, query);
        schema.negotiate_language();
        schema.set_theme();
        schema
    }

    pub fn new_with_post(headers: &HeaderMap, route: &str, post_data: HashMap<String, String>, query: Option<&str>) -> Self {
        let mut schema = Self {
            headers: headers.clone(),
            route: route.trim_matches('/').to_string(),
            schema: Value::Null,
        };
        schema.default();
        schema.populate_context(Some(post_data), query);
        schema.negotiate_language();
        schema.set_theme();
        schema
    }

    fn default(&mut self) {
        let schema_content = fs::read_to_string(DEFAULT_SCHEMA)
            .expect("Failed to read default schema");
        self.schema = serde_json::from_str(&schema_content)
            .expect("Failed to parse default schema");

        // Ensure required structure exists
        if self.schema.get("data").is_none() {
            self.schema["data"] = json!({});
        }
        if self.schema["data"].get("CONTEXT").is_none() {
            self.schema["data"]["CONTEXT"] = json!({});
        }
        if self.schema["data"]["CONTEXT"].get("GET").is_none() {
            self.schema["data"]["CONTEXT"]["GET"] = json!({});
        }
        if self.schema["data"]["CONTEXT"].get("POST").is_none() {
            self.schema["data"]["CONTEXT"]["POST"] = json!({});
        }
        if self.schema["data"]["CONTEXT"].get("COOKIES").is_none() {
            self.schema["data"]["CONTEXT"]["COOKIES"] = json!({});
        }
        if self.schema["data"]["CONTEXT"].get("HEADERS").is_none() {
            self.schema["data"]["CONTEXT"]["HEADERS"] = json!({});
        }
    }

    fn populate_context(&mut self, post_data: Option<HashMap<String, String>>, query: Option<&str>) {
        self.schema["data"]["CONTEXT"]["ROUTE"] = json!(self.route);

        // Get Host header
        if let Some(host) = self.headers.get("Host") {
            if let Ok(host_str) = host.to_str() {
                self.schema["data"]["CONTEXT"]["HEADERS"]["HOST"] = json!(host_str);
            }
        }

        // Parse query parameters (GET)
        if let Some(query_str) = query {
            let params: Vec<&str> = query_str.split('&').collect();
            for param in params {
                if let Some(eq_pos) = param.find('=') {
                    let key = &param[..eq_pos];
                    let value = &param[eq_pos + 1..];
                    self.schema["data"]["CONTEXT"]["GET"][key] = json!(value);
                }
            }
        }

        // Parse POST data if available
        if let Some(post_params) = post_data {
            for (key, value) in post_params {
                self.schema["data"]["CONTEXT"]["POST"][key] = json!(value);
            }
        }

        // Parse headers
        for (key, value) in &self.headers {
            if let Ok(value_str) = value.to_str() {
                self.schema["data"]["CONTEXT"]["HEADERS"][key.as_str()] = json!(value_str);
            }
        }

        // Parse cookies
        if let Some(cookie_header) = self.headers.get("Cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in cookie_str.split(';') {
                    let cookie = cookie.trim();
                    if let Some(eq_pos) = cookie.find('=') {
                        let key = &cookie[..eq_pos];
                        let value = &cookie[eq_pos + 1..];
                        self.schema["data"]["CONTEXT"]["COOKIES"][key] = json!(value);
                    }
                }
            }
        }

        // Fake session
        let session = self.schema["data"]["CONTEXT"]["COOKIES"]
            .get("SESSION")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        if session.is_some() {
            self.schema["data"]["CONTEXT"]["SESSION"] = json!(session.unwrap());
        }
    }

    fn negotiate_language(&mut self) {
        let languages = self.schema["data"]["site"]["validLanguages"].clone();
        let empty_vec = vec![];
        let languages_array = languages.as_array().unwrap_or(&empty_vec);
        let languages_vec: Vec<String> = languages_array
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        // Get language from query params, cookies, or default to first language
        let lang_from_get = self.schema["data"]["CONTEXT"]["GET"]
            .get(LANG_KEY)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let lang_from_cookies = self.schema["data"]["CONTEXT"]["COOKIES"]
            .get(LANG_KEY)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let current_lang = lang_from_get
            .or(lang_from_cookies)
            .filter(|lang| languages_vec.contains(lang))
            .unwrap_or_else(|| languages_vec.first().cloned().unwrap_or_default());

        self.schema["inherit"]["locale"]["current"] = json!(current_lang);
    }

    fn set_theme(&mut self) {
        let theme_from_get = self.schema["data"]["CONTEXT"]["GET"]
            .get(THEME_KEY)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let theme_from_cookies = self.schema["data"]["CONTEXT"]["COOKIES"]
            .get(THEME_KEY)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let valid_themes = self.schema["data"]["site"]["validThemes"].clone();
        let empty_vec = vec![];
        let valid_themes_array = valid_themes.as_array().unwrap_or(&empty_vec);
        let default_theme = valid_themes_array
            .first()
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        let current_theme = theme_from_get
            .or(theme_from_cookies)
            .filter(|theme| valid_themes_array.contains(&json!(theme)))
            .unwrap_or_else(|| default_theme.to_string());

        self.schema["data"]["site"]["theme"] = json!(current_theme);
    }

    pub fn get_mut(&mut self) -> &mut Value {
        &mut self.schema
    }

    pub fn get(&self) -> &Value {
        &self.schema
    }
}
