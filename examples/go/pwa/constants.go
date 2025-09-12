package main

import (
	"path/filepath"
	"runtime"
)

var BASE_DIR string

var (
	TEMPLATE_ROUTER     string
	TEMPLATE_ERROR      string
	STATIC_FOLDER       string
	DEFAULT_SCHEMA      string
	LANG_KEY            = "lang"
	THEME_KEY           = "theme"
	SIMULATE_SECRET_KEY = "69bdd1e4b4047d8f4e3"
	PORT                = ":8000"
)

func init() {
	_, filename, _, _ := runtime.Caller(0)
	BASE_DIR = filepath.Dir(filename)

	TEMPLATE_ROUTER = filepath.Join(BASE_DIR, "../../neutral/tpl/cache.ntpl")
	TEMPLATE_ERROR = filepath.Join(BASE_DIR, "../../neutral/tpl/cache_error.ntpl")
	STATIC_FOLDER = filepath.Join(BASE_DIR, "../../neutral/static")
	DEFAULT_SCHEMA = filepath.Join(BASE_DIR, "../../neutral/data/schema.json")
}
