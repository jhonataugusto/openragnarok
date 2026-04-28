const LOG_TIMESTAMP_FORMAT: &str = "%d/%m/%y %H:%M:%S";

pub fn format_log_timestamp(timestamp: chrono::DateTime<chrono::Local>) -> String {
    timestamp.format(LOG_TIMESTAMP_FORMAT).to_string()
}

pub fn log_timestamp() -> String {
    format_log_timestamp(chrono::Local::now())
}
