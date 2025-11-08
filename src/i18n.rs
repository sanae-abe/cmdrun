//! 国際化(i18n)サポート
//!
//! 言語設定に応じたメッセージ出力

use crate::config::Language;

/// メッセージキー
#[derive(Debug, Clone, Copy)]
pub enum MessageKey {
    // ====== 実行状態 ======
    Running,
    Completed,
    Error,
    Warning,
    Success,

    // ====== CRUD操作 ======
    CommandAdded,
    CommandRemoved,
    CommandUpdated,

    // ====== 検証 ======
    Validating,
    ConfigValid,
    ValidationFailed,
    ValidatingConfiguration,
    ConfigurationIsValid,

    // ====== 対話プロンプト ======
    PromptCommandId,
    PromptCommand,
    PromptDescription,
    PromptCategory,
    PromptTags,
    PromptConfirm,
    PromptSelectCommand,
    PromptWhatToDo,
    PromptEnterNumber,
    PromptSelectTemplate,
    PromptSelectLanguage,

    // ====== 選択肢 ======
    OptionYesAdd,
    OptionNoEdit,
    OptionCancel,
    OptionEnglish,
    OptionJapanese,

    // ====== プレビュー・ラベル ======
    LabelPreview,
    LabelId,
    LabelCommand,
    LabelDescription,
    LabelCategory,
    LabelTags,
    LabelCurrentSettings,
    LabelDependencies,
    LabelPlatforms,
    LabelWorkingDirectory,
    LabelEnvironmentVariables,
    LabelExecutionSettings,
    LabelParallel,
    LabelConfirm,
    LabelTimeout,
    LabelCommandDetails,
    LabelConfiguration,
    LabelLanguage,
    LabelShell,
    LabelStrictMode,
    LabelBackupCreated,
    LabelYes,
    LabelNo,

    // ====== 警告メッセージ ======
    WarningShellBuiltinNoEffect,
    HintShellFunction,
    HintCdCommand,

    // ====== エラーメッセージ ======
    ErrorEmptyCommandId,
    ErrorEmptyCommand,
    ErrorEmptyDescription,
    ErrorCommandNotFound,
    ErrorCommandExists,
    ErrorConfigNotFound,
    ErrorInvalidConfig,
    ErrorInvalidSelection,
    ErrorSelectionOutOfRange,
    ErrorNoCommandsAvailable,
    ErrorUnknownTemplate,
    ErrorFileAlreadyExists,
    ErrorCircularDependency,
    ErrorValidationFailed,
    ErrorCommandFailed,
    ErrorUnknownConfigKey,
    ErrorAliasTargetNotFound,

    // ====== ヘルプテキスト ======
    HelpAddCommand,
    HelpRemoveCommand,
    HelpEditCommand,
    HelpListCommands,
    HelpRunCommand,
    HelpValidateConfig,
    HelpSearchCommand,
    HelpInfoCommand,
    HelpConfigCommand,
    HelpWatchCommand,
    HelpInitCommand,

    // ====== List コマンド ======
    ListNoCommandsDefined,
    ListAvailableCommands,
    ListCommandCount,
    ListAliasCount,

    // ====== Run コマンド ======
    RunRunningCommand,
    RunWithParallelDependencies,
    RunExecutionPlan,
    RunGroup,
    RunAllCommandsCompleted,
    RunCompletedIn,
    RunCommandFailedWithCode,

    // ====== Search コマンド ======
    SearchSearchingFor,
    SearchNoCommandsMatching,
    SearchFound,
    SearchMatchedIn,
    SearchUseInfoToSeeDetails,

    // ====== Info コマンド ======
    InfoSelectCommandToView,
    InfoBasicInformation,
    InfoCommandSpecification,
    InfoExecutionSettings,
    InfoPlatformSupport,

    // ====== Config コマンド ======
    ConfigSet,
    ConfigShowingConfiguration,

    // ====== Validate コマンド ======
    ValidateLoadedConfigFrom,
    ValidateCheckingCircularDependencies,
    ValidateNoCircularDependenciesFor,
    ValidateValidatingCommands,
    ValidateValidatingAliases,
    ValidateBuildingDependencyGraph,
    ValidateDependencyGraphBuilt,
    ValidateExecutionOrder,
    ValidateErrors,
    ValidateWarnings,
    ValidateInformation,
    ValidateFailedWithErrors,
    ValidateCommandsDefined,
    ValidateAliasesDefined,

