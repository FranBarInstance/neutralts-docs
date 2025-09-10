"""
    Neutral TS Python package example with Flask
    See: https://github.com/FranBarInstance/neutralts-docs
"""

import os

from constants import STATIC_FOLDER, SIMULATE_SECRET_KEY
from flask import Flask, request, send_from_directory
from schema import Schema
from template import Template

app = Flask(__name__)

# Catch all route
@app.route('/<path:route>', methods=['GET'])
def catch_all(route):
    """
    Default handle.

    To prevent arbitrary routes contents-[route]-snippets.ntpl must exist,
    to create a simple view/route you wouldn't need a function or view, just create
    the contents-[route]-snippets.ntpl template file.

    The /simulate-... and /help routes don't have a handle and are handled here.
    """

    # Serve static files directly
    file_path = os.path.join(STATIC_FOLDER, route)
    if os.path.exists(file_path) and not os.path.isdir(file_path):
        return send_from_directory(STATIC_FOLDER, route)

    # Serve dynamic content
    schema = Schema(request, route)
    template = Template(schema.get())
    return template.render()

# Container for /form-login (form-login is loaded via ajax)
@app.route('/login', defaults={'route': 'login'}, methods=['GET'])
def login(route):
    """Handle GET request login route"""
    schema = Schema(request, route)
    template = Template(schema.get())
    return template.render()

# Display form login
@app.route('/form-login', defaults={'route': 'form-login'}, methods=['GET'])
def form_login_get(route):
    """Handle GET request for form-login route"""
    schema = Schema(request, route)
    template = Template(schema.get())
    return template.render()

# Process login form in POST (Fake login)
@app.route('/form-login', defaults={'route': 'form-login'}, methods=['POST'])
def form_login_post(route):
    """Handle POST request for form-login route"""
    schema = Schema(request, route)
    schema.schema["data"]["send_form_login"] = 1

    # Fake login, any user, password: 1234
    if schema.schema["data"]["CONTEXT"]["POST"]["passwd"] == "1234":
        schema.schema["data"]["send_form_login_fails"] = None
        schema.schema["data"]["CONTEXT"]["SESSION"] = SIMULATE_SECRET_KEY
    else:
        schema.schema["data"]["send_form_login_fails"] = True

    template = Template(schema.get())
    return template.render()

# Logout
@app.route('/logout', defaults={'route': 'logout'}, methods=['GET'])
def logout(route):
    """Handle logout route"""
    schema = Schema(request, route)
    template = Template(schema.get())
    return template.render()

# Home GET and POST
@app.route('/', defaults={'route': 'home'}, methods=['GET', 'POST'])
def home(route):
    """Handle GET and POST request home route"""
    schema = Schema(request, route)
    template = Template(schema.get())
    return template.render()


if __name__ == '__main__':
    app.run(debug=True)
