//! Output formatting utilities
//!
//! カラー出力とメッセージフォーマット機能

use colored::*;

/// 出力フォーマッター
#[derive(Debug, Clone)]
pub struct OutputFormatter {
    /// カラー出力を有効化
    pub colors_enabled: bool,

    /// 詳細モード
    pub verbose: bool,
}

impl OutputFormatter {
    /// 新しいフォーマッターを作成
    pub fn new() -> Self {
        Self {
            colors_enabled: colored::control::SHOULD_COLORIZE.should_colorize(),
            verbose: false,
        }
    }

    /// カラー出力を設定
    pub fn with_colors(mut self, enabled: bool) -> Self {
        self.colors_enabled = enabled;
        if !enabled {
            colored::control::set_override(false);
        }
        self
    }

    /// 詳細モードを設定
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// 成功メッセージをフォーマット
    pub fn success(&self, message: &str) -> String {
        if self.colors_enabled {
            format!("{} {}", "✓".green().bold(), message)
        } else {
            format!("✓ {}", message)
        }
    }

    /// エラーメッセージをフォーマット
    pub fn error(&self, message: &str) -> String {
        if self.colors_enabled {
            format!("{} {}", "✗".red().bold(), message.red())
        } else {
            format!("✗ {}", message)
        }
    }

    /// 警告メッセージをフォーマット
    pub fn warning(&self, message: &str) -> String {
        if self.colors_enabled {
            format!("{} {}", "⚠".yellow().bold(), message.yellow())
        } else {
            format!("⚠ {}", message)
        }
    }

    /// 情報メッセージをフォーマット
    pub fn info(&self, message: &str) -> String {
        if self.colors_enabled {
            format!("{} {}", "ℹ".blue().bold(), message)
        } else {
            format!("ℹ {}", message)
        }
    }

    /// コマンド実行開始メッセージ
    pub fn command_start(&self, command_name: &str) -> String {
        if self.colors_enabled {
            format!(
                "{} Running command: {}",
                "▶".cyan().bold(),
                command_name.cyan().bold()
            )
        } else {
            format!("▶ Running command: {}", command_name)
        }
    }

    /// コマンド実行完了メッセージ
    pub fn command_complete(&self, command_name: &str, duration_ms: u64) -> String {
        if self.colors_enabled {
            format!(
                "{} Completed: {} ({}ms)",
                "✓".green().bold(),
                command_name.green(),
                duration_ms.to_string().dimmed()
            )
        } else {
            format!("✓ Completed: {} ({}ms)", command_name, duration_ms)
        }
    }

    /// コマンド実行失敗メッセージ
    pub fn command_failed(&self, command_name: &str, exit_code: i32) -> String {
        if self.colors_enabled {
            format!(
                "{} Failed: {} (exit code: {})",
                "✗".red().bold(),
                command_name.red(),
                exit_code.to_string().red().bold()
            )
        } else {
            format!("✗ Failed: {} (exit code: {})", command_name, exit_code)
        }
    }

    /// セクションヘッダー
    pub fn section_header(&self, title: &str) -> String {
        if self.colors_enabled {
            format!(
                "\n{}\n{}",
                title.bold().underline(),
                "─".repeat(title.len()).dimmed()
            )
        } else {
            format!("\n{}\n{}", title, "─".repeat(title.len()))
        }
    }

    /// リストアイテム
    pub fn list_item(&self, item: &str) -> String {
        if self.colors_enabled {
            format!("  {} {}", "•".cyan(), item)
        } else {
            format!("  • {}", item)
        }
    }

    /// コマンド出力（stdout）
    pub fn command_output(&self, output: &str) -> String {
        if output.is_empty() {
            return String::new();
        }

        if self.verbose {
            if self.colors_enabled {
                format!("{}\n{}", "Output:".dimmed(), output.dimmed())
            } else {
                format!("Output:\n{}", output)
            }
        } else {
            output.to_string()
        }
    }

    /// コマンドエラー出力（stderr）
    pub fn command_error_output(&self, output: &str) -> String {
        if output.is_empty() {
            return String::new();
        }

        if self.colors_enabled {
            format!("{}\n{}", "Error:".red().bold(), output.red())
        } else {
            format!("Error:\n{}", output)
        }
    }

    /// 進捗バー風の表示
    pub fn progress(&self, current: usize, total: usize, item: &str) -> String {
        if self.colors_enabled {
            format!(
                "{} [{}/{}] {}",
                "⏳".cyan(),
                current.to_string().cyan().bold(),
                total.to_string().dimmed(),
                item
            )
        } else {
            format!("⏳ [{}/{}] {}", current, total, item)
        }
    }
}

impl Default for OutputFormatter {
    fn default() -> Self {
        Self::new()
    }
}

/// 簡易フォーマット関数（グローバル）
/// 成功メッセージをフォーマット
pub fn format_success(message: &str) -> String {
    OutputFormatter::new().success(message)
}

/// エラーメッセージをフォーマット
pub fn format_error(message: &str) -> String {
    OutputFormatter::new().error(message)
}

/// 警告メッセージをフォーマット
pub fn format_warning(message: &str) -> String {
    OutputFormatter::new().warning(message)
}

/// 情報メッセージをフォーマット
pub fn format_info(message: &str) -> String {
    OutputFormatter::new().info(message)
}

/// コマンド出力をフォーマット（互換性のため）
pub fn format_output(output: &str) -> String {
    OutputFormatter::new().command_output(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_without_colors() {
        let formatter = OutputFormatter::new().with_colors(false);

        let success = formatter.success("Task completed");
        assert!(success.contains("✓"));
        assert!(success.contains("Task completed"));

        let error = formatter.error("Something went wrong");
        assert!(error.contains("✗"));
        assert!(error.contains("Something went wrong"));
    }

    #[test]
    fn test_formatter_with_verbose() {
        let formatter = OutputFormatter::new().with_verbose(true);

        let output = formatter.command_output("test output");
        assert!(output.contains("test output"));
    }

    #[test]
    fn test_command_messages() {
        let formatter = OutputFormatter::new().with_colors(false);

        let start = formatter.command_start("build");
        assert!(start.contains("build"));

        let complete = formatter.command_complete("build", 1234);
        assert!(complete.contains("build"));
        assert!(complete.contains("1234"));

        let failed = formatter.command_failed("test", 1);
        assert!(failed.contains("test"));
        assert!(failed.contains("1"));
    }

    #[test]
    fn test_section_and_list() {
        let formatter = OutputFormatter::new().with_colors(false);

        let header = formatter.section_header("Available Commands");
        assert!(header.contains("Available Commands"));

        let item = formatter.list_item("build - Build project");
        assert!(item.contains("•"));
        assert!(item.contains("build"));
    }

    #[test]
    fn test_progress() {
        let formatter = OutputFormatter::new().with_colors(false);

        let progress = formatter.progress(3, 10, "Running tests");
        assert!(progress.contains("[3/10]"));
        assert!(progress.contains("Running tests"));
    }

    #[test]
    fn test_global_format_functions() {
        let success = format_success("Done");
        assert!(success.contains("Done"));

        let error = format_error("Failed");
        assert!(error.contains("Failed"));

        let warning = format_warning("Warning");
        assert!(warning.contains("Warning"));

        let info = format_info("Info");
        assert!(info.contains("Info"));
    }
}
