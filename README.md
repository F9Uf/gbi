# GBI - Git Branch Interactive

A terminal-based interactive Git branch selector built with Rust. Navigate and switch between Git branches with an intuitive keyboard-driven UI.

## Description

GBI (Git Branch Interactive) is a lightweight terminal user interface application that provides a fast and efficient way to view and switch between Git branches. Built with Rust, it offers a responsive interactive experience with vim-style navigation keybindings.

## Technology Stack

- **Language**: Rust (Edition 2021)
- **Dependencies**:
  - `clap 4.0` - Command-line argument parsing (with derive features)
  - `crossterm 0.29.0` - Cross-platform terminal manipulation
  - `git2 0.20.3` - Git repository interaction
  - `ratatui 0.20.0` - Terminal user interface framework

## Project Architecture

GBI follows a modular architecture with clear separation of concerns:

- **UI Layer**: Built with Ratatui, handling terminal rendering and user interactions
- **Git Layer**: Abstracts Git operations using libgit2 bindings
- **Application Layer**: Coordinates between UI and Git functionality with a stateful app model

The application uses a single-threaded event loop pattern that:
1. Fetches branch information from the Git repository
2. Renders an interactive list of branches
3. Processes keyboard input events
4. Executes Git operations based on user selection

## Getting Started

### Prerequisites

- Rust toolchain (1.56.0 or later)
- Git installed on your system

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd gbi
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

Or use the compiled binary:
```bash
./target/release/gbi
```

### Usage

Navigate to any Git repository and run `gbi`. The application will display:
- A list of all available branches
- Current branch marked with `*`
- Keyboard shortcuts in the footer

**Keyboard Controls**:
- `↑/↓` or `k/j` - Navigate through branches
- `ENTER` - Checkout selected branch
- `q` - Quit application

## Project Structure

```
gbi/
├── Cargo.toml           # Project dependencies and metadata
├── src/
│   ├── main.rs          # Application entry point and UI logic
│   ├── lib.rs           # Library root module
│   └── git/
│       ├── mod.rs       # Git module declaration
│       ├── branch.rs    # Branch operations (list, checkout, delete)
│       └── constants.rs # Git-related constants
└── target/              # Build artifacts (generated)
```

## Key Features

- **Interactive Branch List**: Visual display of all Git branches with highlighted selection
- **Current Branch Indicator**: Clear marking of the currently checked-out branch
- **Vim-style Navigation**: Familiar `j/k` keybindings for navigation
- **Fast Branch Switching**: Quick checkout with a single keypress
- **Terminal UI**: Clean, responsive interface that works in any terminal
- **Cross-platform**: Works on Linux, macOS, and Windows

## Development Workflow

### Building

```bash
# Debug build
cargo build

# Release build with optimizations
cargo build --release
```

### Running in Development

```bash
cargo run
```

### Code Structure Guidelines

- Git operations are isolated in the `git` module for maintainability
- UI rendering logic is separated from event handling
- Error handling uses Rust's `Result` type with boxed trait objects for flexibility

## Coding Standards

- **Rust Edition**: 2021
- **Formatting**: Follow standard Rust formatting conventions (use `cargo fmt`)
- **Error Handling**: Use `Result<T, Box<dyn Error>>` for error propagation
- **Naming Conventions**: 
  - Snake case for functions and variables
  - Pascal case for types and structs
  - Constants in SCREAMING_SNAKE_CASE
- **Module Organization**: Group related functionality into dedicated modules
- **Documentation**: Use doc comments (`///`) for public APIs

## Testing

Run tests with:
```bash
cargo test
```

The project currently uses Rust's built-in testing framework. Additional integration tests can be added in the `tests/` directory.

## Git Module API

### Branch Operations

- `get_current_branch(repo_name: &str)` - Returns the name of the currently checked-out branch
- `list_branches(repo_name: &str)` - Returns a vector of all branch names
- `checkout_branch(repo_name: &str, branch_name: &str)` - Checks out the specified branch
- `delete_branch(repo_name: &str, branch_name: &str)` - Deletes a local branch

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following the coding standards
4. Ensure tests pass (`cargo test`)
5. Format your code (`cargo fmt`)
6. Run clippy for linting (`cargo clippy`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Code Review Guidelines

- Keep changes focused and atomic
- Write clear commit messages
- Add tests for new functionality
- Update documentation as needed

## Future Enhancements

Potential features for future development:
- Branch creation interface
- Branch deletion with confirmation
- Remote branch support
- Branch search/filter functionality
- Customizable keybindings
- Configuration file support

## License

This project's license information is not currently specified. Please add a LICENSE file to clarify usage terms.

## Acknowledgments

Built with:
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [git2-rs](https://github.com/rust-lang/git2-rs) - Rust bindings to libgit2
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation library

---

**Note**: This README was generated with assistance from AI/LLM technology based on code analysis and project structure examination.