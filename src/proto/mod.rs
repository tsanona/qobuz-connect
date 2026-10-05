#![allow(clippy::all, clippy::pedantic, clippy::restriction)]
//! Message types generated from `proto/qcloud.proto` and `proto/qconnect.proto`.

pub mod qcloud {
    include!("qcloud.rs");
}

pub mod qconnect {
    include!("qconnect.rs");
    #[cfg(feature = "clap")]
    include!("qconnect.clap.rs");

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

    macro_rules! implment_otional_id {
        ($(($type:ident, $field:ident)),* $(,)?) => {
            $(
                impl $type {
                    #[doc = concat!("Returns ", stringify!($field), " as Some if positive, None otherwise.")]
                    pub fn $field(&self) -> Option<i32> {
                        (self.$field >= 0).then_some(self.$field)
                    }
                }
            )*
        };
    }

    implment_otional_id!{
        (RendererState, current_queue_item_id),
        (SrvrCtrlSessionState, active_renderer_id),
        (SrvrCtrlActiveRendererChanged, active_renderer_id)
    }
}
