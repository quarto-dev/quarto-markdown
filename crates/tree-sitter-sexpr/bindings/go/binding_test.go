package tree_sitter_sexpr_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_sexpr "github.com/tree-sitter/tree-sitter-sexpr/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_sexpr.Language())
	if language == nil {
		t.Errorf("Error loading Sexpr grammar")
	}
}
