use lsp_server::{Connection, Message};
use lsp_types::{
    request::GotoDefinition, GotoDefinitionParams, InitializeParams, Position,
    TextDocumentIdentifier, Url,
};
use std::path::PathBuf;

fn main() {
    // Initialize the LSP connection to Rust Analyzer
    let (connection, io_threads) = Connection::stdio();

    // Initialize params for the LSP (to initialize rust-analyzer)
    let initialize_params = InitializeParams {
        process_id: None,
        root_uri: Some(
            Url::from_file_path(PathBuf::from(".")) // Point to the crate root
                .expect("Invalid path"),
        ),
        ..Default::default()
    };

    // Send the initialize request
    let initialize_request = connection.initialize(initialize_params).unwrap();
    if let Message::Response(response) = initialize_request {
        println!("Received initialize response: {:?}", response);
    }

    // Now you can interact with rust-analyzer via LSP methods, for example:
    let text_document = TextDocumentIdentifier {
        uri: Url::from_file_path(file!()).unwrap(),
    };

    let params = GotoDefinitionParams {
        text_document_position_params: lsp_types::TextDocumentPositionParams {
            text_document,
            position: Position::new(3, 5), // Line and column of the function you're analyzing
        },
        work_done_progress_params: Default::default(),
    };

    let request = connection
        .send_request::<GotoDefinition>(params)
        .expect("Failed to send definition request");

    println!("Sent definition request");

    // Listen for responses from rust-analyzer
    for msg in connection.receiver.iter() {
        println!("Received message: {:?}", msg);
    }

    // Wait for IO threads to finish
    io_threads.join().unwrap();
}
