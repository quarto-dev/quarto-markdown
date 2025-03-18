/**
 * @file sexpr parser
 * @author CES <ces@no.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: 'sexpr',

  rules: {
    // The program is the top-level rule that matches the entire source file
    program: $ => $._expression,

    // An expression is either a symbol or a list
    _expression: $ => choice(
      $.symbol,
      $.list
    ),

    // A symbol is a sequence of one or more alphanumeric characters
    symbol: $ => /[a-zA-Z0-9]+/,

    // A list is a sequence of expressions enclosed in parentheses
    list: $ => seq(
      '(',
      $.list_item,
      ')'
    ),

    list_item: $ => repeat($._expression),
  }
});