use wayland_client::{protocol::wl_registry, Dispatch};

use crate::WaylandDisplayInner;

impl Dispatch<wl_registry::WlRegistry, ()> for WaylandDisplayInner {
    fn event(
        state: &mut Self,
        proxy: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            wl_registry::Event::Global { name, interface, version } => {
                if interface == "xdg-wm-base" && version >= 1 {
                    state.xdg_wm_base = Some(proxy.bind(name, version, qhandle, ()));
                }
            }
            _ => (),
        }
    }
}
