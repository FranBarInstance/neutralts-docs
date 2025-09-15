/**
 * Neutral TS Hello World Rust example
 * https://github.com/FranBarInstance/neutralts-docs/
 */

use neutralts::Template;
use serde_json::json;

fn main() {
    // The schema contains among other things the data and variables for the template
    let schema = json!({
        "data": {
            "hello": "Hello World"
        }
    });

    // Determine the template full path
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let template_path = format!("{}/template.ntpl", manifest_dir);

    // Create an instance of Template
    let mut template = Template::from_file_value(&template_path, schema).unwrap();

    // Render the template
    let contents = template.render();

    // Get the status code, text and parameter
    let status_code: &str = template.get_status_code();
    let status_text: &str = template.get_status_text();
    let status_param: &str = template.get_status_param();

    // Print the rendered content, in other cases contents will be sent to output according to framework.
    println!("{}", contents);
    println!("Status: {} {} {}", status_code, status_text, status_param);
    println!("Render time: {:?}", template.get_time_duration());
}
