use std::{fs::File, path::PathBuf, io::BufReader};

use tree_sitter::{Language, Parser, Query, QueryCapture, QueryCursor, Node};
mod utils;
use utils::GrammarJSON;

use once_cell::sync::Lazy;

static GRAMMAR: Lazy<String> = Lazy::new(||get_grammar());


extern "C" {
    fn tree_sitter_c() -> Language;
}
extern "C" {
    fn tree_sitter_rust() -> Language;
}

fn main() {
    let mut parser = Parser::new();
    // Store some source code in an array of lines.
    let lines = "pub fn foo() {1}".as_bytes();
    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();

    // Parse the source code using a custom callback. The callback is called
    // with both a byte offset and a row/column offset.
    let tree = parser.parse(lines.as_ref(), None).unwrap();

    // (source_file (function_item (visibility_modifier) name: (identifier) parameters: (parameters) body: (block (integer_literal))))

    println!("{}", tree.root_node().to_sexp());

    let query = Query::new(language, &"(
  (function_item (visibility_modifier) (identifier) @blo)
  (#eq? @blo foo)
)").unwrap();
    let mut cursor = QueryCursor::new();
    let results = cursor
        .matches(&query, tree.root_node(), lines)
        .map(|m| {
            println!("{:?}", m.captures[0].node);
            (m.pattern_index, m.captures.to_owned())
        })
        .collect::<Vec<(usize, Vec<QueryCapture>)>>();

    print!("{:?}", gen(tree.root_node()));
}

fn get_grammar() -> GrammarJSON {
    let dir: PathBuf = ["parsers", "tree-sitter-rust", "src"].iter().collect();
    let file = File::open(dir.join("grammar.json")).unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

fn gen(node: Node) -> String {
    // node.children(Node::walk).map(f)
    // if node.child_count() > 0 {

    // } else {
        
    // }

    // grammar.rules[node.kind().to_string()];
    
    node.kind().to_string()

}

// https://github.com/github/semantic
// https://github.blog/2020-08-04-codegen-semantics-improved-language-support-system/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gen() {

        let mut parser = Parser::new();
        let language = unsafe { tree_sitter_rust() };
        parser.set_language(language).unwrap();

        let input = "pub fn foo() {1}";
        
        let lines = input.as_bytes();
        let s_exp = "(source_file (function_item (visibility_modifier) name: (identifier) parameters: (parameters) body: (block (integer_literal))))";
        let tree = parser.parse(lines.as_ref(), None).unwrap();

        assert_eq!(tree.root_node().to_sexp(), s_exp);
        
        // assert_eq!(gen(tree.root_node()), input);
    }
}
