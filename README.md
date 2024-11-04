# CIT - Command Line Interface Tool for Git

CIT (Command Interface Tool) is a Rust-based command-line tool that serves as a wrapper around Git, providing an enhanced user experience with built-in logging and a simplified command interface. CIT allows you to perform common Git operations seamlessly while logging actions for better tracking and debugging.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Building the Project](#building-the-project)
- [Running the Tool](#running-the-tool)
- [Commands](#commands)
- [Contributing](#contributing)
- [Contact](#contact)
- [License](#license)

## Overview

CIT is designed for developers who want a more intuitive interface for Git operations. It simplifies common tasks such as committing, pushing, switching branches, and viewing logs. The tool logs each action taken, providing a clear history of operations for troubleshooting and audits.

## Features

- **User-Friendly Command Structure**: Simplified commands for common Git operations.
- **Logging**: Automatic logging of each action performed, aiding in debugging and tracking.
- **Error Handling**: Graceful error handling with informative messages.
- **Custom Help Command**: User-friendly help output to guide users on available commands.

## Installation

To install CIT, you need to have [Rust](https://www.rust-lang.org/tools/install) installed on your machine. Follow these steps to set up your development environment:

1. **Clone the repository**:
``` 
git clone https://github.com/ByteShifters/cit.git
```

2. **Navigate to the project directory**:
``` 
cd cit
```

3. **Build the project**:
``` 
cargo build --release
```

## Usage

To use CIT, run the compiled binary from the command line. You can execute various Git commands by specifying the desired subcommand.

### Commands

Use `cit [COMMAND]` to execute a command. For example:
``` 
./target/release/cit commit -m "Your commit message"
```

### Available Commands

- `commit` - Commits with a meaningful commit message.
- `push` - Pushes the current branch.
- `upd` - Adds all changes.
- `undo` - Undoes the last commit.
- `log` - Shows the git log.
- `switch` - Switches branches, creating if needed.
- `diff` - Shows the git diff.
- `upload` - Adds, commits, and pushes changes.

## Building the Project

To build the project, make sure you have the following prerequisites:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

After cloning the repository, you can build the project by running:

``` 
cargo build --release
```

This command compiles the source code and generates an executable in the `target/release/` directory.

## Running the Tool

After building, you can run the tool directly from the `target/release/` directory or set up a global command for easier access.

To run the tool, use the following command:

``` 
./target/release/cit [COMMAND]
```

Replace `[COMMAND]` with one of the available commands listed in the Commands section.

## Contributing

We welcome contributions! To contribute to CIT, please follow these steps:

1. **Fork the repository**.
2. **Create a new branch**: 
``` 
git checkout -b feature/YourFeatureName
``` 
3. **Make your changes**.
4. **Commit your changes**: 
``` 
git commit -m "Add a feature"
``` 
5. **Push to the branch**: 
``` 
git push origin feature/YourFeatureName
``` 
6. **Open a Pull Request**.

Please ensure that your code follows our coding standards and that you've tested your changes.

## Contact

For any inquiries or issues, please contact:

- **Maintainer**: Ren
- **Email**: ren@byteshifters.com

You can also open an issue on the GitHub repository for bugs or feature requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

