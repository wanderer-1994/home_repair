//! Error details adapted from google apis error details.
//! Utilizing `prost-build` crate to convert `error_details.proto` to rust code.

use backtrace::Backtrace;

include!(concat!(env!("OUT_DIR"), "/error.rs"));

impl DebugInfo {
    /// Captures a backtrace and populates the debug info.
    /// NOTE: requires `RUST_BACKTRACE=1` for backtrace to be captured.
    pub fn collect() -> Self {
        let backtrace = Backtrace::new();
        Self {
            stack_entries: backtrace
                .frames()
                .iter()
                .map(|f| format!("{f:?}"))
                .collect(),
            detail: String::from(""),
        }
    }
}
