<?php
/**
 * Template wrapper for PHP example
 * Uses NeutralIpcTemplate to render templates via IPC.
 * See: https://github.com/FranBarInstance/neutralts-docs
 */

require_once __DIR__ . '/constants.php';
require_once __DIR__ . '/../ipc-client/NeutralIpcTemplate.php';

class Template
{
    protected $schema;

    public function __construct($schema)
    {
        $this->schema = $schema;
    }

    public function render()
    {
        $json = json_encode($this->schema);
        $template = new NeutralIpcTemplate(TEMPLATE_ROUTER, $json);
        $contents = $template->render();
        $status_code = intval($template->get_status_code());
        $status_text = $template->get_status_text();
        $status_param = $template->get_status_param();

        // Redirects from template
        if (in_array($status_code, [301, 302, 307, 308], true)) {
            header('Location: ' . $status_param, true, $status_code);
            exit;
        }

        // HTTP errors from template
        if ($status_code >= 400) {
            $error = [
                'data' => [
                    'CONTEXT' => ['ROUTE' => 'error'],
                    'error' => [
                        'code' => $status_code,
                        'text' => $status_text,
                        'param' => $status_param
                    ]
                ]
            ];
            $template->set_path(TEMPLATE_ERROR);
            $template->merge_schema(json_encode($error));
            $contents = $template->render();
        }

        http_response_code($status_code ?: 200);
        header('Content-Type: text/html');
        echo $contents;
        return null;
    }
}
