//! 国際化(i18n)サポート
//!
//! 言語設定に応じたメッセージ出力

use crate::config::Language;

/// メッセージキー
#[derive(Debug, Clone, Copy)]
pub enum MessageKey {
    // 実行状態
    Running,
    Completed,
    Error,
    Warning,

    // CRUD操作
    CommandAdded,
    CommandRemoved,
    CommandUpdated,

    // 検証
    Validating,
    ConfigValid,

    // 対話プロンプト
    PromptCommandId,
    PromptCommand,
    PromptDescription,
    PromptCategory,
    PromptTags,
    PromptConfirm,
    PromptSelectCommand,
    PromptWhatToDo,

    // 選択肢
    OptionYesAdd,
    OptionNoEdit,
    OptionCancel,

    // プレビュー・ラベル
    LabelPreview,
    LabelId,
    LabelCommand,
    LabelDescription,
    LabelCategory,
    LabelTags,
    LabelCurrentSettings,

    // エラーメッセージ
    ErrorEmptyCommandId,
    ErrorEmptyCommand,
    ErrorEmptyDescription,
    ErrorCommandNotFound,
    ErrorCommandExists,
    ErrorConfigNotFound,
    ErrorInvalidConfig,

    // ヘルプテキスト
    HelpAddCommand,
    HelpRemoveCommand,
    HelpEditCommand,
    HelpListCommands,
    HelpRunCommand,
    HelpValidateConfig,

    // その他
    AddingCommand,
    RemovingCommand,
    UpdatingCommand,
    OpeningEditor,
    SearchResults,
    NoCommandsFound,
    Cancelled,
}

/// メッセージの取得
pub fn get_message(key: MessageKey, language: Language) -> &'static str {
    use MessageKey::*;

    match language {
        Language::English => match key {
            // 実行状態
            Running => "Running",
            Completed => "Completed",
            Error => "Error",
            Warning => "Warning",

            // CRUD操作
            CommandAdded => "Command added successfully",
            CommandRemoved => "Command removed successfully",
            CommandUpdated => "Command updated successfully",

            // 検証
            Validating => "Validating configuration",
            ConfigValid => "Configuration is valid",

            // 対話プロンプト
            PromptCommandId => "Command ID",
            PromptCommand => "Command",
            PromptDescription => "Description",
            PromptCategory => "Category",
            PromptTags => "Tags (comma-separated)",
            PromptConfirm => "Are you sure?",
            PromptSelectCommand => "Select command to edit",
            PromptWhatToDo => "What would you like to do?",

            // 選択肢
            OptionYesAdd => "Yes, add this command",
            OptionNoEdit => "No, edit again",
            OptionCancel => "Cancel",

            // プレビュー・ラベル
            LabelPreview => "Preview",
            LabelId => "ID",
            LabelCommand => "Command",
            LabelDescription => "Description",
            LabelCategory => "Category",
            LabelTags => "Tags",
            LabelCurrentSettings => "Current settings",

            // エラーメッセージ
            ErrorEmptyCommandId => "Command ID cannot be empty",
            ErrorEmptyCommand => "Command cannot be empty",
            ErrorEmptyDescription => "Description cannot be empty",
            ErrorCommandNotFound => "Command not found",
            ErrorCommandExists => "Command already exists",
            ErrorConfigNotFound => "Configuration file not found",
            ErrorInvalidConfig => "Invalid configuration",

            // ヘルプテキスト
            HelpAddCommand => "Add a new command to the configuration",
            HelpRemoveCommand => "Remove a command from the configuration",
            HelpEditCommand => "Edit an existing command",
            HelpListCommands => "List all available commands",
            HelpRunCommand => "Run a command",
            HelpValidateConfig => "Validate configuration file",

            // その他
            AddingCommand => "Adding command",
            RemovingCommand => "Removing command",
            UpdatingCommand => "Updating command",
            OpeningEditor => "Opening editor",
            SearchResults => "Search results",
            NoCommandsFound => "No commands found",
            Cancelled => "Cancelled",
        },
        Language::Japanese => match key {
            // 実行状態
            Running => "実行中",
            Completed => "完了",
            Error => "エラー",
            Warning => "警告",

            // CRUD操作
            CommandAdded => "コマンドを追加しました",
            CommandRemoved => "コマンドを削除しました",
            CommandUpdated => "コマンドを更新しました",

            // 検証
            Validating => "設定を検証中",
            ConfigValid => "設定は有効です",

            // 対話プロンプト
            PromptCommandId => "コマンドID",
            PromptCommand => "コマンド",
            PromptDescription => "説明",
            PromptCategory => "カテゴリ",
            PromptTags => "タグ（カンマ区切り）",
            PromptConfirm => "よろしいですか？",
            PromptSelectCommand => "編集するコマンドを選択",
            PromptWhatToDo => "どうしますか？",

            // 選択肢
            OptionYesAdd => "はい、このコマンドを追加",
            OptionNoEdit => "いいえ、再編集",
            OptionCancel => "キャンセル",

            // プレビュー・ラベル
            LabelPreview => "プレビュー",
            LabelId => "ID",
            LabelCommand => "コマンド",
            LabelDescription => "説明",
            LabelCategory => "カテゴリ",
            LabelTags => "タグ",
            LabelCurrentSettings => "現在の設定",

            // エラーメッセージ
            ErrorEmptyCommandId => "コマンドIDは空にできません",
            ErrorEmptyCommand => "コマンドは空にできません",
            ErrorEmptyDescription => "説明は空にできません",
            ErrorCommandNotFound => "コマンドが見つかりません",
            ErrorCommandExists => "コマンドは既に存在します",
            ErrorConfigNotFound => "設定ファイルが見つかりません",
            ErrorInvalidConfig => "無効な設定です",

            // ヘルプテキスト
            HelpAddCommand => "設定に新しいコマンドを追加",
            HelpRemoveCommand => "設定からコマンドを削除",
            HelpEditCommand => "既存のコマンドを編集",
            HelpListCommands => "利用可能なコマンド一覧を表示",
            HelpRunCommand => "コマンドを実行",
            HelpValidateConfig => "設定ファイルを検証",

            // その他
            AddingCommand => "コマンドを追加中",
            RemovingCommand => "コマンドを削除中",
            UpdatingCommand => "コマンドを更新中",
            OpeningEditor => "エディタを起動中",
            SearchResults => "検索結果",
            NoCommandsFound => "コマンドが見つかりません",
            Cancelled => "キャンセルしました",
        },
    }
}

/// フォーマット付きメッセージの取得
pub fn format_message(key: MessageKey, language: Language, args: &[&str]) -> String {
    let template = get_message(key, language);
    let mut result = template.to_string();

    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, arg);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_message_english() {
        assert_eq!(
            get_message(MessageKey::Running, Language::English),
            "Running"
        );
    }

    #[test]
    fn test_get_message_japanese() {
        assert_eq!(
            get_message(MessageKey::Running, Language::Japanese),
            "実行中"
        );
    }
}
