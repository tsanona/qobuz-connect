#![allow(clippy::all, clippy::pedantic, clippy::restriction)]
//! Message types generated from `proto/qcloud.proto` and `proto/qconnect.proto`.

pub mod qcloud {
    include!("qcloud.rs");
    #[cfg(feature = "json")]
    include!("qcloud.serde.rs");
}

pub mod qconnect {
    include!("qconnect.rs");
    #[cfg(feature = "json")]
    include!("qconnect.serde.rs");

    impl From<std::time::Duration> for Position {
        fn from(value: std::time::Duration) -> Self {
            Self::now(value.as_millis() as u32)
        }
    }

    impl Position {
        /// Creates a new `Position ` at current timestamp.
        pub fn now(value: u32) -> Self {
            Self {
                value,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time may have gone backwards")
                    .as_millis() as u64
            }
        }
    }
}
