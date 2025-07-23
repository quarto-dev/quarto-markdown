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