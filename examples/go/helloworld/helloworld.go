/*
Neutral TS Hello World IPC example
https://github.com/FranBarInstance/neutralts-docs/
*/

package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"neutral_ipc_template"
)

func main() {
	// The schema contains among other things the data and variables for the template
	schemaDict := map[string]interface{}{
		"data": map[string]string{
			"hello": "Hello World",
		},
	}

	// Determine the template path
	dir, err := os.Getwd()
	if err != nil {
		fmt.Println("Error getting current directory:", err)
		os.Exit(1)
	}
	templatePath := filepath.Join(dir, "template.ntpl")

	// Marshal schema to JSON string
	schemaJSON, err := json.Marshal(schemaDict)
	if err != nil {
		fmt.Println("Error marshaling schema:", err)
		os.Exit(1)
	}

	// Create an instance of NeutralIpcTemplate
	ipcTemplate := neutral_ipc_template.NewNeutralIpcTemplate(templatePath, string(schemaJSON))

	// Render the template
	contents := ipcTemplate.Render()

	// Print the rendered content, in other cases contents will be sent to output according to framework.
	fmt.Println(contents)
}
