## Superscript

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