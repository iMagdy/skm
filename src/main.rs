mod cli;
mod discovery;
mod error;
mod git;
mod lockfile;
mod manifest;
mod skill;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "skm", version, about = "Agentic skills package manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new skills.json manifest
    Init {
        /// Directory path where skills.json will be created
        path: String,
    },
    /// Install skills from skills.json
    Install {
        /// Optional: skill name and repo URL (format: name:url)
        target: Option<String>,
    },
    /// Upgrade all installed skills to latest versions
    Upgrade,
    /// List installed skills
    List,
    /// Show details for a specific skill
    Show {
        /// Name of the skill to inspect
        package_name: String,
    },
    /// Remove a skill from the project
    Uninstall {
        /// Name of the skill to remove
        package_name: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => cli::init::run(&path),
        Commands::Install { target } => cli::install::run(target.as_deref()),
        Commands::Upgrade => cli::upgrade::run(),
        Commands::List => cli::list::run(),
        Commands::Show { package_name } => cli::show::run(&package_name),
        Commands::Uninstall { package_name } => cli::uninstall::run(&package_name),
    }
}
