# Mojo Zed Extension

This is a Zed extension for the Mojo programming language.

## Features

*   **Syntax Highlighting**: Based on the `lsh/tree-sitter-mojo` grammar with full Mojo syntax support
*   **File Association**: Associates `.mojo` and `.🔥` files with the Mojo language
*   **Language Server Protocol (LSP)**: Auto-complete, go-to-definition, and other IDE features via `mojo-lsp-server`

## Prerequisites

To use the language server features, you need to have Mojo installed with `mojo-lsp-server` available in your PATH or in a pixi environment. Install Mojo from [https://docs.modular.com/mojo/](https://docs.modular.com/mojo/).

## Build Instructions

The extension can be built by running:

```bash
# Initialize submodules (first time only)
git submodule update --init --recursive

# Build the extension
cargo build --target wasm32-wasip1
```

## Development

The extension uses:
- Tree-sitter grammar as a git submodule at `grammars/mojo/`
- Symlinked query files from the grammar to `languages/mojo/`
- Rust WASM module for LSP server discovery

Run tests with:
```bash
cargo test
```
