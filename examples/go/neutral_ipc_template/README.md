# Neutral TS IPC Client Go

**IPC Client Go is experimental.**

This package provides a minimal Go client for Neutral TS templates using the IPC server.

The client exposes `NeutralIpcTemplate` which lets you render a template by sending a schema
and template path (or source) to the IPC server and receiving the rendered content.

Example
-------

```go
package main

import (
    "encoding/json"
    "fmt"
    "path/filepath"
    "runtime"
    "os"

    // Import the Neutral IPC client package.
    // The client sources are available at:
    // https://github.com/FranBarInstance/neutral-ipc/tree/master/clients
    // Replace the import path below according to your module layout.
    "neutral_ipc_template"
)

func main() {
    // The schema contains the data and variables for the template
    schema := map[string]interface{}{
        "data": map[string]interface{}{
            "hello": "Hello World",
        },
    }

    // Convert schema to JSON string
    schemaBytes, _ := json.Marshal(schema)
    schemaJSON := string(schemaBytes)

    // Determine the template full path (template.ntpl should be next to this README)
    _, b, _, _ := runtime.Caller(0)
    dir := filepath.Dir(b)
    template := filepath.Join(dir, "template.ntpl")

    // Create a NeutralIpcTemplate instance
    ipc := neutral_ipc_template.NewNeutralIpcTemplate(template, schemaJSON)

    // Render the template
    contents := ipc.Render()

    // e.g.: 200
    statusCode := ipc.GetStatusCode()

    // e.g.: OK
    statusText := ipc.GetStatusText()

    // empty if no error
    statusParam := ipc.GetStatusParam()

    // Act according to your framework to display the content
    // for this example, simply output
    fmt.Println(contents)
}
```

Links
-----

Neutral TS template engine.

- [Template docs](https://github.com/FranBarInstance/neutralts-docs/docs/neutralts/doc/)
- [Repository](https://github.com/FranBarInstance/neutralts)
- [Crate](https://crates.io/crates/neutralts)
- [Examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples)
