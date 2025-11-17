use crate::ast::*;
use anyhow::Result;
use std::path::Path;

pub struct WasmGenerator {
    // WASM generation state
}

impl WasmGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, program: &Program, output: &Path) -> Result<()> {
        // Generate WAT (WebAssembly Text) format
        let mut wat = String::from("(module\n");
        
        wat.push_str("  (memory 1)\n");
        wat.push_str("  (export \"memory\" (memory 0))\n");
        wat.push_str("  (func $print (param i32))\n");
        wat.push_str("  (func (export \"main\")\n");
        
        for stmt in &program.statements {
            wat.push_str(&self.generate_statement_wat(stmt, 2)?);
        }
        
        wat.push_str("  )\n");
        wat.push_str(")\n");
        
        // Convert WAT to WASM binary
        let wasm_bytes = wat::parse_str(&wat)?;
        std::fs::write(output, wasm_bytes)?;
        
        Ok(())
    }

    fn generate_statement_wat(&self, stmt: &Statement, indent: usize) -> Result<String> {
        let spaces = " ".repeat(indent);
        match stmt {
            Statement::Let { name, value } => {
                Ok(format!("{};; let {}\n", spaces, name))
            }
            Statement::Print { args } => {
                Ok(format!("{};; print\n", spaces))
            }
            _ => Ok(format!("{};; TODO: statement\n", spaces)),
        }
    }
}

