# Mojo Zed Extension

This is a Zed extension for the Mojo programming language.

## Current Status

The extension is in the early stages of development. So far, it provides:

*   **Syntax Highlighting**: Based on the `lsh/tree-sitter-mojo` grammar.
*   **File Association**: Associates `.mojo` and `.🔥` files with the Mojo language.

## Build Instructions

The extension can be built by running the following command:

```bash
cargo build --target wasm32-wasip1
```

## Next Steps

The next step is to add support for the Mojo Language Server to provide features like auto-complete and go-to-definition.
