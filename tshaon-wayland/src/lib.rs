mod display;
mod handler;
mod toplevel;

pub use display::*;
pub use toplevel::*;

/// Error type for tshaon-wayland
#[derive(Debug, thiserror::Error)]
pub enum WaylandError {
    #[error("{0}")]
    Connect(#[from] wayland_client::ConnectError),
    #[error("{0}")]
    Dispatch(#[from] wayland_client::DispatchError),
}
