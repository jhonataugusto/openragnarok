mod colors;
mod stack;
pub mod symbols;
mod timestamp;
#[macro_use]
mod print;
mod timer;

pub use self::colors::{Colorize, Colorized};
pub use self::print::{print_debug, print_indented};
pub use self::timer::Timer;
pub use self::timestamp::{format_log_timestamp, log_timestamp};

#[cfg(test)]
mod tests {
    #[test]
    fn formats_log_timestamp_with_short_date_and_time() {
        let timestamp = chrono::NaiveDate::from_ymd_opt(2026, 4, 28)
            .unwrap()
            .and_hms_opt(14, 33, 51)
            .unwrap()
            .and_local_timezone(chrono::Local)
            .unwrap();

        assert_eq!(super::format_log_timestamp(timestamp), "28/04/26 14:33:51");
    }
}
