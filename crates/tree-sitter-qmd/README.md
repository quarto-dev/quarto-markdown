# tree-sitter-qmd

`tree-sitter-qmd` is a fork of [`tree-sitter-markdown`](https://github.com/tree-sitter-grammars/tree-sitter-markdown). 
It has a number of non-standard syntax that are used by Quarto in .qmd files specifically.

The original `tree-sitter-markdown` grammar was written by [Matthias Deiml](https://github.com/MDeiml).

At a high level, tree-sitter-qmd should be used like tree-sitter-md, by using the results of a block parse
to trigger the inline grammar parsing.

For the original tree-sitter-md readme, see [README.tree-sitter-md.md].

## QMD changes/additions

- A fixed set of extensions

- shortcode syntax

- attribute syntax

  - including Quarto's `{lang}` syntax that isn't commonmark

  - including raw block/inline `{=format}` syntax

## QMD _REMOVALS_

- Simplified link syntax. the only link syntax supported are _inline links_: `[text](destination title)`

  - no wikilink support

  - no shortcut reference link support, we use that syntax for spans instead

- Similarly, the only image syntax supported is the one corresponding to inline links: `![text](image-name title)`

- no HTML support: QMD is meant to translate into more than one syntax. We use rawblock instead.
