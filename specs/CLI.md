# Command Line Interface Specification

## Overview

Ozymandias uses the `clap` crate for command-line argument parsing, providing a clean and intuitive interface for users.

## Command Structure

### Base Command

```
ozy [OPTIONS] <COMMAND>
```

### Global Options

- `-v, --verbose`: Increase verbosity (can be used multiple times)
- `-q, --quiet`: Decrease verbosity (can be used multiple times)
- `--log-level <LEVEL>`: Set the logging level
- `--log-format <FORMAT>`: Set the log output format

### Commands

#### init

```
ozy init [OPTIONS]
```

Initializes a new knowledge base.

## Command Implementation

### Command Traits

Each command implements the following trait:

```rust
pub trait Command {
    fn execute(&self) -> Result<(), anyhow::Error>;
}
```

### Error Handling

- Uses `anyhow` for error handling
- Provides clear, user-friendly error messages
- Includes detailed error information in logs

## Examples

```bash
# Initialize a new knowledge base
ozy init

# Initialize with verbose logging
ozy -v init

# Set specific log level
ozy --log-level debug init
```

