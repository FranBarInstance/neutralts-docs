//! Constants
//! See: https://github.com/FranBarInstance/neutralts-docs

// Use cached templates
pub const TEMPLATE_ROUTER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../neutral/tpl/cache.ntpl");

// Uncomment for no cached templates
// pub const TEMPLATE_ROUTER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../neutral/tpl/index.ntpl");

// HTTP errors template
pub const TEMPLATE_ERROR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../neutral/tpl/cache_error.ntpl");

// static files
pub const STATIC_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../neutral/static");

// Default schema
pub const DEFAULT_SCHEMA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../neutral/data/schema.json");

pub const LANG_KEY: &str = "lang";
pub const THEME_KEY: &str = "theme";
pub const SIMULATE_SECRET_KEY: &str = "69bdd1e4b4047d8f4e3";
