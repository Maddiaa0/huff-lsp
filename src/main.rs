use dashmap::DashMap;
use huff_language_server::completion::{completion, HuffCompletionItem};

use huff_language_server::parser::{self, parse};
use huff_utils::prelude::CompilerError;
use ropey::Rope;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::CodeLens;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

// Huff lang items
use huff_utils::ast::Contract;

#[derive(Debug)]
struct Backend {
    /// The lsp client
    client: Client,

    /// A mapping of tracked sources to compiled contracts - TODO: replace with dahs map?
    ast_map: DashMap<String, Contract>,

    /// A mapping of file name to the text content - using ropey to efficiently parse large files
    document_map: DashMap<String, Rope>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                // hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                }),
                // code_lens_provider: Some(CodeLensOptions {
                // resolve_provider: Some(false),
                // }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            language_id: params.text_document.language_id,
            version: params.text_document.version,
            text: params.text_document.text,
        })
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;

        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            // TODO: make this constant
            language_id: "huff".to_string(),
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
        })
        .await
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        self.client
            .log_message(MessageType::INFO, "Triggering completion")
            .await;

        let completions = || -> Option<Vec<CompletionItem>> {
            // Get the auto complete file location
            let rope = self.document_map.get(&uri.to_string())?;
            let ast = self.ast_map.get(&uri.to_string())?;
            let char = rope.try_line_to_char(position.line as usize).ok()?;
            let offset = char + position.character as usize;
            let completions = completion(&ast, offset);

            let mut ret = Vec::with_capacity(completions.len());
            for (_, item) in completions {
                match item {
                    HuffCompletionItem::Macro(name, args) => ret.push(CompletionItem {
                        label: name.clone(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        // TODO: documentation: (),
                        insert_text: Some(format!(
                            "{}({})",
                            name,
                            args.iter()
                                .enumerate()
                                // TODO: printing the wrong display value lmao
                                .map(|(index, item)| { format!("${{{}:{:#?}}}", index + 1, item) })
                                .collect::<Vec<_>>()
                                .join(",")
                        )),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        ..Default::default()
                    }),
                    HuffCompletionItem::Opcode(opcode) => ret.push(CompletionItem {
                        label: opcode.clone(),
                        kind: Some(CompletionItemKind::VALUE),
                        insert_text: Some(opcode.clone()),
                        // TODO: show evm codes detail here :)
                        ..Default::default()
                    }),
                    HuffCompletionItem::BuiltinFunction(func_type) => ret.push(CompletionItem {
                        label: func_type.clone(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        insert_text: Some(format!("{}()", func_type.clone())),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        ..Default::default()
                    }),
                }
            }

            Some(ret)
        }();

        self.client
            .log_message(MessageType::INFO, format!("{completions:?}"))
            .await;

        Ok(completions.map(CompletionResponse::Array))
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;
    }
    async fn code_lens(&self, _params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        // TODO: implement getting the code lenses here the same way we did with
        // the vscode extension.
        // For this we will need to receive the files

        // Get the document,
        // Get an ast from the document, Getting all of the file locations from them

        self.client
            .log_message(MessageType::ERROR, "code_lens_request")
            .await;

        let range = Range {
            start: Position {
                line: 5,
                character: 5,
            },
            end: Position {
                line: 5,
                character: 7,
            },
        };
        let command = None;
        let data = None;

        let return_lens = CodeLens {
            range,
            command,
            data,
        };
        let lenses = vec![return_lens];
        Ok(Some(lenses))
    }

    async fn code_lens_resolve(&self, _params: CodeLens) -> Result<CodeLens> {
        self.client
            .log_message(MessageType::ERROR, "code lens resolve")
            .await;

        let range = Range {
            start: Position {
                line: 5,
                character: 5,
            },
            end: Position {
                line: 5,
                character: 7,
            },
        };
        let command = None;
        let data = None;

        let return_lens = CodeLens {
            range,
            command,
            data,
        };
        Ok(return_lens)
    }
}

impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        // Save the updated file in our source map
        let rope = ropey::Rope::from_str(&params.text);
        self.document_map
            .insert(params.uri.to_string(), rope.clone());

        // TODO implement semantic tokens
        let contract = parse(params.text, params.uri.to_string());

        match contract {
            Ok(ast) => {
                // Parsed the contract successfully
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!("Parsed {} successfully", params.uri.to_string()),
                    )
                    .await;

                self.ast_map.insert(params.uri.to_string(), ast);
            }
            Err(err) => {
                // Compiler error, create a diagnostic
                self.client
                    .log_message(MessageType::INFO, format!("{:?}", err))
                    .await;

                let diagnostic = match err {
                    CompilerError::ParserError(parser_err) => {
                        // TODO: create a default if there isnt a found span
                        let error_span = parser_err.spans.0.first().unwrap();
                        let diag = || -> Option<Diagnostic> {
                            let start_position = offset_to_position(error_span.start, &rope)?;
                            let end_position = offset_to_position(error_span.end, &rope)?;
                            let range = Range::new(start_position, end_position);
                            Some(Diagnostic::new_simple(range, format!("{error_span:#?}")))
                        }();
                        diag
                    }
                    // TODO: remove / handle Unrecognized errors better
                    _ => {
                        let diag = || -> Option<Diagnostic> {
                            let start_position = offset_to_position(0, &rope)?;
                            let end_position = offset_to_position(1, &rope)?;
                            let range = Range::new(start_position, end_position);
                            Some(Diagnostic::new_simple(range, format!("Unrecognized error")))
                        }();
                        diag
                    }
                };
                match diagnostic {
                    Some(x) => {
                        self.client
                            .publish_diagnostics(params.uri.clone(), vec![x], Some(params.version))
                            .await
                    }
                    None => {
                        self.client
                            .log_message(
                                MessageType::INFO,
                                format!(
                                    "Failed to publish diagnostics {} successfully",
                                    params.uri.to_string()
                                ),
                            )
                            .await;
                    }
                };
            }
        }
    }
}

fn offset_to_position(offset: usize, rope: &Rope) -> Option<Position> {
    let line = rope.try_char_to_line(offset).ok()?;
    let first_char = rope.try_line_to_char(line).ok()?;
    let column = offset - first_char;
    Some(Position::new(line as u32, column as u32))
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        ast_map: DashMap::new(),
        document_map: DashMap::new(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
