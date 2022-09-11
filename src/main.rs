use std::{fs::{self, File}, io::BufReader, path::PathBuf};

use tree_sitter::{Language, Parser, Query, QueryCapture, QueryCursor, Node};
mod utils;
use utils::GrammarJSON;

use once_cell::sync::Lazy;

static GRAMMAR: Lazy<GrammarJSON> = Lazy::new(||get_grammar());


extern "C" {
    fn tree_sitter_c() -> Language;
}
extern "C" {
    fn tree_sitter_rust() -> Language;
}

trait Sexp2 {}

impl<'tree> Sexp2 for Node<'tree> {
    fn to_sexp2(&self) -> String {
        let c_string = unsafe { ffi::ts_node_string(self.0) };
        let result = unsafe { CStr::from_ptr(c_string) }
            .to_str()
            .unwrap()
            .to_string();
        unsafe { (FREE_FN)(c_string as *mut c_void) };
        result
    }
}

fn main() {
    let mut parser = Parser::new();
    // Store some source code in an array of lines.
    let source = fs::read_to_string("./src/main.rs").expect("Could not find file");
    // let source = "pub fn foo() {1}pub fn bar() {2}";
    let lines = source.as_bytes();
    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();

    // Parse the source code using a custom callback. The callback is called
    // with both a byte offset and a row/column offset.
    let tree = parser.parse(lines.as_ref(), None).unwrap();

    // (source_file (function_item (visibility_modifier) name: (identifier) parameters: (parameters) body: (block (integer_literal))))

    println!("{:?}", tree.root_node().to_sexp());
    let print_node =|node: Node| {
        // println!("{}: {}", node.kind(), &source[node.start_byte()..node.end_byte()]);
    };
    
    let mut cursor = tree.walk();
    loop {

        // Keep travelling down the tree as far as we can
        if cursor.goto_first_child() {
            continue;
        }

        let node = cursor.node();

        // If we can't travel any further down, try going to next sibling and repeating
        if cursor.goto_next_sibling() {
            // If we succeed in going to the previous nodes sibling,
            // we won't be encountering that node again, so we'll call if postorder
            
            print_node(node);
            continue;
        }

        // Otherwise, we must travel back up; we'll loop until we reach the root or can
        // go to the next sibling of a node again.
        loop {
            // Since we're retracing back up the tree, this is the last time we'll encounter
            // this node, so we'll call if postorder
            
                print_node(node);
            if !cursor.goto_parent() {
                // We have arrived back at the root, so we are done.
                return;
            }

            let node = cursor.node();

            if cursor.goto_next_sibling() {
                // If we succeed in going to the previous node's sibling,
                // we will go back to travelling down that sibling's tree, and we also
                // won't be encountering the previous node again, so we'll call if postorder
                
                print_node(node);
                break;
            }
        }
    }
    let query = Query::new(language, &"(
  (function_item (visibility_modifier) (identifier) @blo)
  (#eq? @blo foo)
)").unwrap();
    // let mut cursor = QueryCursor::new();
    // let results = cursor
    //     .matches(&query, tree.root_node(), lines)
    //     .map(|m| {
    //         println!("{:?}", m.captures[0].node);
    //         (m.pattern_index, m.captures.to_owned())
    //     })
    //     .collect::<Vec<(usize, Vec<QueryCapture>)>>();

    // print!("{:?}", gen(tree.root_node()));
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
