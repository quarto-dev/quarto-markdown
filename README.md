# quarto-markdown

> [!WARNING]
> This code isn't ready for public consumption.

This repository hosts a standalone parser for "Quarto Markdown", the Markdown dialect recognized by [Quarto](https://quarto.org).

## Features

- standalone Rust binary
- built from a pair of tree-sitter grammars forked from the [tree-sitter grammar repo](https://github.com/tree-sitter-grammars/tree-sitter-markdown/)
- recognizes syntax errors
- supports Quarto-specific syntax constructs:
  - code cell blocks with `{language}` syntax
  - shortcodes
- emits parse tree in Pandoc's `json` and `native` formats

## Background

In a number of ways, Quarto Markdown _isn't_ Markdown. Most importantly, Quarto Markdown gives syntax errors in malformed documents.
Modern markdown dialects include a number of syntax constructs that can be the source of mistakes.
Standards such as Commonmark dictate that [no documents ever contain mistakes](https://spec.commonmark.org/0.31.2/#preliminaries):

> Any sequence of characters is a valid CommonMark document.

This isn't a tenable situation in large documents.

## TODOs

- actually good error messages
- source tracking
- WASM distribution
- solve glaring performance issues
