use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tabula_compiler::Compiler;

#[derive(Parser)]
#[command(name = "tabula-test")]
#[command(about = "Tabula Test Framework")]
#[command(version)]
struct Cli {
    /// Test file or directory
    #[arg(default_value = "tests")]
    path: PathBuf,
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    /// Run specific test
    #[arg(short, long)]
    test: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestCase {
    name: String,
    code: String,
    expected_output: Option<String>,
    expected_error: Option<String>,
    should_fail: bool,
}

struct TestRunner {
    compiler: Compiler,
    passed: usize,
    failed: usize,
    verbose: bool,
}

impl TestRunner {
    fn new(verbose: bool) -> Self {
        Self {
            compiler: Compiler::new(),
            passed: 0,
            failed: 0,
            verbose,
        }
    }

    fn run_test(&mut self, test: &TestCase) -> bool {
        if self.verbose {
            println!("Running: {}", test.name);
        }

        match self.compiler.lexer.tokenize(&test.code) {
            Ok(tokens) => {
                match self.compiler.parser.parse(tokens) {
                    Ok(ast) => {
                        if test.should_fail {
                            if self.verbose {
                                println!("  ✗ Test should have failed but passed");
                            }
                            self.failed += 1;
                            return false;
                        }

                        // Execute test
                        match self.compiler.codegen::Interpreter::new().interpret(&ast) {
                            Ok(_) => {
                                if self.verbose {
                                    println!("  ✓ Test passed");
                                }
                                self.passed += 1;
                                true
                            }
                            Err(e) => {
                                if test.expected_error.is_some() {
                                    if self.verbose {
                                        println!("  ✓ Test passed (expected error)");
                                    }
                                    self.passed += 1;
                                    true
                                } else {
                                    if self.verbose {
                                        println!("  ✗ Test failed: {}", e);
                                    }
                                    self.failed += 1;
                                    false
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if test.should_fail || test.expected_error.is_some() {
                            if self.verbose {
                                println!("  ✓ Test passed (expected parse error)");
                            }
                            self.passed += 1;
                            true
                        } else {
                            if self.verbose {
                                println!("  ✗ Parse error: {}", e);
                            }
                            self.failed += 1;
                            false
                        }
                    }
                }
            }
            Err(e) => {
                if test.should_fail || test.expected_error.is_some() {
                    if self.verbose {
                        println!("  ✓ Test passed (expected lex error)");
                    }
                    self.passed += 1;
                    true
                } else {
                    if self.verbose {
                        println!("  ✗ Lex error: {}", e);
                    }
                    self.failed += 1;
                    false
                }
            }
        }
    }

    fn run_file(&mut self, path: &PathBuf) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(path)?;
        let tests: Vec<TestCase> = serde_json::from_str(&content)
            .or_else(|_| {
                // Try parsing as single test
                Ok(vec![TestCase {
                    name: path.file_stem().unwrap().to_string_lossy().to_string(),
                    code: content,
                    expected_output: None,
                    expected_error: None,
                    should_fail: false,
                }])
            })?;

        for test in &tests {
            if self.verbose {
                println!("\n--- {} ---", test.name);
            }
            self.run_test(test);
        }

        Ok(())
    }

    fn print_summary(&self) {
        println!("\n=== Test Summary ===");
        println!("Passed: {}", self.passed);
        println!("Failed: {}", self.failed);
        println!("Total:  {}", self.passed + self.failed);
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut runner = TestRunner::new(cli.verbose);

    if cli.path.is_file() {
        runner.run_file(&cli.path)?;
    } else if cli.path.is_dir() {
        let test_files: Vec<PathBuf> = std::fs::read_dir(&cli.path)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|e| e == "tab" || e == "json").unwrap_or(false))
            .collect();

        for file in test_files {
            if let Some(test_name) = cli.test.as_ref() {
                if !file.to_string_lossy().contains(test_name) {
                    continue;
                }
            }
            runner.run_file(&file)?;
        }
    }

    runner.print_summary();

    if runner.failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

