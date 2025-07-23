## Testing

tree-sitter-markdown-inline has a tree-sitter test suite that can be run with

```
$ tree-sitter test
```

Many tests there were inherited from the grammar we forked. Many of those fail, and some shouldn't actually pass.

In addition to a fixed test suite, we have `./scripts/shortcode_generator.py` to test the shortcode parsing subsystem specifically.
It uses random testing to generate large numbers of shortcodes, calls `tree-sitter parse` on them, and checks if the output matches expectations.

We use it to generate failing tests that are then fixed and added to the test suite (crates/tree-sitter-qmd/tree-sitter-markdown-inline/test/corpus/shortcodes.txt).
At present time, we have generated over 10k random tests without failures.

## TODO

- Inlines

  - `markdown+smart` extension

    - question: do we enable/disable this?

  - Process [.... @notes ...] into a single cite

    - this needs to happen in the inline_link pandoc.rs code

  - Process noteNum counts, or ignore it and knowingly produce output that doesn't quite match Pandoc?

  - note references [^1]

- blocks

  - note definitions

  - all tests

- Filter for note definition/note reference resolution

