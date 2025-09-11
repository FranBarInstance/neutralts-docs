<?php

// Configuration module for Neutral IPC client.
// Reads configuration from /etc/neutral-ipc-cfg.json or uses default values.
// neutral-ipc-cfg.json is the configuration file used by the IPC server.
// https://github.com/FranBarInstance/neutral-ipc

class NeutralIpcConfig {
    const HOST = '127.0.0.1';
    const PORT = '4273';
    const TIMEOUT = 5;
    const BUFFER_SIZE = 8192;

    private static $configLoaded = false;
    private static $configValues = [];

    public static function getHost() {
        self::loadConfig();
        return self::$configValues['host'] ?? self::HOST;
    }

    public static function getPort() {
        self::loadConfig();
        return self::$configValues['port'] ?? self::PORT;
    }

    public static function getTimeout() {
        self::loadConfig();
        return self::$configValues['timeout'] ?? self::TIMEOUT;
    }

    public static function getBufferSize() {
        self::loadConfig();
        return self::$configValues['buffer_size'] ?? self::BUFFER_SIZE;
    }

    private static function loadConfig() {
        if (self::$configLoaded) {
            return;
        }

        $configFile = '/etc/neutral-ipc-cfg.json';

        if (file_exists($configFile) && is_readable($configFile)) {
            try {
                $configContent = file_get_contents($configFile);
                $configData = json_decode($configContent, true);

                if (json_last_error() === JSON_ERROR_NONE && is_array($configData)) {
                    self::$configValues = $configData;
                }
            } catch (Exception $e) {
                // Silently fall back to default values if config file is invalid
                error_log("NeutralIpcConfig: Error loading config file: " . $e->getMessage());
            }
        }

        self::$configLoaded = true;
    }
}
