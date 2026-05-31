mod cli;
mod discovery;
mod error;
mod git;
mod lockfile;
mod manifest;
mod skill;
mod ui;

use clap::{CommandFactory, Parser, Subcommand};

const HELP_FOOTER: &str = concat!(
    "License: ",
    env!("CARGO_PKG_LICENSE"),
    "\nRepository: ",
    env!("CARGO_PKG_REPOSITORY")
);

const INIT_AFTER_HELP: &str = "\
Details:
  Creates an empty manifest with skills and exports objects. If skills.json
  already exists, Ktesio leaves it untouched.

Example:
  kt init .";

const INSTALL_AFTER_HELP: &str = "\
Details:
  With no argument, installs every skill declared in skills.json. With
  name:repo, adds one skill after the repo is fetched and copied successfully.
  With a bare repo URL or local path, reads exports from that repo and lets you
  choose which skills to install.

Examples:
  kt install
  kt install docs:https://github.com/example/agent-docs.git
  kt install --all https://github.com/example/agent-docs.git";

const UPGRADE_AFTER_HELP: &str = "\
Details:
  Fetches latest upstream commits, checks out each skill's default branch, and
  updates skills.lock. Per-skill failures are reported after the rest run.

Example:
  kt upgrade";

const EXPORT_AFTER_HELP: &str = "\
Details:
  Preserves existing exports, imports entries from skills.lock, and adds
  untracked directories under .agents/skills using their local paths. Use
  export add to expose a local file or directory from this repo.

Example:
  kt export
  kt export add docs skills/docs";

const LIST_AFTER_HELP: &str = "\
Details:
  Shows each known skill with repo, commit, and status. Statuses include
  installed, missing, not locked, and orphaned.

Example:
  kt list";

const SHOW_AFTER_HELP: &str = "\
Details:
  Shows the repo URL, locked commit, local installation path, and current status
  for one skill.

Example:
  kt show docs";

const DOCTOR_AFTER_HELP: &str = "\
Details:
  Validates skills.json, skills.lock, installed skill directories, local exports,
  orphaned lock entries, and git availability.

Example:
  kt doctor";

const UNINSTALL_AFTER_HELP: &str = "\
Details:
  Removes one skill from skills.json, skills.lock, and .agents/skills. The
  remove subcommand is an alias for uninstall.

Examples:
  kt uninstall docs
  kt remove docs";

#[derive(Parser)]
#[command(
    name = "kt",
    version,
    about = "Agentic skills package manager",
    after_help = HELP_FOOTER
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new skills.json manifest
    #[command(
        about = "Initialize a new skills.json manifest",
        after_help = INIT_AFTER_HELP
    )]
    Init {
        /// Directory path where skills.json will be created
        path: String,
    },
    /// Install skills from skills.json
    #[command(
        about = "Install skills from skills.json",
        after_help = INSTALL_AFTER_HELP
    )]
    Install {
        /// Install every discovered export from a repo target
        #[arg(long)]
        all: bool,
        /// Accept safe defaults for prompts
        #[arg(long)]
        yes: bool,
        /// Fail instead of prompting for interactive choices
        #[arg(long = "no-input")]
        no_input: bool,
        /// Optional: skill name and repo URL (format: name:url)
        target: Option<String>,
    },
    /// Upgrade all installed skills to latest versions
    #[command(
        about = "Upgrade all installed skills to latest versions",
        after_help = UPGRADE_AFTER_HELP
    )]
    Upgrade,
    /// Export installed skills back into skills.json
    #[command(
        about = "Export installed skills back into skills.json",
        after_help = EXPORT_AFTER_HELP
    )]
    Export {
        #[command(subcommand)]
        command: Option<ExportCommands>,
    },
    /// List installed skills
    #[command(
        about = "List installed skills",
        after_help = LIST_AFTER_HELP
    )]
    List {
        /// Emit machine-readable JSON
        #[arg(long)]
        json: bool,
    },
    /// Show details for a specific skill
    #[command(
        about = "Show details for a specific skill",
        after_help = SHOW_AFTER_HELP
    )]
    Show {
        /// Emit machine-readable JSON
        #[arg(long)]
        json: bool,
        /// Name of the skill to inspect
        package_name: String,
    },
    /// Validate project skill state
    #[command(
        about = "Validate project skill state",
        after_help = DOCTOR_AFTER_HELP
    )]
    Doctor,
    /// Remove a skill from the project
    #[command(
        alias = "remove",
        about = "Remove a skill from the project",
        after_help = UNINSTALL_AFTER_HELP
    )]
    Uninstall {
        /// Name of the skill to remove
        package_name: String,
    },
}

#[derive(Subcommand)]
enum ExportCommands {
    /// Add or update a local export in skills.json
    Add {
        /// Export name
        name: String,
        /// Local file or directory path to export
        path: String,
    },
}

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { path }) => cli::init::run(&path),
        Some(Commands::Install {
            all,
            yes,
            no_input,
            target,
        }) => cli::install::run_with_options(
            target.as_deref(),
            cli::install::InstallOptions { all, yes, no_input },
        ),
        Some(Commands::Upgrade) => cli::upgrade::run(),
        Some(Commands::Export { command }) => match command {
            Some(ExportCommands::Add { name, path }) => cli::export::run_add(&name, &path),
            None => cli::export::run(),
        },
        Some(Commands::List { json }) => cli::list::run_with_options(json),
        Some(Commands::Show { json, package_name }) => {
            cli::show::run_with_options(&package_name, json)
        }
        Some(Commands::Doctor) => cli::doctor::run(),
        Some(Commands::Uninstall { package_name }) => cli::uninstall::run(&package_name),
        None => {
            Cli::command().print_help()?;
            println!();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_struct_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_cli_subcommands_exist() {
        let cmd = Cli::command();
        assert!(cmd.find_subcommand("init").is_some());
        assert!(cmd.find_subcommand("install").is_some());
        assert!(cmd.find_subcommand("upgrade").is_some());
        assert!(cmd.find_subcommand("export").is_some());
        assert!(cmd.find_subcommand("list").is_some());
        assert!(cmd.find_subcommand("show").is_some());
        assert!(cmd.find_subcommand("doctor").is_some());
        assert!(cmd.find_subcommand("uninstall").is_some());
        assert!(cmd.find_subcommand("remove").is_some());
    }

    #[test]
    fn test_cli_help_includes_license_and_repository() {
        let help = Cli::command().render_help().to_string();
        assert!(help.contains("License: Apache-2.0"));
        assert!(help.contains("Repository: https://github.com/iMagdy/ktesio"));
    }

    #[test]
    fn test_cli_without_subcommand_is_allowed_for_help_display() {
        let cli = Cli::try_parse_from(["kt"]).expect("bare kt should parse");
        assert!(cli.command.is_none());
    }

    #[test]
    fn test_subcommand_help_includes_details_and_examples() {
        for (command, detail) in [
            ("init", "Creates an empty manifest"),
            ("install", "installs every skill"),
            ("upgrade", "Fetches latest upstream commits"),
            ("export", "Preserves existing exports"),
            ("list", "Shows each known skill"),
            ("show", "Shows the repo URL"),
            ("doctor", "Validates skills.json"),
            ("uninstall", "Removes one skill"),
        ] {
            let mut cmd = Cli::command();
            let help = cmd
                .find_subcommand_mut(command)
                .expect("subcommand should exist")
                .render_help()
                .to_string();
            assert!(help.contains(detail), "{} help missing detail", command);
            assert!(help.contains("Example"), "{} help missing example", command);
        }
    }
}
