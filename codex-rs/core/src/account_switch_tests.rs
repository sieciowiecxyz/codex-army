use super::AccountSwitchBackoff;
use super::MAX_ACCOUNT_SWITCH_BACKOFF;
use super::parse_account_switch_retry_after;
use super::reports_no_available_account;
use super::should_attempt_rate_limit_account_switch;
use codex_protocol::error::CodexErr;
use codex_protocol::error::CodexErrorDetails;
use codex_protocol::error::UsageLimitReachedError;
use codex_protocol::protocol::RateLimitReachedType;
use pretty_assertions::assert_eq;
use std::time::Duration;

#[test]
fn backoff_grows_and_caps() {
    let mut backoff = AccountSwitchBackoff::default();

    assert_eq!(backoff.next_delay(None), Duration::from_secs(30));
    assert_eq!(backoff.next_delay(None), Duration::from_secs(60));
    assert_eq!(
        backoff.next_delay(Some(Duration::from_secs(60 * 60))),
        MAX_ACCOUNT_SWITCH_BACKOFF
    );
    assert_eq!(backoff.next_delay(None), MAX_ACCOUNT_SWITCH_BACKOFF);
}

#[test]
fn server_retry_after_is_respected_without_exceeding_cap() {
    let mut backoff = AccountSwitchBackoff::default();

    assert_eq!(
        backoff.next_delay(Some(Duration::from_secs(90))),
        Duration::from_secs(90)
    );
    assert_eq!(
        parse_account_switch_retry_after(r#"{"retry_after_seconds":120}"#),
        Some(Duration::from_secs(120))
    );
}

#[test]
fn no_available_account_output_is_recognized() {
    assert!(reports_no_available_account("status=no_available_account"));
    assert!(reports_no_available_account("All accounts are exhausted"));
    assert!(reports_no_available_account(
        "no account has a readable weekly limit"
    ));
    assert!(!reports_no_available_account("switched to account-b"));
}

#[test]
fn only_usage_limits_trigger_switching() {
    let usage_limit = CodexErr::new(CodexErrorDetails::UsageLimitReached(
        UsageLimitReachedError {
            plan_type: None,
            resets_at: None,
            rate_limits: None,
            promo_message: None,
            rate_limit_reached_type: Some(RateLimitReachedType::RateLimitReached),
        },
    ));
    let workspace_limit = CodexErr::new(CodexErrorDetails::UsageLimitReached(
        UsageLimitReachedError {
            plan_type: None,
            resets_at: None,
            rate_limits: None,
            promo_message: None,
            rate_limit_reached_type: Some(RateLimitReachedType::WorkspaceOwnerUsageLimitReached),
        },
    ));

    assert!(should_attempt_rate_limit_account_switch(&usage_limit));
    assert!(!should_attempt_rate_limit_account_switch(&workspace_limit));
}
