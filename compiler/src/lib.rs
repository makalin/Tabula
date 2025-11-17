pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod typechecker;
pub mod wasm;

use anyhow::Result;
use std::path::Path;

pub struct Compiler {
    pub lexer: lexer::Lexer,
    pub parser: parser::Parser,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            lexer: lexer::Lexer::new(),
            parser: parser::Parser::new(),
        }
    }

    pub fn compile(
        &self,
        input: &Path,
        output: Option<&Path>,
        target: &str,
    ) -> Result<()> {
        let source = std::fs::read_to_string(input)?;
        let tokens = self.lexer.tokenize(&source)?;
        let ast = self.parser.parse(tokens)?;

        match target {
            "native" => {
                let output_path = output
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| input.with_extension(""));
                codegen::Codegen::new().generate_native(&ast, &output_path)?;
            }
            "wasm" => {
                let output_path = output
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| input.with_extension("wasm"));
                wasm::WasmGenerator::new().generate(&ast, &output_path)?;
            }
            _ => anyhow::bail!("Unknown target: {}", target),
        }

        Ok(())
    }

    pub fn format(&self, input: &Path) -> Result<String> {
        let source = std::fs::read_to_string(input)?;
        let tokens = self.lexer.tokenize(&source)?;
        let ast = self.parser.parse(tokens)?;
        Ok(ast.format())
    }

    pub fn run(&self, input: &Path) -> Result<()> {
        let source = std::fs::read_to_string(input)?;
        let tokens = self.lexer.tokenize(&source)?;
        let ast = self.parser.parse(tokens)?;
        codegen::Interpreter::new().interpret(&ast)?;
        Ok(())
    }
}

