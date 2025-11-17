use clap::Parser;
use regex::Regex;
use std::path::PathBuf;
use tabula_compiler::Compiler;

#[derive(Parser)]
#[command(name = "tabula-lint")]
#[command(about = "Tabula Linter")]
#[command(version)]
struct Cli {
    /// Files or directories to lint
    files: Vec<PathBuf>,
    /// Fix issues automatically
    #[arg(short, long)]
    fix: bool,
    /// Show all warnings
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone)]
struct LintIssue {
    file: PathBuf,
    line: usize,
    column: usize,
    severity: Severity,
    message: String,
    rule: String,
}

#[derive(Debug, Clone)]
enum Severity {
    Error,
    Warning,
    Info,
}

struct Linter {
    compiler: Compiler,
    issues: Vec<LintIssue>,
}

impl Linter {
    fn new() -> Self {
        Self {
            compiler: Compiler::new(),
            issues: Vec::new(),
        }
    }

    fn lint_file(&mut self, path: &PathBuf) -> anyhow::Result<()> {
        let source = std::fs::read_to_string(path)?;
        let lines: Vec<&str> = source.lines().collect();

        // Check for mixed tabs and spaces
        self.check_indentation(path, &lines);
        
        // Check for trailing whitespace
        self.check_trailing_whitespace(path, &lines);
        
        // Check for long lines
        self.check_line_length(path, &lines);
        
        // Check naming conventions
        self.check_naming(path, &source);
        
        // Try to parse and check for syntax issues
        if let Err(e) = self.compiler.lexer.tokenize(&source) {
            self.issues.push(LintIssue {
                file: path.clone(),
                line: 1,
                column: 1,
                severity: Severity::Error,
                message: format!("Parse error: {}", e),
                rule: "syntax-error".to_string(),
            });
        }

        Ok(())
    }

    fn check_indentation(&mut self, path: &PathBuf, lines: &[&str]) {
        let mut has_tabs = false;
        let mut has_spaces = false;

        for (i, line) in lines.iter().enumerate() {
            if line.starts_with('\t') {
                has_tabs = true;
            }
            if line.starts_with(' ') {
                has_spaces = true;
            }
        }

        if has_tabs && has_spaces {
            self.issues.push(LintIssue {
                file: path.clone(),
                line: 1,
                column: 1,
                severity: Severity::Warning,
                message: "Mixed tabs and spaces for indentation".to_string(),
                rule: "mixed-indentation".to_string(),
            });
        }
    }

    fn check_trailing_whitespace(&mut self, path: &PathBuf, lines: &[&str]) {
        for (i, line) in lines.iter().enumerate() {
            if line.ends_with(' ') || line.ends_with('\t') {
                self.issues.push(LintIssue {
                    file: path.clone(),
                    line: i + 1,
                    column: line.len(),
                    severity: Severity::Warning,
                    message: "Trailing whitespace".to_string(),
                    rule: "trailing-whitespace".to_string(),
                });
            }
        }
    }

    fn check_line_length(&mut self, path: &PathBuf, lines: &[&str]) {
        for (i, line) in lines.iter().enumerate() {
            if line.len() > 100 {
                self.issues.push(LintIssue {
                    file: path.clone(),
                    line: i + 1,
                    column: 100,
                    severity: Severity::Info,
                    message: format!("Line too long ({} characters)", line.len()),
                    rule: "line-length".to_string(),
                });
            }
        }
    }

    fn check_naming(&mut self, path: &PathBuf, source: &str) {
        let func_re = Regex::new(r"func\s+([a-z_][a-z0-9_]*)").unwrap();
        let var_re = Regex::new(r"let\s+([a-z_][a-z0-9_]*)").unwrap();

        for cap in func_re.captures_iter(source) {
            let name = &cap[1];
            if name.contains("__") {
                self.issues.push(LintIssue {
                    file: path.clone(),
                    line: 1,
                    column: 1,
                    severity: Severity::Warning,
                    message: format!("Function name '{}' contains double underscores", name),
                    rule: "naming-convention".to_string(),
                });
            }
        }

        for cap in var_re.captures_iter(source) {
            let name = &cap[1];
            if name == "i" || name == "j" || name == "k" {
                // Allow single letter in loops
                continue;
            }
            if name.len() == 1 {
                self.issues.push(LintIssue {
                    file: path.clone(),
                    line: 1,
                    column: 1,
                    severity: Severity::Info,
                    message: format!("Variable '{}' is too short", name),
                    rule: "naming-convention".to_string(),
                });
            }
        }
    }

    fn print_issues(&self) {
        let mut errors = 0;
        let mut warnings = 0;
        let mut infos = 0;

        for issue in &self.issues {
            let symbol = match issue.severity {
                Severity::Error => {
                    errors += 1;
                    "✗"
                }
                Severity::Warning => {
                    warnings += 1;
                    "⚠"
                }
                Severity::Info => {
                    infos += 1;
                    "ℹ"
                }
            };

            println!(
                "{} {}:{}:{} [{}] {}",
                symbol,
                issue.file.display(),
                issue.line,
                issue.column,
                issue.rule,
                issue.message
            );
        }

        println!("\nSummary: {} errors, {} warnings, {} infos", errors, warnings, infos);
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut linter = Linter::new();

    let files: Vec<PathBuf> = if cli.files.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        cli.files
    };

    for file in files {
        if file.is_dir() {
            for entry in std::fs::read_dir(&file)? {
                let path = entry?.path();
                if path.extension().map(|e| e == "tab").unwrap_or(false) {
                    linter.lint_file(&path)?;
                }
            }
        } else {
            linter.lint_file(&file)?;
        }
    }

    linter.print_issues();

    if linter.issues.iter().any(|i| matches!(i.severity, Severity::Error)) {
        std::process::exit(1);
    }

    Ok(())
}

