# Deployment Guide

Guide for deploying genmcp in various environments.

## Docker Deployment

### Building the Image

```bash
docker build -t genmcp .
```

### Running in STDIN/STDOUT Mode

For VS Code integration or other stdio-based MCP clients:

```bash
docker run -i genmcp serve --mode stdio
```

### Running in WebSocket Mode

```bash
docker run -p 8080:8080 genmcp serve \
  --mode websocket \
  --port 8080 \
  --host 0.0.0.0
```

### Custom Configuration

Mount your configuration file:

```bash
docker run -i -v /path/to/config.toml:/app/config.toml \
  -e GENMCP_CONFIG=/app/config.toml \
  genmcp serve --mode stdio
```

### Environment Variables

The Docker image runs as a non-root user (`genmcp`, UID 1000) for security.

## Bare Metal Installation

### Prerequisites

- Rust 1.92 or later
- Cargo

### Building from Source

```bash
git clone <repository-url>
cd genmcp
cargo build --release
```

The binary will be at `target/release/genmcp`.

### Installation

```bash
# Install to /usr/local/bin
sudo cp target/release/genmcp /usr/local/bin/

# Or install to user directory
cp target/release/genmcp ~/.local/bin/
```

### Running

`genmcp` uses the same TOML configuration for both transports. The transport is selected at runtime with `--mode`:

- `--mode stdio` (default): STDIN/STDOUT transport (typical for VS Code integration)
- `--mode websocket`: WebSocket transport (typical for hosted deployments)

```bash
# STDIN/STDOUT mode
genmcp serve --config /path/to/config.toml --mode stdio

# WebSocket mode
genmcp serve --config /path/to/config.toml --mode websocket --port 8080
```

## Systemd Service

Create `/etc/systemd/system/genmcp.service`:

```ini
[Unit]
Description=Generic MCP Script Adapter
After=network.target

[Service]
Type=simple
User=genmcp
WorkingDirectory=/opt/genmcp
ExecStart=/usr/local/bin/genmcp serve \
  --config /opt/genmcp/config.toml \
  --mode websocket \
  --port 8080
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable genmcp
sudo systemctl start genmcp
```

## VS Code Integration

Configure in VS Code settings:

```json
{
  "mcp.servers": {
    "genmcp": {
      "command": "genmcp",
      "args": ["serve", "--config", "/path/to/config.toml", "--mode", "stdio"]
    }
  }
}
```

## WebSocket Client Connection

Connect to WebSocket server:

```javascript
const ws = new WebSocket('ws://localhost:8080');
ws.onopen = () => {
  // Send MCP initialize request
  ws.send(JSON.stringify({
    jsonrpc: "2.0",
    id: 1,
    method: "initialize",
    params: {
      protocolVersion: "2024-11-05",
      capabilities: {},
      clientInfo: { name: "client", version: "1.0.0" }
    }
  }));
};
```

## Security Considerations

1. **File Permissions**: Ensure configuration file has appropriate permissions
2. **Command Paths**: Use absolute paths for commands in configuration
3. **Network Access**: In WebSocket mode, consider firewall rules
4. **Transport Security (TLS)**: WebSocket mode is plain `ws://` by default. For real deployments, run behind TLS termination (reverse proxy/ingress) and use `wss://` externally so Bearer tokens are not sent in cleartext.
5. **Authentication**: WebSocket auth supports JWT signature verification (shared secret or JWKS via OIDC/JWKS URL), but it does not currently expose detailed policy controls (e.g., audience/issuer allowlists, required claims) and should be reviewed for your threat model.
6. **User Permissions**: Run with minimal required privileges

## Troubleshooting

### Permission Denied

Ensure the binary and configuration file have appropriate permissions:

```bash
chmod +x genmcp
chmod 644 config.toml
```

### Port Already in Use

Change the port:

```bash
genmcp serve --config config.toml --mode websocket --port 8081
```

### Configuration Errors

Validate configuration:

```bash
genmcp config example > /tmp/config.toml
# Compare with your config
```

