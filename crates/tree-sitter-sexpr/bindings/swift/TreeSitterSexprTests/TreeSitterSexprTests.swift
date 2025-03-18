import XCTest
import SwiftTreeSitter
import TreeSitterSexpr

final class TreeSitterSexprTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_sexpr())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Sexpr grammar")
    }
}
