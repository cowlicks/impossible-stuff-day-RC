//! Parse source using tree-sitter
//! cargo run src/main.rs
use std::fs;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor, Tree};

fn foo() {}
// pass in path to source
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path_to_source = args.get(1).map(String::from).unwrap_or(file!().to_string());
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
    //println!("{:?}", tree.root_node().to_sexp());
    //walk_tree(tree);
    query(tree, source_code);
    foo()
}

fn walk_tree(tree: Tree) {
    let root_node = tree.root_node();
    let mut cursor = root_node.walk();

    dbg!(&tree);
    println!("Root Node: {:?}", root_node.kind());

    // Iterate through all child nodes
    loop {
        // Check if the current node is a function definition or call
        if dbg!(cursor.node().kind()) == "function_item" {
            println!(
                "Found a function definition at: {:?}",
                cursor.node().range()
            );
        }

        let _x = dbg!(cursor.goto_next_sibling());
        // Go to the next node
        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

fn query(tree: Tree, source: String) {
    let query_source = r#"
    (function_item name: (identifier) @func_name)
"#;
    let query_source = r#"
(call_expression
  function: (field_identifier) @method.name
  arguments: (arguments))
"#;
    let query_source = r#"
(call_expression
  function: _ @the-function
  arguments: _)
"#;
    let language = tree_sitter_rust::LANGUAGE;
    let query = Query::new(&language.into(), query_source).expect("Error creating query");
    let mut query_cursor = QueryCursor::new();
    let root_node = tree.root_node();

    let mut matches = query_cursor.matches(&query, root_node, source.as_bytes());
    while let Some(m) = matches.next() {
        //for m in query_cursor.matches(&query, root_node, source.as_bytes()) {
        for capture in m.captures {
            dbg!(capture);
            let func_name = capture.node.utf8_text(source.as_bytes()).unwrap();
            println!("Found function: {}", func_name);
        }
    }
}
