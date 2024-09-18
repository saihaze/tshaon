use wayland_client::Dispatch;
use wayland_protocols::xdg::shell::client::xdg_wm_base;

use crate::WaylandDisplayInner;

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for WaylandDisplayInner {
    fn event(
        _state: &mut Self,
        proxy: &xdg_wm_base::XdgWmBase,
        event: <xdg_wm_base::XdgWmBase as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            xdg_wm_base::Event::Ping { serial } => {
                // Reply to the ping, so that the server knows we're still alive
                proxy.pong(serial);
            }
            _ => (),
        }
    }
}
