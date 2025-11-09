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
    InfoConfigurationPaths,
    InfoGlobalConfigPath,
    InfoLocalConfigPath,
    InfoActualWorkingDirectory,
    InfoExecutionStatistics,
    InfoTotalExecutions,
    InfoSuccessfulRuns,
    InfoFailedRuns,
    InfoLastRun,
    InfoAverageDuration,

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

    // ====== Typo検出 ======
    TypoUnknownCommand,
    TypoDidYouMean,
    TypoSuggestions,
    TypoRunHelp,

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
            InfoConfigurationPaths => "Configuration paths",
            InfoGlobalConfigPath => "Global config",
            InfoLocalConfigPath => "Local config",
            InfoActualWorkingDirectory => "Actual working directory",
            InfoExecutionStatistics => "Execution Statistics",
            InfoTotalExecutions => "Total executions",
            InfoSuccessfulRuns => "Successful runs",
            InfoFailedRuns => "Failed runs",
            InfoLastRun => "Last run",
            InfoAverageDuration => "Average duration",

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

            // ====== Typo検出 ======
            TypoUnknownCommand => "Unknown command",
            TypoDidYouMean => "Did you mean one of these?",
            TypoSuggestions => "Suggestions",
            TypoRunHelp => "Run 'cmdrun --help' for available commands",

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
            InfoConfigurationPaths => "設定ファイルパス",
            InfoGlobalConfigPath => "グローバル設定",
            InfoLocalConfigPath => "ローカル設定",
            InfoActualWorkingDirectory => "実際の作業ディレクトリ",
            InfoExecutionStatistics => "実行統計",
            InfoTotalExecutions => "総実行回数",
            InfoSuccessfulRuns => "成功回数",
            InfoFailedRuns => "失敗回数",
            InfoLastRun => "最終実行",
            InfoAverageDuration => "平均実行時間",

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

            // ====== Typo検出 ======
            TypoUnknownCommand => "不明なコマンド",
            TypoDidYouMean => "もしかして:",
            TypoSuggestions => "候補",
            TypoRunHelp => "'cmdrun --help' で利用可能なコマンドを確認できます",

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
        Language::ChineseSimplified => match key {
            // ====== 实行状态 ======
            Running => "运行中",
            Completed => "已完成",
            Error => "错误",
            Warning => "警告",
            Success => "成功",

            // ====== CRUD操作 ======
            CommandAdded => "成功添加命令",
            CommandRemoved => "成功删除命令",
            CommandUpdated => "成功更新命令",

            // ====== 验证 ======
            Validating => "正在验证配置",
            ConfigValid => "配置有效",
            ValidationFailed => "验证失败",
            ValidatingConfiguration => "正在验证配置...",
            ConfigurationIsValid => "配置有效",

            // ====== 交互提示 ======
            PromptCommandId => "命令ID",
            PromptCommand => "命令",
            PromptDescription => "描述",
            PromptCategory => "分类",
            PromptTags => "标签（逗号分隔）",
            PromptConfirm => "确定吗？",
            PromptSelectCommand => "选择要编辑的命令",
            PromptWhatToDo => "您想做什么？",
            PromptEnterNumber => "输入数字",
            PromptSelectTemplate => "选择模板",
            PromptSelectLanguage => "选择首选语言",

            // ====== 选项 ======
            OptionYesAdd => "是，添加此命令",
            OptionNoEdit => "否，重新编辑",
            OptionCancel => "取消",
            OptionEnglish => "English (英语)",
            OptionJapanese => "日本語 (日语)",

            // ====== 预览·标签 ======
            LabelPreview => "预览",
            LabelId => "ID",
            LabelCommand => "命令",
            LabelDescription => "描述",
            LabelCategory => "分类",
            LabelTags => "标签",
            LabelCurrentSettings => "当前设置",
            LabelDependencies => "依赖关系",
            LabelPlatforms => "平台",
            LabelWorkingDirectory => "工作目录",
            LabelEnvironmentVariables => "环境变量",
            LabelExecutionSettings => "执行设置",
            LabelParallel => "并行执行",
            LabelConfirm => "执行前确认",
            LabelTimeout => "超时",
            LabelCommandDetails => "命令详情",
            LabelConfiguration => "配置",
            LabelLanguage => "语言",
            LabelShell => "shell",
            LabelStrictMode => "严格模式",
            LabelBackupCreated => "已创建备份",
            LabelYes => "是",
            LabelNo => "否",

            // ====== 警告消息 ======
            WarningShellBuiltinNoEffect => "⚠ 此shell内置命令在子进程中运行，不会影响当前shell",
            HintShellFunction => "💡 提示：使用shell函数进行目录导航",
            HintCdCommand => "   添加到 ~/.cmdrun/shell-functions.sh：",

            // ====== 错误消息 ======
            ErrorEmptyCommandId => "命令ID不能为空",
            ErrorEmptyCommand => "命令不能为空",
            ErrorEmptyDescription => "描述不能为空",
            ErrorCommandNotFound => "找不到命令",
            ErrorCommandExists => "命令已存在",
            ErrorConfigNotFound => "找不到配置文件",
            ErrorInvalidConfig => "无效配置",
            ErrorInvalidSelection => "无效选择",
            ErrorSelectionOutOfRange => "选择超出范围",
            ErrorNoCommandsAvailable => "没有可用命令",
            ErrorUnknownTemplate => "未知模板",
            ErrorFileAlreadyExists => "配置文件已存在",
            ErrorCircularDependency => "检测到循环依赖",
            ErrorValidationFailed => "验证失败",
            ErrorCommandFailed => "命令执行失败",
            ErrorUnknownConfigKey => "未知配置键",
            ErrorAliasTargetNotFound => "找不到别名目标",

            // ====== 帮助文本 ======
            HelpAddCommand => "向配置中添加新命令",
            HelpRemoveCommand => "从配置中删除命令",
            HelpEditCommand => "编辑现有命令",
            HelpListCommands => "列出所有可用命令",
            HelpRunCommand => "运行命令",
            HelpValidateConfig => "验证配置文件",
            HelpSearchCommand => "按关键字搜索命令",
            HelpInfoCommand => "显示命令详细信息",
            HelpConfigCommand => "管理配置设置",
            HelpWatchCommand => "监视文件并在更改时运行命令",
            HelpInitCommand => "初始化新配置文件",

            // ====== List 命令 ======
            ListNoCommandsDefined => "未定义命令",
            ListAvailableCommands => "可用命令",
            ListCommandCount => "个已定义命令",
            ListAliasCount => "个已定义别名",

            // ====== Run 命令 ======
            RunRunningCommand => "运行中",
            RunWithParallelDependencies => "（含并行依赖）",
            RunExecutionPlan => "执行计划",
            RunGroup => "组",
            RunAllCommandsCompleted => "所有命令已完成",
            RunCompletedIn => "完成时间",
            RunCommandFailedWithCode => "命令执行失败，退出代码",

            // ====== Search 命令 ======
            SearchSearchingFor => "搜索中",
            SearchNoCommandsMatching => "没有匹配的命令",
            SearchFound => "找到",
            SearchMatchedIn => "匹配位置",
            SearchUseInfoToSeeDetails => "使用 cmdrun info <命令> 查看详情",

            // ====== Info 命令 ======
            InfoSelectCommandToView => "选择要查看详情的命令",
            InfoBasicInformation => "基本信息",
            InfoCommandSpecification => "命令规范",
            InfoExecutionSettings => "执行设置",
            InfoPlatformSupport => "平台支持",
            InfoConfigurationPaths => "配置文件路径",
            InfoGlobalConfigPath => "全局配置",
            InfoLocalConfigPath => "本地配置",
            InfoActualWorkingDirectory => "实际工作目录",
            InfoExecutionStatistics => "执行统计",
            InfoTotalExecutions => "总执行次数",
            InfoSuccessfulRuns => "成功次数",
            InfoFailedRuns => "失败次数",
            InfoLastRun => "最后执行",
            InfoAverageDuration => "平均执行时间",

            // ====== Config 命令 ======
            ConfigSet => "已设置",
            ConfigShowingConfiguration => "显示配置",

            // ====== Validate 命令 ======
            ValidateLoadedConfigFrom => "已加载配置文件",
            ValidateCheckingCircularDependencies => "正在检查循环依赖...",
            ValidateNoCircularDependenciesFor => "无循环依赖",
            ValidateValidatingCommands => "正在验证命令",
            ValidateValidatingAliases => "正在验证别名",
            ValidateBuildingDependencyGraph => "正在构建依赖关系图...",
            ValidateDependencyGraphBuilt => "依赖关系图构建成功",
            ValidateExecutionOrder => "执行顺序",
            ValidateErrors => "错误",
            ValidateWarnings => "警告",
            ValidateInformation => "信息",
            ValidateFailedWithErrors => "配置验证失败，错误数",
            ValidateCommandsDefined => "个已定义命令",
            ValidateAliasesDefined => "个已定义别名",

            // ====== Init 命令 ======
            InitCreated => "已创建",
            InitUsing => "使用中",
            InitNextSteps => "下一步",
            InitStep1EditFile => "编辑 {0} 来定义您的命令",
            InitStep2ListCommands => "运行 cmdrun list 列出可用命令",
            InitStep3RunCommand => "运行 cmdrun run <名称> 执行命令",
            InitExampleCommands => "示例命令",
            InitTemplateDescription => "模板",
            InitLanguageSet => "语言已设置为",

            // ====== Watch 命令 ======
            WatchConfiguration => "监视配置",
            WatchCommand => "命令",
            WatchWatching => "监视中",
            WatchPatterns => "模式",
            WatchExclude => "排除",
            WatchDebounce => "防抖",
            WatchModeStarted => "监视模式已启动。按 Ctrl+C 停止。",
            WatchPresCtrlCToStop => "按 Ctrl+C 停止",
            WatchModeStoppedByUser => "用户已停止监视模式",

            // ====== Remove 命令 ======
            RemoveRemovalTarget => "删除目标",
            RemoveType => "类型",
            RemovePlatformSpecific => "平台特定",

            // ====== Edit 命令 ======
            EditParallelExecution => "并行执行",
            EditConfirmBeforeExecution => "执行前确认",

            // ====== Graph 命令 ======
            GraphSavedTo => "图表已保存至",
            GraphRenderWith => "渲染工具",
            GraphViewAt => "查看位置",

            // ====== Env 命令 ======
            EnvCurrent => "当前环境",
            EnvAvailableEnvironments => "可用环境",
            EnvSwitchedTo => "已切换到环境",
            EnvCreated => "已创建环境",
            EnvVariableSet => "已设置变量",
            EnvEnvironment => "环境",
            EnvDescription => "描述",
            EnvConfigFile => "配置文件",
            EnvEnvironmentVariables => "环境变量",
            EnvErrorNotFound => "找不到环境",
            EnvErrorAlreadyExists => "环境已存在",
            EnvErrorCannotSetDefault => "无法为'default'环境设置变量",

            // ====== Typo检测 ======
            TypoUnknownCommand => "未知命令",
            TypoDidYouMean => "您是否想输入:",
            TypoSuggestions => "建议",
            TypoRunHelp => "运行 'cmdrun --help' 查看可用命令",

            // ====== 其他 ======
            AddingCommand => "正在添加命令",
            RemovingCommand => "正在删除命令",
            UpdatingCommand => "正在更新命令",
            OpeningEditor => "正在打开编辑器",
            SearchResults => "搜索结果",
            NoCommandsFound => "找不到命令",
            Cancelled => "已取消",
            LoadingConfiguration => "正在加载配置",
            CreatingBackup => "正在创建备份",
            MatchingCommands => "个匹配命令",
            Template => "模板",
        },
        Language::ChineseTraditional => match key {
            // ====== 執行狀態 ======
            Running => "執行中",
            Completed => "已完成",
            Error => "錯誤",
            Warning => "警告",
            Success => "成功",

            // ====== CRUD操作 ======
            CommandAdded => "成功新增命令",
            CommandRemoved => "成功刪除命令",
            CommandUpdated => "成功更新命令",

            // ====== 驗證 ======
            Validating => "正在驗證配置",
            ConfigValid => "配置有效",
            ValidationFailed => "驗證失敗",
            ValidatingConfiguration => "正在驗證配置...",
            ConfigurationIsValid => "配置有效",

            // ====== 互動提示 ======
            PromptCommandId => "命令ID",
            PromptCommand => "命令",
            PromptDescription => "描述",
            PromptCategory => "分類",
            PromptTags => "標籤（逗號分隔）",
            PromptConfirm => "您確定嗎？",
            PromptSelectCommand => "選擇要編輯的命令",
            PromptWhatToDo => "您想做什麼？",
            PromptEnterNumber => "輸入數字",
            PromptSelectTemplate => "選擇範本",
            PromptSelectLanguage => "選擇偏好語言",

            // ====== 選項 ======
            OptionYesAdd => "是，新增此命令",
            OptionNoEdit => "否，重新編輯",
            OptionCancel => "取消",
            OptionEnglish => "English (英語)",
            OptionJapanese => "日本語 (日語)",

            // ====== 預覽·標籤 ======
            LabelPreview => "預覽",
            LabelId => "ID",
            LabelCommand => "命令",
            LabelDescription => "描述",
            LabelCategory => "分類",
            LabelTags => "標籤",
            LabelCurrentSettings => "目前設定",
            LabelDependencies => "相依性",
            LabelPlatforms => "平台",
            LabelWorkingDirectory => "工作目錄",
            LabelEnvironmentVariables => "環境變數",
            LabelExecutionSettings => "執行設定",
            LabelParallel => "並行執行",
            LabelConfirm => "執行前確認",
            LabelTimeout => "逾時",
            LabelCommandDetails => "命令詳情",
            LabelConfiguration => "配置",
            LabelLanguage => "語言",
            LabelShell => "shell",
            LabelStrictMode => "嚴格模式",
            LabelBackupCreated => "已建立備份",
            LabelYes => "是",
            LabelNo => "否",

            // ====== 警告訊息 ======
            WarningShellBuiltinNoEffect => "⚠ 此shell內建命令在子處理序中執行，不會影響目前shell",
            HintShellFunction => "💡 提示：使用shell函式進行目錄導覽",
            HintCdCommand => "   新增至 ~/.cmdrun/shell-functions.sh：",

            // ====== 錯誤訊息 ======
            ErrorEmptyCommandId => "命令ID不能為空",
            ErrorEmptyCommand => "命令不能為空",
            ErrorEmptyDescription => "描述不能為空",
            ErrorCommandNotFound => "找不到命令",
            ErrorCommandExists => "命令已存在",
            ErrorConfigNotFound => "找不到配置檔案",
            ErrorInvalidConfig => "無效配置",
            ErrorInvalidSelection => "無效選擇",
            ErrorSelectionOutOfRange => "選擇超出範圍",
            ErrorNoCommandsAvailable => "沒有可用命令",
            ErrorUnknownTemplate => "未知範本",
            ErrorFileAlreadyExists => "配置檔案已存在",
            ErrorCircularDependency => "偵測到循環相依",
            ErrorValidationFailed => "驗證失敗",
            ErrorCommandFailed => "命令執行失敗",
            ErrorUnknownConfigKey => "未知配置鍵",
            ErrorAliasTargetNotFound => "找不到別名目標",

            // ====== 說明文字 ======
            HelpAddCommand => "向配置中新增命令",
            HelpRemoveCommand => "從配置中刪除命令",
            HelpEditCommand => "編輯現有命令",
            HelpListCommands => "列出所有可用命令",
            HelpRunCommand => "執行命令",
            HelpValidateConfig => "驗證配置檔案",
            HelpSearchCommand => "按關鍵字搜尋命令",
            HelpInfoCommand => "顯示命令詳細資訊",
            HelpConfigCommand => "管理配置設定",
            HelpWatchCommand => "監視檔案並在變更時執行命令",
            HelpInitCommand => "初始化新配置檔案",

            // ====== List 命令 ======
            ListNoCommandsDefined => "未定義命令",
            ListAvailableCommands => "可用命令",
            ListCommandCount => "個已定義命令",
            ListAliasCount => "個已定義別名",

            // ====== Run 命令 ======
            RunRunningCommand => "執行中",
            RunWithParallelDependencies => "（含並行相依）",
            RunExecutionPlan => "執行計畫",
            RunGroup => "群組",
            RunAllCommandsCompleted => "所有命令已完成",
            RunCompletedIn => "完成時間",
            RunCommandFailedWithCode => "命令執行失敗，結束代碼",

            // ====== Search 命令 ======
            SearchSearchingFor => "搜尋中",
            SearchNoCommandsMatching => "沒有符合的命令",
            SearchFound => "找到",
            SearchMatchedIn => "符合位置",
            SearchUseInfoToSeeDetails => "使用 cmdrun info <命令> 檢視詳情",

            // ====== Info 命令 ======
            InfoSelectCommandToView => "選擇要檢視詳情的命令",
            InfoBasicInformation => "基本資訊",
            InfoCommandSpecification => "命令規範",
            InfoExecutionSettings => "執行設定",
            InfoPlatformSupport => "平台支援",
            InfoConfigurationPaths => "配置檔案路徑",
            InfoGlobalConfigPath => "全域配置",
            InfoLocalConfigPath => "本機配置",
            InfoActualWorkingDirectory => "實際工作目錄",
            InfoExecutionStatistics => "執行統計",
            InfoTotalExecutions => "總執行次數",
            InfoSuccessfulRuns => "成功次數",
            InfoFailedRuns => "失敗次數",
            InfoLastRun => "最後執行",
            InfoAverageDuration => "平均執行時間",

            // ====== Config 命令 ======
            ConfigSet => "已設定",
            ConfigShowingConfiguration => "顯示配置",

            // ====== Validate 命令 ======
            ValidateLoadedConfigFrom => "已載入配置檔案",
            ValidateCheckingCircularDependencies => "正在檢查循環相依...",
            ValidateNoCircularDependenciesFor => "無循環相依",
            ValidateValidatingCommands => "正在驗證命令",
            ValidateValidatingAliases => "正在驗證別名",
            ValidateBuildingDependencyGraph => "正在建立相依性圖...",
            ValidateDependencyGraphBuilt => "相依性圖建立成功",
            ValidateExecutionOrder => "執行順序",
            ValidateErrors => "錯誤",
            ValidateWarnings => "警告",
            ValidateInformation => "資訊",
            ValidateFailedWithErrors => "配置驗證失敗，錯誤數",
            ValidateCommandsDefined => "個已定義命令",
            ValidateAliasesDefined => "個已定義別名",

            // ====== Init 命令 ======
            InitCreated => "已建立",
            InitUsing => "使用中",
            InitNextSteps => "下一步",
            InitStep1EditFile => "編輯 {0} 來定義您的命令",
            InitStep2ListCommands => "執行 cmdrun list 列出可用命令",
            InitStep3RunCommand => "執行 cmdrun run <名稱> 執行命令",
            InitExampleCommands => "範例命令",
            InitTemplateDescription => "範本",
            InitLanguageSet => "語言已設定為",

            // ====== Watch 命令 ======
            WatchConfiguration => "監視配置",
            WatchCommand => "命令",
            WatchWatching => "監視中",
            WatchPatterns => "模式",
            WatchExclude => "排除",
            WatchDebounce => "防抖",
            WatchModeStarted => "監視模式已啟動。按 Ctrl+C 停止。",
            WatchPresCtrlCToStop => "按 Ctrl+C 停止",
            WatchModeStoppedByUser => "使用者已停止監視模式",

            // ====== Remove 命令 ======
            RemoveRemovalTarget => "刪除目標",
            RemoveType => "類型",
            RemovePlatformSpecific => "平台特定",

            // ====== Edit 命令 ======
            EditParallelExecution => "並行執行",
            EditConfirmBeforeExecution => "執行前確認",

            // ====== Graph 命令 ======
            GraphSavedTo => "圖表已儲存至",
            GraphRenderWith => "算繪工具",
            GraphViewAt => "檢視位置",

            // ====== Env 命令 ======
            EnvCurrent => "目前環境",
            EnvAvailableEnvironments => "可用環境",
            EnvSwitchedTo => "已切換至環境",
            EnvCreated => "已建立環境",
            EnvVariableSet => "已設定變數",
            EnvEnvironment => "環境",
            EnvDescription => "描述",
            EnvConfigFile => "配置檔案",
            EnvEnvironmentVariables => "環境變數",
            EnvErrorNotFound => "找不到環境",
            EnvErrorAlreadyExists => "環境已存在",
            EnvErrorCannotSetDefault => "無法為'default'環境設定變數",

            // ====== Typo檢測 ======
            TypoUnknownCommand => "未知命令",
            TypoDidYouMean => "您是否想輸入:",
            TypoSuggestions => "建議",
            TypoRunHelp => "執行 'cmdrun --help' 檢視可用命令",

            // ====== 其他 ======
            AddingCommand => "正在新增命令",
            RemovingCommand => "正在刪除命令",
            UpdatingCommand => "正在更新命令",
            OpeningEditor => "正在開啟編輯器",
            SearchResults => "搜尋結果",
            NoCommandsFound => "找不到命令",
            Cancelled => "已取消",
            LoadingConfiguration => "正在載入配置",
            CreatingBackup => "正在建立備份",
            MatchingCommands => "個符合命令",
            Template => "範本",
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
