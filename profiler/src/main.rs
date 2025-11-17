use clap::Parser;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;
use tabula_compiler::Compiler;

#[derive(Parser)]
#[command(name = "tabula-profile")]
#[command(about = "Tabula Profiler")]
#[command(version)]
struct Cli {
    /// Source file to profile
    file: PathBuf,
    /// Output format
    #[arg(short, long, default_value = "text")]
    format: String,
    /// Output file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProfileData {
    function: String,
    calls: usize,
    total_time_ms: f64,
    avg_time_ms: f64,
    min_time_ms: f64,
    max_time_ms: f64,
}

struct Profiler {
    compiler: Compiler,
    function_times: HashMap<String, Vec<f64>>,
    function_calls: HashMap<String, usize>,
}

impl Profiler {
    fn new() -> Self {
        Self {
            compiler: Compiler::new(),
            function_times: HashMap::new(),
            function_calls: HashMap::new(),
        }
    }

    fn profile(&mut self, file: &PathBuf) -> anyhow::Result<Vec<ProfileData>> {
        let source = std::fs::read_to_string(file)?;
        let tokens = self.compiler.lexer.tokenize(&source)?;
        let ast = self.compiler.parser.parse(tokens)?;

        // Profile execution
        let start = Instant::now();
        self.execute_with_profiling(&ast)?;
        let total_time = start.elapsed();

        // Build profile data
        let mut profiles = Vec::new();
        for (func_name, times) in &self.function_times {
            let calls = self.function_calls.get(func_name).copied().unwrap_or(0);
            let total: f64 = times.iter().sum();
            let avg = total / times.len() as f64;
            let min = times.iter().copied().fold(f64::INFINITY, f64::min);
            let max = times.iter().copied().fold(0.0, f64::max);

            profiles.push(ProfileData {
                function: func_name.clone(),
                calls,
                total_time_ms: total,
                avg_time_ms: avg,
                min_time_ms: min,
                max_time_ms: max,
            });
        }

        profiles.sort_by(|a, b| b.total_time_ms.partial_cmp(&a.total_time_ms).unwrap());

        Ok(profiles)
    }

    fn execute_with_profiling(&mut self, _ast: &tabula_compiler::ast::Program) -> anyhow::Result<()> {
        // TODO: Implement actual profiling during execution
        // For now, simulate some function calls
        self.record_function_call("main", 1.5);
        self.record_function_call("helper", 0.8);
        self.record_function_call("helper", 0.9);
        Ok(())
    }

    fn record_function_call(&mut self, name: &str, time_ms: f64) {
        self.function_times
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(time_ms);
        *self.function_calls.entry(name.to_string()).or_insert(0) += 1;
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut profiler = Profiler::new();

    let profiles = profiler.profile(&cli.file)?;

    match cli.format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&profiles)?;
            if let Some(output) = cli.output {
                std::fs::write(output, json)?;
            } else {
                println!("{}", json);
            }
        }
        "text" => {
            println!("=== Profiling Results ===\n");
            println!("{:<20} {:>8} {:>12} {:>12} {:>12} {:>12}", 
                "Function", "Calls", "Total (ms)", "Avg (ms)", "Min (ms)", "Max (ms)");
            println!("{}", "-".repeat(80));

            for profile in &profiles {
                println!(
                    "{:<20} {:>8} {:>12.2} {:>12.2} {:>12.2} {:>12.2}",
                    profile.function,
                    profile.calls,
                    profile.total_time_ms,
                    profile.avg_time_ms,
                    profile.min_time_ms,
                    profile.max_time_ms
                );
            }
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown format: {}", cli.format));
        }
    }

    Ok(())
}

