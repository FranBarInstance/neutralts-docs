package main

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	http.HandleFunc("/", catchAllHandler)
	fmt.Printf("Starting Neutral TS PWA Go server on %s\n", PORT)
	http.ListenAndServe(PORT, nil)
}

func catchAllHandler(w http.ResponseWriter, r *http.Request) {
	route := strings.Trim(r.URL.Path, "/\\")

	// Serve static files
	staticPath := filepath.Join(STATIC_FOLDER, route)
	if _, err := os.Stat(staticPath); err == nil && !isDir(staticPath) {
		http.ServeFile(w, r, staticPath)
		return
	}

	// Dynamic content
	switch r.Method {
	case "GET":
		handleGet(w, r, route)
	case "POST":
		handlePost(w, r, route)
	default:
		http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
	}
}

func handleGet(w http.ResponseWriter, r *http.Request, route string) {
	if route == "" || route == "home" {
		homeGet(w, r)
		return
	}

	if route == "form-login" {
		formLoginGet(w, r)
		return
	}

	if route == "logout" {
		logoutGet(w, r)
		return
	}

	// Catch-all dynamic
	schema := NewSchema(r, route)
	tmpl := NewTemplate(schema.Get())
	tmpl.Render(w, r)
}

func handlePost(w http.ResponseWriter, r *http.Request, route string) {
	if route == "" || route == "home" {
		homePost(w, r)
		return
	}

	if route == "form-login" {
		formLoginPost(w, r)
		return
	}

	// Other POSTs dynamic
	schema := NewSchema(r, route)
	tmpl := NewTemplate(schema.Get())
	tmpl.Render(w, r)
}

func homeGet(w http.ResponseWriter, r *http.Request) {
	schema := NewSchema(r, "home")
	tmpl := NewTemplate(schema.Get())
	tmpl.Render(w, r)
}

func homePost(w http.ResponseWriter, r *http.Request) {
	schema := NewSchema(r, "home")
	tmpl := NewTemplate(schema.Get())
	tmpl.Render(w, r)
}

func formLoginGet(w http.ResponseWriter, r *http.Request) {
	schema := NewSchema(r, "form-login")
	tmpl := NewTemplate(schema.Get())
	tmpl.Render(w, r)
}

func formLoginPost(w http.ResponseWriter, r *http.Request) {
	schema := NewSchema(r, "form-login")
	sch := schema.Get()
	sch["data"].(map[string]interface{})["send_form_login"] = 1.0 // float for json

	passwd := ""
	if post, ok := sch["data"].(map[string]interface{})["CONTEXT"].(map[string]interface{})["POST"].(map[string]interface{}); ok {
		if p, ok := post["passwd"]; ok {
			passwd = p.(string)
		}
	}

	if passwd == "1234" {
		sch["data"].(map[string]interface{})["send_form_login_fails"] = nil
		http.SetCookie(w, &http.Cookie{
			Name:  "SESSION",
			Value: SIMULATE_SECRET_KEY,
			Path:  "/",
		})
		sch["data"].(map[string]interface{})["CONTEXT"].(map[string]interface{})["SESSION"] = SIMULATE_SECRET_KEY
	} else {
		sch["data"].(map[string]interface{})["send_form_login_fails"] = true
	}

	tmpl := NewTemplate(sch)
	tmpl.Render(w, r)
}

func logoutGet(w http.ResponseWriter, r *http.Request) {
	schema := NewSchema(r, "logout")
	tmpl := NewTemplate(schema.Get())
	tmpl.Render(w, r)
}

func isDir(path string) bool {
	info, err := os.Stat(path)
	if err != nil {
		return false
	}
	return info.IsDir()
}
