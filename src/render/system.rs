use std::cell::RefCell;
use std::rc::Rc;
use hal::{Adapter, Backend, Instance, Surface };
use crate::gfx::{GfxBackend, GfxBackendType, GfxDevice, GfxSwapchain, GfxSync };

pub struct RenderSystem {
    backend : GfxBackend,
    device : Option<Rc<RefCell<GfxDevice<GfxBackendType>>>>,
    sync : Option<GfxSync<GfxBackendType>>,
    swapchain : Option<GfxSwapchain<GfxBackendType>>,
}

impl Drop for RenderSystem {
    fn drop(&mut self) {
        self.swapchain.take();
        debug_assert!(self.swapchain.is_none());
        self.sync.take();
        debug_assert!(self.sync.is_none());
        self.device.take();
        debug_assert!(self.device.is_none());
    }
}

impl RenderSystem {
    pub fn new(window : &winit::Window) -> Self {
        let mut backend = GfxBackend::new(window);
        let device = Some(Rc::new(RefCell::new(GfxDevice::new(
            backend.adapters.remove(0),
            &backend.surface
        ))));

        // Initialize syncronization primitives.
        let sync = GfxSync::new(
            Rc::clone(&device.clone().unwrap()),
            2).ok();

        // Create initial swapchain for rendering.
        let swapchain = GfxSwapchain::new(
            Rc::clone(&device.clone().unwrap()),
            &mut backend.surface, 2).ok();
        Self { backend, device, sync, swapchain }
    }

    pub fn create_render_world() { unimplemented!() }
    pub fn begin_frame() { unimplemented!() }
    pub fn end_frame() { unimplemented!() }
}
