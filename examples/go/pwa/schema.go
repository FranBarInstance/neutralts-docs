package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"strings"
)

type Schema struct {
	schema map[string]interface{}
	route  string
}

func NewSchema(r *http.Request, route string) *Schema {
	s := &Schema{
		route:  strings.Trim(route, "/\\"),
		schema: make(map[string]interface{}),
	}
	s.defaultSchema()
	s.populateContext(r)
	s.negotiateLanguage(r)
	s.setTheme(r)
	return s
}

func (s *Schema) defaultSchema() {
	fullPath := DEFAULT_SCHEMA
	data, err := os.ReadFile(fullPath)
	if err != nil {
		fmt.Printf("Error reading schema: %v\n", err)
		return
	}
	if err := json.Unmarshal(data, &s.schema); err != nil {
		fmt.Printf("Error unmarshaling schema: %v\n", err)
		return
	}
	if _, ok := s.schema["data"]; !ok {
		s.schema["data"] = make(map[string]interface{})
	}
	if _, ok := s.schema["data"].(map[string]interface{})["CONTEXT"]; !ok {
		s.schema["data"].(map[string]interface{})["CONTEXT"] = make(map[string]interface{})
	}
	s.context()["GET"] = make(map[string]interface{})
	s.context()["POST"] = make(map[string]interface{})
	s.context()["COOKIES"] = make(map[string]interface{})
	s.context()["HEADERS"] = make(map[string]interface{})
}

func (s *Schema) schemaData() map[string]interface{} {
	return s.schema["data"].(map[string]interface{})
}

func (s *Schema) context() map[string]interface{} {
	return s.schemaData()["CONTEXT"].(map[string]interface{})
}

func (s *Schema) populateContext(r *http.Request) {
	s.context()["ROUTE"] = s.route

	// Headers
	headers := make(map[string]interface{})
	for name, values := range r.Header {
		headerName := strings.ToUpper(name)
		if len(values) > 0 {
			headers[headerName] = values[0]
		}
	}
	s.context()["HEADERS"] = headers
	headers["HOST"] = r.Host

	// GET
	for key, values := range r.URL.Query() {
		if len(values) > 0 {
			s.context()["GET"].(map[string]interface{})[key] = values[0]
		}
	}

	// POST
	if r.Method == "POST" {
		if err := r.ParseForm(); err == nil {
			for key, values := range r.PostForm {
				if len(values) > 0 {
					s.context()["POST"].(map[string]interface{})[key] = values[0]
				}
			}
		}
	}

	// Cookies
	cookies := make(map[string]interface{})
	for _, cookie := range r.Cookies() {
		cookies[cookie.Name] = cookie.Value
	}
	s.context()["COOKIES"] = cookies

	// Session
	session, exists := cookies["SESSION"]
	if exists {
		s.context()["SESSION"] = session
	} else {
		s.context()["SESSION"] = nil
	}
}

func (s *Schema) negotiateLanguage(r *http.Request) {
	siteData, ok := s.schema["data"].(map[string]interface{})["site"].(map[string]interface{})
	if !ok {
		return
	}
	validLanguages, ok := siteData["validLanguages"].([]interface{})
	if !ok {
		return
	}
	langList := make([]string, len(validLanguages))
	for i, l := range validLanguages {
		langList[i] = l.(string)
	}

	current := s.getParam(LANG_KEY, "GET", "COOKIES")
	if current == "" {
		// Simple Accept-Language match
		acceptLang := r.Header.Get("Accept-Language")
		if acceptLang != "" {
			parts := strings.Split(acceptLang, ",")
			for _, part := range parts {
				langPart := strings.SplitN(strings.TrimSpace(part), ";", 2)[0]
				lang := strings.ToLower(langPart[:2]) // First two letters
				for _, valid := range langList {
					if strings.HasPrefix(strings.ToLower(valid), lang) {
						current = valid
						break
					}
				}
				if current != "" {
					break
				}
			}
		}
	}
	if current == "" || !s.contains(langList, current) {
		if len(langList) > 0 {
			current = langList[0]
		}
	}
	if _, ok := s.schema["inherit"]; !ok {
		s.schema["inherit"] = make(map[string]interface{})
	}
	if _, ok := s.schema["inherit"].(map[string]interface{})["locale"]; !ok {
		s.schema["inherit"].(map[string]interface{})["locale"] = make(map[string]interface{})
	}
	s.schema["inherit"].(map[string]interface{})["locale"].(map[string]interface{})["current"] = current
}

func (s *Schema) getParam(key, source1, source2 string) string {
	ctx := s.context()
	switch source1 {
	case "GET":
		if get, ok := ctx["GET"].(map[string]interface{}); ok {
			if val, ok := get[key]; ok {
				return val.(string)
			}
		}
	case "COOKIES":
		if cookies, ok := ctx["COOKIES"].(map[string]interface{}); ok {
			if val, ok := cookies[key]; ok {
				return val.(string)
			}
		}
	}
	// source2 same logic
	switch source2 {
	case "GET":
		if get, ok := ctx["GET"].(map[string]interface{}); ok {
			if val, ok := get[key]; ok {
				return val.(string)
			}
		}
	case "COOKIES":
		if cookies, ok := ctx["COOKIES"].(map[string]interface{}); ok {
			if val, ok := cookies[key]; ok {
				return val.(string)
			}
		}
	}
	return ""
}

func (s *Schema) contains(slice []string, item string) bool {
	for _, a := range slice {
		if a == item {
			return true
		}
	}
	return false
}

func (s *Schema) setTheme(r *http.Request) {
	theme := s.getParam(THEME_KEY, "GET", "COOKIES")
	if theme == "" {
		siteData, ok := s.schema["data"].(map[string]interface{})["site"].(map[string]interface{})
		if ok {
			validThemes, ok := siteData["validThemes"].([]interface{})
			if ok && len(validThemes) > 0 {
				theme = validThemes[0].(string)
			}
		}
	}
	schemaData := s.schema["data"].(map[string]interface{})
	if _, ok := schemaData["site"]; !ok {
		schemaData["site"] = make(map[string]interface{})
	}
	s.schema["data"].(map[string]interface{})["site"].(map[string]interface{})["theme"] = theme
}

func (s *Schema) Get() map[string]interface{} {
	return s.schema
}
