/**
 * Constants
 * See: https://github.com/FranBarInstance/neutralts-docs
 */

const path = require('path');

const BASE_DIR = path.dirname(__filename);

// Use cached templates
const TEMPLATE_ROUTER = path.join(__dirname, '../../neutral/tpl/cache.ntpl');

// Uncomment for no cached templates
// const TEMPLATE_ROUTER = path.join(__dirname, '../../neutral/tpl/index.ntpl');

// HTTP errors template
const TEMPLATE_ERROR = path.join(__dirname, '../../neutral/tpl/cache_error.ntpl');

// static files
const STATIC_FOLDER = path.join(__dirname, '../../neutral/static');

// Default schema
const DEFAULT_SCHEMA = path.join(__dirname, '../../neutral/data/schema.json');

const LANG_KEY = 'lang';
const THEME_KEY = 'theme';
const SIMULATE_SECRET_KEY = '69bdd1e4b4047d8f4e3';

module.exports = {
    BASE_DIR,
    TEMPLATE_ROUTER,
    TEMPLATE_ERROR,
    STATIC_FOLDER,
    DEFAULT_SCHEMA,
    LANG_KEY,
    THEME_KEY,
    SIMULATE_SECRET_KEY
};
