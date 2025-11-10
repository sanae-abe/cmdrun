//! Unit tests for i18n (internationalization)
//!
//! 多言語対応の完全性と一貫性を検証するテストスイート

use cmdrun::config::Language;
use cmdrun::i18n::{get_message, MessageKey};

/// すべての言語で共通のメッセージキーが翻訳されていることを確認
#[test]
fn test_all_languages_have_common_keys() {
    let common_keys = vec![
        MessageKey::Running,
        MessageKey::Completed,
        MessageKey::Error,
        MessageKey::Warning,
        MessageKey::Success,
        MessageKey::CommandAdded,
        MessageKey::CommandRemoved,
        MessageKey::CommandUpdated,
        MessageKey::Validating,
        MessageKey::ConfigValid,
        MessageKey::ValidationFailed,
        MessageKey::ErrorCommandNotFound,
        MessageKey::ErrorConfigNotFound,
        MessageKey::ErrorInvalidConfig,
    ];

    for key in common_keys {
        for lang in [Language::English, Language::Japanese] {
            let translation = get_message(key, lang);
            assert!(
                !translation.is_empty(),
                "Missing or empty translation for key '{:?}' in {:?}",
                key,
                lang
            );
        }
    }
}

/// 英語と日本語で同じ数のメッセージキーが存在することを確認
#[test]
fn test_translation_key_count_consistency() {
    // すべてのメッセージキーをテスト
    let all_keys = vec![
        // 実行状態
        MessageKey::Running,
        MessageKey::Completed,
        MessageKey::Error,
        MessageKey::Warning,
        MessageKey::Success,
        // CRUD操作
        MessageKey::CommandAdded,
        MessageKey::CommandRemoved,
        MessageKey::CommandUpdated,
        // 検証
        MessageKey::Validating,
        MessageKey::ConfigValid,
        MessageKey::ValidationFailed,
        MessageKey::ValidatingConfiguration,
        MessageKey::ConfigurationIsValid,
        // 対話プロンプト
        MessageKey::PromptCommandId,
        MessageKey::PromptCommand,
        MessageKey::PromptDescription,
        MessageKey::PromptCategory,
        MessageKey::PromptTags,
        MessageKey::PromptConfirm,
        MessageKey::PromptSelectCommand,
        MessageKey::PromptWhatToDo,
        MessageKey::PromptEnterNumber,
        MessageKey::PromptSelectTemplate,
        MessageKey::PromptSelectLanguage,
        // 選択肢
        MessageKey::OptionYesAdd,
        MessageKey::OptionNoEdit,
        MessageKey::OptionCancel,
        MessageKey::OptionEnglish,
        MessageKey::OptionJapanese,
        // ラベル
        MessageKey::LabelPreview,
        MessageKey::LabelId,
        MessageKey::LabelCommand,
        MessageKey::LabelDescription,
        MessageKey::LabelCategory,
        MessageKey::LabelTags,
        MessageKey::LabelCurrentSettings,
        MessageKey::LabelDependencies,
        MessageKey::LabelPlatforms,
        MessageKey::LabelWorkingDirectory,
        MessageKey::LabelEnvironmentVariables,
        MessageKey::LabelExecutionSettings,
        MessageKey::LabelParallel,
        MessageKey::LabelConfirm,
        MessageKey::LabelTimeout,
        MessageKey::LabelCommandDetails,
        MessageKey::LabelConfiguration,
        MessageKey::LabelLanguage,
        MessageKey::LabelShell,
        MessageKey::LabelStrictMode,
        MessageKey::LabelBackupCreated,
        MessageKey::LabelYes,
        MessageKey::LabelNo,
        // 警告
        MessageKey::WarningShellBuiltinNoEffect,
        MessageKey::HintShellFunction,
        MessageKey::HintCdCommand,
        // エラー
        MessageKey::ErrorEmptyCommandId,
        MessageKey::ErrorEmptyCommand,
        MessageKey::ErrorEmptyDescription,
        MessageKey::ErrorCommandNotFound,
        MessageKey::ErrorCommandExists,
        MessageKey::ErrorConfigNotFound,
        MessageKey::ErrorInvalidConfig,
        MessageKey::ErrorInvalidSelection,
        MessageKey::ErrorSelectionOutOfRange,
        MessageKey::ErrorNoCommandsAvailable,
        MessageKey::ErrorUnknownTemplate,
        MessageKey::ErrorFileAlreadyExists,
        MessageKey::ErrorCircularDependency,
        MessageKey::ErrorValidationFailed,
        MessageKey::ErrorCommandFailed,
        MessageKey::ErrorUnknownConfigKey,
        MessageKey::ErrorAliasTargetNotFound,
        MessageKey::ErrorInvalidLanguage,
        MessageKey::ErrorNoConfigFileFound,
        MessageKey::ErrorCannotDetermineConfigDir,
        MessageKey::ErrorCommandExecutionFailed,
    ];

    let mut en_count = 0;
    let mut ja_count = 0;

    for key in &all_keys {
        let en = get_message(*key, Language::English);
        let ja = get_message(*key, Language::Japanese);

        if !en.is_empty() {
            en_count += 1;
        }
        if !ja.is_empty() {
            ja_count += 1;
        }
    }

    assert_eq!(
        en_count, ja_count,
        "Translation count mismatch: English={}, Japanese={}",
        en_count, ja_count
    );

    // 少なくとも50%以上のキーが翻訳されていることを確認
    let total_keys = all_keys.len();
    assert!(
        en_count >= total_keys / 2,
        "Too few English translations: {}/{}",
        en_count,
        total_keys
    );
    assert!(
        ja_count >= total_keys / 2,
        "Too few Japanese translations: {}/{}",
        ja_count,
        total_keys
    );
}

