"""
    Constants
    See: https://github.com/FranBarInstance/neutralts-docs
"""

import os

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

# Use cached templates
# TEMPLATE_ROUTER = BASE_DIR + "/../../neutral/tpl/cache.ntpl"

# Uncomment for no cached templates
TEMPLATE_ROUTER = BASE_DIR + "/../../neutral/tpl/index.ntpl"

# HTTP errors template
TEMPLATE_ERROR = BASE_DIR + "/../../neutral/tpl/cache_error.ntpl"

# static files
STATIC_FOLDER = BASE_DIR + "/../../neutral/static"

# Default schema
DEFAULT_SCHEMA = BASE_DIR + "/../../neutral/data/schema.json"

LANG_KEY = "lang"
THEME_KEY = "theme"
SIMULATE_SECRET_KEY = "69bdd1e4b4047d8f4e3"
