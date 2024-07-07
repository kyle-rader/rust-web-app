use std::time::{Duration, SystemTime};

pub fn assert_within(actual: SystemTime, expected: SystemTime, tolerance: Duration) {
    let diff = actual
        .duration_since(expected)
        .unwrap_or_else(|_| Duration::from_secs(0));
    assert!(diff <= tolerance, "Expected time difference between {actual:?} and {expected:?} to be within {tolerance:?}, but was {diff:?}");
}
