<?php
/**
 * Constants for PHP PWA example
 * See: https://github.com/FranBarInstance/neutralts-docs
 */

define('BASE_DIR', dirname(__FILE__));

// Use cached templates by default
define('TEMPLATE_ROUTER', BASE_DIR . '/../../neutral/tpl/cache.ntpl');

// Uncomment to disable cache and use source templates
// define('TEMPLATE_ROUTER', BASE_DIR . '/../../neutral/tpl/index.ntpl');

// HTTP errors template
define('TEMPLATE_ERROR', BASE_DIR . '/../../neutral/tpl/cache_error.ntpl');

// Static files root
define('STATIC_FOLDER', BASE_DIR . '/../../neutral/static');

// Default schema path
define('DEFAULT_SCHEMA', BASE_DIR . '/../../neutral/data/schema.json');

define('LANG_KEY', 'lang');
define('THEME_KEY', 'theme');

// Fake session secret for examples
define('SIMULATE_SECRET_KEY', '69bdd1e4b4047d8f4e3');
