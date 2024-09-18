use std::{
    ptr::NonNull,
    sync::{Arc, Mutex},
};

use wayland_client::{protocol::wl_display, EventQueue, Proxy};
use wayland_protocols::xdg::shell::client::xdg_wm_base;

use crate::WaylandError;

/// A wayland display
pub struct WaylandDisplay {
    event_queue: Arc<Mutex<EventQueue<WaylandDisplayInner>>>,
    inner: Arc<WaylandDisplayInner>,
}

pub(crate) struct WaylandDisplayInner {
    conn: wayland_client::Connection,
    display: wl_display::WlDisplay,

    // Advertised globals
    pub(crate) xdg_wm_base: Option<xdg_wm_base::XdgWmBase>,
}

impl WaylandDisplay {
    /// Create a new WaylandDisplay
    ///
    /// It will read the `WAYLAND_DISPLAY` environment variable and connect to the corresponding display.
    pub fn new() -> Result<Self, WaylandError> {
        let conn = wayland_client::Connection::connect_to_env()?;
        let display = conn.display();
        let mut event_queue = conn.new_event_queue();

        let mut inner = WaylandDisplayInner {
            conn: conn.clone(),
            display,

            xdg_wm_base: None,
        };

        // Collect advertised globals
        event_queue.roundtrip(&mut inner)?;

        Ok(Self {
            event_queue: Arc::new(Mutex::new(event_queue)),
            inner: Arc::new(inner),
        })
    }
}

impl raw_window_handle::HasDisplayHandle for WaylandDisplay {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        let handle = raw_window_handle::WaylandDisplayHandle::new(
            NonNull::new(self.inner.display.id().as_ptr() as *mut _).unwrap(),
        );
        // Safety: the raw pointer is valid for the lifetime of the display
        unsafe {
            Ok(raw_window_handle::DisplayHandle::borrow_raw(
                raw_window_handle::RawDisplayHandle::Wayland(handle),
            ))
        }
    }
}
