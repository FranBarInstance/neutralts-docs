/**
 * Template and response
 * See: https://github.com/FranBarInstance/neutralts-docs
 */

const NeutralIpcTemplate = require('../neutral_ipc_template/NeutralIpcTemplate');

const {
    TEMPLATE_ROUTER,
    TEMPLATE_ERROR
} = require('./constants');

class Template {
    constructor(schema) {
        this.schema = schema;
    }

    async render(req, res) {
        let template = new NeutralIpcTemplate(TEMPLATE_ROUTER, JSON.stringify(this.schema));
        let contents = await template.render();
        let statusCode = template.getStatusCode();
        let statusText = template.getStatusText();
        let statusParam = template.getStatusParam();

        // The template may generate redirects.
        if (statusCode && [301, 302, 307, 308].includes(parseInt(statusCode))) {
            return res.redirect(parseInt(statusCode), statusParam);
        }

        // The template may generate HTTP errors.
        if (statusCode && parseInt(statusCode) >= 400) {
            const error = {
                data: {
                    CONTEXT: {
                        ROUTE: 'error'
                    },
                    error: {
                        code: parseInt(statusCode),
                        text: statusText,
                        param: statusParam
                    }
                }
            };
            template.setPath(TEMPLATE_ERROR);
            template.mergeSchema(JSON.stringify(error));
            contents = await template.render();
        }

        return res.status(parseInt(statusCode) || 200).type('text/html').send(contents);
    }
}

module.exports = Template;
