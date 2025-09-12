# Neutral TS IPC Client Python

This package provides a minimal Python client for Neutral TS templates using the IPC server.

The client exposes `NeutralIpcTemplate` which lets you render a template by sending a schema
and template path (or source) to the IPC server and receiving the rendered content.

Alternatively, you can use Neutral TS as a package without needing an IPC server:
[pypi.org/project/neutraltemplate/](https://pypi.org/project/neutraltemplate/)

Example
-------

```python
import json  # Required to pass the schema as json to NeutralIpcTemplate
import os  # Required to determine the path of the .ntpl template
import sys

# Add parent directory to sys.path to find sibling packages
sys.path.append(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))

# Import NeutralIpcTemplate:
# https://github.com/FranBarInstance/neutral-ipc/tree/master/clients
from neutral_ipc_template import NeutralIpcTemplate

# The schema contains among other things the data and variables for the template
schema = {
    "data": {
        "hello": "Hello World"
    }
}

# Determine the template full path
template = os.path.dirname(os.path.abspath(__file__)) + "/template.ntpl"

# Pass the schema as json to NeutralTemplate
schema_json = json.dumps(schema_dict)

# Create an instance of NeutralTemplate
ipc_template = NeutralIpcTemplate(template, schema_json)

# Render the template
contents = ipc_template.render()

# e.g.: 200
status_code = ipc_template.get_status_code()

# e.g.: OK
status_text = ipc_template.get_status_text()

# empty if no error
status_param = ipc_template.get_status_param()

# Act according to your framework to display the content
# for this example, simply output
print(contents)

```

Links
-----

Neutral TS template engine.

- [Template docs](https://github.com/FranBarInstance/neutralts-docs/docs/neutralts/doc/)
- [Repository](https://github.com/FranBarInstance/neutralts)
- [Crate](https://crates.io/crates/neutralts)
- [Examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples)
