This repository contains a Rust library and binary crate that converts Markdown text to
Pandoc's AST representation using a custom tree-sitter grammar for Markdown.

This tree-sitter setup is somewhat unique because Markdown requires a two-step process:
one tree-sitter grammar to establish the block structure, and another tree-sitter grammar
to parse the inline structure within each block.

As a result, in this repository all traversals of the tree-sitter data structure
need to be done with the traversal helpers in traversals.rs.

## Best practices in this repo

- If you want to create a test file, do so in the `tests/` directory.
