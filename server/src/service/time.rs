use std::time::SystemTime;

#[derive(Debug, thiserror::Error)]
#[error("Failed to get system time! (Time went backwards?)")]
pub struct ErrorTime;

pub fn now_unix() -> Result<u64, ErrorTime> {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| ErrorTime)
        .map(|d| d.as_secs())
}
