use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse,
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, DocumentSymbolParams,
    DocumentSymbolResponse, GotoDefinitionParams, GotoDefinitionResponse,
    Hover, HoverParams, HoverResponse, InitializeParams, InitializeResult,
    Location, Position, Range, ServerCapabilities, ServerInfo, SymbolKind,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    Url,
};
use std::collections::HashMap;
use tower_lsp::{jsonrpc::Result, lsp_types::*, LanguageServer, LspService, Server};

struct TabulaLanguageServer {
    documents: HashMap<Url, String>,
    compiler: tabula_compiler::Compiler,
}

#[tower_lsp::async_trait]
impl LanguageServer for TabulaLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "tabula-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: None,
                        will_save_wait_until: None,
                        save: None,
                    },
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), " ".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        eprintln!("Tabula LSP initialized");
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.documents
            .insert(params.text_document.uri, params.text_document.text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(text) = params.content_changes.into_iter().next() {
            self.documents.insert(params.text_document.uri, text.text);
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(text) = self.documents.get(&uri) {
            self.validate_document(&uri, text).await;
        }
    }

    async fn completion(&self, params: CompletionParams) -> jsonrpc::Result<Option<CompletionResponse>> {
        let items = vec![
            CompletionItem {
                label: "let".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Variable declaration".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "func".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Function definition".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "if".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Conditional statement".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "for".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Loop statement".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "print".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Print to console".to_string()),
                ..Default::default()
            },
        ];
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> jsonrpc::Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "Tabula language symbol".to_string(),
            )),
            range: None,
        }))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> jsonrpc::Result<Option<GotoDefinitionResponse>> {
        // TODO: Implement definition lookup
        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> jsonrpc::Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        if let Some(text) = self.documents.get(&uri) {
            if let Ok(tokens) = self.compiler.lexer.tokenize(text) {
                if let Ok(ast) = self.compiler.parser.parse(tokens) {
                    let symbols: Vec<DocumentSymbol> = ast
                        .statements
                        .iter()
                        .filter_map(|stmt| match stmt {
                            tabula_compiler::ast::Statement::Function { name, .. } => {
                                Some(DocumentSymbol {
                                    name: name.clone(),
                                    kind: SymbolKind::FUNCTION,
                                    range: Range::default(),
                                    selection_range: Range::default(),
                                    ..Default::default()
                                })
                            }
                            _ => None,
                        })
                        .collect();
                    return Ok(Some(DocumentSymbolResponse::Flat(symbols)));
                }
            }
        }
        Ok(None)
    }
}

impl TabulaLanguageServer {
    fn new() -> Self {
        Self {
            documents: HashMap::new(),
            compiler: tabula_compiler::Compiler::new(),
        }
    }

    async fn validate_document(&self, uri: &Url, text: &str) {
        // Validate and send diagnostics
        if let Err(e) = self.compiler.lexer.tokenize(text) {
            // Send diagnostic
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(TabulaLanguageServer::new());
    Server::new(stdin, stdout, socket).serve(service).await;
}

