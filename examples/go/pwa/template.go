package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strconv"

	"neutral_ipc_template"
)

type Template struct {
	schema map[string]interface{}
}

func NewTemplate(schema map[string]interface{}) *Template {
	return &Template{schema: schema}
}

func (t *Template) Render(w http.ResponseWriter, r *http.Request) {
	jsonSchema, err := json.Marshal(t.schema)
	if err != nil {
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}

	nt := neutral_ipc_template.NewNeutralIpcTemplate(TEMPLATE_ROUTER, string(jsonSchema))
	contents := nt.Render()

	statusCode := nt.GetStatusCode()
	if statusCode != nil {
		codeStr := fmt.Sprintf("%v", statusCode)
		code, err := strconv.Atoi(codeStr)
		if err == nil {
			if code >= 300 && code < 400 {
				param := nt.GetStatusParam()
				paramStr := ""
				if p, ok := param.(string); ok {
					paramStr = p
				} else {
					paramStr = fmt.Sprintf("%v", param)
				}
				http.Redirect(w, r, paramStr, code)
				return
			}
			if code >= 400 {
				statusText := nt.GetStatusText()
				statusParam := nt.GetStatusParam()
				errorSchema := map[string]interface{}{
					"data": map[string]interface{}{
						"CONTEXT": map[string]interface{}{
							"ROUTE": "error",
						},
						"error": map[string]interface{}{
							"code":  float64(code),
							"text":  fmt.Sprintf("%v", statusText),
							"param": fmt.Sprintf("%v", statusParam),
						},
					},
				}
				jsonError, _ := json.Marshal(errorSchema)
				nt.SetPath(TEMPLATE_ERROR)
				nt.MergeSchema(string(jsonError))
				contents = nt.Render()
				w.WriteHeader(code)
			}
		}
	}

	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	w.Write([]byte(contents))
}
