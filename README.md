# RuTeWall

A Rust-based command execution firewall that provides controlled access to system commands through a FIFO pipe interface. RuTeWall enforces security policies by validating commands against allow/deny lists, filesystem permissions, and network access rules before execution.

## Why RuTeWall?

RuTeWall addresses the need for secure command execution in constrained environments. It provides:

- **Command Whitelisting/Blacklisting**: Explicit control over which commands can be executed
- **Filesystem Access Control**: Fine-grained permissions (read/write/execute) for specific paths
- **Network Access Control**: Ability to block or allow network-capable commands
- **FIFO-based Interface**: Secure IPC mechanism for command submission
- **Configuration-driven**: Simple, human-readable configuration file format

Use cases include:
- Sandboxed development environments
- Restricted shell access for automated systems
- Secure CI/CD pipeline command execution
- Controlled access in multi-user systems

## Dependencies

- **Rust**: Edition 2021 or later
- **libc**: 0.2 (for FIFO creation and non-blocking I/O)

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd RuTeWall

# Build the project
cargo build --release

# The binary will be available at target/release/RuTeWall
```

### Using Cargo

```bash
cargo install --path .
```

## Configuration

RuTeWall uses a configuration file to define security policies. The configuration file is divided into sections:

### Command Lists

```ini
allow = [
    "cargo test",
    "cargo check",
    "cargo build",
    "git diff",
]

deny = [
    "git push",
    "rm -rf *",
]
```

- **allow**: Commands that are explicitly permitted (prefix matching supported)
- **deny**: Commands that are explicitly blocked (prefix matching supported)

### Filesystem Permissions

```ini
[filesystem]
read = ["./", "/home/user/documents"]
write = ["./src", "./target"]
execute = ["/usr/bin/git"]
```

- **read**: Paths from which files can be read
- **write**: Paths to which files can be written
- **execute**: Paths from which executables can be run

Permission modes can also use shorthand notation:
- `r` for read
- `w` for write
- `x` for execute

### Network Access

```ini
[network]
allow = false
```

- **allow**: Set to `true` to permit network commands (curl, wget, ssh, etc.), `false` to block

## Usage

### Basic Usage

```bash
# Start RuTeWall with a configuration file
./target/release/RuTeWall --path=/path/to/config.conf

# Specify a custom FIFO path
./target/release/RuTeWall --path=/path/to/config.conf --fifo=/tmp/myfifo

# Enable debug mode
./target/release/RuTeWall --path=/path/to/config.conf --debug
```

### Command Line Options

- `-d, --debug, -D`: Enable debug output
- `-h, --help, -H`: Display help information
- `--path=<file>, --conf=<file>`: Path to configuration file (required)
- `--fifo=<path>, --pipe=<path>`: Path to FIFO pipe (default: /tmp/fifo)

### Sending Commands

RuTeWall reads commands from the FIFO pipe. To send commands:

```bash
# Write to the FIFO
echo "cargo build" > /tmp/fifo

# Read the response
cat /tmp/fifo
```

### Example Workflow

1. **Create a configuration file** (`myconfig.conf`):
```ini
allow = [
    "ls",
    "cat",
    "echo",
]

deny = [
    "rm",
]

[filesystem]
read = ["./"]
write = ["./tmp"]

[network]
allow = false
```

2. **Start RuTeWall**:
```bash
./target/release/RuTeWall --path=myconfig.conf --fifo=/tmp/rutewall
```

3. **Send commands** (in another terminal):
```bash
# Allowed command
echo "ls -la" > /tmp/rutewall
cat /tmp/rutewall  # Output: Command Executed

# Denied command
echo "rm file.txt" > /tmp/rutewall
cat /tmp/rutewall  # Output: Command denied: rm

# Network command (blocked)
echo "curl http://example.com" > /tmp/rutewall
cat /tmp/rutewall  # Output: Network access disabled: curl
```

## Security Features

### Command Validation

- **Shell operator blocking**: Prevents command chaining with `|`, `&&`, `||`, `;`, `&`
- **Quote handling**: Properly parses quoted arguments
- **Prefix matching**: Allow/deny rules match command prefixes (e.g., `git` matches `git push`)

### Filesystem Protection

- **Path normalization**: Resolves `.` and `..` in paths
- **Absolute/relative path detection**: Identifies path-like arguments
- **Permission enforcement**: Checks read/write/execute access against configured rules

### Network Control

Blocks network-capable commands when disabled:
- curl, wget, ssh, scp, sftp, nc, ncat, telnet, ftp, git

## Examples

### Development Environment

```ini
allow = [
    "cargo",
    "git",
    "rustc",
]

deny = [
    "cargo publish",
    "git push",
]

[filesystem]
read = ["./"]
write = ["./src", "./target", "./Cargo.toml"]

[network]
allow = false
```

### CI/CD Pipeline

```ini
allow = [
    "npm install",
    "npm test",
    "npm build",
    "docker build",
]

deny = [
    "docker push",
    "npm publish",
]

[filesystem]
read = ["/workspace"]
write = ["/workspace/dist", "/workspace/node_modules"]

[network]
allow = true
```

## License

GPL-3.0-only - See LICENSE file for details.

## Contributing

Contributions are welcome! Please ensure:
- Code follows Rust best practices
- All tests pass (`cargo test`)
- New features include documentation
- Security implications are considered

## Troubleshooting

### FIFO Already Exists

If you see "FIFO already exists" errors, it's safe to ignore - RuTeWall will use the existing FIFO.

### Permission Denied

Ensure the FIFO path is writable:
```bash
chmod 666 /tmp/fifo
```

### Configuration Errors

- Check that the configuration file path is correct
- Verify the configuration file syntax matches the examples
- Enable debug mode with `-d` for detailed output
