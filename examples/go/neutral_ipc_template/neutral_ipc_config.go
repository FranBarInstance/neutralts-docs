package neutral_ipc_template

import (
	"encoding/json"
	"os"
)

// Default values
const (
	DefaultHost    = "127.0.0.1"
	DefaultPort    = 4273
	DefaultTimeout = 10
	DefaultBufSize = 8192
)

// ConfigFile is the path to the IPC server configuration file.
const ConfigFile = "/etc/neutral-ipc-cfg.json"

// LoadConfig loads configuration from the JSON file if it exists.
func LoadConfig() map[string]interface{} {
	if _, err := os.Stat(ConfigFile); os.IsNotExist(err) {
		return make(map[string]interface{})
	}

	file, err := os.Open(ConfigFile)
	if err != nil {
		return make(map[string]interface{})
	}
	defer file.Close()

	var config map[string]interface{}
	if err := json.NewDecoder(file).Decode(&config); err != nil {
		return make(map[string]interface{})
	}
	return config
}

// GetHost returns the configured host or default.
func GetHost() string {
	config := LoadConfig()
	if host, ok := config["host"].(string); ok && host != "" {
		return host
	}
	return DefaultHost
}

// GetPort returns the configured port or default.
func GetPort() int {
	config := LoadConfig()
	if port, ok := config["port"].(float64); ok {
		return int(port)
	}
	if port, ok := config["port"].(int); ok {
		return port
	}
	return DefaultPort
}

// GetTimeout returns the configured timeout or default.
func GetTimeout() int {
	config := LoadConfig()
	if timeout, ok := config["timeout"].(float64); ok {
		return int(timeout)
	}
	if timeout, ok := config["timeout"].(int); ok {
		return timeout
	}
	return DefaultTimeout
}

// GetBufferSize returns the configured buffer size or default.
func GetBufferSize() int {
	config := LoadConfig()
	if bufSize, ok := config["buffer_size"].(float64); ok {
		return int(bufSize)
	}
	if bufSize, ok := config["buffer_size"].(int); ok {
		return bufSize
	}
	return DefaultBufSize
}
