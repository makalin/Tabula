use clap::Parser;
use std::path::PathBuf;
use tabula_compiler::Compiler;

#[derive(Parser)]
#[command(name = "tabula-doc")]
#[command(about = "Tabula Documentation Generator")]
#[command(version)]
struct Cli {
    /// Source files or directories
    files: Vec<PathBuf>,
    /// Output directory
    #[arg(short, long, default_value = "docs")]
    output: PathBuf,
    /// Format: html, markdown
    #[arg(short, long, default_value = "html")]
    format: String,
}

struct DocGenerator {
    compiler: Compiler,
}

impl DocGenerator {
    fn new() -> Self {
        Self {
            compiler: Compiler::new(),
        }
    }

    fn generate(&self, files: &[PathBuf], output: &PathBuf, format: &str) -> anyhow::Result<()> {
        std::fs::create_dir_all(output)?;

        let mut all_functions = Vec::new();
        let mut all_modules = Vec::new();

        for file in files {
            if file.is_dir() {
                for entry in std::fs::read_dir(file)? {
                    let path = entry?.path();
                    if path.extension().map(|e| e == "tab").unwrap_or(false) {
                        self.process_file(&path, &mut all_functions, &mut all_modules)?;
                    }
                }
            } else {
                self.process_file(file, &mut all_functions, &mut all_modules)?;
            }
        }

        match format {
            "html" => self.generate_html(&all_functions, &all_modules, output)?,
            "markdown" => self.generate_markdown(&all_functions, &all_modules, output)?,
            _ => return Err(anyhow::anyhow!("Unknown format: {}", format)),
        }

        println!("Documentation generated in {}", output.display());
        Ok(())
    }

    fn process_file(
        &self,
        file: &PathBuf,
        functions: &mut Vec<FunctionDoc>,
        modules: &mut Vec<ModuleDoc>,
    ) -> anyhow::Result<()> {
        let source = std::fs::read_to_string(file)?;
        let tokens = self.compiler.lexer.tokenize(&source)?;
        let ast = self.compiler.parser.parse(tokens)?;

        let module_name = file
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let mut module_functions = Vec::new();

        for stmt in &ast.statements {
            if let tabula_compiler::ast::Statement::Function {
                name,
                params,
                body: _,
            } = stmt
            {
                let doc = FunctionDoc {
                    name: name.clone(),
                    params: params.clone(),
                    description: self.extract_comment(&source, name),
                };
                module_functions.push(doc.clone());
                functions.push(doc);
            }
        }

        if !module_functions.is_empty() {
            modules.push(ModuleDoc {
                name: module_name,
                path: file.clone(),
                functions: module_functions,
            });
        }

        Ok(())
    }

    fn extract_comment(&self, source: &str, function_name: &str) -> Option<String> {
        // Simple comment extraction - look for comments before function
        let lines: Vec<&str> = source.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.contains(&format!("func {}", function_name)) {
                // Look backwards for comments
                if i > 0 && lines[i - 1].trim().starts_with("#") {
                    return Some(lines[i - 1].trim_start_matches('#').trim().to_string());
                }
            }
        }
        None
    }

    fn generate_html(
        &self,
        functions: &[FunctionDoc],
        modules: &[ModuleDoc],
        output: &PathBuf,
    ) -> anyhow::Result<()> {
        let mut html = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Tabula Documentation</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        h1 { color: #333; }
        h2 { color: #666; margin-top: 30px; }
        .function { background: #f5f5f5; padding: 15px; margin: 10px 0; border-radius: 5px; }
        .function-name { font-weight: bold; color: #0066cc; }
        .params { color: #666; margin-top: 5px; }
    </style>
</head>
<body>
    <h1>Tabula Documentation</h1>
"#,
        );

        for module in modules {
            html.push_str(&format!("<h2>Module: {}</h2>\n", module.name));
            for func in &module.functions {
                html.push_str("<div class=\"function\">\n");
                html.push_str(&format!(
                    "<div class=\"function-name\">func {}({})</div>\n",
                    func.name,
                    func.params.join(", ")
                ));
                if let Some(desc) = &func.description {
                    html.push_str(&format!("<p>{}</p>\n", desc));
                }
                html.push_str("</div>\n");
            }
        }

        html.push_str("</body>\n</html>");
        std::fs::write(output.join("index.html"), html)?;
        Ok(())
    }

    fn generate_markdown(
        &self,
        functions: &[FunctionDoc],
        modules: &[ModuleDoc],
        output: &PathBuf,
    ) -> anyhow::Result<()> {
        let mut md = String::from("# Tabula Documentation\n\n");

        for module in modules {
            md.push_str(&format!("## Module: {}\n\n", module.name));
            for func in &module.functions {
                md.push_str(&format!("### `func {}({})`\n\n", func.name, func.params.join(", ")));
                if let Some(desc) = &func.description {
                    md.push_str(&format!("{}\n\n", desc));
                }
            }
        }

        std::fs::write(output.join("README.md"), md)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct FunctionDoc {
    name: String,
    params: Vec<String>,
    description: Option<String>,
}

#[derive(Debug, Clone)]
struct ModuleDoc {
    name: String,
    path: PathBuf,
    functions: Vec<FunctionDoc>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let generator = DocGenerator::new();

    let files: Vec<PathBuf> = if cli.files.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        cli.files
    };

    generator.generate(&files, &cli.output, &cli.format)?;
    Ok(())
}