    // ====== Init コマンド ======
    InitCreated,
    InitUsing,
    InitNextSteps,
    InitStep1EditFile,
    InitStep2ListCommands,
    InitStep3RunCommand,
    InitExampleCommands,
    InitTemplateDescription,
    InitLanguageSet,

    // ====== Watch コマンド ======
    WatchConfiguration,
    WatchCommand,
    WatchWatching,
    WatchPatterns,
    WatchExclude,
    WatchDebounce,
    WatchModeStarted,
    WatchPresCtrlCToStop,
    WatchModeStoppedByUser,

    // ====== Remove コマンド ======
    RemoveRemovalTarget,
    RemoveType,
    RemovePlatformSpecific,

    // ====== Edit コマンド ======
    EditParallelExecution,
    EditConfirmBeforeExecution,

    // ====== Graph コマンド ======
    GraphSavedTo,
    GraphRenderWith,
    GraphViewAt,

    // ====== Env コマンド ======
    EnvCurrent,
    EnvAvailableEnvironments,
    EnvSwitchedTo,
    EnvCreated,
    EnvVariableSet,
    EnvEnvironment,
    EnvDescription,
    EnvConfigFile,
    EnvEnvironmentVariables,
    EnvErrorNotFound,
    EnvErrorAlreadyExists,
    EnvErrorCannotSetDefault,

    // ====== その他 ======
    AddingCommand,
    RemovingCommand,
    UpdatingCommand,
    OpeningEditor,
    SearchResults,
    NoCommandsFound,
    Cancelled,
    LoadingConfiguration,
    CreatingBackup,
    MatchingCommands,
    Template,
}

