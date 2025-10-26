# dbgc

`dbgc` (debug comment controller) is a command-line tool for toggling debug printf statements in C/C++ code. It recursively searches through your codebase to find debug logging statements and allows you to comment or uncomment them with a single command.

[![Build status](https://github.com/Justhiro55/dbgc/workflows/ci/badge.svg)](https://github.com/Justhiro55/dbgc/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

### Features

* **Fast and recursive** - Processes entire directory trees including subdirectories
* **Interactive selection** - Choose specific statements to process with arrow keys and spacebar
* **Safe and reversible** - Comment out debug logs for production, uncomment them for debugging
* **Smart detection** - Automatically detects printf-family functions and C++ streams containing "debug" or "DEBUG" keywords
* **Flexible filtering** - Use `--all` flag to detect all output functions, not just debug statements
* **Syntax highlighting** - Color-coded output similar to ripgrep for easy reading
* **Multiple modes** - Comment out, uncomment, or permanently delete debug statements

### Quick example

```bash
# Comment out all debug statements in the current directory
$ dbgc off

# Uncomment all debug statements in a specific directory
$ dbgc on src/

# Interactively select which statements to comment out
$ dbgc off --interactive src/

# Comment out ALL output functions (not just debug)
$ dbgc off --all src/

# Delete all debug statements permanently
$ dbgc delete src/

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
dbgc <COMMAND> [OPTIONS] [PATH]

Commands:
  off      Comment out debug printf statements
  on       Uncomment debug printf statements
  delete   Delete debug printf statements permanently

Arguments:
  [PATH]  Path to file or directory (defaults to current directory if not specified)

Options:
  -y, --yes          Skip confirmation prompt
  -a, --all          Detect all output functions, not just debug statements
  -i, --interactive  Interactive mode for selecting specific statements
  -h, --help         Print help
```

### How it works

`dbgc` searches for C/C++ output functions where the string or stream contains the keyword "debug" or "DEBUG" (case-sensitive). This includes:
- C standard I/O functions (printf family, puts family, write, perror)
- C++ stream operators (std::cout, std::cerr, std::clog)

**Supported file extensions:**
- `.c`
- `.h`
- `.cpp`
- `.hpp`
- `.cc`
- `.cxx`

**Detected functions:**

C standard functions:
- `printf()`, `fprintf()`, `sprintf()`, `snprintf()`, `dprintf()`
- `puts()`, `fputs()`
- `write()`
- `perror()`
- `printf_debug()`

C++ streams:
- `std::cout`
- `std::cerr`
- `std::clog`

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

#### Delete debug logs permanently

```bash
dbgc delete src/
```

**Warning**: This permanently removes debug statement lines from your files. Use with caution!

#### Non-interactive mode (skip confirmation)

Use `-y` or `--yes` flag to skip the confirmation prompt, useful for scripts and automation:

```bash
# Comment out without confirmation
dbgc off --yes

# Delete without confirmation (use with caution!)
dbgc delete -y src/
```

#### Cancel operation

When prompted, type `n` to cancel without making changes:

```
Do you want to comment out these statements? (y/n): n
Operation cancelled.
```

#### Detect all output functions with --all flag

By default, `dbgc` only detects output functions containing "debug" or "DEBUG" keywords. Use the `--all` flag to detect all output functions regardless of content:

```bash
# Comment out ALL printf/cout statements, not just debug ones
dbgc off --all src/

# This will detect:
# - printf("debug: message") ← debug statement
# - printf("Regular message") ← also detected with --all
# - std::cout << "Any message" ← also detected with --all
```

This is useful for removing all logging before production builds.

#### Interactive mode with --interactive flag

Use interactive mode to selectively choose which statements to process:

```bash
# Interactively select which debug statements to comment out
dbgc off --interactive src/

# You'll see a list of all matches with:
# - Arrow keys to navigate
# - Space to toggle selection
# - Enter to confirm
# - Ctrl-C or q to cancel
```

Interactive mode works with all commands (`off`, `on`, `delete`) and can be combined with `--all`:

```bash
# Interactively select from ALL output functions
dbgc off --interactive --all src/
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
* **Avoid mistakes** - Interactive selection and confirmation prevents accidental changes
* **Clean commits** - Easily remove debug logs before committing
* **Quick debugging** - Re-enable all debug logs when investigating issues
* **Production cleanup** - Permanently delete debug statements with the delete command
* **Selective control** - Use interactive mode to choose exactly which statements to process
* **Comprehensive detection** - Use `--all` flag to find all output functions before releases

### Why not use dbgc?

* **Language specific** - Only works with C/C++ code
* **Simple pattern matching** - May not catch all debug logging patterns
* **Printf-based only** - Doesn't work with logging libraries (log4c, spdlog, etc.)

### License

This project is dual-licensed under the MIT License and Apache License 2.0.

See [LICENSE](LICENSE) for details.
