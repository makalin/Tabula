use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tabula_compiler::Compiler;

#[derive(Parser)]
#[command(name = "tabula")]
#[command(about = "Tabula compiler - A whitespace-structured programming language")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Tabula source file
    Build {
        /// Input source file
        #[arg(short, long)]
        input: PathBuf,
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Target: native or wasm
        #[arg(short, long, default_value = "native")]
        target: String,
    },
    /// Format Tabula source code
    Fmt {
        /// Input source file
        #[arg(short, long)]
        input: PathBuf,
        /// Write formatted output to file
        #[arg(short, long)]
        write: bool,
    },
    /// Run Tabula program (interpreted)
    Run {
        /// Input source file
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { input, output, target } => {
            let compiler = Compiler::new();
            compiler.compile(&input, output.as_ref(), &target)?;
            println!("Compilation successful!");
        }
        Commands::Fmt { input, write } => {
            let compiler = Compiler::new();
            let formatted = compiler.format(&input)?;
            if write {
                std::fs::write(&input, formatted)?;
                println!("Formatted: {}", input.display());
            } else {
                print!("{}", formatted);
            }
        }
        Commands::Run { input } => {
            let compiler = Compiler::new();
            compiler.run(&input)?;
        }
    }

    Ok(())
}

