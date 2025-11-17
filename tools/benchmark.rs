// Benchmarking tool for Tabula programs
use std::path::PathBuf;
use std::time::Instant;
use tabula_compiler::Compiler;

pub fn benchmark(file: PathBuf, iterations: usize) -> anyhow::Result<()> {
    let compiler = Compiler::new();
    let source = std::fs::read_to_string(&file)?;
    let tokens = compiler.lexer.tokenize(&source)?;
    let ast = compiler.parser.parse(tokens)?;

    println!("Running benchmark: {} iterations", iterations);
    
    let mut times = Vec::new();
    for i in 0..iterations {
        let start = Instant::now();
        compiler.codegen::Interpreter::new().interpret(&ast)?;
        let elapsed = start.elapsed();
        times.push(elapsed.as_millis() as f64);
        
        if (i + 1) % 10 == 0 {
            println!("Completed {} iterations", i + 1);
        }
    }

    let avg = times.iter().sum::<f64>() / times.len() as f64;
    let min = times.iter().copied().fold(f64::INFINITY, f64::min);
    let max = times.iter().copied().fold(0.0, f64::max);

    println!("\n=== Benchmark Results ===");
    println!("Average: {:.2}ms", avg);
    println!("Min: {:.2}ms", min);
    println!("Max: {:.2}ms", max);

    Ok(())
}

