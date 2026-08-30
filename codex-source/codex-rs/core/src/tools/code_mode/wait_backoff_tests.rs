use super::MAX_BACKOFF_MS;
use super::MAX_WAIT_RUNTIME;
use super::WaitBackoff;
use pretty_assertions::assert_eq;
use std::time::Duration;
use std::time::Instant;

#[test]
fn wait_backoff_doubles_until_the_five_minute_ceiling() {
    let started_at = Instant::now();
    let mut backoff = WaitBackoff::new(started_at);
    let mut observed = Vec::new();

    for _ in 0..10 {
        let next = backoff.next_yield_time_ms(1_000).unwrap();
        observed.push(next);
        backoff.record_yield(next);
    }

    assert_eq!(
        observed,
        vec![
            1_000,
            2_000,
            4_000,
            8_000,
            16_000,
            32_000,
            64_000,
            128_000,
            256_000,
            MAX_BACKOFF_MS,
        ]
    );
}

#[test]
fn explicit_longer_wait_can_extend_the_schedule() {
    let mut backoff = WaitBackoff::new(Instant::now());
    backoff.record_yield(1_000);

    assert_eq!(backoff.next_yield_time_ms(60_000), Some(60_000));
}

#[test]
fn wait_backoff_stops_at_the_absolute_runtime_limit() {
    let started_at = Instant::now() - MAX_WAIT_RUNTIME;
    let backoff = WaitBackoff::new(started_at);

    assert_eq!(backoff.next_yield_time_ms(1_000), None);
}

#[test]
fn zero_wait_is_preserved_once_then_uses_a_safe_minimum() {
    let mut backoff = WaitBackoff::new(Instant::now());

    assert_eq!(backoff.next_yield_time_ms(0), Some(0));
    backoff.record_yield(0);
    assert_eq!(backoff.next_yield_time_ms(0), Some(1_000));
}

#[test]
fn remaining_runtime_caps_the_next_wait() {
    let started_at = Instant::now() - MAX_WAIT_RUNTIME + Duration::from_millis(50);
    let backoff = WaitBackoff::new(started_at);

    let remaining = backoff.next_yield_time_ms(300_000).unwrap();
    assert!(remaining > 0 && remaining <= 50);
}
