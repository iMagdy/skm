use std::fmt::Display;
use std::time::Duration;

use console::{measure_text_width, style};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

const CHECK: &str = "✓";
const CROSS: &str = "✗";
const WARN: &str = "!";
const INFO: &str = "•";
const ARROW: &str = "→";

pub fn install_progress(mp: &MultiProgress, name: &str) -> ProgressBar {
    let pb = mp.add(ProgressBar::new(100));
    pb.set_style(progress_style());
    pb.set_prefix(format!(
        "{} {}",
        style(ARROW).cyan(),
        style("install").bold()
    ));
    pb.set_message(format!("Preparing {}", skill_name(name)));
    pb.enable_steady_tick(Duration::from_millis(90));
    pb
}

pub fn upgrade_progress(mp: &MultiProgress, name: &str) -> ProgressBar {
    let pb = mp.add(ProgressBar::new(100));
    pb.set_style(progress_style());
    pb.set_prefix(format!(
        "{} {}",
        style(ARROW).cyan(),
        style("upgrade").bold()
    ));
    pb.set_message(format!("Preparing {}", skill_name(name)));
    pb.enable_steady_tick(Duration::from_millis(90));
    pb
}

pub fn finish_success(pb: &ProgressBar, message: impl Display) {
    pb.set_position(100);
    pb.finish_with_message(success_text(message));
}

pub fn finish_error(pb: &ProgressBar, message: impl Display) {
    pb.finish_with_message(error_text(message));
}

pub fn success(message: impl Display) {
    println!("{}", success_text(message));
}

pub fn warning(message: impl Display) {
    eprintln!("{}", warning_text(message));
}

pub fn error(message: impl Display) {
    eprintln!("{}", error_text(message));
}

pub fn info(message: impl Display) {
    println!("{} {}", style(INFO).cyan(), message);
}

pub fn success_text(message: impl Display) -> String {
    format!("{} {}", style(CHECK).green().bold(), message)
}

pub fn warning_text(message: impl Display) -> String {
    format!("{} {}", style(WARN).yellow().bold(), message)
}

pub fn error_text(message: impl Display) -> String {
    format!("{} {}", style(CROSS).red().bold(), message)
}

pub fn skill_name(name: &str) -> String {
    style(name).cyan().bold().to_string()
}

pub fn label(name: &str) -> String {
    style(name).dim().bold().to_string()
}

pub fn table_header(label: &str) -> String {
    style(label).cyan().bold().to_string()
}

pub fn padded(value: impl Display, plain_text: &str, width: usize) -> String {
    let value = value.to_string();
    let padding = width.saturating_sub(measure_text_width(plain_text));
    format!("{}{}", value, " ".repeat(padding))
}

pub fn status_label(status: &str) -> String {
    match status {
        "installed" => format!("{} {}", style(CHECK).green().bold(), style(status).green()),
        "missing" => format!("{} {}", style(CROSS).red().bold(), style(status).red()),
        "not locked" | "not installed" => {
            format!("{} {}", style(WARN).yellow().bold(), style(status).yellow())
        }
        "orphaned" => format!(
            "{} {}",
            style(INFO).magenta().bold(),
            style(status).magenta()
        ),
        other => other.to_string(),
    }
}

fn progress_style() -> ProgressStyle {
    ProgressStyle::with_template("{spinner:.cyan} {prefix} [{wide_bar:.cyan/blue}] {pos:>3}% {msg}")
        .unwrap()
        .progress_chars("=> ")
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_labels_keep_status_text() {
        assert!(status_label("installed").contains("installed"));
        assert!(status_label("missing").contains("missing"));
        assert!(status_label("not locked").contains("not locked"));
        assert!(status_label("orphaned").contains("orphaned"));
        assert_eq!(status_label("custom"), "custom");
    }

    #[test]
    fn test_message_helpers_keep_plain_text() {
        assert!(success_text("Installed docs").contains("Installed docs"));
        assert!(warning_text("Skipped docs").contains("Skipped docs"));
        assert!(error_text("Failed docs").contains("Failed docs"));
    }

    #[test]
    fn test_padded_uses_plain_text_width() {
        assert_eq!(padded("docs", "docs", 8), "docs    ");
        assert_eq!(padded("long-value", "long-value", 4), "long-value");
    }
}
