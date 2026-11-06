#![forbid(unsafe_code)]
//! Qobuz Connect protocol for renderers and controllers.

mod controller;
mod device;
#[cfg(feature = "discovery")]
pub mod discovery;
mod error;
mod event;
pub mod proto;
mod renderer;
mod session;
mod token;
mod transport;
pub mod wire;

pub use controller::{Autoplay, ControllerCommand};
pub use device::Device;
#[cfg(feature = "discovery")]
pub use discovery::{Discovery, Handover};
pub use error::Error;
pub use event::{Event, QueueEvent, RendererEvent, SessionState};
pub use renderer::{PlayerState, RendererCommand, RendererReport, StateChange};
pub use session::Session;
pub use token::{TOKEN_URL, TokenRequest};
pub use transport::{Credentials, Event as TransportEvent, Transport};

fn uuid() -> Vec<u8> {
    uuid::Uuid::new_v4().into_bytes().to_vec()
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
