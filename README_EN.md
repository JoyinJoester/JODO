# Jodo - Simple Command Line Todo Application

Jodo is a command line Todo application developed using Rust, helping you manage tasks in your terminal.

[English](./README_EN.md) | [日本語](./README_JA.md) | [简体中文](./README.md)

## Key Features

- Manage to-do lists, supporting adding, editing, deleting, and marking completion status
- Set deadlines for tasks
- Mark important tasks and display them at the top
- Automatically categorize completed tasks
- View detailed task information
- Support for deleting multiple tasks at once

## Installation

### Direct Installation from GitHub

```bash
# Clone repository
git clone https://github.com/JoyinJoester/JODO.git
cd JODO

# Compile and install
cargo build --release
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

### Install Directly from GitHub Using Cargo

```bash
cargo install --git https://github.com/JoyinJoester/JODO.git
```

### Installation on Linux

1. First build the release:

```bash
cargo build --release
```

2. Then run the provided installation script:

```bash
sudo chmod +x ./debian_install.sh
sudo ./debian_install.sh
```

This script will copy the compiled binary to the `/usr/local/bin` directory and optionally configure the PATH environment variable.

3. Manual installation method:

If you don't want to use the installation script, you can install manually:

```bash
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

4. Verify installation:

After installation, you can open a new terminal and type:

```bash
jodo --version
```

If version information is displayed, the installation is successful.

### Install Using Cargo

If you have Rust and Cargo installed, you can also use the following command to install:

```bash
cargo install --path .
```

## Usage

### Basic Operations

```bash
jodo "Complete project report"         # Add new task
jodo "Complete project report" -t 2023-12-31 # Add task with deadline
jodo -l, --list                       # List all tasks
jodo                                  # Same as above, list all tasks
```

### Task Management Commands

```bash
jodo -e, --edit <id> --content "content"   # Edit task content
jodo -e, --edit <id> -t, --time date       # Edit task deadline
jodo -d, --delete <id>                     # Delete single task
jodo -d, --delete <id1> <id2> <id3>        # Delete multiple tasks at once
jodo -c, --complete <id>                   # Mark task as completed
jodo -u, --undo <id>                       # Unmark task as completed
jodo --star <id>                           # Mark task as important (pin to top)
jodo --unstar <id>                         # Remove important mark
jodo --show <id>                           # Show detailed task information
```

### Subcommand Form

```bash
jodo list                  # List all tasks
jodo done <id>             # Mark task as completed
jodo undo <id>             # Unmark task as completed
jodo remove <id>           # Delete task
jodo edit <id> "content"   # Edit task content
jodo edit <id> -t date     # Edit task deadline
jodo star <id>             # Mark task as important
jodo unstar <id>           # Remove important mark
jodo show <id>             # Show detailed task information
```

### Other Options

```bash
jodo -h, --help            # Display help information
jodo -v, --version         # Display version information
jodo -L, --language <lang> # Set language (zh-cn/en/ja)
```

### Notes

- Completed tasks have a 'c' suffix after their ID, e.g. `1c`
- Starred tasks are displayed at the top of the list
- Task IDs are automatically reassigned after modification or deletion to maintain continuity

## File Storage

Task data is stored in the `~/.jodo/tasks.json` file. Changes are automatically saved after each operation.

## Examples

```bash
# Add a new task
jodo "Complete project report"

# Add a task with deadline
jodo "Complete project report" -t 2023-12-31

# View all tasks
jodo -l

# Edit task content
jodo -e 1 --content "Modified content"

# Set task deadline
jodo -e 1 -t 2023-12-31

# Mark task as important
jodo --star 1

# View task details
jodo --show 1

# Mark task as completed
jodo -c 1

# Unmark task as completed
jodo -u 1c

# Delete single task
jodo -d 1

# Delete multiple tasks
jodo -d 1 3 5

# Switch to Chinese interface
jodo -L zh-cn
```
