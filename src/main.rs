use std::fs;
use tree_sitter::{Language, Parser};

// Load the `tree-sitter-rust` language from the crate.
extern "C" {
    fn tree_sitter_rust() -> Language;
}

fn main() {
    // Create a new parser instance
    let mut parser = Parser::new();

    // Set the parser to use the Rust language grammar
    let language = unsafe { tree_sitter_rust() };
    parser
        .set_language(language)
        .expect("Error loading Rust grammar");

    // Read a Rust source file (or crate's file)
    let source_code =
        fs::read_to_string("src/main.rs").expect("Error reading the Rust source file");

    // Parse the source code into a syntax tree
    let tree = parser
        .parse(&source_code, None)
        .expect("Error parsing code");

    // Print the root node of the parsed syntax tree
    println!("{:?}", tree.root_node().to_sexp());
}