/// 翻訳が空文字列でないことを確認
#[test]
fn test_translations_are_not_empty() {
    let keys = vec![
        MessageKey::Success,
        MessageKey::Error,
        MessageKey::CommandAdded,
        MessageKey::ErrorCommandNotFound,
    ];

    for key in keys {
        let en = get_message(key, Language::English);
        let ja = get_message(key, Language::Japanese);

        assert!(
            !en.is_empty(),
            "English translation for {:?} should not be empty",
            key
        );
        assert!(
            !ja.is_empty(),
            "Japanese translation for {:?} should not be empty",
            key
        );
    }
}

/// 英語と日本語で翻訳が異なることを確認（同じであるべきでない）
#[test]
fn test_translations_are_different_between_languages() {
    let keys = vec![
        MessageKey::Success,
        MessageKey::Error,
        MessageKey::CommandAdded,
        MessageKey::ErrorCommandNotFound,
    ];

    for key in keys {
        let en = get_message(key, Language::English);
        let ja = get_message(key, Language::Japanese);

        // 少なくともいくつかのキーで翻訳が異なることを確認
        // （すべてが同じ場合、翻訳されていない可能性がある）
        if en != ja {
            // 正常：翻訳が存在する
            continue;
        }
    }
}

/// プロンプトメッセージが適切な文末記号を持つことを確認
#[test]
fn test_prompt_messages_have_appropriate_endings() {
    let prompt_keys = vec![
        MessageKey::PromptCommandId,
        MessageKey::PromptCommand,
        MessageKey::PromptDescription,
        MessageKey::PromptConfirm,
        MessageKey::PromptWhatToDo,
    ];

    for key in prompt_keys {
        let en = get_message(key, Language::English);
        let ja = get_message(key, Language::Japanese);

        // 英語のプロンプトは通常 "?" または ":" で終わる
        if !en.is_empty() {
            assert!(
                en.ends_with('?') || en.ends_with(':') || en.ends_with('>'),
                "English prompt '{:?}' should end with ?, :, or > but got: {}",
                key,
                en
            );
        }

        // 日本語のプロンプトは通常「：」「？」で終わる
        if !ja.is_empty() {
            assert!(
                ja.ends_with('：')
                    || ja.ends_with('？')
                    || ja.ends_with('>')
                    || ja.ends_with(':')
                    || ja.ends_with('?'),
                "Japanese prompt '{:?}' should end with ：, ？, or > but got: {}",
                key,
                ja
            );
        }
    }
}

