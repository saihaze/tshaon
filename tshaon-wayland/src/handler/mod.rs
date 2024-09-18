mod registry;
mod xdg;

use wayland_client::{delegate_noop, protocol::wl_display};

use crate::WaylandDisplayInner;

delegate_noop!(WaylandDisplayInner: wl_display::WlDisplay);
