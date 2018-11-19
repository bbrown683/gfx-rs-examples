use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use std::sync::{Arc,Mutex};
use winit::dpi::{LogicalPosition, LogicalSize};
use super::{CmdBuffer, CmdPool, CmdState, Device, Framebuffer, Instance, RenderPass,
            Swapchain, Queue};
use crate::util::CapturedEvent;

/// The highest level of the rhi module, the `Renderer` manages all render state.
pub struct Renderer {
    instance : Option<Rc<RefCell<Instance>>>,
    device : Option<Rc<RefCell<Device>>>,
    compute_queue : Option<Rc<RefCell<Queue>>>,
    graphics_queue : Option<Rc<RefCell<Queue>>>,
    transfer_queue : Option<Rc<RefCell<Queue>>>,
    swapchain : Option<Swapchain>,
    default_render_pass : Option<RenderPass>,
    framebuffers : Option<Vec<Framebuffer>>,
    graphics_pool : Option<Rc<RefCell<CmdPool>>>,
    graphics_buffer : Option<CmdBuffer>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.graphics_buffer.take();
        debug_assert!(self.graphics_buffer.is_none());
        self.graphics_pool.take();
        debug_assert!(self.graphics_pool.is_none());
        self.framebuffers.take();
        debug_assert!(self.framebuffers.is_none());
        self.default_render_pass.take();
        debug_assert!(self.default_render_pass.is_none());
        self.swapchain.take();
        debug_assert!(self.swapchain.is_none());
        self.compute_queue.take();
        debug_assert!(self.compute_queue.is_none());
        self.graphics_queue.take();
        debug_assert!(self.graphics_queue.is_none());
        self.transfer_queue.take();
        debug_assert!(self.transfer_queue.is_none());
        self.device.take();
        debug_assert!(self.device.is_none());
        self.instance.take();
        debug_assert!(self.instance.is_none());
        info!("Dropped Renderer.");
    }
}

impl CapturedEvent for Renderer {
    /// When this event is captured, the swapchain is recreated, and regenerates all framebuffers from the swapchain images.
    fn on_resize(&mut self, _size : LogicalSize) {
        self.swapchain.as_mut().unwrap().recreate();
        self.framebuffers.as_mut().unwrap().clear();
        for image in self.swapchain.as_ref().unwrap().get_images() {
            self.framebuffers.as_mut().unwrap().push(Framebuffer::new(
                Rc::clone(&self.device.clone().unwrap()),
                &self.default_render_pass.as_ref().unwrap(),
                image,
                self.swapchain.as_ref().unwrap().get_capabilities().current_extent));
        }
    }
}

impl Renderer {
    /// Initializes the renderer for the specified window.
    pub fn new(window : &winit::Window) -> Self {
        info!("Initializing Renderer.");
        // TODO: Properly handle errors here and present them to the output.

        let instance = Rc::new(RefCell::new(Instance::new()));
        let device = Rc::new(RefCell::new(Device::new(&instance.borrow())
            .ok()
            .unwrap()));

        // Create our queues.
        let compute_queue = Rc::new(RefCell::new(Queue::new(
            Rc::clone(&device),
            device.borrow().get_compute_queue_index())));
        let graphics_queue = Rc::new(RefCell::new(Queue::new(
            Rc::clone(&device),
            device.borrow().get_graphics_queue_index())));
        let transfer_queue = Rc::new(RefCell::new(Queue::new(
            Rc::clone(&device),
            device.borrow().get_transfer_queue_index())));

        // Create the swapchain.
        let swapchain = Swapchain::new(
            Rc::clone(&instance),
            Rc::clone(&device),
            Rc::clone(&graphics_queue),
            window,
            2).ok()
            .unwrap();

        let default_render_pass = RenderPass::new(
            Rc::clone(&device));

        // Grab the swapchain images to create the framebuffers.
        let mut framebuffers = Vec::<Framebuffer>::new();
        for image in swapchain.get_images() {
            framebuffers.push(Framebuffer::new(
                Rc::clone(&device),
                &default_render_pass,
                image,
                swapchain.get_capabilities().current_extent
            ));
        }

        let graphics_pool = Rc::new(RefCell::new(CmdPool::new(
            Rc::clone(&device),
            &graphics_queue.borrow())));

        let graphics_buffer = CmdBuffer::new(
            Rc::clone(&device),
            Rc::clone(&graphics_pool));

        info!("Renderer has been initialized.");
        Self {
            instance: Some(instance),
            device: Some(device),
            compute_queue: Some(compute_queue),
            graphics_queue: Some(graphics_queue),
            transfer_queue: Some(transfer_queue),
            swapchain: Some(swapchain),
            default_render_pass: Some(default_render_pass),
            framebuffers: Some(framebuffers),
            graphics_pool: Some(graphics_pool),
            graphics_buffer: Some(graphics_buffer),
        }
    }

    pub fn begin_frame(&mut self) {
        let next_image = &self.swapchain.as_mut().unwrap().get_next_image();
        let cmd_state = CmdState {
            format: self.swapchain.as_ref().unwrap().get_surface_format().format,
            extent: self.swapchain.as_ref().unwrap().get_capabilities().current_extent
        };

        self.graphics_buffer
            .as_mut()
            .unwrap()
            .record_graphics(
                cmd_state,
                self.default_render_pass.as_ref().unwrap(),
                self.framebuffers.as_ref().unwrap().get(next_image.clone() as usize).unwrap());
    }

    pub fn end_frame(&self) {
        self.graphics_queue
            .as_ref()
            .unwrap()
            .borrow()
            .submit(self.graphics_buffer.as_ref().unwrap());
        self.swapchain.as_ref().unwrap().present();
    }
}
