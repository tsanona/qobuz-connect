#![allow(clippy::all, clippy::pedantic, clippy::restriction)]
//! Message types generated from `proto/qcloud.proto` and `proto/qconnect.proto`.

pub mod qcloud;
pub mod qconnect {
    use crate::wire::now_ms;

    include!("qconnect.rs");

    impl Position {
        /// Creates a new `Position ` at current timestamp.
        pub fn now(value: u32) -> Self {
            Self {
                value,
                timestamp: now_ms(),
            }
        }
    }
}
