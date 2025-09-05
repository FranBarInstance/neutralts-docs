"""
Neutral TS Hello World IPC example
https://github.com/FranBarInstance/neutralts-docs/
"""

import json  # Required to pass the schema as json to NeutralTemplate
import os  # Required to determine the path of the .ntpl template

# Import NeutralIpcTemplate: https://github.com/FranBarInstance/neutral-ipc/clients
from neutral_ipc_template import NeutralIpcTemplate

# The schema contains among other things the data and variables for the template
schema_dict = {
    "data": {
        "hello": "Hello World"
    }
}

# Determine the template path
template = os.path.dirname(os.path.abspath(__file__)) + "/template.ntpl"

# Pass the schema as json to NeutralTemplate
schema_json = json.dumps(schema_dict)

# Create an instance of NeutralTemplate
ipc_template = NeutralIpcTemplate(template, schema_json)

# Render the template
contents = ipc_template.render()

# Print the rendered content, in other cases contents will be sent to output according to framework.
print(contents)