/// メッセージの取得
pub fn get_message(key: MessageKey, language: Language) -> &'static str {
    use MessageKey::*;

    match language {
        Language::English => match key {
            // ====== 実行状態 ======
            Running => "Running",
            Completed => "Completed",
            Error => "Error",
            Warning => "Warning",
            Success => "Success",

            // ====== CRUD操作 ======
            CommandAdded => "Command added successfully",
            CommandRemoved => "Command removed successfully",
            CommandUpdated => "Command updated successfully",

            // ====== 検証 ======
            Validating => "Validating configuration",
            ConfigValid => "Configuration is valid",
            ValidationFailed => "Validation failed",
            ValidatingConfiguration => "Validating configuration...",
            ConfigurationIsValid => "Configuration is valid",

            // ====== 対話プロンプト ======
            PromptCommandId => "Command ID",
            PromptCommand => "Command",
            PromptDescription => "Description",
            PromptCategory => "Category",
            PromptTags => "Tags (comma-separated)",
            PromptConfirm => "Are you sure?",
            PromptSelectCommand => "Select command to edit",
            PromptWhatToDo => "What would you like to do?",
            PromptEnterNumber => "Enter number",
            PromptSelectTemplate => "Select a template",
            PromptSelectLanguage => "Select your preferred language",

            // ====== 選択肢 ======
            OptionYesAdd => "Yes, add this command",
            OptionNoEdit => "No, edit again",
            OptionCancel => "Cancel",
            OptionEnglish => "English",
            OptionJapanese => "日本語 (Japanese)",

            // ====== プレビュー・ラベル ======
            LabelPreview => "Preview",
            LabelId => "ID",
            LabelCommand => "Command",
            LabelDescription => "Description",
            LabelCategory => "Category",
            LabelTags => "Tags",
            LabelCurrentSettings => "Current settings",
            LabelDependencies => "Dependencies",
            LabelPlatforms => "Platforms",
            LabelWorkingDirectory => "Working directory",
            LabelEnvironmentVariables => "Environment variables",
            LabelExecutionSettings => "Execution settings",
            LabelParallel => "Parallel",
            LabelConfirm => "Confirm",
            LabelTimeout => "Timeout",
            LabelCommandDetails => "Command details",
            LabelConfiguration => "Configuration",
            LabelLanguage => "language",
            LabelShell => "shell",
            LabelStrictMode => "strict_mode",
            LabelBackupCreated => "Backup created",
            LabelYes => "yes",
            LabelNo => "no",

            // ====== 警告メッセージ ======
            WarningShellBuiltinNoEffect => "⚠ This shell builtin command runs in a subprocess and won't affect the current shell",
            HintShellFunction => "💡 Hint: Use shell functions for directory navigation",
            HintCdCommand => "   Add to ~/.cmdrun/shell-functions.sh:",

            // ====== エラーメッセージ ======
            ErrorEmptyCommandId => "Command ID cannot be empty",
            ErrorEmptyCommand => "Command cannot be empty",
            ErrorEmptyDescription => "Description cannot be empty",
            ErrorCommandNotFound => "Command not found",
            ErrorCommandExists => "Command already exists",
            ErrorConfigNotFound => "Configuration file not found",
            ErrorInvalidConfig => "Invalid configuration",
            ErrorInvalidSelection => "Invalid selection",
            ErrorSelectionOutOfRange => "Selection out of range",
            ErrorNoCommandsAvailable => "No commands available",
            ErrorUnknownTemplate => "Unknown template",
            ErrorFileAlreadyExists => "Configuration file already exists",
            ErrorCircularDependency => "Circular dependency detected",
            ErrorValidationFailed => "Validation failed",
            ErrorCommandFailed => "Command failed",
            ErrorUnknownConfigKey => "Unknown configuration key",
            ErrorAliasTargetNotFound => "Alias target not found",

            // ====== ヘルプテキスト ======
            HelpAddCommand => "Add a new command to the configuration",
            HelpRemoveCommand => "Remove a command from the configuration",
            HelpEditCommand => "Edit an existing command",
            HelpListCommands => "List all available commands",
            HelpRunCommand => "Run a command",
            HelpValidateConfig => "Validate configuration file",
            HelpSearchCommand => "Search commands by keyword",
            HelpInfoCommand => "Show detailed command information",
            HelpConfigCommand => "Manage configuration settings",
            HelpWatchCommand => "Watch files and run command on changes",
            HelpInitCommand => "Initialize a new configuration file",

            // ====== List コマンド ======
            ListNoCommandsDefined => "No commands defined",
            ListAvailableCommands => "Available commands",
            ListCommandCount => "commands defined",
            ListAliasCount => "aliases defined",

            // ====== Run コマンド ======
            RunRunningCommand => "Running",
            RunWithParallelDependencies => "with parallel dependencies",
            RunExecutionPlan => "Execution plan",
            RunGroup => "Group",
            RunAllCommandsCompleted => "All commands completed in",
            RunCompletedIn => "Completed in",
            RunCommandFailedWithCode => "Command failed with exit code",

            // ====== Search コマンド ======
            SearchSearchingFor => "Searching for",
            SearchNoCommandsMatching => "No commands matching",
            SearchFound => "Found",
            SearchMatchedIn => "Matched in",
            SearchUseInfoToSeeDetails => "Use cmdrun info <command> to see details",

            // ====== Info コマンド ======
            InfoSelectCommandToView => "Select command to view details",
            InfoBasicInformation => "Basic information",
            InfoCommandSpecification => "Command specification",
            InfoExecutionSettings => "Execution settings",
            InfoPlatformSupport => "Platform support",

            // ====== Config コマンド ======
            ConfigSet => "Set",
            ConfigShowingConfiguration => "Showing configuration",

            // ====== Validate コマンド ======
            ValidateLoadedConfigFrom => "Loaded configuration from",
            ValidateCheckingCircularDependencies => "Checking for circular dependencies...",
            ValidateNoCircularDependenciesFor => "No circular dependencies for",
            ValidateValidatingCommands => "Validating commands",
            ValidateValidatingAliases => "Validating aliases",
            ValidateBuildingDependencyGraph => "Building dependency graph...",
            ValidateDependencyGraphBuilt => "Dependency graph built successfully",
            ValidateExecutionOrder => "Execution order",
            ValidateErrors => "Errors",
            ValidateWarnings => "Warnings",
            ValidateInformation => "Information",
            ValidateFailedWithErrors => "Configuration validation failed with",
            ValidateCommandsDefined => "commands defined",
            ValidateAliasesDefined => "aliases defined",

            // ====== Init コマンド ======
            InitCreated => "Created",
            InitUsing => "Using",
            InitNextSteps => "Next steps",
            InitStep1EditFile => "Edit {0} to define your commands",
            InitStep2ListCommands => "Run cmdrun list to list available commands",
            InitStep3RunCommand => "Run cmdrun run <name> to execute a command",
            InitExampleCommands => "Example commands",
            InitTemplateDescription => "template",
            InitLanguageSet => "Language set to",

            // ====== Watch コマンド ======
            WatchConfiguration => "Watch Configuration",
            WatchCommand => "Command",
            WatchWatching => "Watching",
            WatchPatterns => "Patterns",
            WatchExclude => "Exclude",
            WatchDebounce => "Debounce",
            WatchModeStarted => "Watch mode started. Press Ctrl+C to stop.",
            WatchPresCtrlCToStop => "Press Ctrl+C to stop",
            WatchModeStoppedByUser => "Watch mode stopped by user",

            // ====== Remove コマンド ======
            RemoveRemovalTarget => "Removal target",
            RemoveType => "Type",
            RemovePlatformSpecific => "Platform-specific",

            // ====== Edit コマンド ======
            EditParallelExecution => "Parallel execution",
            EditConfirmBeforeExecution => "Confirm before execution",

            // ====== Graph コマンド ======
            GraphSavedTo => "Graph saved to",
            GraphRenderWith => "Render with",
            GraphViewAt => "View at",

            // ====== Env コマンド ======
            EnvCurrent => "Current environment",
            EnvAvailableEnvironments => "Available environments",
            EnvSwitchedTo => "Switched to environment",
            EnvCreated => "Created environment",
            EnvVariableSet => "Set variable",
            EnvEnvironment => "Environment",
            EnvDescription => "Description",
            EnvConfigFile => "Config file",
            EnvEnvironmentVariables => "Environment variables",
            EnvErrorNotFound => "Environment not found",
            EnvErrorAlreadyExists => "Environment already exists",
            EnvErrorCannotSetDefault => "Cannot set variables for 'default' environment",

            // ====== その他 ======
            AddingCommand => "Adding command",
            RemovingCommand => "Removing command",
            UpdatingCommand => "Updating command",
            OpeningEditor => "Opening editor",
            SearchResults => "Search results",
            NoCommandsFound => "No commands found",
            Cancelled => "Cancelled",
            LoadingConfiguration => "Loading configuration",
            CreatingBackup => "Creating backup",
            MatchingCommands => "matching command(s)",
            Template => "template",
        },
        Language::Japanese => match key {
            // ====== 実行状態 ======
            Running => "実行中",
            Completed => "完了",
            Error => "エラー",
            Warning => "警告",
            Success => "成功",

            // ====== CRUD操作 ======
            CommandAdded => "コマンドを追加しました",
            CommandRemoved => "コマンドを削除しました",
            CommandUpdated => "コマンドを更新しました",

            // ====== 検証 ======
            Validating => "設定を検証中",
            ConfigValid => "設定は有効です",
            ValidationFailed => "検証に失敗しました",
            ValidatingConfiguration => "設定を検証中...",
            ConfigurationIsValid => "設定は有効です",

            // ====== 対話プロンプト ======
            PromptCommandId => "コマンドID",
            PromptCommand => "コマンド",
            PromptDescription => "説明",
            PromptCategory => "カテゴリ",
            PromptTags => "タグ（カンマ区切り）",
            PromptConfirm => "よろしいですか？",
            PromptSelectCommand => "編集するコマンドを選択",
            PromptWhatToDo => "どうしますか？",
            PromptEnterNumber => "番号を入力",
            PromptSelectTemplate => "テンプレートを選択",
            PromptSelectLanguage => "言語を選択してください",

            // ====== 選択肢 ======
            OptionYesAdd => "はい、このコマンドを追加",
            OptionNoEdit => "いいえ、再編集",
            OptionCancel => "キャンセル",
            OptionEnglish => "English (英語)",
            OptionJapanese => "日本語",

            // ====== プレビュー・ラベル ======
            LabelPreview => "プレビュー",
            LabelId => "ID",
            LabelCommand => "コマンド",
            LabelDescription => "説明",
            LabelCategory => "カテゴリ",
            LabelTags => "タグ",
            LabelCurrentSettings => "現在の設定",
            LabelDependencies => "依存関係",
            LabelPlatforms => "プラットフォーム",
            LabelWorkingDirectory => "作業ディレクトリ",
            LabelEnvironmentVariables => "環境変数",
            LabelExecutionSettings => "実行設定",
            LabelParallel => "並列実行",
            LabelConfirm => "実行前確認",
            LabelTimeout => "タイムアウト",
            LabelCommandDetails => "コマンド詳細",
            LabelConfiguration => "設定",
            LabelLanguage => "言語",
            LabelShell => "シェル",
            LabelStrictMode => "厳格モード",
            LabelBackupCreated => "バックアップを作成しました",
            LabelYes => "はい",
            LabelNo => "いいえ",

            // ====== 警告メッセージ ======
            WarningShellBuiltinNoEffect => "⚠ このシェルビルトインコマンドはサブプロセスで実行されるため、現在のシェルには影響しません",
            HintShellFunction => "💡 ヒント: ディレクトリ移動にはシェル関数を使ってください",
            HintCdCommand => "   ~/.cmdrun/shell-functions.sh に追加:",

            // ====== エラーメッセージ ======
            ErrorEmptyCommandId => "コマンドIDは空にできません",
            ErrorEmptyCommand => "コマンドは空にできません",
            ErrorEmptyDescription => "説明は空にできません",
            ErrorCommandNotFound => "コマンドが見つかりません",
            ErrorCommandExists => "コマンドは既に存在します",
            ErrorConfigNotFound => "設定ファイルが見つかりません",
            ErrorInvalidConfig => "無効な設定です",
            ErrorInvalidSelection => "無効な選択です",
            ErrorSelectionOutOfRange => "選択が範囲外です",
            ErrorNoCommandsAvailable => "利用可能なコマンドがありません",
            ErrorUnknownTemplate => "不明なテンプレートです",
            ErrorFileAlreadyExists => "設定ファイルは既に存在します",
            ErrorCircularDependency => "循環依存が検出されました",
            ErrorValidationFailed => "検証に失敗しました",
            ErrorCommandFailed => "コマンドが失敗しました",
            ErrorUnknownConfigKey => "不明な設定キーです",
            ErrorAliasTargetNotFound => "エイリアス先が見つかりません",

            // ====== ヘルプテキスト ======
            HelpAddCommand => "設定に新しいコマンドを追加",
            HelpRemoveCommand => "設定からコマンドを削除",
            HelpEditCommand => "既存のコマンドを編集",
            HelpListCommands => "利用可能なコマンド一覧を表示",
            HelpRunCommand => "コマンドを実行",
            HelpValidateConfig => "設定ファイルを検証",
            HelpSearchCommand => "キーワードでコマンドを検索",
            HelpInfoCommand => "コマンドの詳細情報を表示",
            HelpConfigCommand => "設定を管理",
            HelpWatchCommand => "ファイルを監視して変更時にコマンドを実行",
            HelpInitCommand => "新しい設定ファイルを初期化",

            // ====== List コマンド ======
            ListNoCommandsDefined => "コマンドが定義されていません",
            ListAvailableCommands => "利用可能なコマンド",
            ListCommandCount => "個のコマンドが定義されています",
            ListAliasCount => "個のエイリアスが定義されています",

            // ====== Run コマンド ======
            RunRunningCommand => "実行中",
            RunWithParallelDependencies => "（並列依存関係あり）",
            RunExecutionPlan => "実行計画",
            RunGroup => "グループ",
            RunAllCommandsCompleted => "すべてのコマンドが完了しました",
            RunCompletedIn => "完了時間",
            RunCommandFailedWithCode => "コマンドが失敗しました（終了コード",

            // ====== Search コマンド ======
            SearchSearchingFor => "検索中",
            SearchNoCommandsMatching => "一致するコマンドが見つかりません",
            SearchFound => "見つかりました",
            SearchMatchedIn => "一致箇所",
            SearchUseInfoToSeeDetails => "詳細を見るには cmdrun info <コマンド> を使用してください",

            // ====== Info コマンド ======
            InfoSelectCommandToView => "詳細を表示するコマンドを選択",
            InfoBasicInformation => "基本情報",
            InfoCommandSpecification => "コマンド仕様",
            InfoExecutionSettings => "実行設定",
            InfoPlatformSupport => "プラットフォームサポート",

            // ====== Config コマンド ======
            ConfigSet => "設定しました",
            ConfigShowingConfiguration => "設定を表示中",

            // ====== Validate コマンド ======
            ValidateLoadedConfigFrom => "設定を読み込みました",
            ValidateCheckingCircularDependencies => "循環依存を確認中...",
            ValidateNoCircularDependenciesFor => "循環依存はありません",
            ValidateValidatingCommands => "コマンドを検証中",
            ValidateValidatingAliases => "エイリアスを検証中",
            ValidateBuildingDependencyGraph => "依存関係グラフを構築中...",
            ValidateDependencyGraphBuilt => "依存関係グラフを構築しました",
            ValidateExecutionOrder => "実行順序",
            ValidateErrors => "エラー",
            ValidateWarnings => "警告",
            ValidateInformation => "情報",
            ValidateFailedWithErrors => "設定の検証に失敗しました（エラー数",
            ValidateCommandsDefined => "個のコマンドが定義されています",
            ValidateAliasesDefined => "個のエイリアスが定義されています",

            // ====== Init コマンド ======
            InitCreated => "作成しました",
            InitUsing => "使用中",
            InitNextSteps => "次のステップ",
            InitStep1EditFile => "{0} を編集してコマンドを定義",
            InitStep2ListCommands => "cmdrun list で利用可能なコマンド一覧を表示",
            InitStep3RunCommand => "cmdrun run <名前> でコマンドを実行",
            InitExampleCommands => "コマンド例",
            InitTemplateDescription => "テンプレート",
            InitLanguageSet => "言語を設定しました",

            // ====== Watch コマンド ======
            WatchConfiguration => "監視設定",
            WatchCommand => "コマンド",
            WatchWatching => "監視中",
            WatchPatterns => "パターン",
            WatchExclude => "除外",
            WatchDebounce => "デバウンス",
            WatchModeStarted => "監視モードを開始しました。Ctrl+C で停止します。",
            WatchPresCtrlCToStop => "Ctrl+C で停止",
            WatchModeStoppedByUser => "監視モードをユーザーが停止しました",

            // ====== Remove コマンド ======
            RemoveRemovalTarget => "削除対象",
            RemoveType => "タイプ",
            RemovePlatformSpecific => "プラットフォーム固有",

            // ====== Edit コマンド ======
            EditParallelExecution => "並列実行",
            EditConfirmBeforeExecution => "実行前確認",

            // ====== Graph コマンド ======
            GraphSavedTo => "グラフを保存しました",
            GraphRenderWith => "レンダリング",
            GraphViewAt => "表示",

            // ====== Env コマンド ======
            EnvCurrent => "現在の環境",
            EnvAvailableEnvironments => "利用可能な環境",
            EnvSwitchedTo => "環境を切り替えました",
            EnvCreated => "環境を作成しました",
            EnvVariableSet => "変数を設定しました",
            EnvEnvironment => "環境",
            EnvDescription => "説明",
            EnvConfigFile => "設定ファイル",
            EnvEnvironmentVariables => "環境変数",
            EnvErrorNotFound => "環境が見つかりません",
            EnvErrorAlreadyExists => "環境は既に存在します",
            EnvErrorCannotSetDefault => "デフォルト環境には変数を設定できません",

            // ====== その他 ======
            AddingCommand => "コマンドを追加中",
            RemovingCommand => "コマンドを削除中",
            UpdatingCommand => "コマンドを更新中",
            OpeningEditor => "エディタを起動中",
            SearchResults => "検索結果",
            NoCommandsFound => "コマンドが見つかりません",
            Cancelled => "キャンセルしました",
            LoadingConfiguration => "設定を読み込み中",
            CreatingBackup => "バックアップを作成中",
            MatchingCommands => "件の一致するコマンド",
            Template => "テンプレート",
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
        assert_eq!(
            get_message(MessageKey::ListAvailableCommands, Language::English),
            "Available commands"
        );
    }

    #[test]
    fn test_get_message_japanese() {
        assert_eq!(
            get_message(MessageKey::Running, Language::Japanese),
            "実行中"
        );
        assert_eq!(
            get_message(MessageKey::ListAvailableCommands, Language::Japanese),
            "利用可能なコマンド"
        );
    }

    #[test]
    fn test_format_message() {
        let result = format_message(
            MessageKey::InitStep1EditFile,
            Language::English,
            &["commands.toml"],
        );
        assert_eq!(result, "Edit commands.toml to define your commands");

        let result_ja = format_message(
            MessageKey::InitStep1EditFile,
            Language::Japanese,
            &["commands.toml"],
        );
        assert_eq!(result_ja, "commands.toml を編集してコマンドを定義");
    }

    #[test]
    fn test_language_selection_messages() {
        // Test English language selection messages
        assert_eq!(
            get_message(MessageKey::PromptSelectLanguage, Language::English),
            "Select your preferred language"
        );
        assert_eq!(
            get_message(MessageKey::OptionEnglish, Language::English),
            "English"
        );
        assert_eq!(
            get_message(MessageKey::OptionJapanese, Language::English),
            "日本語 (Japanese)"
        );

        // Test Japanese language selection messages
        assert_eq!(
            get_message(MessageKey::PromptSelectLanguage, Language::Japanese),
            "言語を選択してください"
        );
        assert_eq!(
            get_message(MessageKey::OptionEnglish, Language::Japanese),
            "English (英語)"
        );
        assert_eq!(
            get_message(MessageKey::OptionJapanese, Language::Japanese),
            "日本語"
        );
    }
}
