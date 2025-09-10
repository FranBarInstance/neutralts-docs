/**
 * Neutral TS Hello World package example
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


    // Determine the template path and read content
    let template_path = "template.ntpl";

    // Create an instance of Template
    let mut template = Template::from_file_value(template_path, schema).unwrap();

    // Render the template
    let contents = template.render();

    // Print the rendered content, in other cases contents will be sent to output according to framework.
    println!("{}", contents);
    println!("Status code: {} {}", template.get_status_code(), template.get_status_text());
    println!("Render time: {:?}", template.get_time_duration());
}
