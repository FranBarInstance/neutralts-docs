//! Template and response
//! See: https://github.com/FranBarInstance/neutralts-docs

use warp::http::StatusCode;
use warp::http::Uri;
use neutralts::Template as NeutralTemplate;
use serde_json::{json, Value};
use crate::constants::{TEMPLATE_ERROR, TEMPLATE_ROUTER};

pub struct Template {
    schema: Value,
}

impl Template {
    pub fn new(schema: Value) -> Self {
        Self { schema }
    }

    pub fn render(&self) -> Result<impl warp::Reply, Box<dyn std::error::Error>> {

        // Create template from file with schema
        let mut template = NeutralTemplate::from_file_value(TEMPLATE_ROUTER, self.schema.clone())?;
        let contents = template.render();

        // Get the status code.
        // Neutral TS may send an HTTP error code, with {:redirect; ... :} or {:exit; ... :}
        let status_code = template.get_status_code();
        let status_text = template.get_status_text();
        let status_param = template.get_status_param();

        if let Ok(code) = status_code.parse::<u16>() {

            // The template may generate redirects.
            if [301, 302, 307, 308].contains(&code) {
                let uri: Uri = status_param.parse().unwrap();
                let _response = warp::redirect::temporary(uri);
                let response = warp::reply::html(format!("<html><body>Redirecting to {}</body></html>", status_param));
                return Ok(warp::reply::with_status(response, StatusCode::from_u16(code).unwrap()));
            }

            // The template may generate HTTP errors.
            if code >= 400 {
                let error = json!({
                    "data": {
                        "CONTEXT": {
                            "ROUTE": "error"
                        },
                        "error": {
                            "code": code,
                            "text": status_text,
                            "param": status_param
                        }
                    }
                });

                // Again with error page
                template.set_src_path(TEMPLATE_ERROR).unwrap();

                // Error variables are added to the schema.
                template.merge_schema_value(error);

                // Render the error page
                let error_contents = template.render();

                let response = warp::reply::html(error_contents);
                return Ok(warp::reply::with_status(response, StatusCode::from_u16(code).unwrap()));
            }
        }

        // 200 Ok
        let response = warp::reply::html(contents);
        Ok(warp::reply::with_status(response, StatusCode::OK))
    }
}
