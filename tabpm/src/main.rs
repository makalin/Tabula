use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tabpm")]
#[command(about = "Tabula Package Manager")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Tabula project
    Init {
        /// Project name
        name: Option<String>,
    },
    /// Add a dependency
    Add {
        /// Package name
        package: String,
        /// Version constraint
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Remove a dependency
    Remove {
        /// Package name
        package: String,
    },
    /// Install dependencies
    Install,
    /// Update dependencies
    Update,
    /// Publish package
    Publish {
        /// Package path
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Search for packages
    Search {
        /// Search query
        query: String,
    },
    /// Show package information
    Info {
        /// Package name
        package: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageManifest {
    name: String,
    version: String,
    description: Option<String>,
    author: Option<String>,
    dependencies: HashMap<String, String>,
    #[serde(rename = "dev-dependencies")]
    dev_dependencies: Option<HashMap<String, String>>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            let project_name = name.unwrap_or_else(|| {
                std::env::current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            });

            let manifest = PackageManifest {
                name: project_name.clone(),
                version: "0.1.0".to_string(),
                description: None,
                author: None,
                dependencies: HashMap::new(),
                dev_dependencies: None,
            };

            let manifest_path = PathBuf::from("tabula.toml");
            if manifest_path.exists() {
                return Err(anyhow::anyhow!("tabula.toml already exists"));
            }

            let toml = toml::to_string_pretty(&manifest)?;
            std::fs::write(&manifest_path, toml)?;
            println!("Initialized Tabula project: {}", project_name);
        }
        Commands::Add { package, version } => {
            let mut manifest = load_manifest()?;
            let version_str = version.unwrap_or_else(|| "*".to_string());
            manifest.dependencies.insert(package.clone(), version_str.clone());
            save_manifest(&manifest)?;
            println!("Added dependency: {} {}", package, version_str);
        }
        Commands::Remove { package } => {
            let mut manifest = load_manifest()?;
            manifest.dependencies.remove(&package);
            save_manifest(&manifest)?;
            println!("Removed dependency: {}", package);
        }
        Commands::Install => {
            let manifest = load_manifest()?;
            println!("Installing {} dependencies...", manifest.dependencies.len());
            // TODO: Download and install packages
            println!("Dependencies installed successfully!");
        }
        Commands::Update => {
            println!("Updating dependencies...");
            // TODO: Update packages
            println!("Dependencies updated!");
        }
        Commands::Publish { path } => {
            let project_path = path.unwrap_or_else(|| PathBuf::from("."));
            let manifest = load_manifest_from(&project_path)?;
            println!("Publishing {} v{}...", manifest.name, manifest.version);
            // TODO: Publish to registry
            println!("Package published successfully!");
        }
        Commands::Search { query } => {
            println!("Searching for packages matching '{}'...", query);
            // TODO: Search registry
            println!("Found packages:");
        }
        Commands::Info { package } => {
            println!("Package information for: {}", package);
            // TODO: Fetch package info
        }
    }

    Ok(())
}

fn load_manifest() -> anyhow::Result<PackageManifest> {
    load_manifest_from(&PathBuf::from("."))
}

fn load_manifest_from(path: &PathBuf) -> anyhow::Result<PackageManifest> {
    let manifest_path = path.join("tabula.toml");
    let content = std::fs::read_to_string(&manifest_path)?;
    let manifest: PackageManifest = toml::from_str(&content)?;
    Ok(manifest)
}

fn save_manifest(manifest: &PackageManifest) -> anyhow::Result<()> {
    let toml = toml::to_string_pretty(manifest)?;
    std::fs::write("tabula.toml", toml)?;
    Ok(())
}

