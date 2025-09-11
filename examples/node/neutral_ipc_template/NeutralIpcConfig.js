/**
 * Configuration module for Neutral IPC client.
 * Reads configuration from /etc/neutral-ipc-cfg.json or uses default values.
 * https://github.com/FranBarInstance/neutral-ipc
 */

const fs = require('fs');

class NeutralIpcConfig {
    static HOST = '127.0.0.1';
    static PORT = 4273;
    static TIMEOUT = 10000; // milliseconds
    static BUFFER_SIZE = 8192;
    static CONFIG_FILE = '/etc/neutral-ipc-cfg.json';

    static loadConfig() {
        if (!fs.existsSync(this.CONFIG_FILE)) {
            return {};
        }
        try {
            return JSON.parse(fs.readFileSync(this.CONFIG_FILE, 'utf8'));
        } catch (e) {
            return {};
        }
    }

    static getConfigValue(configDict, key, defaultValue) {
        const value = configDict[key];
        if (value === undefined) {
            return defaultValue;
        }
        if (key === 'host' && typeof value === 'string') {
            return value;
        }
        if (['port', 'timeout', 'buffer_size'].includes(key) && typeof value === 'number') {
            return value;
        }
        return defaultValue;
    }

    static getHost() {
        const config = this.loadConfig();
        return this.getConfigValue(config, 'host', this.HOST);
    }

    static getPort() {
        const config = this.loadConfig();
        return this.getConfigValue(config, 'port', this.PORT);
    }

    static getTimeout() {
        const config = this.loadConfig();
        return this.getConfigValue(config, 'timeout', this.TIMEOUT);
    }

    static getBufferSize() {
        const config = this.loadConfig();
        return this.getConfigValue(config, 'buffer_size', this.BUFFER_SIZE);
    }
}

// Module exports for direct access
module.exports.HOST = NeutralIpcConfig.getHost();
module.exports.PORT = NeutralIpcConfig.getPort();
module.exports.TIMEOUT = NeutralIpcConfig.getTimeout();
module.exports.BUFFER_SIZE = NeutralIpcConfig.getBufferSize();
module.exports.NeutralIpcConfig = NeutralIpcConfig;
