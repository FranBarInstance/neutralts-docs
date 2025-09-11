<?php
/**
 * Neutral TS Hello World PHP example
 * https://github.com/FranBarInstance/neutralts-docs/
 */

# Include NeutralIpcTemplate: https://github.com/FranBarInstance/neutral-ipc/clients
include '../ipc-client/NeutralIpcTemplate.php';

# The schema contains among other things the data and variables for the template
$schema = [
    "data" => [
        "hello" => "Hello World"
    ]
];

// Determine the template path
$template = __DIR__ . "/template.ntpl";

# Pass the schema as json to NeutralIpcTemplate
$schema_json = json_encode($schema);

# Create an instance of NeutralTemplate
$ipc_template = new NeutralIpcTemplate($template, $schema_json);

# Render the template
$contents = $ipc_template->render();

# Print the rendered content, in other cases contents will be sent to output according to framework.
echo $contents . PHP_EOL;
