use std::time::Duration;

use codex_protocol::error::CodexErr;
use codex_protocol::error::CodexErrorDetails;
use codex_protocol::protocol::RateLimitReachedType;

pub(crate) const MAX_ACCOUNT_SWITCH_BACKOFF: Duration = Duration::from_secs(15 * 60);
const INITIAL_ACCOUNT_SWITCH_BACKOFF: Duration = Duration::from_secs(30);

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum AccountSwitchResult {
    Switched { from: String, to: String },
    Unavailable { retry_after: Option<Duration> },
    Failed,
}

#[derive(Debug)]
pub(crate) struct AccountSwitchBackoff {
    next_delay: Duration,
}

impl Default for AccountSwitchBackoff {
    fn default() -> Self {
        Self {
            next_delay: INITIAL_ACCOUNT_SWITCH_BACKOFF,
        }
    }
}

impl AccountSwitchBackoff {
    pub(crate) fn next_delay(&mut self, retry_after: Option<Duration>) -> Duration {
        let delay = retry_after
            .unwrap_or(self.next_delay)
            .max(self.next_delay)
            .min(MAX_ACCOUNT_SWITCH_BACKOFF);
        self.next_delay = self
            .next_delay
            .checked_mul(2)
            .unwrap_or(MAX_ACCOUNT_SWITCH_BACKOFF)
            .max(delay)
            .min(MAX_ACCOUNT_SWITCH_BACKOFF);
        delay
    }

    pub(crate) fn reset(&mut self) {
        self.next_delay = INITIAL_ACCOUNT_SWITCH_BACKOFF;
    }
}

pub(crate) fn should_attempt_rate_limit_account_switch(error: &CodexErr) -> bool {
    match error.details() {
        CodexErrorDetails::UsageLimitReached(limit) => {
            !matches!(
                limit.rate_limit_reached_type,
                Some(
                    RateLimitReachedType::WorkspaceOwnerUsageLimitReached
                        | RateLimitReachedType::WorkspaceMemberUsageLimitReached
                )
            ) || limit.resets_at.is_some()
        }
        CodexErrorDetails::RateLimitExceeded(message) | CodexErrorDetails::Stream(message) => {
            usage_limit_message(message)
        }
        CodexErrorDetails::UnexpectedStatus(error) => {
            account_switch_status(error.status) && usage_limit_message(&error.body)
        }
        CodexErrorDetails::RetryLimit(error) => {
            account_switch_status(error.status)
                && error.status != http::StatusCode::TOO_MANY_REQUESTS
        }
        CodexErrorDetails::ResponseStreamFailed(error) => usage_limit_message(&error.to_string()),
        CodexErrorDetails::ConnectionFailed(error) => usage_limit_message(&error.to_string()),
        CodexErrorDetails::QuotaExceeded | CodexErrorDetails::UsageNotIncluded => true,
        _ => false,
    }
}

fn account_switch_status(status: http::StatusCode) -> bool {
    matches!(
        status,
        http::StatusCode::PAYMENT_REQUIRED
            | http::StatusCode::FORBIDDEN
            | http::StatusCode::TOO_MANY_REQUESTS
            | http::StatusCode::UPGRADE_REQUIRED
    )
}

fn usage_limit_message(message: &str) -> bool {
    let message = message.to_ascii_lowercase();
    [
        "usage limit",
        "usage_limit",
        "rate_limit_reached",
        "rate limit exceeded",
        "quota",
        "insufficient_quota",
        "billing",
        "credit",
        "not included",
        "workspace owner usage limit reached",
        "workspace member usage limit reached",
    ]
    .iter()
    .any(|needle| message.contains(needle))
}

pub(crate) fn parse_account_switch_retry_after(output: &str) -> Option<Duration> {
    let value = serde_json::from_str::<serde_json::Value>(output).ok()?;
    let seconds = value
        .get("retry_after_seconds")
        .and_then(serde_json::Value::as_u64)
        .or_else(|| {
            value
                .get("retryAfterSeconds")
                .and_then(serde_json::Value::as_u64)
        })?;
    Some(Duration::from_secs(seconds))
}

pub(crate) fn reports_no_available_account(output: &str) -> bool {
    let output = output.to_ascii_lowercase();
    [
        "no_available_account",
        "no available account",
        "all accounts are exhausted",
        "all accounts are rate-limited",
        "no account available",
        "no accounts are available",
        "no account has a readable weekly limit",
        "no usable account",
        "no eligible account",
    ]
    .iter()
    .any(|needle| output.contains(needle))
}

#[cfg(test)]
#[path = "account_switch_tests.rs"]
mod tests;
