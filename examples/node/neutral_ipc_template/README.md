Neutral TS IPC Client Node.js
=============================

**IPC Client Node.js is experimental.**

This package provides a minimal Node.js client for Neutral TS templates using the IPC server.

The client exposes `NeutralIpcTemplate` which lets you render a template by sending a schema
and template path (or source) to the IPC server and receiving the rendered content.

Example
-------

```javascript
// Require the client files included in this example
// https://github.com/FranBarInstance/neutral-ipc/tree/master/clients
const NeutralIpcTemplate = require('./neutral_ipc_template/NeutralIpcTemplate');

// The schema contains among other things the data and variables for the template
const schema = {
  data: {
    hello: 'Hello World'
  }
};

// Determine the template full path
const path = require('path');
const template = path.join(__dirname, 'template.ntpl');

// Create an instance of NeutralIpcTemplate (accepts object or JSON string for schema)
const ipcTemplate = new NeutralIpcTemplate(template, schema);

// Render the template (synchronous or asynchronous depending on the client implementation)
// Here we assume a synchronous-style API for simplicity. Consult the implementation in
// `NeutralIpcTemplate.js` for details.
const contents = ipcTemplate.render();

// e.g.: 200
const status_code = ipcTemplate.get_status_code();

// e.g.: OK
const status_text = ipcTemplate.get_status_text();

// empty if no error
const status_param = ipcTemplate.get_status_param();

// Act according to your framework to display the content
// for this example, simply output
console.log(contents);
```

Links
-----

Neutral TS template engine.

- [Template docs](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/)
- [Repository](https://github.com/FranBarInstance/neutralts)
- [Crate](https://crates.io/crates/neutralts)
- [Examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples)
