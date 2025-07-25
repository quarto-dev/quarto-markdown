# Quarto Markdown Syntax

## Goals

We aim to be largely compatible with Pandoc's `markdown` and `Commonmark` formats.

## Syntax extensions

Syntax extensions are handled by [desugaring](https://cs.brown.edu/courses/cs173/2012/book/Desugaring_as_a_Language_Feature.html).

### Shortcodes

We have "native" shortcode support in the "Pandoc" AST in pandoc.rs, and
we desugar them to Pandoc spans in a Rust filter.

### Notes

We parse footnotes differently from Pandoc.
We use NoteReference (Inline) and NoteDefinition (block) nodes.
These are desugared into spans and divs in a Rust filter.


## Pandoc syntax quirks

### Cites

### Superscript

Superscript in `-f markdown` behaves sort of magically, and I think it involves backtracking. Consider:

```
$ echo 'a^a*a^a^a*a^a' | pandoc -t native
[ Para
    [ Str "a"
    , Superscript
        [ Str "a"
        , Emph [ Str "a" , Superscript [ Str "a" ] , Str "a" ]
        , Str "a"
        ]
    , Str "a"
    ]
]
```

How does it know to match the carets in the away it does? `-f commonmark+superscript` doesn't support this:

```
$ echo 'a^a*a^a^a*a^a' | pandoc -t native -f commonmark+superscript
[ Para
    [ Str "a"
    , Superscript [ Str "a*a" ]
    , Str "a"
    , Superscript [ Str "a*a" ]
    , Str "a"
    ]
]
```

This inconsistency gives me moral space for our parser to be inconsistent here as well.