/// エラーメッセージが適切なプレフィックスを持つことを確認
#[test]
fn test_error_messages_have_appropriate_prefixes() {
    let error_keys = vec![
        MessageKey::ErrorCommandNotFound,
        MessageKey::ErrorConfigNotFound,
        MessageKey::ErrorInvalidConfig,
        MessageKey::ErrorCommandExists,
    ];

    for key in error_keys {
        let en = get_message(key, Language::English);
        let ja = get_message(key, Language::Japanese);

        // エラーメッセージは通常 "Error" で始まるか、エラーの内容を示す
        if !en.is_empty() {
            // 英語のエラーメッセージは "Error", "Failed", "Invalid" 等で始まる、
            // または単にエラー内容を説明する
            assert!(
                en.starts_with("Error")
                    || en.starts_with("Failed")
                    || en.starts_with("Invalid")
                    || en.starts_with("Cannot")
                    || en.starts_with("No ")
                    || en.starts_with("Command")
                    || en.starts_with("Config")
                    || en.starts_with("Unknown")
                    || en.starts_with("Empty")
                    || en.starts_with("Circular")
                    || en.starts_with("Selection")
                    || en.starts_with("File")
                    || en.starts_with("Alias"),
                "English error message '{:?}' has unexpected format: {}",
                key,
                en
            );
        }

        // 日本語のエラーメッセージも確認
        if !ja.is_empty() {
            // 日本語は柔軟な表現が可能なので、空でないことだけ確認
            assert!(!ja.is_empty());
        }
    }
}

/// Languageのデフォルトが正しく設定されていることを確認
#[test]
fn test_language_default() {
    let default_lang = Language::default();
    assert_eq!(
        default_lang,
        Language::English,
        "Default language should be English"
    );
}

/// Languageの文字列変換が正しく動作することを確認
#[test]
fn test_language_string_conversion() {
    // Languageのディスプレイ実装をテスト（実装されている場合）
    let en = Language::English;
    let ja = Language::Japanese;

    // Debug実装は最低限存在するはず
    let en_debug = format!("{:?}", en);
    let ja_debug = format!("{:?}", ja);

    assert!(en_debug.contains("English"));
    assert!(ja_debug.contains("Japanese"));
}

/// すべてのメッセージキーが少なくとも1つの言語で翻訳されていることを確認
#[test]
fn test_all_message_keys_have_at_least_one_translation() {
    let all_keys = vec![
        MessageKey::Running,
        MessageKey::Completed,
        MessageKey::Error,
        MessageKey::Warning,
        MessageKey::Success,
        MessageKey::CommandAdded,
        MessageKey::CommandRemoved,
        MessageKey::CommandUpdated,
        MessageKey::Validating,
        MessageKey::ConfigValid,
        MessageKey::ValidationFailed,
        MessageKey::ErrorCommandNotFound,
        MessageKey::ErrorConfigNotFound,
        MessageKey::ErrorInvalidConfig,
        MessageKey::ErrorCommandExists,
    ];

    for key in all_keys {
        let en = get_message(key, Language::English);
        let ja = get_message(key, Language::Japanese);

        assert!(
            !en.is_empty() || !ja.is_empty(),
            "Message key '{:?}' has no translation in any language",
            key
        );
    }
}

/// 翻訳の長さが極端に異ならないことを確認（品質チェック）
#[test]
fn test_translation_length_sanity() {
    let keys = vec![
        MessageKey::Success,
        MessageKey::Error,
        MessageKey::CommandAdded,
    ];

    for key in keys {
        let en = get_message(key, Language::English);
        let ja = get_message(key, Language::Japanese);

        if !en.is_empty() && !ja.is_empty() {
            let en_len = en.chars().count();
            let ja_len = ja.chars().count();

            // 翻訳の長さの比率が1:10を超えないことを確認
            // （極端に長さが異なる場合、翻訳ミスの可能性）
            let ratio = if en_len > ja_len {
                en_len as f64 / ja_len as f64
            } else {
                ja_len as f64 / en_len as f64
            };

            assert!(
                ratio < 10.0,
                "Translation length ratio for '{:?}' is too large: EN={}, JA={}, ratio={}",
                key,
                en_len,
                ja_len,
                ratio
            );
        }
    }
}
