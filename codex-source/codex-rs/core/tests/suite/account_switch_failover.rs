use anyhow::Result;
use codex_core::ModelClient;
use codex_core::TurnInputRequest;
use codex_login::CodexAuth;
use codex_protocol::protocol::EventMsg;
use core_test_support::responses::ev_assistant_message;
use core_test_support::responses::ev_response_created;
use core_test_support::responses::mount_sse_sequence;
use core_test_support::responses::sse;
use core_test_support::responses::start_mock_server;
use core_test_support::skip_if_no_network;
use core_test_support::test_codex::test_codex;
use core_test_support::wait_for_event;
use pretty_assertions::assert_eq;
use serde_json::json;
use serial_test::serial;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

struct AccountSwitchCommandGuard;

impl Drop for AccountSwitchCommandGuard {
    fn drop(&mut self) {
        ModelClient::set_account_switch_command_for_tests(None);
    }
}

fn shell_quote(path: &Path) -> String {
    format!("'{}'", path.to_string_lossy().replace('\'', "'\\''"))
}

fn write_chatgpt_auth(home: &Path, account_id: &str) -> Result<()> {
    std::fs::write(
        home.join("account-b.json"),
        serde_json::to_string_pretty(&json!({
            "auth_mode": "chatgpt",
            "tokens": {
                "id_token": "a.e30.a",
                "access_token": "dummy-access-token",
                "refresh_token": "dummy-refresh-token",
                "account_id": account_id,
            }
        }))?,
    )?;
    Ok(())
}

fn install_mock_codex_accounts(home: &Path) -> Result<PathBuf> {
    let bin_dir = home.join("mock-bin");
    std::fs::create_dir_all(&bin_dir)?;
    let command = bin_dir.join("codex-accounts");
    std::fs::write(
        &command,
        format!(
            "#!/bin/sh\ntouch {invoked}\ncp {source} {destination}\nprintf 'switched\\n'\n",
            invoked = shell_quote(&home.join("switch-invoked")),
            source = shell_quote(&home.join("account-b.json")),
            destination = shell_quote(&home.join("auth.json")),
        ),
    )?;
    std::fs::set_permissions(&command, std::fs::Permissions::from_mode(0o755))?;
    Ok(command)
}

async fn run_turn_with_mock_switch(auth: CodexAuth, expected_from: &str) -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let home = Arc::new(TempDir::new()?);
    write_chatgpt_auth(home.path(), "account-b")?;
    let command = install_mock_codex_accounts(home.path())?;
    ModelClient::set_account_switch_command_for_tests(Some(command.clone().into_os_string()));
    let _command_guard = AccountSwitchCommandGuard;

    let response_mock = mount_sse_sequence(
        &server,
        vec![
            sse(vec![
                ev_response_created("limited-response"),
                json!({
                    "type": "response.failed",
                    "response": {
                        "id": "limited-response",
                        "error": {
                            "code": "insufficient_quota",
                            "message": "You exceeded your current quota."
                        }
                    }
                }),
            ]),
            sse(vec![
                ev_response_created("continued-response"),
                ev_assistant_message("continued-message", "continued"),
                json!({
                    "type": "response.completed",
                    "response": {"id": "continued-response"}
                }),
            ]),
        ],
    )
    .await;

    let test = test_codex()
        .with_home(home)
        .with_auth(auth)
        .build(&server)
        .await?;
    test.codex
        .start_or_steer_turn(TurnInputRequest::user_input(vec![
            codex_protocol::user_input::UserInput::Text {
                text: "continue after quota".to_string(),
                text_elements: Vec::new(),
            },
        ]))
        .await?;

    let mut warning = None;
    let mut error = None;
    loop {
        match wait_for_event(&test.codex, |_| true).await {
            EventMsg::Warning(event) => warning = Some(event.message),
            EventMsg::Error(event) => error = Some(event),
            EventMsg::TurnComplete(_) => break,
            _ => {}
        }
    }

    assert!(
        test.codex_home_path().join("switch-invoked").exists(),
        "mock codex-accounts was not invoked; error: {error:?}"
    );
    assert_eq!(
        response_mock.requests().len(),
        2,
        "account switch did not retry"
    );
    assert!(
        error.is_none(),
        "account switch test emitted an error: {error:?}"
    );
    let expected_warning =
        format!("Changing account: {expected_from} → account-b. Continuing previous task…");
    assert_eq!(warning.as_deref(), Some(expected_warning.as_str()));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[serial(account_switch_command)]
async fn ordinary_conversation_switches_from_one_dummy_account_to_another() -> Result<()> {
    run_turn_with_mock_switch(
        CodexAuth::create_dummy_chatgpt_auth_for_testing(),
        "account_id",
    )
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[serial(account_switch_command)]
async fn ordinary_conversation_switches_when_no_account_is_connected() -> Result<()> {
    run_turn_with_mock_switch(CodexAuth::from_api_key("dummy-api-key"), "not-connected").await
}
