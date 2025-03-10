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

## TODO

- attribute handling in ATX headers

- equation handling

  - should it have attributes? Where?

    - Currently, we ask users to put the attribute after DisplayMath, like this:

      ```
      $$
      e = mc^2
      $$ {#eq-special-relativity}
      ```

      That syntax is inconsistent with other blocks. It could instead be

      ```
      $$ {#eq-special-relativity}
      e = mc^2
      $$
      ```

      The principle here would be something like "blocks get attributes after
      the opening bracket, and inlines get attributes after the closing bracket".

    - the real problem with equations is that users want to number individual
      equations inside a eqnarray* or something like that, and we have no mechanism to
      do it.

        - in addition, if we _do_ add support for in-block equation ids, we should consider that the output
          will not only need to exist for LaTeX, but will need to exist for html and typst as well.


## notes

- I really think Pandoc would benefit from the following rawblock extensions, and I 
  wonder if we should try to adopt this into Quarto somehow:

  - `{<format}` parses input from the rawblock as if it were Pandoc's `format`

    - This would be, then, how code opts into Quarto's html-table-is-ast feature.

  - 
    ```
    ::: {>format}
    :::
    ```

    process this output as if it were markdown, but only produce output in `format`.

    In Quarto, this is "just" syntax for

    ```
    ::: {.content-visible when-format="format"}
    :::
    ```
