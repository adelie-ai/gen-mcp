# Docker Configuration Examples

This document shows how to use the Docker configuration file with the genmcp server.

## Configuration File

The `docker_config.toml` file defines Docker commands as MCP tools. Each tool accepts a single `args` parameter that contains the complete docker command arguments.

## Usage Examples

### Running a Container

**Tool:** `docker_run`

**Example 1: Simple container**
```json
{
  "name": "docker_run",
  "arguments": {
    "args": "run --name my-nginx -p 8080:80 -d nginx:latest"
  }
}
```

**Example 2: Container with volumes and environment variables**
```json
{
  "name": "docker_run",
  "arguments": {
    "args": "run --name my-app -p 3000:3000 -v /host/data:/app/data -e NODE_ENV=production -e DATABASE_URL=postgres://localhost/db -d myapp:latest"
  }
}
```

**Example 3: Container with restart policy**
```json
{
  "name": "docker_run",
  "arguments": {
    "args": "run --name my-service --restart always -p 8080:80 nginx:latest"
  }
}
```

**Example 4: Container with custom command**
```json
{
  "name": "docker_run",
  "arguments": {
    "args": "run --name my-nginx -p 8080:80 nginx:latest nginx -g 'daemon off;'"
  }
}
```

### Listing Containers

**Tool:** `docker_ps`

**Example 1: List running containers**
```json
{
  "name": "docker_ps",
  "arguments": {
    "args": "ps"
  }
}
```

**Example 2: List all containers**
```json
{
  "name": "docker_ps",
  "arguments": {
    "args": "ps -a"
  }
}
```

**Example 3: Filter containers**
```json
{
  "name": "docker_ps",
  "arguments": {
    "args": "ps --filter name=my-container"
  }
}
```

### Stopping Containers

**Tool:** `docker_stop`

**Example: Stop a container**
```json
{
  "name": "docker_stop",
  "arguments": {
    "args": "stop my-container"
  }
}
```

**Example: Stop with timeout**
```json
{
  "name": "docker_stop",
  "arguments": {
    "args": "stop -t 10 my-container"
  }
}
```

### Removing Containers

**Tool:** `docker_rm`

**Example 1: Remove a stopped container**
```json
{
  "name": "docker_rm",
  "arguments": {
    "args": "rm my-container"
  }
}
```

**Example 2: Force remove a running container**
```json
{
  "name": "docker_rm",
  "arguments": {
    "args": "rm -f my-container"
  }
}
```

### Viewing Logs

**Tool:** `docker_logs`

**Example 1: View logs**
```json
{
  "name": "docker_logs",
  "arguments": {
    "args": "logs my-container"
  }
}
```

**Example 2: Follow logs (with stop_after)**
```json
{
  "name": "docker_logs",
  "arguments": {
    "args": "logs -f my-container",
    "stop_after": 30
  }
}
```

**Example 3: Last 100 lines**
```json
{
  "name": "docker_logs",
  "arguments": {
    "args": "logs --tail 100 my-container"
  }
}
```

### Executing Commands in Containers

**Tool:** `docker_exec`

**Example 1: Execute a command**
```json
{
  "name": "docker_exec",
  "arguments": {
    "args": "exec my-container ls -la /app"
  }
}
```

**Example 2: Interactive shell**
```json
{
  "name": "docker_exec",
  "arguments": {
    "args": "exec -it my-container /bin/bash"
  }
}
```

### Inspecting Containers

**Tool:** `docker_inspect`

**Example 1: Full inspection**
```json
{
  "name": "docker_inspect",
  "arguments": {
    "args": "inspect my-container"
  }
}
```

**Example 2: Formatted output**
```json
{
  "name": "docker_inspect",
  "arguments": {
    "args": "inspect --format '{{.State.Status}}' my-container"
  }
}
```

### Listing Images

**Tool:** `docker_images`

**Example 1: List images**
```json
{
  "name": "docker_images",
  "arguments": {
    "args": "images"
  }
}
```

**Example 2: List all images**
```json
{
  "name": "docker_images",
  "arguments": {
    "args": "images -a"
  }
}
```

### Pulling Images

**Tool:** `docker_pull`

**Example: Pull an image**
```json
{
  "name": "docker_pull",
  "arguments": {
    "args": "pull nginx:latest"
  }
}
```

### Building Images

**Tool:** `docker_build`

**Example 1: Simple build**
```json
{
  "name": "docker_build",
  "arguments": {
    "args": "build -t myapp:latest ."
  }
}
```

**Example 2: Build with custom Dockerfile**
```json
{
  "name": "docker_build",
  "arguments": {
    "args": "build -f Dockerfile.prod -t myapp:v1.0 ./myapp"
  }
}
```

## Runtime Overrides

All tools support runtime overrides for timeout, stop_after, and output limits:

**Example: Run with extended timeout**
```json
{
  "name": "docker_build",
  "arguments": {
    "args": "build -t myapp:latest .",
    "timeout": 1800
  }
}
```

**Example: Follow logs for 60 seconds**
```json
{
  "name": "docker_logs",
  "arguments": {
    "args": "logs -f my-container",
    "stop_after": 60
  }
}
```

## Notes

- All Docker commands are executed via `/usr/bin/docker`
- The `args` parameter is automatically split by spaces, so you can pass the full command line as a single string
- The `args` parameter should include the docker subcommand (run, ps, stop, etc.) and all flags
- For complex commands, construct the full argument string including all flags and values
- Basic quote handling is supported (quotes around arguments are stripped)
- Use `stop_after` runtime override for long-running operations like `docker logs -f`
- Example: `"run --name my-container -p 8080:80 nginx:latest"` will be split into: `["run", "--name", "my-container", "-p", "8080:80", "nginx:latest"]`

