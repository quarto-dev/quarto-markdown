// This grammar only concerns the inline structure according to the CommonMark Spec
// (https://spec.commonmark.org/0.30/#inlines)
// For more information see README.md

/// <reference types="tree-sitter-cli/dsl" />

const common = require('../common/common');

// key_value defs are in common.js

// Levels used for dynmic precedence. Ideally
// n * PRECEDENCE_LEVEL_EMPHASIS > PRECEDENCE_LEVEL_LINK for any n, so maybe the
// maginuted of these values should be increased in the future
const PRECEDENCE_LEVEL_EMPHASIS = 1;
const PRECEDENCE_LEVEL_LINK = 10;
const PRECEDENCE_LEVEL_HTML = 100;

// Punctuation characters as specified in
// https://github.github.com/gfm/#ascii-punctuation-character
const PUNCTUATION_CHARACTERS_REGEX = '!-/:-@\\[-`\\{-~\'';


// !!!
// Notice the call to `add_inline_rules` which generates some additional rules related to parsing
// inline contents in different contexts.
// !!!
module.exports = grammar(add_inline_rules({
    name: 'markdown_inline',

    externals: $ => [
        // An `$._error` token is never valid  and gets emmited to kill invalid parse branches. Concretely
        // this is used to decide wether a newline closes a paragraph and together and it gets emitted
        // when trying to parse the `$._trigger_error` token in `$.link_title`.
        $._error,
        $._trigger_error,

        // Opening and closing delimiters for code spans. These are sequences of one or more backticks.
        // An opening token does not mean the text after has to be a code span if there is no closing token
        $._code_span_start,
        $._code_span_close,

        // Opening and closing delimiters for emphasis.
        $._emphasis_open_star,
        $._emphasis_open_underscore,
        $._emphasis_close_star,
        $._emphasis_close_underscore,

        // For emphasis we need to tell the parser if the last character was a whitespace (or the
        // beginning of a line) or a punctuation. These tokens never actually get emitted.
        $._last_token_whitespace,
        $._last_token_punctuation,

        $._strikethrough_open,
        $._strikethrough_close,

        // Opening and closing delimiters for latex. These are sequences of one or more dollar signs.
        // An opening token does not mean the text after has to be latex if there is no closing token
        $._latex_span_start,
        $._latex_span_close,

        $._single_quote_open,
        $._single_quote_close,
        $._double_quote_open,
        $._double_quote_close,

        // Token emmited when encountering opening delimiters for a leaf span
        // e.g. a code span, that does not have a matching closing span
        $._unclosed_span
    ],
    precedences: $ => [
        // [$._strong_emphasis_star, $._inline_element_no_star],
        [$._strong_emphasis_star_no_link, $._inline_element_no_star_no_link],
        // [$._strong_emphasis_underscore, $._inline_element_no_underscore],
        [$._strong_emphasis_underscore_no_link, $._inline_element_no_underscore_no_link],
        [$.hard_line_break, $._whitespace],
        [$.hard_line_break, $._text_base],
    ],
    // More conflicts are defined in `add_inline_rules`
    conflicts: $ => [

        [$._html_comment, $._text_base],

        [$._link_text_non_empty, $._inline_element],
        [$._link_text_non_empty, $._inline_element_no_star],
        [$._link_text_non_empty, $._inline_element_no_underscore],
        [$._link_text_non_empty, $._inline_element_no_tilde],
        [$._link_text, $._inline_element],
        [$._link_text, $._inline_element_no_star],
        [$._link_text, $._inline_element_no_underscore],
        [$._link_text, $._inline_element_no_tilde],

        [$._image_description, $._image_description_non_empty, $._text_base],
        // [$._image_description, $._image_description_non_empty, $._text_inline],
        // [$._image_description, $._image_description_non_empty, $._text_inline_no_star],
        // [$._image_description, $._image_description_non_empty, $._text_inline_no_underscore],

        // [$._image_shortcut_link, $._image_description],
        // [$.shortcut_link, $._link_text],
        [$.link_destination, $.link_title],
        [$._link_destination_parenthesis, $.link_title],

        [$.commonmark_attribute, $.language_attribute],
    ],
    extras: $ => [],

    rules: {
        inline: $ => seq(optional($._last_token_whitespace), $._inline),

        ...common.rules,

        // A lot of inlines are defined in `add_inline_rules`, including:
        //
        // * collections of inlines
        // * emphasis
        // * textual content
        //
        // This is done to reduce code duplication, as some inlines need to be parsed differently
        // depending on the context. For example inlines in ATX headings may not contain newlines.

        code_span: $ => prec.right(seq(
            alias($._code_span_start, $.code_span_delimiter),
            alias(repeat(choice($._text_base, '[', ']', $._soft_line_break)), $.code_content),
            alias($._code_span_close, $.code_span_delimiter),
            optional($._qmd_attribute)
        )),

        // QMD CHANGE: we call this a latex_span because we need a display math mode
        // in the block grammar and that will be called latex_block
        latex_span: $ => seq(
            alias($._latex_span_start, $.latex_span_delimiter),
            alias(repeat(choice($._text_base, '[', ']', $._soft_line_break, $.backslash_escape)), $.latex_content),
            alias($._latex_span_close, $.latex_span_delimiter),
        ),

        quoted_span: $ => choice(
            prec.right(seq(
                alias($._single_quote_open, $.single_quoted_span_delimiter),
                repeat($._inline_element),
                alias($._single_quote_close, $.single_quoted_span_delimiter),
            )),
            prec.right(seq(
                alias($._double_quote_open, $.double_quoted_span_delimiter),
                repeat($._inline_element),
                alias($._double_quote_close, $.double_quoted_span_delimiter),
            )),
        ),

        // Different kinds of links:
        // * inline links (https://github.github.com/gfm/#inline-link)
        // * full reference links (https://github.github.com/gfm/#full-reference-link)
        // * collapsed reference links (https://github.github.com/gfm/#collapsed-reference-link)
        // * shortcut links (https://github.github.com/gfm/#shortcut-reference-link)
        //
        // Dynamic precedence is distributed as granular as possible to help the parser decide
        // while parsing which branch is the most important.
        //
        // https://github.github.com/gfm/#links
        _link_text: $ => prec.dynamic(PRECEDENCE_LEVEL_LINK, prec.right(choice(
            seq($._link_text_non_empty, optional($._qmd_attribute)),
            seq('[', ']', optional($._qmd_attribute)),
        ))),
        _link_text_non_empty: $ => prec(2, seq('[', alias($._inline_no_link, $.link_text), ']')),
        // shortcut_link: $ => prec.dynamic(PRECEDENCE_LEVEL_LINK, $._link_text_non_empty),
        inline_link: $ => prec.dynamic(PRECEDENCE_LEVEL_LINK, prec.right(seq(
            $._link_text,
            optional(seq(
              '(',
              repeat(choice($._whitespace, $._soft_line_break)),
              optional(seq(
                  choice(
                      seq(
                          $.link_destination,
                          optional(seq(
                              repeat1(choice($._whitespace, $._soft_line_break)),
                              $.link_title
                          ))
                      ),
                      $.link_title,
                  ),
                  repeat(choice($._whitespace, $._soft_line_break)),
              )),
              ')',
            )),
            optional($._qmd_attribute)
        ))),

        // _span_text_non_empty:         $ => prec(1,seq(     '[', alias($._inline, $.span_text), prec(1, ']'))),
        // _span_text: $ => prec.dynamic(5 * PRECEDENCE_LEVEL_LINK, choice(
        //     seq($._span_text_non_empty, $._qmd_attribute),
        //     seq('[', ']', $._qmd_attribute),
        // )),
        // fenced_span: $ => prec.dynamic(5 * PRECEDENCE_LEVEL_LINK, prec.right(seq(
        //     $._span_text,
        //     optional($._qmd_attribute)
        // ))),

        // Images work exactly like links with a '!' added in front.
        //
        // https://github.github.com/gfm/#images
        image: $ => choice(
            $._image_inline_link,
        ),
        _image_inline_link: $ => prec.dynamic(3 * PRECEDENCE_LEVEL_LINK, prec.right(seq(
            $._image_description,
            '(',
            repeat(choice($._whitespace, $._soft_line_break)),
            optional(seq(
                choice(
                    seq(
                        $.link_destination,
                        optional(seq(
                            repeat1(choice($._whitespace, $._soft_line_break)),
                            $.link_title
                        ))
                    ),
                    $.link_title,
                ),
                repeat(choice($._whitespace, $._soft_line_break)),
            )),
            ')',
            optional($._qmd_attribute)
        ))),
        _image_description: $ => prec.dynamic(3 * PRECEDENCE_LEVEL_LINK, choice($._image_description_non_empty, seq('!', '[', prec(1, ']')))),
        _image_description_non_empty: $ => seq('!', '[', alias($._inline, $.image_description), prec(1, ']')),

        // Autolinks. Uri autolinks actually accept protocolls of arbitrary length which does not
        // align with the spec. This is because the binary for the grammar gets to large if done
        // otherwise as tree-sitters code generation is not very concise for this type of regex.
        //
        // Email autolinks do not match every valid email (emails normally should not be parsed
        // using regexes), but this is how they are defined in the spec.
        //
        // https://github.github.com/gfm/#autolinks
        uri_autolink: $ => /<[a-zA-Z][a-zA-Z0-9+\.\-][a-zA-Z0-9+\.\-]*:[^ \t\r\n<>]*>/,
        email_autolink: $ =>
            /<[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*>/,

        // Raw html. As with html blocks we do not emit additional information as this is best done
        // by a proper html tree-sitter grammar.
        //
        // https://github.github.com/gfm/#raw-html
        _html_comment: $ => prec.dynamic(PRECEDENCE_LEVEL_HTML, seq(
            '<!--',
            optional(seq(
                choice(
                    $._word,
                    $._whitespace,
                    $._soft_line_break,
                    common.punctuation_without($, ['-', '>']),
                    seq(
                        '-',
                        common.punctuation_without($, ['>']),
                    )
                ),
                repeat(prec.right(choice(
                    $._word,
                    $._whitespace,
                    $._soft_line_break,
                    common.punctuation_without($, ['-']),
                    seq(
                        '-',
                        choice(
                            $._word,
                            $._whitespace,
                            $._soft_line_break,
                            common.punctuation_without($, ['-']),
                        )
                    )
                ))),
            )),
            '-->'
        )),

        // A hard line break.
        //
        // https://github.github.com/gfm/#hard-line-breaks
        hard_line_break: $ => seq(choice('\\', $._whitespace_ge_2), $._soft_line_break),
        _text: $ => choice($._word, common.punctuation_without($, []), $._whitespace),

        // Whitespace is divided into single whitespaces and multiple whitespaces as wee need this
        // information for hard line breaks.
        _whitespace_ge_2: $ => /\t| [ \t]+/,
        _whitespace: $ => seq(choice($._whitespace_ge_2, / /), optional($._last_token_whitespace)),

        // Other than whitespace we tokenize into strings of digits, punctuation characters
        // (handled by `common.punctuation_without`) and strings of any other characters. This way the
        // lexer does not have to many different states, which makes it a lot easier to make
        // conflicts work.
        _word: $ => choice($._word_no_digit, $._digits),
        _word_no_digit: $ => new RegExp('[^' + PUNCTUATION_CHARACTERS_REGEX + ' \\t\\n\\r0-9]+(_+[^' + PUNCTUATION_CHARACTERS_REGEX + ' \\t\\n\\r0-9]+)*'),
        _digits: $ => /[0-9][0-9_]*/,
        _soft_line_break: $ => seq($._newline_token, optional($._last_token_whitespace)),

        _inline_base: $ => prec.right(repeat1(choice(
            $.image,
            alias($._soft_line_break, $.soft_line_break),
            $.backslash_escape,
            $.hard_line_break,
            $.uri_autolink,
            $.email_autolink,
            $.entity_reference,
            $.numeric_character_reference,
            $.latex_span,
            $.code_span,
            $.quoted_span,

            // QMD CHANGE: WE DO NOT ALLOW HTML TAGS OUTSIDE OF RAW HTML INLINES AND BLOCKS            
            // alias($._html_tag, $.html_tag),
            alias($._text_base, $.text_base),
            common.EXTENSION_TAGS ? $.tag : choice(),
            $._unclosed_span,
        ))),
        _text_base: $ => choice(
            $._word,
            common.punctuation_without($, ['[', '{', '}', ']', "'"]),
            $._whitespace,
        ),

        ...(common.EXTENSION_TAGS ? {
            tag: $ => /#[0-9]*[a-zA-Z_\-\/][a-zA-Z_\-\/0-9]*/,
        } : {}),

    },
}));

// This function adds some extra inline rules. This is done to reduce code duplication, as some
// rules may not contain newlines, characters like '*' and '_', ... depending on the context.
//
// This is by far the most ugly part of this code and should be cleaned up.
function add_inline_rules(grammar) {
    let conflicts = [];
    for (let link of [true, false]) {
        let suffix_link = link ? "" : "_no_link";
        for (let delimiter of [false, "star", "underscore", "tilde"]) {
            let suffix_delimiter = delimiter ? "_no_" + delimiter : "";
            let suffix = suffix_delimiter + suffix_link;
            grammar.rules["_inline_element" + suffix] = $ => {
                let elements = [
                    $._inline_base,
                    alias($['_emphasis_star' + suffix_link], $.emphasis),
                    alias($['_strong_emphasis_star' + suffix_link], $.strong_emphasis),
                    alias($['_emphasis_underscore' + suffix_link], $.emphasis),
                    alias($['_strong_emphasis_underscore' + suffix_link], $.strong_emphasis),
                ];
                if (common.EXTENSION_STRIKETHROUGH) {
                    elements.push(alias($['_strikethrough' + suffix_link], $.strikethrough));
                }
                if (delimiter !== "star") {
                    elements.push($._emphasis_open_star);
                }
                if (delimiter !== "underscore") {
                    elements.push($._emphasis_open_underscore);
                }
                if (delimiter !== "tilde") {
                    elements.push($._strikethrough_open);
                }
                if (link) {
                    elements = elements.concat([
                        $.inline_link,
                        seq(choice('[', ']'), optional($._last_token_punctuation)),
                    ]);
                }
                return choice(...elements);
            };
            // if (suffix === "") {
            //   grammar.rules["_inline"] = $ => prec.left(1, seq(
            //     repeat1($._inline_element), 
            //     optional(seq($._whitespace, $._qmd_attribute))));
            // } else {
            //   grammar.rules["_inline" + suffix] = $ => repeat1($["_inline_element" + suffix]);
            // }
            grammar.rules["_inline" + suffix] = $ => repeat1($["_inline_element" + suffix]);
            if (delimiter !== "star") {
                conflicts.push(['_emphasis_star' + suffix_link, '_inline_element' + suffix_delimiter + suffix_link]);
                conflicts.push(['_emphasis_star' + suffix_link, '_strong_emphasis_star' + suffix_link, '_inline_element' + suffix_delimiter + suffix_link]);
            }
            if (delimiter == 'star' || delimiter == 'underscore') {
                conflicts.push(['_strong_emphasis_' + delimiter + suffix_link, '_inline_element_no_' + delimiter]);
            }
            if (delimiter !== "underscore") {
                conflicts.push(['_emphasis_underscore' + suffix_link, '_inline_element' + suffix_delimiter + suffix_link]);
                conflicts.push(['_emphasis_underscore' + suffix_link, '_strong_emphasis_underscore' + suffix_link, '_inline_element' + suffix_delimiter + suffix_link]);
            }
            if (delimiter !== "tilde") {
                conflicts.push(['_strikethrough' + suffix_link, '_inline_element' + suffix_delimiter + suffix_link]);
            }
        }

        if (common.EXTENSION_STRIKETHROUGH) {
            grammar.rules['_strikethrough' + suffix_link] = $ => prec.dynamic(PRECEDENCE_LEVEL_EMPHASIS, seq(alias($._strikethrough_open, $.emphasis_delimiter), optional($._last_token_punctuation), $['_inline' + '_no_tilde' + suffix_link], alias($._strikethrough_close, $.emphasis_delimiter)));
        }
        grammar.rules['_emphasis_star' + suffix_link] = $ => prec.dynamic(PRECEDENCE_LEVEL_EMPHASIS, seq(alias($._emphasis_open_star, $.emphasis_delimiter), optional($._last_token_punctuation), $['_inline' + '_no_star' + suffix_link], alias($._emphasis_close_star, $.emphasis_delimiter)));
        grammar.rules['_strong_emphasis_star' + suffix_link] = $ => prec.dynamic(2 * PRECEDENCE_LEVEL_EMPHASIS, seq(alias($._emphasis_open_star, $.emphasis_delimiter), $['_emphasis_star' + suffix_link], alias($._emphasis_close_star, $.emphasis_delimiter)));
        grammar.rules['_emphasis_underscore' + suffix_link] = $ => prec.dynamic(PRECEDENCE_LEVEL_EMPHASIS, seq(alias($._emphasis_open_underscore, $.emphasis_delimiter), optional($._last_token_punctuation), $['_inline' + '_no_underscore' + suffix_link], alias($._emphasis_close_underscore, $.emphasis_delimiter)));
        grammar.rules['_strong_emphasis_underscore' + suffix_link] = $ => prec.dynamic(2 * PRECEDENCE_LEVEL_EMPHASIS, seq(alias($._emphasis_open_underscore, $.emphasis_delimiter), $['_emphasis_underscore' + suffix_link], alias($._emphasis_close_underscore, $.emphasis_delimiter)));
    }

    let old = grammar.conflicts
    grammar.conflicts = $ => {
        let cs = old($);
        for (let conflict of conflicts) {
            let c = [];
            for (let rule of conflict) {
                c.push($[rule]);
            }
            cs.push(c);
        }
        return cs;
    }

    return grammar;
}
