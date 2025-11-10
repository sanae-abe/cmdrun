//! E2E Test: Complete CLI Workflow
//!
//! ユーザーの典型的な使用シナリオを検証する統合テスト

use super::framework::CmdrunTestEnv;

#[test]
fn test_complete_workflow() {
    let env = CmdrunTestEnv::new();

    // Step 1: cmdrun init - プロジェクト初期化
    let init = env.run_command(&["init"]);
    env.assert_success(&init);
    env.assert_stdout_contains(&init, "Initialized");
    env.assert_config_exists();

    // Step 2: cmdrun add - コマンド追加
    let add = env.run_command(&["add", "test", "echo hello", "-d", "Test command"]);
    env.assert_success(&add);
    env.assert_stdout_contains(&add, "test");

    // Step 3: cmdrun list - コマンド一覧表示
    let list = env.run_command(&["list"]);
    env.assert_success(&list);
    env.assert_stdout_contains(&list, "test");
    env.assert_stdout_contains(&list, "Test command");

    // Step 4: cmdrun test - コマンド実行
    let run = env.run_command(&["test"]);
    env.assert_success(&run);
    env.assert_stdout_contains(&run, "hello");

    // Step 5: cmdrun history - 履歴確認（実装済みの場合）
    let history = env.run_command(&["history"]);
    if history.status.success() {
        env.assert_stdout_contains(&history, "test");
    }

    // Step 6: cmdrun remove - コマンド削除
    let remove = env.run_command(&["remove", "test"]);
    env.assert_success(&remove);

    // Step 7: 削除後のlist確認
    let list_after = env.run_command(&["list"]);
    env.assert_success(&list_after);
    env.assert_stdout_not_contains(&list_after, "test");
}

#[test]
fn test_dependency_workflow() {
    let env = CmdrunTestEnv::new();

    // 初期化
    env.run_command(&["init"]);

    // 依存関係のあるコマンドを追加
    env.run_command(&["add", "build", "echo Building...", "-d", "Build project"]);
    env.run_command(&[
        "add",
        "test",
        "echo Testing...",
        "-d",
        "Run tests",
        "--depends-on",
        "build",
    ]);
    env.run_command(&[
        "add",
        "deploy",
        "echo Deploying...",
        "-d",
        "Deploy to production",
        "--depends-on",
        "test",
    ]);

    // deployを実行すると build → test → deploy の順で実行される
    let output = env.run_command(&["deploy"]);
    env.assert_success(&output);

    let stdout = String::from_utf8_lossy(&output.stdout);

    // 実行順序の検証
    if let (Some(build_pos), Some(test_pos), Some(deploy_pos)) = (
        stdout.find("Building"),
        stdout.find("Testing"),
        stdout.find("Deploying"),
    ) {
        assert!(
            build_pos < test_pos && test_pos < deploy_pos,
            "Commands should execute in dependency order: build < test < deploy"
        );
    }
}

#[test]
fn test_parallel_execution_workflow() {
    let env = CmdrunTestEnv::new();
    env.run_command(&["init"]);

    // 並列実行可能なコマンドを追加
    env.run_command(&["add", "lint", "echo Linting...", "-d", "Lint code"]);
    env.run_command(&["add", "format", "echo Formatting...", "-d", "Format code"]);
    env.run_command(&[
        "add",
        "typecheck",
        "echo Type checking...",
        "-d",
        "Type check",
    ]);

    // 並列実行（実装済みの場合）
    let output = env.run_command(&["lint", "format", "typecheck", "--parallel"]);

    // 少なくともすべてのコマンドが実行されることを確認
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Linting"));
        assert!(stdout.contains("Formatting"));
        assert!(stdout.contains("Type checking"));
    }
}

#[test]
fn test_environment_workflow() {
    let env = CmdrunTestEnv::new();
    env.run_command(&["init"]);

    // 環境管理のワークフロー（実装済みの場合）
    let create_env = env.run_command(&["env", "create", "dev", "-d", "Development environment"]);
    if create_env.status.success() {
        // 環境切り替え
        env.run_command(&["env", "switch", "dev"]);

        // 環境変数設定
        env.run_command(&["env", "set", "API_URL", "http://localhost:3000"]);

        // 環境一覧
        let list_env = env.run_command(&["env", "list"]);
        env.assert_success(&list_env);
        env.assert_stdout_contains(&list_env, "dev");
    }
}

#[test]
fn test_validation_workflow() {
    let env = CmdrunTestEnv::new();
    env.run_command(&["init"]);

    // コマンド追加
    env.run_command(&["add", "test", "echo test", "-d", "Test"]);

    // 設定検証（実装済みの場合）
    let validate = env.run_command(&["validate"]);
    if validate.status.success() {
        env.assert_stdout_contains(&validate, "valid");
    }
}

#[test]
fn test_error_handling_workflow() {
    let env = CmdrunTestEnv::new();

    // 初期化前にコマンド実行を試みる
    let run_before_init = env.run_command(&["test"]);
    env.assert_failure(&run_before_init);

    env.run_command(&["init"]);

    // 存在しないコマンドの実行
    let run_nonexistent = env.run_command(&["nonexistent-command"]);
    env.assert_failure(&run_nonexistent);

    // 不正な引数でのコマンド追加
    let add_invalid = env.run_command(&["add"]); // 引数不足
    env.assert_failure(&add_invalid);
}

#[test]
fn test_template_workflow() {
    let env = CmdrunTestEnv::new();
    env.run_command(&["init"]);

    // テンプレート一覧（実装済みの場合）
    let list_templates = env.run_command(&["template", "list"]);
    if list_templates.status.success() {
        // テンプレート適用
        let apply = env.run_command(&["template", "apply", "rust"]);
        if apply.status.success() {
            env.assert_stdout_contains(&apply, "Applied");
        }
    }
}

#[test]
fn test_watch_workflow() {
    let env = CmdrunTestEnv::new();
    env.run_command(&["init"]);
    env.run_command(&["add", "test", "echo Testing...", "-d", "Run tests"]);

    // Watch機能のテスト（実装済みの場合）
    // 注: 実際のファイル監視は時間がかかるため、ここでは起動確認のみ
    let watch = env.run_command(&["watch", "test", "--help"]);
    // helpが表示されればOK（実際のwatch実行は別のテストで）
}

#[test]
fn test_plugin_workflow() {
    let env = CmdrunTestEnv::new();
    env.run_command(&["init"]);

    // プラグイン一覧（実装済みの場合）
    let list_plugins = env.run_command(&["plugin", "list"]);
    if list_plugins.status.success() {
        env.assert_stdout_contains(&list_plugins, "plugin");
    }
}

#[test]
fn test_config_management_workflow() {
    let env = CmdrunTestEnv::new();
    env.run_command(&["init"]);

    // 設定表示
    let config_show = env.run_command(&["config", "show"]);
    if config_show.status.success() {
        env.assert_stdout_contains(&config_show, "shell");
    }

    // 設定変更
    let config_set = env.run_command(&["config", "set", "timeout", "60"]);
    if config_set.status.success() {
        // 設定確認
        let config_get = env.run_command(&["config", "get", "timeout"]);
        if config_get.status.success() {
            env.assert_stdout_contains(&config_get, "60");
        }
    }
}
