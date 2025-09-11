<?php
/**
 *   Neutral TS PHP example
 *   See: https://github.com/FranBarInstance/neutralts-docs
 */

require_once __DIR__ . '/schema.php';
require_once __DIR__ . '/template.php';

$requestUri = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);
$route = trim($requestUri, '/');

// Serve static files if they exist under STATIC_FOLDER
$staticPath = rtrim(STATIC_FOLDER, '/') . '/' . ltrim($route, '/');
if ($route !== '' && file_exists($staticPath) && !is_dir($staticPath)) {
    // Map common extensions to MIME types for consistent responses
    $mime_map = [
        'css' => 'text/css',
        'js' => 'application/javascript',
        'png' => 'image/png',
        'jpg' => 'image/jpeg',
        'jpeg' => 'image/jpeg',
        'gif' => 'image/gif',
        'svg' => 'image/svg+xml',
        'ico' => 'image/x-icon',
        'json' => 'application/json',
        'woff' => 'font/woff',
        'woff2' => 'font/woff2',
        'ttf' => 'font/ttf',
        'eot' => 'application/vnd.ms-fontobject',
    ];

    $ext = strtolower(pathinfo($staticPath, PATHINFO_EXTENSION));
    if (isset($mime_map[$ext])) {
        $mime = $mime_map[$ext];
    } else {
        // Fallback to system detection, then to a safe default
        $mime = mime_content_type($staticPath) ?: 'application/octet-stream';
    }

    header('Content-Type: ' . $mime);
    readfile($staticPath);
    exit;
}

// Route handling
switch ($_SERVER['REQUEST_METHOD']) {
    case 'GET':
        // Home
        if ($route === '' || $route === 'home') {
            $schema = new Schema('home');
            $template = new Template($schema->get());
            $template->render();
            exit;
        }

        // Display form login
        if ($route === 'form-login') {
            $schema = new Schema('form-login');
            $template = new Template($schema->get());
            $template->render();
            exit;
        }

        if ($route === 'logout') {
            $schema = new Schema('logout');
            $template = new Template($schema->get());
            $template->render();
            exit;
        }

        // Default catch-all dynamic render
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
        $schema = new Schema($route);
        $template = new Template($schema->get());
        $template->render();
        break;

    case 'POST':
        // Home POST
        if ($route === '' || $route === 'home') {
            $schema = new Schema('home');
            $template = new Template($schema->get());
            $template->render();
            exit;
        }

        // Process login form in POST (Fake login)
        if ($route === 'form-login') {
            $schema = new Schema('form-login');
            $s = $schema->get();
            // Indicate form was submitted
            $s['data']['send_form_login'] = 1;

            // Fake login: password must be '1234'
            $passwd = $s['data']['CONTEXT']['POST']['passwd'] ?? null;
            if ($passwd === '1234') {
                $s['data']['send_form_login_fails'] = null;
                // set a cookie to simulate session
                setcookie('SESSION', SIMULATE_SECRET_KEY, time() + 3600, '/');
                $s['data']['CONTEXT']['SESSION'] = SIMULATE_SECRET_KEY;
            } else {
                $s['data']['send_form_login_fails'] = true;
            }

            $template = new Template($s);
            $template->render();
            exit;
        }

        // For other POSTs, just render the route
        $schema = new Schema($route);
        $template = new Template($schema->get());
        $template->render();
        break;

    default:
        http_response_code(405);
        echo 'Method Not Allowed';
        break;
}
