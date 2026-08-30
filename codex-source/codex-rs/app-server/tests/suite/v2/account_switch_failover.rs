use anyhow::Result;
use app_test_support::ChatGptAuthFixture;
use app_test_support::MockResponsesConfig;
use app_test_support::TestAppServer;
use app_test_support::create_mock_responses_server_sequence_unchecked;
use app_test_support::write_chatgpt_auth;
use codex_app_server_protocol::ThreadGoalSetResponse;
use codex_app_server_protocol::ThreadStartParams;
use codex_app_server_protocol::TurnStartParams;
use codex_app_server_protocol::UserInput;
use codex_app_server_protocol::WarningNotification;
use codex_config::types::AuthCredentialsStoreMode;
use codex_features::Feature;
use core_test_support::responses;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::timeout;

const READ_TIMEOUT: Duration = Duration::from_secs(10);

fn shell_quote(path: &Path) -> String {
    format!("'{}'", path.to_string_lossy().replace('\'', "'\\''"))
}

fn install_mock_codex_accounts(home: &Path) -> Result<PathBuf> {
    let bin_dir = home.join("mock-bin");
    std::fs::create_dir_all(&bin_dir)?;
    let command = bin_dir.join("codex-accounts");
    std::fs::write(
        &command,
        format!(
            "#!/bin/sh\ncp {source} {destination}\nprintf 'switched\\n'\n",
            source = shell_quote(&home.join("account-b/auth.json")),
            destination = shell_quote(&home.join("auth.json")),
        ),
    )?;
    std::fs::set_permissions(&command, std::fs::Permissions::from_mode(0o755))?;
    Ok(command)
}

#[tokio::test]
async fn goal_continuation_switches_accounts_and_keeps_running() -> Result<()> {
    let server = create_mock_responses_server_sequence_unchecked(vec![
        responses::sse(vec![
            responses::ev_response_created("materialize-thread"),
            responses::ev_completed("materialize-thread"),
        ]),
        responses::sse(vec![
            responses::ev_response_created("limited-response"),
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
        responses::sse(vec![
            responses::ev_response_created("continued-response"),
            responses::ev_completed_with_tokens("continued-response", 200),
        ]),
    ])
    .await;

    let home = TempDir::new()?;
    MockResponsesConfig::new(&server.uri())
        .enable_feature(Feature::Goals)
        .write(home.path())?;
    write_chatgpt_auth(
        home.path(),
        ChatGptAuthFixture::new("account-a-token").account_id("account-a"),
        AuthCredentialsStoreMode::File,
    )?;
    write_chatgpt_auth(
        &home.path().join("account-b"),
        ChatGptAuthFixture::new("account-b-token").account_id("account-b"),
        AuthCredentialsStoreMode::File,
    )?;
    let command = install_mock_codex_accounts(home.path())?;
    let inherited_path = std::env::var_os("PATH").unwrap_or_default();
    let path = format!(
        "{}:{}",
        command
            .parent()
            .expect("mock command should have a parent")
            .to_string_lossy(),
        inherited_path.to_string_lossy()
    );

    let mut app = TestAppServer::builder()
        .with_codex_home(home.path())
        .with_env_overrides(&[("PATH", Some(path.as_str()))])
        .build_initialized()
        .await?;
    let thread = app.start_thread(ThreadStartParams::default()).await?.thread;

    app.start_turn_and_wait_for_completion(TurnStartParams {
        thread_id: thread.id.clone(),
        input: vec![UserInput::Text {
            text: "materialize this thread".to_string(),
            text_elements: Vec::new(),
        }],
        ..Default::default()
    })
    .await?;

    let goal_request = app
        .send_raw_request(
            "thread/goal/set",
            Some(json!({
                "threadId": thread.id,
                "objective": "finish after the account switch",
                "tokenBudget": 100,
            })),
        )
        .await?;
    let _: ThreadGoalSetResponse = timeout(READ_TIMEOUT, app.read_response(goal_request)).await??;
    timeout(
        READ_TIMEOUT,
        app.read_stream_until_notification_message("thread/goal/updated"),
    )
    .await??;

    let warning = app
        .read_stream_until_matching_notification("account switch warning", |notification| {
            notification.method == "warning"
                && notification.params.as_ref().is_some_and(|params| {
                    params["message"]
                        .as_str()
                        .is_some_and(|message| message.starts_with("Changing account:"))
                })
        })
        .await?;
    let warning: WarningNotification = serde_json::from_value(
        warning
            .params
            .expect("account switch warning should include parameters"),
    )?;
    assert_eq!(
        warning.message,
        "Changing account: account-a → account-b. Continuing previous task…"
    );

    timeout(
        READ_TIMEOUT,
        app.read_stream_until_notification_message("turn/completed"),
    )
    .await??;

    let requests = server
        .received_requests()
        .await
        .expect("mock server should record requests");
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.url.path().ends_with("/responses"))
            .count(),
        3
    );
    Ok(())
}
