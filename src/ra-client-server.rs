use lsp_server::{Connection, Message, Request, RequestId};
use lsp_types::{
    request::GotoDefinition, GotoDefinitionParams, InitializeParams, Position,
    TextDocumentIdentifier, Url,
};
use std::path::PathBuf;

fn main() {
    // Initialize the LSP connection to Rust Analyzer
    let (connection, io_threads) = Connection::stdio();

    // Initialize params for the LSP (to initialize rust-analyzer)
    let initialize_params = serde_json::to_value(InitializeParams {
        process_id: None,
        root_uri: Some(
            Url::from_file_path(PathBuf::from("/home/blake/git/impossible")) // Point to the crate root
                .expect("Invalid path"),
        ),
        ..Default::default()
    })
    .unwrap();
    dbg!();

    // Send the initialize request
    let initialize_response = connection.initialize(initialize_params).unwrap();
    println!("Received initialize response: {:?}", initialize_response);
    dbg!();

    // Now you can interact with rust-analyzer via LSP methods, for example:
    let text_document = TextDocumentIdentifier {
        uri: Url::from_file_path("src/ra-client-server.rs").unwrap(),
    };

    dbg!();
    let params = GotoDefinitionParams {
        text_document_position_params: lsp_types::TextDocumentPositionParams {
            text_document,
            position: Position::new(3, 5), // Line and column of the function you're analyzing
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    };

    let request = connection
        .sender
        .try_send(Message::Request(Request {
            id: RequestId::from("foo".to_string()),
            method: "method_string".to_string(),
            params: serde_json::to_value(params).unwrap(),
        }))
        .expect("Failed to send definition request");

    dbg!();
    println!("Sent definition request");

    // Listen for responses from rust-analyzer
    for msg in connection.receiver.iter() {
        println!("Received message: {:?}", msg);
    }
    dbg!();

    // Wait for IO threads to finish
    io_threads.join().unwrap();
}
