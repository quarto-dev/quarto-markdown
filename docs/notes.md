## Source tracking

We really want to have relatively precise location tracking.
But Markdown, as always, makes things complicated.

Simple inline nodes are easy to handle.
But footnotes offer a good counter-example. Consider:

```markdown
Paragraph[^1].

[^1]: footnote.
```

Pandoc's `markdown` reader produces this native AST:

```
[ Para
    [ Str "Paragraph"
    , Note [ Para [ Str "footnote." ] ]
    , Str "."
    ]
]
```

The note contents are in a completely different block, and get incorporated into the actual paragraph itself.

As I think about this more carefully, I think we actually have a solid opportunity here.
I think quarto_markdown_pandoc should emit special Pandoc AST syntax that separate "footnote definitions" and "footnote references", which could then be handled correctly by Quarto's pipeline.

# TODO

- Inlines

  - `markdown+smart` extension

    - question: do we enable/disable this?

  - Process [.... @notes ...] into a single cite

    - this needs to happen in the inline_link pandoc.rs code

  - Process noteNum counts, or ignore it and knowingly produce output that doesn't quite match Pandoc?
  
    - precise matching of noteNum will be hard because that state is shared across a number of other nodes
      (though I don't understand _why_ pandoc does this.)

- blocks

  - note definitions

  - all tests

- Filter for note definition/note reference resolution

  - I actually think this should happen at quarto-cli

