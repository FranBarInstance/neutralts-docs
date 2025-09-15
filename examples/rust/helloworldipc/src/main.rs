/**
 * Neutral TS Hello World Rust IPC example
 * https://github.com/FranBarInstance/neutralts-docs/
 */

use neutralipcrs::NeutralIpcTemplate;
use serde_json::json;

fn main() {
    // The schema contains among other things the data and variables for the template
    let schema = json!({
        "data": {
            "hello": "Hello World"
        }
    });

    // This will also work by doing schema_str.into() later
    let _schema_str = r#"{
        "data": {
            "hello": "Hello World"
        }
    }"#;

    // Determine the template full path.
    // Since the IPC server runs in a separate process,
    // paths relative to this process will not work.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let template_path = format!("{}/template.ntpl", manifest_dir);

    // Create an instance of Template
    let mut template = NeutralIpcTemplate::from_file_value(&template_path, schema).unwrap();

    // Render the template
    let contents = template.render().unwrap();

    // Get the status code, text and parameter
    let status_code: &str = template.get_status_code();
    let status_text: &str = template.get_status_text();
    let status_param: &str = template.get_status_param();

    // Print the rendered content, in other cases contents will be sent to output according to framework.
    println!("{}", contents);
    println!("Status: {} {} {}", status_code, status_text, status_param);
}
