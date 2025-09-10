//! Template and response
//! See: https://github.com/FranBarInstance/neutralts-docs

use axum::response::{IntoResponse, Response};
use axum::http::{StatusCode, header};
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

    pub fn render(&self) -> Result<Response, Box<dyn std::error::Error>> {

        // Create template from file with schema
        let mut template = NeutralTemplate::from_file_value(TEMPLATE_ROUTER, self.schema.clone())?;
        let contents = template.render();

        // Get the status code.
        let status_code = template.get_status_code();
        let status_text = template.get_status_text();
        let status_param = template.get_status_param();

        if let Ok(code) = status_code.parse::<u16>() {

            // The template may generate redirects.
            if [301, 302, 307, 308].contains(&code) {
                    return Ok((StatusCode::from_u16(code).unwrap(), [(header::LOCATION, status_param.to_string())], String::new()).into_response());
            }

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

                return Ok((StatusCode::from_u16(code).unwrap(), [(header::CONTENT_TYPE, "text/html")], error_contents).into_response());
            }
        }

        Ok((StatusCode::OK, [(header::CONTENT_TYPE, "text/html")], contents).into_response())
    }
}
