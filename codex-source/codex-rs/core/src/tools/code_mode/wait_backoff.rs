use std::time::Duration;
use std::time::Instant;

const MIN_BACKOFF_MS: u64 = 1_000;
pub(super) const MAX_BACKOFF_MS: u64 = 300_000;
pub(super) const MAX_WAIT_RUNTIME: Duration = Duration::from_secs(6 * 60 * 60);

/// Tracks the next observation interval for one code-mode cell.
pub(super) struct WaitBackoff {
    started_at: Instant,
    next_yield_time_ms: Option<u64>,
}

impl WaitBackoff {
    pub(super) fn new(started_at: Instant) -> Self {
        Self {
            started_at,
            next_yield_time_ms: None,
        }
    }

    pub(super) fn next_yield_time_ms(&self, requested_yield_time_ms: u64) -> Option<u64> {
        let elapsed = self.started_at.elapsed();
        let remaining = MAX_WAIT_RUNTIME.checked_sub(elapsed)?;
        let requested_yield_time_ms = requested_yield_time_ms.min(MAX_BACKOFF_MS);
        let scheduled_yield_time_ms = self
            .next_yield_time_ms
            .map_or(requested_yield_time_ms, |scheduled| {
                scheduled.max(requested_yield_time_ms)
            });
        let remaining_ms = remaining.as_millis().max(1).min(u128::from(u64::MAX)) as u64;
        Some(scheduled_yield_time_ms.min(remaining_ms))
    }

    pub(super) fn record_yield(&mut self, observed_yield_time_ms: u64) {
        self.next_yield_time_ms = Some(
            observed_yield_time_ms
                .saturating_mul(2)
                .clamp(MIN_BACKOFF_MS, MAX_BACKOFF_MS),
        );
    }
}

#[cfg(test)]
#[path = "wait_backoff_tests.rs"]
mod tests;
