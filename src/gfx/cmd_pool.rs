use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use hal::{Backend, Capability, CommandPool, Compute, Device, Graphics, QueueFamily, Transfer};
use hal::pool::{CommandPoolCreateFlags};
use crate::gfx::{GfxDevice, GfxQueue};

pub struct GfxCmdPool<B: Backend, C: Capability> {
    device : Rc<RefCell<GfxDevice<B>>>,
    cmd_pool : Option<CommandPool<B, C>>,
}


impl<B: Backend, C: Capability> Drop for GfxCmdPool<B, C> {
    fn drop(&mut self) {
        &self.device.borrow().get_device().destroy_command_pool(self.cmd_pool.take().unwrap().into_raw());
        debug_assert!(self.cmd_pool.is_none());
    }
}

impl<B: Backend, C: 'static> GfxCmdPool<B, C> where C: Capability {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>, queue : Rc<RefCell<GfxQueue<B, C>>>) -> Self {
        let cmd_pool = Some(device
            .borrow()
            .get_device()
            .create_command_pool_typed(queue.borrow().get_queue_group(),
                                       CommandPoolCreateFlags::RESET_INDIVIDUAL,
                                       num_cpus::get())
            .expect("Failed to create command pool"));
        Self { device, cmd_pool }
    }

    pub fn get_cmd_pool(&mut self) -> &mut CommandPool<B, C> {
        self.cmd_pool.as_mut().unwrap()
    }
}