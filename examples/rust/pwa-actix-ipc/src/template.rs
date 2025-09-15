//! Template and response
//! See: https://github.com/FranBarInstance/neutralts-docs

use actix_web::{HttpResponse, http::StatusCode};
use neutralipcrs::NeutralIpcTemplate;
use serde_json::{json, Value};
use crate::constants::{TEMPLATE_ERROR, TEMPLATE_ROUTER};

pub struct Template {
    schema: Value,
}

impl Template {
    pub fn new(schema: Value) -> Self {
        Self { schema }
    }

    pub fn render(&self) -> Result<HttpResponse, Box<dyn std::error::Error>> {

        // Create template from file with schema
        let mut template = NeutralIpcTemplate::from_file_value(TEMPLATE_ROUTER, self.schema.clone())?;
        let contents = template.render().unwrap();

        // Get the status code.
        // Neutral TS may send an HTTP error code, with {:redirect; ... :} or {:exit; ... :}
        let status_code = template.get_status_code();
        let status_text = template.get_status_text();
        let status_param = template.get_status_param();

        if let Ok(code) = status_code.parse::<u16>() {

            // The template may generate redirects.
            if [301, 302, 307, 308].contains(&code) {
                return Ok(HttpResponse::build(StatusCode::from_u16(code).unwrap())
                    .append_header(("Location", status_param.to_string()))
                    .finish());
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
                template.set_path(TEMPLATE_ERROR);

                // Error variables are added to the schema.
                let _ = template.merge_schema(error);

                // Render the error page
                let error_contents = template.render().unwrap();

                return Ok(HttpResponse::build(StatusCode::from_u16(code).unwrap())
                    .content_type("text/html")
                    .body(error_contents));
            }
        }

        // 200 Ok
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(contents))
    }
}
