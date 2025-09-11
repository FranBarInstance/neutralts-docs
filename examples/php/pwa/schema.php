<?php
/**
 * Schema builder for PHP example
 * See: https://github.com/FranBarInstance/neutralts-docs
 */

require_once __DIR__ . '/constants.php';

// It is important to distinguish between data coming from the user and data
// coming from the application. "CONTEXT" has some security measures such as escaping.
class Schema
{
    protected $schema = [];
    protected $route;

    public function __construct($route)
    {
        $this->route = trim($route, '/\\');
        $this->default();
        $this->populate_context();
        $this->negotiate_language();
        $this->set_theme();
    }

    protected function default()
    {
        $json = file_get_contents(DEFAULT_SCHEMA);
        $this->schema = json_decode($json, true);
        if (!isset($this->schema['data'])) $this->schema['data'] = [];
        if (!isset($this->schema['data']['CONTEXT'])) $this->schema['data']['CONTEXT'] = [];
        $context = &$this->schema['data']['CONTEXT'];
        $context['GET'] = $context['GET'] ?? [];
        $context['POST'] = $context['POST'] ?? [];
        $context['COOKIES'] = $context['COOKIES'] ?? [];
        $context['HEADERS'] = $context['HEADERS'] ?? [];
    }

    protected function populate_context()
    {
        $context = &$this->schema['data']['CONTEXT'];
        $context['ROUTE'] = $this->route;

        // Parse headers using getallheaders() function
        if (function_exists('getallheaders')) {
            $headers = getallheaders();
            foreach ($headers as $key => $value) {
               $context['HEADERS'][$key] = $value;
            }
        } else {
            // Fallback for servers that don't have getallheaders()
            foreach ($_SERVER as $key => $value) {
                if (substr($key, 0, 5) === 'HTTP_') {
                    $headerName = str_replace('_', '-', substr($key, 5));
                    $context['HEADERS'][$headerName] = $value;
                }
            }
        }

        // GET params
        foreach ($_GET as $k => $v) {
            $context['GET'][$k] = $v;
        }

        // POST params
        if ($_SERVER['REQUEST_METHOD'] === 'POST') {
            foreach ($_POST as $k => $v) {
                $context['POST'][$k] = $v;
            }
        }

        // Cookies
        foreach ($_COOKIE as $k => $v) {
            $context['COOKIES'][$k] = $v;
        }

        // Fake session
        $context['SESSION'] = $context['COOKIES']['SESSION'] ?? null;
    }

    protected function negotiate_language()
    {
        $languages = $this->schema['data']['site']['validLanguages'];

        // Use GET, COOKIE or Accept-Language (basic parsing)
        $current = $this->schema['data']['CONTEXT']['GET'][LANG_KEY]
            ?? $this->schema['data']['CONTEXT']['COOKIES'][LANG_KEY]
            ?? $this->_best_match_language($languages)
            ?? '';

        if (!in_array($current, $languages, true)) {
            $current = $languages[0];
        }

        $this->schema['inherit']['locale']['current'] = $current;
    }

    protected function _best_match_language(array $languages)
    {
        // Very small Accept-Language parser: return first matching language
        if (!isset($_SERVER['HTTP_ACCEPT_LANGUAGE'])) return null;
        $accept = $_SERVER['HTTP_ACCEPT_LANGUAGE'];
        $parts = explode(',', $accept);
        foreach ($parts as $p) {
            $lang = strtolower(substr(trim($p), 0, 2));
            foreach ($languages as $valid) {
                if (strtolower(substr($valid, 0, 2)) === $lang) return $valid;
            }
        }
        return null;
    }

    protected function set_theme()
    {
        $this->schema['data']['site']['theme'] =
            $this->schema['data']['CONTEXT']['GET'][THEME_KEY]
            ?? $this->schema['data']['CONTEXT']['COOKIES'][THEME_KEY]
            ?? $this->schema['data']['site']['validThemes'][0];
    }

    public function get()
    {
        return $this->schema;
    }
}
