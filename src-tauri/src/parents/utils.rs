use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let datetime = chrono::DateTime::<chrono::Local>::from(UNIX_EPOCH + std::time::Duration::from_secs(now));
    datetime.format("%Y%m%d_%H%M%S").to_string()
}
