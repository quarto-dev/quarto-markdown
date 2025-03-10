(fenced_code_block
  (info_string
    (language) @injection.language)
  (code_fence_content) @injection.content)

(fenced_code_block
  (language_attribute
    (language) @injection.language)
  (code_fence_content) @injection.content)

(document . (section . (thematic_break) (_) @injection.content (thematic_break)) (#set! injection.language "yaml"))

((minus_metadata) @injection.content (#set! injection.language "yaml"))

((plus_metadata) @injection.content (#set! injection.language "toml"))

((inline) @injection.content (#set! injection.language "markdown_inline"))
