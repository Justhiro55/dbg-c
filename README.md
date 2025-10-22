# dbgc

`dbgc` (debug comment controller) is a command-line tool for toggling debug printf statements in C/C++ code. It recursively searches through your codebase to find debug logging statements and allows you to comment or uncomment them with a single command.

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

### Features

* **Fast and recursive** - Processes entire directory trees including subdirectories
* **Interactive confirmation** - Shows all matches with syntax highlighting before making changes
* **Safe and reversible** - Comment out debug logs for production, uncomment them for debugging
* **Smart detection** - Automatically detects printf-family functions containing "debug" or "DEBUG" keywords
* **Syntax highlighting** - Color-coded output similar to ripgrep for easy reading

### Quick example

```bash
# Comment out all debug statements in the current directory
$ dbgc off

# Uncomment all debug statements in a specific directory
$ dbgc on src/

# Process a single file
$ dbgc off main.c
```

### Screenshot

When you run `dbgc`, it displays all matching debug statements grouped by file:
<img width="510" height="398" alt="CleanShot 2025-10-22 at 11 40 07@2x" src="https://github.com/user-attachments/assets/c37704fd-cef6-4cd9-a454-977f59969690" />

File names are displayed in **magenta**, line numbers in **green**, and debug keywords in **bold red**.

### Installation

#### From source

You'll need [Rust](https://www.rust-lang.org/) installed (1.70.0 or newer).

```bash
git clone https://github.com/yourusername/dbgc.git
cd dbgc
cargo build --release
```

The binary will be available at `./target/release/dbgc`.

#### Install to system

```bash
cargo install --path .
# Or copy the binary manually
sudo cp target/release/dbgc /usr/local/bin/
```

### Usage

```
dbgc <COMMAND> [PATH]

Commands:
  off   Comment out debug printf statements
  on    Uncomment debug printf statements

Arguments:
  [PATH]  Path to file or directory (defaults to current directory if not specified)
```

### How it works

`dbgc` searches for printf-family functions (printf, fprintf, sprintf, snprintf, dprintf, printf_debug) where the format string contains the keyword "debug" or "DEBUG" (case-sensitive).

**Supported file extensions:**
- `.c`
- `.h`
- `.cpp`
- `.hpp`
- `.cc`
- `.cxx`

**Detected functions:**
- `printf()`
- `fprintf()`
- `sprintf()`
- `snprintf()`
- `dprintf()`
- `printf_debug()`

### Examples

#### Comment out debug logs in current directory

```bash
dbgc off
```

#### Comment out debug logs in a specific directory

```bash
dbgc off src/
```

#### Re-enable debug logs for debugging

```bash
dbgc on src/
```

#### Process a single file

```bash
dbgc off main.c
```

#### Cancel operation

When prompted, type `n` to cancel without making changes:

```
Do you want to comment out these statements? (y/n): n
Operation cancelled.
```

### Building

```bash
cargo build --release
```

### Testing

The repository includes sample C files in the `tests/` and `sample/` directories for testing:

```bash
# Build the project
cargo build --release

# Navigate to the tests directory and run dbgc without path argument
cd tests/
../target/release/dbgc off

# Comment out all debug statements (type 'y' to confirm)
../target/release/dbgc off

# Verify they are commented
cat main.c

# Uncomment all debug statements (type 'y' to confirm)
../target/release/dbgc on

# Verify they are restored
cat main.c

# Go back to project root
cd ..
```

You can also test with the `sample/` directory which contains more comprehensive examples:

```bash
./target/release/dbgc off sample/
```

### Why use dbgc?

* **Speed up your workflow** - No need to manually search and comment debug statements
* **Avoid mistakes** - Interactive confirmation prevents accidental changes
* **Clean commits** - Easily remove debug logs before committing
* **Quick debugging** - Re-enable all debug logs when investigating issues

### Why not use dbgc?

* **Language specific** - Only works with C/C++ code
* **Simple pattern matching** - May not catch all debug logging patterns
* **Printf-based only** - Doesn't work with logging libraries (log4c, spdlog, etc.)

### License

This project is dual-licensed under the MIT License and Apache License 2.0.

See [LICENSE](LICENSE) for details.
