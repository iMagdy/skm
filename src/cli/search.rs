use std::path::Path;

use dialoguer::Select;

use crate::error::SearchFailed;
use crate::skills_sh::{self, SkillSearchResult};
use crate::ui;

#[derive(Debug, Clone, Copy, Default)]
pub struct SearchOptions {
    pub json: bool,
    pub limit: usize,
    pub install: bool,
    pub no_input: bool,
}

#[cfg(not(tarpaulin_include))]
pub fn run_with_options(
    query: &str,
    options: SearchOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?;
    run_in_with_options(&project_root, query, options)
}

pub(crate) fn run_in_with_options(
    project_root: &Path,
    query: &str,
    options: SearchOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    if options.json && options.install {
        return Err(SearchFailed {
            message: "Cannot combine --json and --install".to_string(),
        }
        .into());
    }

    if !options.json {
        ui::info(
            "Searching skills.sh public listings. Ktesio respects rate limits and retries responsibly.",
        );
    }

    let results = skills_sh::search(query, options.limit, ui::warning)?;

    if options.json {
        println!("{}", serde_json::to_string_pretty(&results)?);
        return Ok(());
    }

    print_results(&results);

    if options.install {
        install_selected(project_root, &results, options.no_input)?;
    }

    Ok(())
}

fn print_results(results: &[SkillSearchResult]) {
    if results.is_empty() {
        ui::info("No skills found.");
        return;
    }

    println!(
        "{} {} {} {}",
        ui::padded(ui::table_header("SKILL"), "SKILL", 34),
        ui::padded(ui::table_header("SOURCE"), "SOURCE", 28),
        ui::padded(ui::table_header("INSTALLS"), "INSTALLS", 12),
        ui::table_header("INSTALL")
    );
    println!("{}", "-".repeat(110));

    for result in results {
        let install = result
            .install_target
            .as_deref()
            .map(|target| format!("kt install {target}"))
            .unwrap_or_else(|| "not installable yet".to_string());
        println!(
            "{} {} {} {}",
            ui::padded(ui::skill_name(&result.name), &result.name, 34),
            ui::padded(&result.source, &result.source, 28),
            ui::padded(result.installs, &result.installs.to_string(), 12),
            install
        );
    }
}

fn install_selected(
    project_root: &Path,
    results: &[SkillSearchResult],
    no_input: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let installable = results
        .iter()
        .filter(|result| result.installable)
        .collect::<Vec<_>>();

    if installable.is_empty() {
        return Err(SearchFailed {
            message: "No installable GitHub-backed skills were found.".to_string(),
        }
        .into());
    }

    let selected = if installable.len() == 1 {
        installable[0]
    } else if no_input {
        return Err(SearchFailed {
            message: "Multiple installable skills found; rerun without --no-input to select one."
                .to_string(),
        }
        .into());
    } else {
        let labels = installable
            .iter()
            .map(|result| {
                let target = result
                    .install_target
                    .as_deref()
                    .unwrap_or("not installable");
                format!("{} ({target})", result.name)
            })
            .collect::<Vec<_>>();
        let index = Select::new()
            .with_prompt("Select skill to install")
            .items(&labels)
            .default(0)
            .interact_opt()?
            .ok_or_else(|| SearchFailed {
                message: "Installation cancelled".to_string(),
            })?;
        installable[index]
    };

    let target = selected
        .install_target
        .as_deref()
        .ok_or_else(|| SearchFailed {
            message: "Selected skill is not installable yet".to_string(),
        })?;
    crate::cli::install::run_in_with_options(
        project_root,
        Some(target),
        crate::cli::install::InstallOptions {
            yes: true,
            no_input,
            ..Default::default()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_results_handles_empty() {
        print_results(&[]);
    }
}
