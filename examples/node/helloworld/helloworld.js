/**
 * Neutral TS Hello World Node.js example
 * https://github.com/FranBarInstance/neutralts-docs/
 */

const path = require('path');

// Import NeutralIpcTemplate (IPC client for Node.js)
const NeutralIpcTemplate = require('../neutral_ipc_template/NeutralIpcTemplate');

// The schema contains among other things the data and variables for the template
const schema = {
    data: {
        hello: "Hello World"
    }
};

// Determine the template path
const templatePath = path.join(__dirname, 'template.ntpl');

// Create an instance of NeutralIpcTemplate
const ipcTemplate = new NeutralIpcTemplate(templatePath, schema);

// Render the template
(async () => {
    try {
        const contents = await ipcTemplate.render();

        // Print the rendered content, in other cases contents will be sent to output according to framework.
        console.log(contents);

    } catch (error) {
        console.error('Error rendering template:', error.message);
        process.exit(1);
    }
})();
