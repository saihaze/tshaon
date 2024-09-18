use std::{ptr::NonNull, sync::Arc};

use wayland_client::{protocol::wl_surface, Proxy};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel};

pub struct WaylandToplevel {
    inner: Arc<WaylandToplevelInner>,
}

pub(crate) struct WaylandToplevelInner {
    surface: wl_surface::WlSurface,
    xdg_surface: xdg_surface::XdgSurface,
    xdg_toplevel: xdg_toplevel::XdgToplevel,
}

impl WaylandToplevel {
    /// Maximize the window
    pub fn maximize(&self) {
        self.inner.xdg_toplevel.set_maximized();
    }

    /// Set the window size
    pub fn set_size(&self, width: u32, height: u32) {
        self.inner
            .xdg_surface
            .set_window_geometry(0, 0, width as i32, height as i32);
    }

    /// Resize the window
    pub fn set_title(&self, title: &str) {
        self.inner.xdg_toplevel.set_title(title.to_string());
    }
}

impl raw_window_handle::HasWindowHandle for WaylandToplevel {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let handle = raw_window_handle::WaylandWindowHandle::new(
            NonNull::new(self.inner.surface.id().as_ptr() as *mut _).unwrap(),
        );
        // Safety: the raw pointer is valid for the lifetime of the surface
        unsafe {
            Ok(raw_window_handle::WindowHandle::borrow_raw(
                raw_window_handle::RawWindowHandle::Wayland(handle),
            ))
        }
    }
}
