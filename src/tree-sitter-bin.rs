//! Parse source using tree-sitter
//! cargo run src/main.rs
use std::fs;
use tree_sitter::Parser;

fn did_stuff() {
    println!("yolo");
}
// pass in path to source
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path_to_source = args
        .get(1)
        .expect("Must provide path to sourc as second argument");
    // Create a new parser instance
    let mut parser = Parser::new();

    // Set the parser to use the Rust language grammar
    let language = tree_sitter_rust::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Rust grammar");

    // Read a Rust source file (or crate's file)
    let source_code =
        fs::read_to_string(path_to_source).expect("Error reading the Rust source file");

    // Parse the source code into a syntax tree
    let tree = parser
        .parse(&source_code, None)
        .expect("Error parsing code");

    // Print the root node of the parsed syntax tree
    println!("{:?}", tree.root_node().to_sexp());
    did_stuff();
}
