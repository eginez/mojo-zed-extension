# Mojo Zed Extension

A Zed editor extension that provides comprehensive Mojo programming language support.

## Features

- **Syntax Highlighting**: Full Mojo syntax highlighting using the `lsh/tree-sitter-mojo` grammar
- **File Association**: Automatic recognition of `.mojo` and `.🔥` files
- **Language Server Protocol (LSP)**: Complete IDE features including:
  - Auto-completion
  - Go-to-definition
  - Error diagnostics
  - Code formatting
  - And more via `mojo-lsp-server`

## Prerequisites

To use the language server features, you need Mojo installed with `mojo-lsp-server` available in your environment:

- Install Mojo from [https://docs.modular.com/mojo/](https://docs.modular.com/mojo/)
- Ensure `mojo-lsp-server` is in your PATH, or available via pixi/conda

The extension will automatically detect `mojo-lsp-server` in:
- System PATH
- Project's `.pixi/envs/default/bin/` directory
- Common installation locations

## Installation

1. Clone this repository
2. Initialize the grammar submodule:
   ```bash
   git submodule update --init --recursive
   ```
3. Build the extension:
   ```bash
   cargo build --target wasm32-wasip1
   ```
4. Install in Zed as a dev extension

## Development

### Project Structure

```
├── extension.toml          # Extension configuration
├── src/
│   └── lib.rs             # Main extension logic and LSP discovery
├── languages/mojo/
│   ├── config.toml        # Language configuration
│   ├── *.scm             # Syntax highlighting queries (symlinked + brackets.scm)
└── grammars/mojo/         # Tree-sitter grammar (git submodule)
    └── queries/           # Original query files
```

### Building and Testing

```bash
# Run tests
cargo test

# Build extension
cargo build --target wasm32-wasip1

# Test in Zed
# Open Zed and install as dev extension
```

## Contributing

1. Make sure submodules are updated: `git submodule update --recursive`
2. Run tests before submitting changes: `cargo test`
3. Follow existing code style and patterns

## License

This extension uses the `lsh/tree-sitter-mojo` grammar as a submodule. See the grammar repository for its license terms.