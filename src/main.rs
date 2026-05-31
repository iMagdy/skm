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
  already exists, skm leaves it untouched.

Example:
  skm init .";

const INSTALL_AFTER_HELP: &str = "\
Details:
  With no argument, installs every skill declared in skills.json. With
  name:repo, adds one skill after the repo is fetched and copied successfully.

Examples:
  skm install
  skm install docs:https://github.com/example/agent-docs.git";

const UPGRADE_AFTER_HELP: &str = "\
Details:
  Fetches latest upstream commits, checks out each skill's default branch, and
  updates skills.lock. Per-skill failures are reported after the rest run.

Example:
  skm upgrade";

const EXPORT_AFTER_HELP: &str = "\
Details:
  Preserves existing exports, imports entries from skills.lock, and adds
  untracked directories under .agents/skills using their local paths.

Example:
  skm export";

const LIST_AFTER_HELP: &str = "\
Details:
  Shows each known skill with repo, commit, and status. Statuses include
  installed, missing, not locked, and orphaned.

Example:
  skm list";

const SHOW_AFTER_HELP: &str = "\
Details:
  Shows the repo URL, locked commit, local installation path, and current status
  for one skill.

Example:
  skm show docs";

const UNINSTALL_AFTER_HELP: &str = "\
Details:
  Removes one skill from skills.json, skills.lock, and .agents/skills. The
  remove subcommand is an alias for uninstall.

Examples:
  skm uninstall docs
  skm remove docs";

#[derive(Parser)]
#[command(
    name = "skm",
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
    Export,
    /// List installed skills
    #[command(
        about = "List installed skills",
        after_help = LIST_AFTER_HELP
    )]
    List,
    /// Show details for a specific skill
    #[command(
        about = "Show details for a specific skill",
        after_help = SHOW_AFTER_HELP
    )]
    Show {
        /// Name of the skill to inspect
        package_name: String,
    },
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

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { path }) => cli::init::run(&path),
        Some(Commands::Install { target }) => cli::install::run(target.as_deref()),
        Some(Commands::Upgrade) => cli::upgrade::run(),
        Some(Commands::Export) => cli::export::run(),
        Some(Commands::List) => cli::list::run(),
        Some(Commands::Show { package_name }) => cli::show::run(&package_name),
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
        assert!(cmd.find_subcommand("uninstall").is_some());
        assert!(cmd.find_subcommand("remove").is_some());
    }

    #[test]
    fn test_cli_help_includes_license_and_repository() {
        let help = Cli::command().render_help().to_string();
        assert!(help.contains("License: Apache-2.0"));
        assert!(help.contains("Repository: https://github.com/iMagdy/skm"));
    }

    #[test]
    fn test_cli_without_subcommand_is_allowed_for_help_display() {
        let cli = Cli::try_parse_from(["skm"]).expect("bare skm should parse");
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
