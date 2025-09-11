/**
 * Fill schema
 * See: https://github.com/FranBarInstance/neutralts-docs
 */

const fs = require('fs');
const path = require('path');

const {
    DEFAULT_SCHEMA,
    LANG_KEY,
    THEME_KEY
} = require('./constants');

class Schema {
    constructor(req, route) {
        this.req = req;
        this.route = route.replace(/^[\/\\]+|[/\\]+$/g, '');
        this.schema = {};
        this.default();
        this.populateContext();
        this.negotiateLanguage();
        this.setTheme();
    }

    default() {
        const schemaJson = fs.readFileSync(DEFAULT_SCHEMA, 'utf8');
        this.schema = JSON.parse(schemaJson);
        if (!this.schema.data) this.schema.data = {};
        if (!this.schema.data.CONTEXT) this.schema.data.CONTEXT = {};
        const context = this.schema.data.CONTEXT;
        context.GET = context.GET || {};
        context.POST = context.POST || {};
        context.COOKIES = context.COOKIES || {};
        context.HEADERS = context.HEADERS || {};
    }

    populateContext() {
        const context = this.schema.data.CONTEXT;
        context.ROUTE = this.route;
        context.HEADERS.HOST = this.req.headers.host || null;

        // GET params
        Object.assign(context.GET, this.req.query);

        // POST params (assuming body-parser middleware)
        if (this.req.method === 'POST') {
            Object.assign(context.POST, this.req.body || {});
        }

        // Headers
        Object.assign(context.HEADERS, this.req.headers);

        // Cookies
        if (this.req.cookies) {
            Object.assign(context.COOKIES, this.req.cookies);
        }

        // Fake session
        context.SESSION = context.COOKIES.SESSION || null;
    }

    negotiateLanguage() {
        const languages = this.schema.data.site.validLanguages;
        const context = this.schema.data.CONTEXT;

        let current = context.GET[LANG_KEY] ||
                      context.COOKIES[LANG_KEY] ||
                      this.bestMatchLanguage(languages) ||
                      '';

        if (!languages.includes(current)) {
            current = languages[0];
        }

        this.schema.inherit.locale.current = current;
    }

    bestMatchLanguage(languages) {
        const acceptLanguage = this.req.headers['accept-language'];
        if (!acceptLanguage) return null;
        const parts = acceptLanguage.split(',');
        for (const p of parts) {
            const lang = p.trim().split(';')[0].split('-')[0].toLowerCase();
            for (const valid of languages) {
                if (valid.toLowerCase().startsWith(lang)) return valid;
            }
        }
        return null;
    }

    setTheme() {
        const context = this.schema.data.CONTEXT;
        this.schema.data.site.theme = context.GET[THEME_KEY] ||
                                      context.COOKIES[THEME_KEY] ||
                                      this.schema.data.site.validThemes[0];
    }

    get() {
        return this.schema;
    }
}

module.exports = Schema;
