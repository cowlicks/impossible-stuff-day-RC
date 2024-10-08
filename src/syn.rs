use std::fs;
use syn::{File, Item, ItemFn};

fn main() {
    // Load the Rust source file (you can change the path)
    let file_content = fs::read_to_string("src/main.rs").expect("Failed to read Rust source file");

    // Parse the file into an AST
    let syntax_tree: File = syn::parse_file(&file_content).expect("Failed to parse file");

    // Traverse the AST and query it
    visit_items(&syntax_tree);
}

fn visit_items(file: &File) {
    for item in &file.items {
        match item {
            // Look for function definitions
            Item::Fn(func) => handle_function(func),
            // Look for other items (structs, enums, etc.)
            _ => (),
        }
    }
}

fn handle_function(func: &ItemFn) {
    // Print the function name
    println!("Function: {}", func.sig.ident);

    // Optional: Traverse the body of the function to find specific constructs like function calls
    for stmt in &func.block.stmts {
        if let syn::Stmt::Expr(expr, _) = stmt {
            handle_expr(expr);
        }
    }
}

fn handle_expr(expr: &syn::Expr) {
    // Look for function calls inside the function body
    if let syn::Expr::Call(call) = expr {
        dbg!(&call);
        if let syn::Expr::Path(path) = &*call.func {
            //println!("Function call: {:?}", path);
            dbg!();
        }
    }
}
