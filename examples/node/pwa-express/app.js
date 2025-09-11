/**
 * Neutral TS Node.js example with Express
 * See: https://github.com/FranBarInstance/neutralts-docs
 */

const express = require('express');
const cookieParser = require('cookie-parser');
const bodyParser = require('body-parser');
const fs = require('fs');
const path = require('path');

const { STATIC_FOLDER, SIMULATE_SECRET_KEY } = require('./constants');
const Schema = require('./schema');
const Template = require('./template');

const app = express();

// Middleware
app.use(express.static(STATIC_FOLDER));
app.use(cookieParser());
app.use(bodyParser.urlencoded({ extended: true }));

// Display form login GET
app.get('/form-login', async (req, res) => {
    const route = 'form-login';
    const schema = new Schema(req, route);
    const template = new Template(schema.get());
    await template.render(req, res);
});

// Process login form in POST (Fake login)
app.post('/form-login', async (req, res) => {
    const route = 'form-login';
    let schema = new Schema(req, route);
    schema.schema.data.send_form_login = 1;

    // Fake login, any user, password: 1234
    const passwd = schema.schema.data.CONTEXT.POST.passwd;
    if (passwd === '1234') {
        schema.schema.data.send_form_login_fails = null;
        schema.schema.data.CONTEXT.SESSION = SIMULATE_SECRET_KEY;
    } else {
        schema.schema.data.send_form_login_fails = true;
    }

    const template = new Template(schema.get());
    await template.render(req, res);
});

// Logout
app.get('/logout', async (req, res) => {
    const route = 'logout';
    let schema = new Schema(req, route);
    schema.schema.data.CONTEXT.SESSION = null;
    const template = new Template(schema.get());
    await template.render(req, res);
});

// Home GET and POST
app.route('/')
    .get(async (req, res) => {
        const route = 'home';
        const schema = new Schema(req, route);
        const template = new Template(schema.get());
        await template.render(req, res);
    })
    .post(async (req, res) => {
        const route = 'home';
        const schema = new Schema(req, route);
        const template = new Template(schema.get());
        await template.render(req, res);
    });

app.use((req, res, next) => {
    let route = req.path.slice(1);
    if (!route) route = 'home';

    // Serve static files directly
    const filePath = path.join(STATIC_FOLDER, route);
    if (fs.existsSync(filePath) && !fs.statSync(filePath).isDirectory()) {
        return res.sendFile(filePath);
    }

    // Serve dynamic content
    //
    // To prevent arbitrary routes contents-[route]-snippets.ntpl must exist,
    // to create a simple view/route you wouldn't need a function or view, just create
    // the contents-[route]-snippets.ntpl template file.
    //
    // Following routes do not have a handler and are dispatched here:
    // /simulate-...
    // /help
    // /login
    //
    // /login is a container for /form-login (/form-login is loaded via ajax).
    const schema = new Schema(req, route);
    const template = new Template(schema.get());
    template.render(req, res).catch(next);
});

const PORT = 8000;
app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
