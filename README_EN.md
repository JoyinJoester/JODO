# JODO - Simple Command-Line Todo Application

English | [日本語](./README_JA.md) | [简体中文](./README.md)

JODO is a lightweight command-line Todo application that helps you efficiently manage your daily tasks.

## Features

- Clean command-line interface
- Task due date support
- Important task marking
- Colorful due date display (overdue, urgent, upcoming)
- Batch task addition and operations
- Multi-language support (Chinese, English, Japanese)

## Installation

### Direct Installation from GitHub

```bash
# Clone the repository
git clone https://github.com/JoyinJoester/JODO.git
cd JODO

# Compile and install
cargo build --release
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

### Install from GitHub using Cargo

```bash
cargo install --git https://github.com/JoyinJoester/JODO.git
```

### Install on Linux

1. First, build the release:

```bash
cargo build --release
```

2. Run the provided installation script:

```bash
sudo chmod +x ./debian_install.sh
sudo ./debian_install.sh
```

This script will copy the compiled binary to the `/usr/local/bin` directory and optionally set the PATH environment variable.

3. Manual installation method:

If you prefer not to use the installation script, you can install manually:

```bash
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

4. Verify the installation:

After installation, open a new terminal and type:

```bash
jodo --version
```

If version information is displayed, the installation was successful.

### Install using Cargo

If you already have Rust and Cargo installed, you can also install using:

```bash
cargo install --path .
```

### Troubleshooting

#### Cargo.lock Version Compatibility Issue

If you encounter errors related to the `Cargo.lock` file when compiling on different devices, this may be due to format version incompatibility. In this project, `Cargo.lock` uses version 4 format (line 3 of the file shows `version = 4`), but some older versions of Rust or Cargo may only support version 3.

**Solutions**:

1. Modify the Cargo.lock file:
   ```bash
   # Change "version = 4" to "version = 3" in the Cargo.lock file
   sed -i 's/version = 4/version = 3/' Cargo.lock
   ```

2. Or update your Rust toolchain:
   ```bash
   rustup update
   ```

3. Alternatively, regenerate the Cargo.lock completely:
   ```bash
   rm Cargo.lock
   cargo build
   ```

Note: Cargo.lock version 4 was introduced in Rust 1.62.0, so you may encounter compatibility issues if you're using an older Rust version.

## Usage

### Basic Operations

```bash
# Add a new task
jodo "Complete project report"

# Add a task with a due date
jodo "Complete project report" -t 2023-12-31

# List all tasks
jodo -l
jodo
```

### Task Management

```bash
# Edit task content
jodo -e 1 "Updated task content"

# Change task due date
jodo -e 1 -t 2023-12-25
jodo -t 1 2023-12-25  # Shortcut

# Mark task as completed
jodo -c 1
jodo -c 1 2 3  # Complete multiple tasks at once

# Mark task as incomplete
jodo -u 1

# Mark task as important
jodo --star 1

# Remove important mark from task
jodo --unstar 1

# Delete task
jodo -d 1
jodo -d 1 2 3  # Delete multiple tasks at once

# View task details
jodo --show 1
```

### Batch Mode

```bash
# Enter batch mode
jodo -m

# Example session:
jodo$> Complete first task
jodo$> Complete second task
jodo$> Learn Rust programming
jodo$> exit  # Exit batch mode
```

### Other Options

```bash
# Display help information
jodo -h

# Display version information
jodo -v

# Change language
jodo -L en  # English
jodo -L zh-cn  # Chinese
jodo -L ja  # Japanese
```

## Due Date Color Legend

- **Bold Red**: Overdue
- **Bright Red**: Due today (urgent)
- **Yellow**: Due within 3 days (soon)
- **Normal color**: Other dates

## Data Files

JODO stores all task data in the following location:

- Task data: `~/.jodo/tasks.json`

Data is automatically saved to this file after each task operation. To backup your data, simply copy this file.

## License

[MIT License](LICENSE)
